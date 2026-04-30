---
audience: design
type: spec
updated: 2026-04-23
---

# Aljam3 Architecture

<!-- @c:vision -->
<!-- @c:philosophy/core-philosophy -->
<!-- @c:philosophy/behavioral-contract -->
<!-- @c:audit/reference/glossary -->
This document is the consolidated architecture read for language designers and architects. It describes the four layers of Aljam3 — language, compiler, service, runtime — and how they connect. Each section cites the authoritative spec file for its layer; this document sequences those specs into one design-shaped read and does not duplicate their prose. Authority chain: [[vision|c:vision]] > [[philosophy/core-philosophy|c:core-philosophy]] > [[philosophy/behavioral-contract|c:behavioral-contract]] > [[audit/README|c:audit/README]]. All terminology matches [[audit/reference/glossary|c:glossary]] exactly.

## Overview

<!-- @c:vision:The Aljam3 Ecosystem -->
<!-- @c:philosophy/behavioral-contract -->
Aljam3 is a trigger-driven programming language whose compiler emits a Behavior Contract — a signal-graph IR — that the Aljam3 Service executes at runtime. Four layers:

| Layer | Produces | Consumes | Authoritative spec |
|-------|----------|----------|--------------------|
| Language | [[audit/reference/glossary\|c:Aljam3 Code]] (`.aj3` files) | EBNF grammar | [[technical/ebnf/INDEX\|u:ebnf-index]] |
| Compiler | [[audit/reference/glossary\|c:Behavior Contract]] | Aljam3 Code | [[technical/spec/compiler-floor\|u:compiler-floor]], [[technical/spec/behavior-contract\|u:behavior-contract]] |
| Service | Signals, Jobs, Variable state | Behavior Contract | [[technical/spec/compiler-floor\|u:compiler-floor]] |
| Runtime execution | Results (outputs, errors) | Jobs and foreign-code blocks | [[technical/spec/aljam3-sdk\|u:aljam3-sdk]], [[technical/spec/native-dispatch\|u:native-dispatch]] |

The boundaries between these layers are enforced by compile rules and runtime contracts; see "Authority Chain and Failure Modes" below for every boundary violation.

## Language Layer

<!-- @c:philosophy/core-philosophy -->
<!-- @c:philosophy/data-trees -->
<!-- @u:user/SPEC-INDEX -->
The language layer defines what `.aj3` source code looks like and what compile-time invariants the compiler enforces. It is fully specified — v0.2 is complete.

### Grammar

| Aspect | Doc |
|--------|-----|
| Formal EBNF grammar (16 sections) | [[technical/ebnf/INDEX\|u:ebnf-index]] |
| Lexical rules | [[technical/ebnf/02-lexical\|u:ebnf-02]] |
| Identifier prefixes | [[technical/ebnf/03-identifiers\|u:ebnf-03]] |
| Type system | [[technical/ebnf/04-type-system\|u:ebnf-04]] |
| Definition blocks | [[technical/ebnf/09-definition-blocks\|u:ebnf-09]] |
| Collector-definition grammar | [[technical/ebnf/16-collector-definitions\|u:ebnf-16]] |
| Edge-case enumeration | [[technical/edge-cases/INDEX\|u:edge-cases-index]] |

The grammar uses three bracket families — `{X}` for definition blocks, `[X]` for block elements, `(X)` for IO markers — codified in [[user/syntax/blocks|u:blocks]]. The identifier-prefix system uses seven prefixes: `@`, `#`, `=`, `-`, `$`, `!`, `%`, plus the `_` permission prefix. Per [[audit/reference/glossary|c:glossary]], Aljam3 Code never contains raw arithmetic operators; arithmetic routes through `-Math.*` pipelines, enforced by PGE04010.

### Type System

| Aspect | Doc |
|--------|-----|
| Prefix tiers (`#` alias / `##` schema / `###` field type) | [[user/syntax/types/prefix-system\|u:prefix-system]] |
| Schema properties | [[user/syntax/types/schema-properties\|u:schema-properties]] |
| Generic types (schema inputs) | [[user/syntax/types/generic-types\|u:generic-types]] |
| Flexible fields | [[user/syntax/types/flexible-fields\|u:flexible-fields]] |
| Structural type identity (spec-level) | [[technical/spec/type-identity\|u:type-identity]] |

