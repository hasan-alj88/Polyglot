---
audience: automation-builder
type: specification
updated: 2026-04-18
status: complete
---

# Permission Enforcement

## Parallel Write Exclusion

<!-- @c:glossary#Reconciliation -->
<!-- @u:technical/spec/job-sandbox -->
<!-- @u:technical/spec/otel-permission-events -->
Concurrent parallel jobs (`[=]`) may not hold write permission to the same resource path — this is a compile error (PGE10008). Read permission to the same resource is allowed across parallel jobs.

This rule makes [[glossary#Reconciliation|reconciliation]] safe by construction: parallel jobs are pure readers, and only sequential code after collection can write to shared resources. No runtime locks, mutexes, or transactional memory are needed — the permission system eliminates write contention at compile time.

The compiler checks for overlapping write targets by comparing the `.scope` and `.path` fields in `{_}` grant objects across all `[=]` jobs in the same parallel scope. Overlap is determined by glob intersection — if two grants can match the same concrete path, PGE10008 fires.

```aljam3
{_} _WriteGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Write
   [.] .scope "output/result.json"
   [.] .path "output/result.json"

[ ] ✗ PGE10008 — two parallel jobs write to the same file
[=] -Write.PartA
   (-) _WriteGrant
[=] -Write.PartB
   (-) _WriteGrant                      [ ] ✗ same write target as PartA

[ ] ✓ Sequential is fine — no contention
[-] -Write.PartA
   (-) _WriteGrant
[-] -Write.PartB
   (-) _WriteGrant                      [ ] ✓ sequential — no overlap
```

See [[technical/compile-rules/PGE/PGE10008-parallel-write-permission-exclusion|PGE10008]] for the full rule with detection algorithm and examples.

## No Instances

Permissions are **compile-time declarations** — they apply across all instances of a pipeline. There are no per-instance permissions. If `-ProcessLogs` declares `(-) _LogFileGrant`, every instance of `-ProcessLogs` shares that grant. The `%_` metadata tree branch has no `:{instance}` level (see [[data-is-trees]]).

## Compile-Time Enforcement

All permission checks are **static analysis** — resolved at compile time, not runtime. The compiler verifies:

1. **Grant within ceiling** — every `_` grant in a `{-}` must reference a `{_}` object whose capabilities fall within the `{@}` package ceiling (PGE10001)
2. **Import ceiling compatibility** — imported package ceilings must fall within the importer's ceiling (PGE10002)
3. **Pure computation enforced** — any IO call in a pipeline with no `_` IO declarations is a compile error (PGE10004)
4. **Fully filled** — every `{_}` object must have all leaf fields assigned (no empty leaves)
5. **Intent validation** — `#Ceiling` objects may use glob patterns; `#Grant` objects must use specific narrow values
6. **Template resolution** — all `(_)` inputs must be provided; unresolved template inputs are a compile error (PGE10009)
7. **Resource validation** — file-category `{_}` objects must point to files that exist at compile time (PGE10010)

For Aljam3-native code, if it compiles, the permissions are satisfied. For foreign code (`[C]` blocks, `-Run.*` pipelines), the compiler performs AST analysis to verify compliance — see [[permissions/foreign-code|u:Foreign Code Permissions]].

## Foreign Code Sandbox

<!-- @c:permissions/foreign-code -->
<!-- @c:technical/spec/job-sandbox -->
For foreign code in `-Run.*` pipelines ([[permissions/foreign-code|c:Foreign Code Permissions]]), the Aljam3 Service applies OS-level restrictions as **defense-in-depth** before spawning the job process. This is not a per-call runtime check (no Java SecurityManager overhead) — it is a one-time load-time sandbox applied before any user code executes.

The compiler emits a **Permission Manifest** as part of the [[technical/spec/behavior-contract#Permission Manifest|Behavior Contract]]. The Runner reads this manifest and configures OS-level restrictions before spawning the job process. The sandbox setup creates an isolated execution environment using Linux kernel features — all without requiring root privileges.

### Permission Category Mapping

Each `{_}` permission category maps to specific OS enforcement:

| {\_} Category | {\_} Fields Used | Sandbox Mechanism |
|---|---|---|
| #File.#Read | `.path` | Landlock filesystem rules (read-only) + mount namespace |
| #File.#Write | `.path` | Landlock filesystem rules (read-write) + mount namespace |
| #File.#Execute | `.path` | Landlock execute rules + seccomp (allow execve) |
| #File.#Delete | `.path` | Landlock remove rules |
| #File.#Create | `.path` | Landlock create rules + parent directory |
| #Web.#Request | `.host`, `.port` | Network namespace + firewall rules for declared endpoints |
| #Web.#Socket | `.host`, `.port` | Network namespace + firewall rules for declared hosts/ports |
| #Web.#Listen | `.port` | Network namespace + firewall rules for declared ports |
| #Database | `.host`, `.port` | Network namespace + firewall rules (DB is TCP) |
| #System.#Process | (capability flag) | seccomp allows fork/clone/execve |
| #System.#Shell | (capability flag) | seccomp allows execve + Landlock for shell path |
| #System.#Env | `.vars` (future) | Runtime prunes environment before exec |
| #RAM.#Limit | `.max` | cgroups v2 `memory.max` — only declared memory available |
| #CPU.#Limit | `.max` | cgroups v2 `cpu.max` — only declared CPU available |
| #CPU.#Weight | `.weight` | cgroups v2 `cpu.weight` — scheduling priority |
| #GPU.#Limit | `.max` | cgroups v2 device controller + vendor API — only declared GPU memory |
| #GPU.#Device | `.device` | cgroups v2 device controller — only declared GPU device |
| #IO.#Limit | `.maxBps`, `.maxIops` | cgroups v2 `io.max` — only declared IO bandwidth |
| #Processes.#Limit | `.max` | cgroups v2 `pids.max` — only declared process count |
| #Duration.#Limit | `.max` | Timer-based SIGTERM/SIGKILL — only declared execution time |

The sandbox catches violations that AST analysis cannot detect — unresolvable variable paths, IO registry gaps, or new library functions not yet in the [[technical/compiler/ast-invisible-registry|registry]]. If foreign code attempts an operation outside declared permissions, the kernel blocks it and the process receives a permission error.

The principle remains: **compilation is a license to launch**. The sandbox narrows that license to exactly what was declared.

### Opaque Code: \_Unsafe.SandboxOnly

When a compiled binary or opaque code cannot be fully analyzed by the compiler (no source code available, no tree-sitter support), the developer must acknowledge sandbox-only enforcement:

```aljam3
{-} -ProcessData
   [.] %Authors << "jane.doe@company.com"
   [.] %Description << "Legacy Go binary for report generation"
   [.] %Version << "1.2.0"
   (-) _FileGrant
   [ ]
   [-] -Run.Go.CLI;GoEnv "compiled-binary"
      [!] _Unsafe.SandboxOnly
      (-) <args#string << "--input /data/in.csv"
      (-) >output#string >> >result
```

`_Unsafe.SandboxOnly` does three things:

1. **Activates ALL isolation layers** — every available defense mechanism, even those normally skipped for performance. This is *more* secure than normal pipelines, not less.
2. **Suppresses AST-invisible errors to warnings** — the compiler still analyzes what it can, but PGE10014 errors become warnings in the [[technical/compiler/compliance-report|compliance report]].
3. **Requires accountability metadata** — `%Authors`, `%Description`, and `%Version` must be filled (PGE10016).

Without `[!] _Unsafe.SandboxOnly`, a `-Run.*.CLI` pipeline is a compile error (PGE10015). The developer must either provide source code (switch to `.Function`/`.Script`) or acknowledge opaque execution.

### Sandbox Inspection

Developers can inspect the effective sandbox configuration for any pipeline:

```text
$ aljam3 inspect -sandbox -ProcessData
```

This shows exactly what OS restrictions will be applied — filesystem rules, network rules, syscall filters, and resource limits — without running the pipeline. See [[technical/spec/job-sandbox|job-sandbox]] for the full implementer-facing specification.

## Resource Limits

Resource limits are permissions — permission to use a certain amount of a resource. Six resource categories extend the permission model:

| Category | Enforcement | Default Limit-Exceeded Behavior |
|----------|-------------|--------------------------------|
| `#RAM` | cgroups v2 `memory.max` | OOM kill → job Failed |
| `#CPU` | cgroups v2 `cpu.max` / `cpu.weight` | Throttle |
| `#GPU` | cgroups v2 device controller | Kill |
| `#IO` | cgroups v2 `io.max` | Throttle |
| `#Processes` | cgroups v2 `pids.max` | Fork fails (EAGAIN) |
| `#Duration` | Timer-based SIGTERM/SIGKILL | Kill after grace period |

When a pipeline does not declare `{_}` resource permissions, the Queue Handler applies defaults from its configuration. Every job has resource limits — explicit or QH-defaulted. Limit-exceeded behavior is configurable per-queue via `{Q}` definitions using `#LimitAction`. See [[concepts/pipelines/queue#Resource Limit Defaults|Queue — Resource Limit Defaults]].

For the full cgroups v2 mapping and sandbox implementation, see [[technical/spec/job-sandbox#Permission Category to Sandbox Mapping|job-sandbox]].

## Compile-Time File Binding

<!-- @c:vision#No Dynamic Code -->

Compile-Time File Binding is a **natural consequence** of the `{_}` permission-as-resource model. Because every external file access is mediated through a `{_}` permission object that carries a `.path` field, the compiler always knows which files are referenced. Content hashing is automatic — not a separate mechanism.

### Compiler Detection Flow

When the compiler encounters a `_` permission reference in any block's IO:

1. **Resolve the `{_}` definition** — if it's a template, substitute all `(_)` inputs and resolve `{<param}` interpolation in field values
2. **Validate completeness** — ensure all required fields for the category are present (e.g., `.path` for File)
3. **For file-category permissions** (`.category #File`):
   - Read the file at `.path`
   - Compute a content hash (SHA-256)
   - Store the hash in the compiled output alongside the permission grant
4. **For credentials paths** — `.credentials` fields on Database permissions are also content-hashed (credential rotation requires recompilation)
5. **For DB/Web permissions** — validate connection parameters are well-formed (host format, port range, endpoint syntax)

### Runtime Enforcement

If a referenced file changes after compilation:

1. The Aljam3 Service **revokes** the associated permission grant
2. The pipeline **refuses to execute** until the developer recompiles with the updated file
3. A **file change watcher trigger** monitors all referenced file paths and notifies the developer that recompilation is required

This ensures that no external code or input runs through the platform without having passed through the compiler's analysis. The principle is simple: **compilation is a license to launch**, and that license is invalidated when the inputs change.

### What Gets Hashed

| Source | Hash Trigger | Example |
|--------|-------------|---------|
| `{_}` with `.category #File` | `.path` field | `_Secrets` with `.path "/config/secrets.yaml"` |
| `{_}` with `.credentials` | `.credentials` field | `_ProductionDB` with `.credentials "/keys/db.json"` |
| `-Run.*` pipelines | `<code.file` input | `-Run.Script.File` with `<code.file "/scripts/etl.py"` |

Templates contribute to hashing after resolution — each instantiation produces its own hash based on the resolved `.path`.

**Note:** `.aj3` source files are covered by the same principle implicitly — changing a `.aj3` file has no effect until the developer recompiles, at which point the compiler re-analyses the entire package.
