---
audience: design
type: spec
updated: 2026-04-18
---

# The Behavior Contract

<!-- @c:glossary#Polyglot Service -->
<!-- @c:glossary#Trigger Monitor -->
<!-- @c:glossary#Queue Handler -->
<!-- @c:glossary#Runner -->
<!-- @c:glossary#Pipeline -->
<!-- @c:glossary#Job -->
<!-- @c:glossary#Instance -->
<!-- @u:philosophy/behavioral-contract -->
<!-- @u:concepts/pipelines/INDEX#Pipeline, Instance, and Job -->
<!-- @c:technical/spec/otel-permission-events -->
<!-- @c:technical/spec/otel-foundation -->
<!-- @c:technical/spec/otel-config -->
<!-- @c:technical/spec/polyglot-sdk -->
Related: [[compiler-floor]], [[native-dispatch]], [[otel-permission-events]], [[otel-foundation]], [[otel-config]], [[polyglot-sdk]]

## What Polyglot Compiles To

<!-- @c:vision#Core Philosophy -->
Polyglot does not compile to a binary. It compiles to a **Behavior Contract** — a serialized signal logic map that the [[glossary#Trigger Monitor|c:Trigger Monitor]] reads and orchestrates.

The Behavior Contract encodes:

- **Signal routing:** When jobA finishes, send trigger signal to jobB
- **Parallelism:** Send "go" signal to jobC and jobD simultaneously
- **Conditional branching:** Only the branch satisfying the condition receives the "go" signal — other branches never fire
- **Error routing:** Same mechanism as conditionals, but the condition is the error state of the preceding job
- **Dispatch rules:** Concurrency, queue strategy, wrapper lifecycle

This is not a sequential step list. It is a signal map that the Trigger Monitor interprets in an async environment. Every action ([[glossary#Job|c:job]]) has triggers and IO.

### Compilation Pipeline

```text
Polyglot Code (.pg)
    → PGCompiler (validates signal logic — buggy concurrency = compile error)
        → Behavior Contract (serialized signal map, one per pipeline)
            → stored in NoSQL DB (registration)
                → user enables pipeline (flag in DB)
                    → Trigger Monitor reads contract, orchestrates execution
```

The Behavior Contract is transmitted via NATS messaging in TOON format (chosen for character efficiency — theoretically any serialized format could work).

## Contract Structure

The contract is organized into five top-level sections:

1. **Triggers** — listener configuration for the Trigger Monitor
2. **Queue** — dispatch rules and active queue commands
3. **Wrappers** — resource lifecycle configuration
4. **Execution** — the async function jobs and flow control
5. **Permissions** — resolved `{_}` grants and sandbox configuration
6. **Type Mapping Descriptors** — per-IO-port type mappings for SDK serialization

Within each section, the `[X]` markers from Polyglot Code inform the kind of action. The compiler transforms these into the serialized signal map — encoding when to send trigger signals between jobs, when to fan out in parallel, which branches receive "go" signals based on conditions or error states.

## Permission Manifest

<!-- @c:concepts/permissions/enforcement#Foreign Code Sandbox -->
<!-- @c:technical/spec/job-sandbox -->

The compiler emits a **Permission Manifest** as part of the Behavior Contract. It contains the resolved `{_}` grants for each pipeline — the concrete permission declarations after template resolution and ceiling validation.

### Manifest Flow

```text
Compiler validates {_} grants
    → resolves templates ((_) inputs substituted)
    → validates against {@} ceiling
    → emits Permission Manifest per pipeline
        → stored alongside signal map in NoSQL DB
            → Runner reads manifest before spawning job
                → translates to OS-level sandbox configuration
```

### Manifest Contents

The Permission Manifest for each pipeline contains:

| Content | Source | Purpose |
|---|---|---|
| Resolved `{_}` grants | Template resolution + ceiling validation | Per-pipeline list of permissions (category, capability, scope/path/host/port) |
| File content hashes | Compile-Time File Binding | SHA-256 hashes of referenced files — invalidated if files change |
| `_Unsafe.SandboxOnly` flag | `[!] _Unsafe.SandboxOnly` in pipeline | Signals Runner to activate all isolation layers |
| Accountability metadata | `%Authors`, `%Description`, `%Version` | Required when `_Unsafe.SandboxOnly` is active (PGE10016) |
| Compliance report | AST analysis results | Best-effort analysis findings, warnings, and verdicts |

The manifest is **read-only at runtime** — the Runner uses it to configure the sandbox but never modifies it. Changes to permissions require recompilation and re-registration.

The Permission Manifest drives both sandbox configuration and OTel event emission. At runtime, violations of the declared permissions are logged as structured OTel events per [[otel-permission-events]]. This creates a closed loop: the compiler validates permissions, the manifest configures the sandbox, and the OTel events record what the sandbox enforced.

See [[job-sandbox]] for how the Runner translates the Permission Manifest into OS-level restrictions (Landlock, seccomp-bpf, Linux namespaces, cgroups v2).

## Type Mapping Descriptors

<!-- @c:technical/spec/polyglot-sdk#Type Mapping Descriptors -->

The compiler emits **type mapping descriptors** alongside the signal map and permission manifest. These descriptors specify the Polyglot type for each IO port in every pipeline, enabling the Runner and [[polyglot-sdk|c:Polyglot SDK]] to serialize and deserialize values without runtime type inspection.

The descriptors are derived from the native registry's IO schema (see [[native-dispatch#Registry Entry Schema]]). For each IO port, the descriptor records the port name, direction, Polyglot type, and whether it carries an array.

See [[polyglot-sdk#Type Mapping Descriptors]] for the descriptor schema and examples.

## Compile-Time Signal Map Validation

<!-- @u:technical/compile-rules/PGE/PGE03001-no-push-across-parallel -->
<!-- @u:technical/compile-rules/PGE/PGE03002-parallel-output-must-be-collected -->
<!-- @u:technical/compile-rules/PGE/PGE09013-circular-pipeline-call -->
<!-- @u:technical/compile-rules/PGE/PGE10008-parallel-write-permission-exclusion -->
The compiler validates the signal map before the Behavior Contract is produced. Twenty-eight error rules and two supporting algorithms ensure the signal logic is sound.

### Race Conditions (eliminated by construction)

- **PGE03001** — No push across parallel boundaries (two concurrent writers)
- **PGE10008** — Parallel jobs cannot both hold write permission to the same resource (glob intersection)
- **PGE03003** — Cannot read a parallel variable before its collector runs (pull isolation)
- **PGE03012** — Parallel labels isolated until collection

### Orphaned Resources and Leaked Jobs

- **PGE03002** — Every parallel output must be collected
- **PGE03025** — Every collector code path must release all jobs [0,N] (no orphans for Trigger Monitor)
- **PGE03024** — Cannot double-release a job
- **PGE02008** — Cannot access a variable after scope closes (Released state)

### Cycle and Infinite Loop Detection

- **PGE09013** — Circular [[glossary#Pipeline|c:pipeline]] calls detected via DFS on call graph (no recursion in Polyglot)
- **PGE05004** — Recursive data definitions without indirection

### Dead-End and Unreachable Path Detection

- **PGE02009** — Code after all output ports reach Final is unreachable (signal paths terminate)
- **PGE06012** — Branches after wildcard `*?` can never fire

### Scope Boundary Violations

- **PGE03004** — Parallel/collector pairs must respect `[\]`/`[/]` section boundaries
- **PGE03021** — No parallel execution inside collectors (strictly sequential)
- **PGE03022** — Collectors triggered only by arrivals, not external events

If the signal logic leads to buggy concurrency — race conditions, orphaned jobs, cycles, dead-ends — it is a compile error. The Behavior Contract is only produced if the signal map is proven sound.

## Service Execution

<!-- @c:glossary#Polyglot Service -->
All components of the [[glossary#Polyglot Service|c:Polyglot Service]] communicate via NATS as a pub-sub hub. NATS messages carry both signal information and output data.

### Trigger Monitor — The Decision-Maker

The central orchestrator. Everything flows through it:

1. Holds all trigger listeners (webhook, daily, call, etc.) — implemented internally, activated per pipeline
2. Reads the signal map from the Behavior Contract
3. Evaluates conditions to decide which branches receive "go" signals
4. Manages the callable pipeline registry per package
5. Single source of truth for "what runs next"

Multiple pipelines can share triggers (e.g., two pipelines both using `-T.Webhook` on the same endpoint). The Trigger Monitor manages listener sharing.

The Trigger Monitor is currently a single instance. Redundancy and clustering are future considerations.

### Queue Handler and Runner

The [[glossary#Queue Handler|c:Queue Handler]] manages dispatch ordering, concurrency, pause/resume. Most queue rules are instructions the Queue Handler follows from the contract; active commands (pause, resume) invoke async functions.

The [[glossary#Runner|c:Runner]] executes async functions when the Trigger Monitor triggers them via NATS. It manages wrapper lifecycle and has built-in behavior for `-W.Polyglot`.

### Communication Flow

```text
Trigger Monitor reads signal map
    → evaluates trigger conditions
        → sends "go" signal via NATS
            → Queue Handler manages dispatch
                → Runner executes async function
                    → publishes completion signal + output data via NATS
                        → Trigger Monitor receives, evaluates, sends next signal
```

### Cross-Package Pipeline Calls

When pipeline A in package X calls pipeline B in package Y:

1. Trigger Monitor checks both package registries
2. If Y is imported via `[@]`, the imported registry gets priority
3. If not found in either registry, it is an error

Pipeline definition lookup is a microservice action.

## Registration and Storage

| What | Where | When |
|---|---|---|
| Behavior Contract | NoSQL DB (one entry per pipeline) | Compile time (registration) |
| Data definitions (`{#}`) | Pipeline [[glossary#Instance|c:instance]] path in % metadata tree | Compile time |
| Data instances | Runtime (in % tree) | Populated as data flows |
| Non-callable pipelines | Registered with required inputs (not supplied by triggers) | Registration |
| Callable pipelines | Callable pipeline registry per package | Registration |
| Collectors (`{*}`) | Special callable pipelines | Registration |
| Permission Manifest | Part of Behavior Contract in NoSQL DB | Compile time (per pipeline) |
| Enable flag | NoSQL DB | User sets; Trigger Monitor checks before firing |

### Re-registration Rules

- **Same version + same package** — overwrites existing record (update-in-place)
- **Different version or package** — creates a new record
- **Recompiled code** must be re-registered to update the NoSQL DB, then re-enabled
- **Multi-version** — not enabled by default; only one version active at a time. Future: `-Q.Allow.Multi.Version` queue strategy to enable multiple versions simultaneously

### Collectors as Special Callable Pipelines

<!-- @c:glossary#Reconciliation -->
Collectors differ from regular callable pipelines:

- **Trigger source:** Not called explicitly — triggered by invocation of parallel branches
- **Multiple triggers:** Receives trigger signals as each parallel branch output arrives
- **Early action:** Can act before all outputs arrive (e.g., `*First` acts on first arrival)
- **No parallel jobs:** Parallel execution strictly forbidden inside collectors (PGE03021)

## How User Code Relates to the Floor

```text
User writes:           Polyglot Code (.pg files)
                           ↓ compiles to
Compiler produces:     Behavior Contract (signal map: instructions + async function refs)
                           ↓ validated by
Compile rules:         28 error rules ensure no races, orphans, cycles, or dead-ends
                           ↓ stored in
Persistence:           NoSQL DB (one contract per pipeline, definitions in % tree)
                           ↓ interpreted by
Service executes:      Trigger Monitor reads signal map, orchestrates via NATS
```

Polyglot Code is a high-level shield over the Behavior Contract. The `[X]` markers in `.pg` code inform the kind of action; the compiler transforms them into serialized signal logic that the Trigger Monitor interprets in an async environment.

## Future Work

- **Signal and communication planning** — formal specification of NATS subject naming, message schemas, and signal types between Trigger Monitor, Queue Handler, and Runner
- **Trigger Monitor redundancy** — currently single instance; clustering is a future consideration
- **`-Q.Allow.Multi.Version`** — queue strategy to enable multiple pipeline versions simultaneously (canary, blue-green, gradual migration as configuration of the same mechanism)