Types are data-tree descriptors. Every data value in Aljam3 is a tree (per [[philosophy/data-trees|c:data-trees]]) keyed by a type hierarchy; `##` schemas define structural constraints; `#` aliases bind a name to a schema instance; `###` field types annotate individual fields. The compiler validates structural identity using the rules in [[technical/spec/type-identity|u:type-identity]].

### Pipelines

<!-- @u:user/concepts/pipelines/INDEX -->
A [[audit/reference/glossary|c:Pipeline]] is the executable unit of Aljam3. Every pipeline has:

| Required | Optional |
|----------|----------|
| Trigger (`[T]`) or implicit IO trigger | Environment wiring (`;EnvName`) |
| Queue (`[Q]`) | Wrapper override (`[W]`) |
| Wrapper (`[W]` defaults to `-W.Aljam3`) | Permission grants (`{_}` + `[_]` IO markers) |
| Execution body (`[\]` setup + body + `[/]` cleanup) | Foreign-code blocks (`[C]`) |

Mandatory structure is enforced by PGE01005 (missing trigger), PGE01006 (missing queue), PGE01007 (missing setup/cleanup). See [[user/concepts/pipelines/INDEX|u:pipelines-index]] for the concept-level overview.

### Collections and Parallelism

<!-- @u:user/concepts/collections/INDEX -->
Aljam3 expresses parallelism via expanders (`=ForEach.*`) and collectors (`*Agg.*`, `*Into.*`, `*Sync.*`, `*All`, `*First`, `*Nth`). An expander fans jobs out; a collector fans results in. The paired expand/collect scope is enforced by PGE03025. Canonical reads:

| Concept | File |
|---------|------|
| Expand operators | [[user/concepts/collections/expand\|u:expand]] |
| Collect operators | [[user/concepts/collections/collect\|u:collect]] |
| Reassemble | [[user/concepts/collections/reassemble\|u:reassemble]] |
| Collector definitions (`{*}` grammar) | [[technical/spec/collector-definitions\|u:collector-definitions]] |

## Compiler Layer

<!-- @c:philosophy/behavioral-contract -->
<!-- @u:technical/spec/compiler-floor -->
<!-- @u:technical/spec/behavior-contract -->
The compiler translates Aljam3 Code into a [[audit/reference/glossary|c:Behavior Contract]] — a signal-graph IR consumed by the Aljam3 Service. The boundary between compile-time and run-time is the "compiler floor" defined in [[technical/spec/compiler-floor|u:compiler-floor]].

### Compiler Floor

Authority: [[technical/spec/compiler-floor|u:compiler-floor]].

| Concern | Above the floor (compile-time) | Below the floor (run-time) |
|---------|---------------------------------|---------------------------|
| Syntax validation | Yes | No |
| Type checking | Yes | No |
| Permission grant resolution | Partial (static permission ceiling) | Dynamic permission check per IO |
| Signal-graph construction | Yes | No |
| Cycle detection | Yes ([[technical/compile-rules/algorithms/cycle-detection\|u:cycle-detection]]) | No |
| Conditional exhaustiveness | Yes ([[technical/compile-rules/algorithms/compound-exhaustiveness\|u:compound-exhaustiveness]]) | No |
| Overlap detection | Yes ([[technical/compile-rules/algorithms/overlap-detection\|u:overlap-detection]]) | No |
| Job lifecycle | No | Yes |
| Collector reconciliation | No | Yes |
| Error propagation | Static schemas | Runtime signal propagation |

### Behavior Contract

Authority: [[technical/spec/behavior-contract|u:behavior-contract]].

The Behavior Contract is the signal-graph IR produced by the compiler. It encodes:

| Contract element | Contents |
|------------------|----------|
| Pipeline descriptors | Queue strategy, wrapper binding, permission ceiling, IO schema |
| Job descriptors | Dispatch constraints, retry policy, collector bindings, trigger signals |
| Signal graph | Trigger → Queue → Dispatch → Body → Collector edges |
| Variable lifecycle map | Default → Final → Failed → Released transitions |
| Collector reconciliation map | Which collectors watch which jobs; compound exhaustiveness claims |
| Permission index | Grant + locator for every IO marker |

The Aljam3 Service reads this contract; it does not re-parse `.aj3` source. See [[philosophy/behavioral-contract|c:behavioral-contract]] for why the contract is the boundary.

### Compile-Rule Catalog

| Catalog | Doc |
|---------|-----|
| Error / warning lookup tables | [[technical/COMPILE-RULES\|u:COMPILE-RULES]] |
| Per-rule files (188 PGE + 31 PGW) | `docs/technical/compile-rules/PGE/`, `docs/technical/compile-rules/PGW/` |
| Compiler algorithms (cycle, overlap, exhaustiveness) | [[technical/compile-rules/algorithms/INDEX\|u:algorithms-index]] |

Rule numbering is range-partitioned; see [[ai-retrieval-index|u:ai-retrieval-index]] for the full range-to-topic table.

### Compiler Modules

Authority: [[technical/compiler/INDEX|u:compiler-index]].

| Module | File |
|--------|------|
| AST-invisible registry (banned functions) | [[technical/compiler/ast-invisible-registry\|u:ast-invisible-registry]] |
| IO-sink registry (AST analysis) | [[technical/compiler/io-registry\|u:io-registry]] |
| Foreign-code parsers (per-language) | [[technical/compiler/foreign-code-parsers\|u:foreign-code-parsers]] |
| Compliance-report format | [[technical/compiler/compliance-report\|u:compliance-report]] |

Standalone compiler algorithms (outside the compile-rules tree):

| Algorithm | File |
|-----------|------|
| `-Run.Bridge` conversion | [[technical/algorithms/bridge-conversion\|u:bridge-conversion]] |
| Foreign-code AST analysis | [[technical/algorithms/foreign-code-analysis\|u:foreign-code-analysis]] |

## Service Layer

<!-- @c:vision:Trigger-Driven Orchestration -->
The [[audit/reference/glossary|c:Aljam3 Service]] executes a Behavior Contract. It has four primary components: Trigger Monitor, Queue Handler, Dispatch Coordinator, and Runner.

### Trigger Monitor

<!-- @u:audit/reference/glossary -->
The [[audit/reference/glossary|c:Trigger Monitor]] (TM) watches trigger conditions and emits signals when they fire. Responsibilities:

| Concern | Responsibility |
|---------|----------------|
| Trigger evaluation | Monitor conditions (IO, time, queue state, git events) |
| Signal emission | Emit trigger signals onto the NATS signal bus |
| Collector ownership | Own `*First` / `*Nth` / `*All` logic for sub-jobs; terminate associated jobs on race resolution |
| Job FSM validation | Validate job state transitions before dispatching commands to the Queue Handler |

Trigger conditions, not business logic, are the TM's decision scope. `-T.*` pipelines are the declarative contracts the TM evaluates; see [[user/aj3lib/pipelines/T/INDEX|u:aj3lib-T]] for the trigger catalog.

### Queue Handler

The [[audit/reference/glossary|c:Queue Handler]] (QH) enforces queue strategies and admits [[audit/reference/glossary|c:Job]]s for dispatch. Responsibilities:

| Concern | Responsibility |
|---------|----------------|
| Queue admission | Apply queue strategy (FIFO, LIFO, Drop, Debounce, Throttle, etc.) |
| Job accounting | Track per-queue job counts, paused/killed flags |
| Trigger/business-logic decisions | None — the QH never makes business decisions |
| Dispatch signalling | Signal the Dispatch Coordinator when a job is eligible |

Queue strategies and control pipelines live under `-Q.*`; see [[user/aj3lib/pipelines/Q/INDEX|u:aj3lib-Q]]. The queue-vs-set distinction and pause/kill flows are specified by queue-manager design decisions under [[audit/decisions/README|c:decisions-index]].

### Dispatch Coordinator

The [[audit/reference/glossary|c:Dispatch Coordinator]] (DC) chooses which Runner executes an admitted Job. Responsibilities:

| Concern | Responsibility |
|---------|----------------|
| Runner selection | Host-based load balancing (two-tier round-robin) |
| Resource accounting | Track per-Runner resource usage |
| Locality | Prefer Runners with warm environments for `-Run.*` jobs |

### Runner

The [[audit/reference/glossary|c:Runner]] is the process that executes a Job's body. Responsibilities:

| Concern | Responsibility |
|---------|----------------|
| Environment setup | Acquire `{;}` environment resources; invoke `[\]` setup |
| Job body execution | Execute `[-]` calls, `[?]` conditionals, `[!]` error scopes |
| Cleanup | Invoke `[/]` cleanup on completion, failure, or cancellation |
| Foreign-code invocation | Dispatch `[C]` blocks through `-RT.*` or `-Run.Bridge` |
| Reporting | Emit lifecycle signals back to the Trigger Monitor and collector bindings |

### Control Plane

Runtime data is Redis-only (per the 2026-04-02 design decision, currently documented in decision records). NATS carries the signal bus. Signal channels are namespaced per job UID; see `-T.Call` signal-path changes in [[audit/decisions/README|c:decisions-index]].

### Job Sandbox

Authority: [[technical/spec/job-sandbox|u:job-sandbox]].

OS-level sandboxing applies to system-level jobs. It uses Linux primitives (Landlock, seccomp, namespaces, cgroups v2). Six resource categories are enforced:

| Resource category | `{_}` permission |
|-------------------|------------------|
| RAM | `#RAMCapability` |
| CPU | `#CPUCapability` |
| GPU | `#GPUCapability` |
| IO | `#IOCapability` |
| Processes | `#ProcessCapability` |
| Duration | `#DurationCapability` |

The sandbox spec defines `#LimitAction` semantics, cgroups mapping, and queue-default limits. Permission-related compile rules live in the PGE10xxx range (PGE10001 through PGE10016 plus PGW10001-07).

## Runtime Execution Layer

<!-- @u:user/aj3lib/pipelines/W/INDEX -->
<!-- @u:user/aj3lib/pipelines/Run/INDEX -->
<!-- @u:user/aj3lib/pipelines/RT/INDEX -->
Execution is delegated via wrappers (`-W.*`) and runtime pipelines (`-RT.*`, `-Run.*`). Wrappers and runtimes form the boundary between Aljam3-controlled execution and foreign-language execution.

### Wrappers (`-W.*`)

Authority: [[user/aj3lib/pipelines/W/INDEX|u:aj3lib-W]]. A wrapper defines the setup/cleanup contract around a pipeline body. `-W.Aljam3` is the default (no-op setup/cleanup). Specialised wrappers:

| Wrapper | Purpose |
|---------|---------|
| `-W.Aljam3` | Default — no setup/cleanup |
| `-W.RT` | Provision a foreign runtime (Python, Rust, Go, JS) for `-RT.*` calls |
| `-W.Env` | Wire a `{;}` environment block into the body |
| `-W.Retry` | Drive queue-level retry semantics (retry strategy lives in `[Q]`, not `[W]`) |

Retry is a queue concern, not a wrapper concern — see `pg_lesson_retry_is_queue` in the project-level conventions.

### Runtime Pipelines (`-RT.*`)

Authority: [[user/aj3lib/pipelines/RT/INDEX|u:aj3lib-RT]]. Runtime pipelines invoke foreign code across four languages and two modes:

| Mode variant | Meaning |
|--------------|---------|
| `.Inline` | Foreign code authored inline in a `[C]` block |
| `.File` | Foreign code loaded from a script file |

Each runtime splits further by binding origin:

| Origin | Meaning |
|--------|---------|
| Script | Aljam3 injects `<Bind` / `>Bind` names into the foreign environment |
| Bind | Foreign code pulls/pushes via the Aljam3 SDK |
| CLI | Compiled binary execution; uses `-W.Aljam3`, not `-W.RT` |

`-RT.*` error namespaces appear under `!RT.*`. See [[user/aj3lib/errors/pipeline-associations|u:pipeline-associations]] for namespace-to-pipeline bindings.

### Run Pipelines (`-Run.*`)

Authority: [[user/aj3lib/pipelines/Run/INDEX|u:aj3lib-Run]]. `-Run.*` provides script, binary, Shell, and Bridge execution outside the `-RT.*` runtime model:

| Pipeline | Purpose |
|----------|---------|
| `-Run.Shell` | Language-agnostic shell command execution |
| `-Run.Bridge` | Pairwise cross-language function binding |
| `-Run.Binary` | Compiled-binary execution |
| `-Run.*` (script variants) | Per-language script execution with `-Run.*` input binding |

`-Run.Shell` uses the `System.Shell` capability; its process lifecycle is tracked in Redis under `job:{UID}:process` and signalled through QH signals. `-Run.Bridge` is the pairwise FFI mechanism (supersedes the SDK's universal-string-only path).

## SDK and Cross-Language Integration

<!-- @c:vision:Cross-Language Integration -->
<!-- @u:technical/spec/aljam3-sdk -->
Authority: [[technical/spec/aljam3-sdk|u:aljam3-sdk]].

The [[audit/reference/glossary|c:Aljam3]] SDK provides encode / decode / call / pull / push primitives in each supported foreign language. It is the contract-defined surface for foreign code to interact with a Aljam3 Job.

### SDK Primitives

| Primitive | Purpose |
|-----------|---------|
| `encode(value)` | Serialise a native value to the Aljam3 universal-string wire format |
| `decode(wire, type)` | Deserialise a wire value into a native value |
| `call(pipeline, inputs)` | Invoke a Aljam3 pipeline from foreign code |
| `pull(name)` | Pull a bound input variable from Aljam3 |
| `push(name, value)` | Push a bound output variable back to Aljam3 |

### Universal String Wire Format

Authority: [[technical/spec/aljam3-sdk|u:aljam3-sdk]]. The wire format is a typed universal-string protocol. Types carried include: `#bytes` (Base64), `#dt` (epoch seconds), primitives, records, collections. Per-language encode/decode normalisation rules cover Boolean, Float, and Null handling with dependencies on `##Inf` and `##Nullable` schemas.

The SDK does not expose FFI. FFI (dynamic code generation) is deferred to `-Run.Bridge`; the SDK is universal-string-only.

### `-Run.Bridge`

Authority: [[technical/algorithms/bridge-conversion|u:bridge-conversion]] + [[audit/decisions/README|c:decisions-index]].

`-Run.Bridge` provides pairwise cross-language binding using a dual-wrapper strategy and per-language `#NativeType` + `#Variable` descriptors. Extensible language pairs are registered via the `-Variable.Convert` pipeline.

### Foreign-Code Compliance

Authority: [[technical/algorithms/foreign-code-analysis|u:foreign-code-analysis]].

All `[C]` foreign-code blocks undergo AST analysis before execution for permission-compliance. Banned functions (eval, exec, dynamic-import primitives) are enforced via PGE10014 against the [[technical/compiler/ast-invisible-registry|u:ast-invisible-registry]]. IO sinks are resolved against [[technical/compiler/io-registry|u:io-registry]]; per-language parsers are defined in [[technical/compiler/foreign-code-parsers|u:foreign-code-parsers]].

| Compile rule | Enforces |
|--------------|----------|
| PGE10011 | Permission-locator mismatch in `[C]` |
| PGE10012 | Foreign-code IO escapes declared grant |
| PGE10013 | Unsupported foreign-code construct |
| PGE10014 | AST-invisible / banned function reference |
| PGW10002 | Conservative foreign-code analysis warning |

## Observability

<!-- @u:technical/spec/otel-foundation -->
<!-- @u:technical/spec/otel-permission-events -->
Authority: [[technical/spec/otel-foundation|u:otel-foundation]].

Aljam3 service components emit OpenTelemetry traces, logs, and metrics. The OTel foundation defines three specs:

| Spec | Purpose |
|------|---------|
| [[technical/spec/otel-foundation\|u:otel-foundation]] | Tracing infrastructure, span hierarchy, attributes |
| [[technical/spec/otel-permission-events\|u:otel-permission-events]] | Permission / sandbox events (8 log events, 9 `aljam3.*` attributes) |
| [[technical/spec/otel-config\|u:otel-config]] | Exporter configuration |

Span hierarchy mirrors the service-component structure: Trigger → Queue admission → Dispatch → Runner Job → Collector reconciliation. Permission events are emitted at `{_}` grant acquisition and at every permission-enforced IO.

## Native Dispatch

<!-- @u:technical/spec/native-dispatch -->
Authority: [[technical/spec/native-dispatch|u:native-dispatch]]. The native-dispatch spec defines how `{N}` native blocks are resolved at compile time. Two native kinds exist:

| Kind | Purpose |
|------|---------|
| Instructions | Configure the Trigger Monitor or Queue Handler at compile time |
| Async functions | Emit jobs triggered by the TM at runtime |

The `#BaseCode` enum and `#NativeKind` registry enumerate supported native implementations. PGE01028 enforces that every `{N}` definition has a valid `.baseCode` field.

## Authority Chain and Failure Modes

<!-- @c:audit/README -->
Authority flows top-down: [[vision|c:vision]] → [[philosophy/core-philosophy|c:core-philosophy]] → [[audit/README|c:audit/README]] → per-spec docs → per-rule files. Contradictions resolve in authority order. Each boundary below has a named failure mode.

| Boundary | Failure mode | Rule |
|----------|--------------|------|
| Pipeline structure | Missing trigger, queue, or setup/cleanup | PGE01005, PGE01006, PGE01007 |
| IO ordering | IO not before `[T]` | PGE01002 |
| Type identity | Structural mismatch | PGE04xxx |
| Variable lifecycle | Write after Final | PGE02003 |
| Self-chain addressing | Chain references without numeric index | PGE08012 |
| Collector reconciliation | Parallel writes to same variable | PGE10008 |
| Collector scope | Release outside paired expand | PGE03025 |
| Conditional exhaustiveness | Missing `*?` in conditional | PGE06xxx |
| Error suppression | Unhandled error without `!*-` or `[!]` | PGE07xxx |
| Permission compliance | Foreign-code violates grant | PGE10011-10014 |
| Constructor invariants | Constructor fails Final guarantee | PGE14xxx |
| Match value redundancy | Duplicate match values | PGE06xxx / EC-11.10 |
| Pipeline call cycle | Cycle detected in the call graph | [[technical/compile-rules/algorithms/cycle-detection\|u:cycle-detection]] |
| Overlap detection | Overlapping match conditions | [[technical/compile-rules/algorithms/overlap-detection\|u:overlap-detection]] |
| Compound exhaustiveness | Compound claim not exhaustive | [[technical/compile-rules/algorithms/compound-exhaustiveness\|u:compound-exhaustiveness]] |

## Design Process

<!-- @c:.paul/PROJECT -->
Architecture decisions route through the PAUL workflow. Each GitHub issue becomes a phase directory under `.paul/phases/`. A decision qualifying as "architecture" (service, runtime, type system, permissions, SDK, cross-language) must be recorded under [[audit/decisions/README|c:decisions-index]] before the merge phase closes the issue. The UNIFY phase checks for `[DR]`-flagged decisions in [[.paul/STATE|u:.paul/STATE]] and prompts for decision records when one is missing.

Relevant decision-record areas:

| Decision area | Triggers a record? |
|---------------|--------------------|
| Grammar productions, operators, markers | Yes |
| Compile rules (new, retired, severity change) | Yes |
| Type-system changes (types, schemas, relationships) | Yes |
| Runtime execution model | Yes |
| Audience definitions, documentation routing | Yes |
| Process and tooling | Yes |
| Documentation-only changes (wording, typo fixes) | No |

## Related Documents

| Related | Purpose |
|---------|---------|
| [[vision\|c:vision]] | Product vision |
| [[philosophy/core-philosophy\|c:core-philosophy]] | Philosophy entry point |
| [[philosophy/behavioral-contract\|c:behavioral-contract]] | Compile-to-contract principle |
| [[technical/INDEX\|u:technical-index]] | Technical doc index |
| [[technical/COMPILE-RULES\|u:COMPILE-RULES]] | Compile-rule catalog |
| [[technical/ebnf/INDEX\|u:ebnf-index]] | EBNF grammar |
| [[user/SPEC-INDEX\|u:SPEC-INDEX]] | User-facing spec |
| [[ai-retrieval-index\|u:ai-retrieval-index]] | Query-shaped retrieval layer |
| [[component-inventory\|u:component-inventory]] | Flat categorical inventory |
| [[source-tree-analysis\|u:source-tree-analysis]] | Repo shape map |
