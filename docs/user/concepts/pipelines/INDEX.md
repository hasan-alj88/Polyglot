---
audience: pg-coder
type: specification
updated: 2026-03-30
status: complete
---

<!-- @concepts/pipelines/INDEX -->

# Pipeline Structure

<!-- @blocks -->
<!-- @io -->
<!-- @operators -->
<!-- @variable-lifecycle -->
Every pipeline definition `{-}` (see [[blocks]]) must contain these elements in order. IO lines use [[io]] parameters with [[operators]] for assignment. Variable states follow [[variable-lifecycle]].

| Order | Element | Marker | Required |
|-------|---------|--------|----------|
| 0 | Metadata | `[%]` | Optional |
| 1 | Permissions | `[_]` | Optional |
| 2 | Trigger / IO / Errors | `[T]`, `(-)` | `[T]` mandatory†, `(-)` optional |
| 3 | Queue | `[Q]` | Mandatory† |
| 4 | Wrapper | `[W]` | Mandatory† |
| 5 | Execution | `[-]`, `[=]`, `[b]`, `[s]`, `[?]` | Yes† |

†Derived `{-}` pipelines only. Native `{N}` definitions contain only `[%]` metadata and `(-)` IO — no trigger, queue, wrapper, or execution body. See [[#Native vs Derived]].

Misordering these sections is a compile error (PGE01001).

**IO marker scoping:** `(-)`, `(=)`, and `(*)` are IO markers that scope to their parent operator via indentation — they are not position-fixed elements. `(-)` appears at Order 2 for top-level pipeline IO, but also nests under `[Q]` (queue parameter wiring), `[W]` (wrapper IO wiring), and execution markers (`[-]`/`[=]`/`[b]`) as call-site IO. In every case `(-)` means the same thing: IO line for a pipeline reference (`-`). The parent operator determines the scope.

**Metadata:** `[%]` lines declare description, version, authors, license, deprecation, and aliases. `.info#serial` holds custom metadata. Duplicate metadata field names are a compile error (PGE01015). See [[blocks#Metadata]].

**Note:** `[T]` triggers, `(-)` IO declarations, and `(-) !ErrorName` error declarations form one section. IO declarations must appear **before** any trigger that pushes into them — the variable must exist before assignment (PGE01002). Error declarations (`(-) !ErrorName`) appear alongside IO declarations. When a trigger produces outputs (e.g., `-T.Folder.NewFiles`), its `(-)` IO lines are indented under the `[T]` line and wire trigger outputs to pipeline inputs.

**Type inputs:** Pipelines can receive type definitions as data tree inputs using `(-) <#type` — the same `<#` syntax used in `{#}` generic type parameters. This extends GT-1 (all definitions are data trees) to runtime pipeline IO. See [[syntax/types/generic-types#`<#type` in Pipeline IO]] for details and [[pglib/pipelines/Schema/INDEX|-#.* Schema Pipelines]] for the `-#.*` validation pipelines that use this pattern.

## Marker Declarations

<!-- @blocks -->
A marker declaration on `{-}` specifies the pipeline's invocation context — which execution markers (`[-]`, `[=]`, `[b]`) can invoke it. See [[blocks#Marker declarations on `{-}`]] for the definition-level summary.

| Declaration | Invocable via | Restriction |
|-------------|---------------|-------------|
| `{-}[exe]` | `[-]`, `[=]`, `[b]` | None — full execution pipeline (default) |
| `{-}[-]` | `[-]` only | Sequential only |
| `{-}[=]` | `[=]` only | Parallel only |
| `{-}[b]` | `[b]` only | Background only — no outputs allowed (fire-and-forget) |
| `{-}[-=]` | `[-]`, `[=]` | Sequential or parallel (no background) |
| `{-}[-b]` | `[-]`, `[b]` | Sequential or background (no parallel) |
| `{-}[=b]` | `[=]`, `[b]` | Parallel or background (no sequential) |

**Default:** `{-}` without a marker is equivalent to `{-}[exe]` — no warning, no error.

**Subtypes have fixed markers:** `{T}` = `{-}[T]`, `{W}` = `{-}[W]`, `{Q}` = `{-}[Q]`. These cannot take additional `marker_decl`.

**Examples:**

```polyglot
{ } Default — same as {-}[exe]
{-} -ProcessData
   [T] -T.Call
   ...

{ } Explicit execution marker — identical to above
{-}[exe] -ProcessData
   [T] -T.Call
   ...

{ } Background-only — no outputs, fire-and-forget
{-}[b] -LogEvent
   [T] -T.Call
   (-) <message#string
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -File.Text.Append"{$logPath}"
      (-) <text << $message
```

See [[technical/ebnf/09-definition-blocks#9.3|EBNF §9.3]] for the formal `marker_decl` grammar.

## Native vs Derived

<!-- @pglib/types/NativeKind -->
Every pipeline definition is either **native** or **derived**. The distinction determines whether execution is handled by the host language or by a Polyglot body.

| Property | Native `{N}` | Derived `{-}` |
|----------|-------------|---------------|
| Block type | `{N}` | `{-}` |
| Execution body | None — `[%]` metadata + `(-)` IO only | Full Polyglot body (`[T]`, `[Q]`, `[W]`, `[-]`/`[=]`/`[b]`) |
| Metadata scope | `%Native.*` (implicit) — `.Kind`, `.<Language>` | `%Pipeline.*` (implicit) — `.description`, `.version`, etc. |
| Where defined | pglib `.pg` files only | pglib or user `.pg` files |
| Implementation | Host language (e.g., Rust) | Polyglot pipelines |
| User-extendable | No — compiler-controlled | Yes |

**Mutual exclusion:** `{N}` and `{-}` are separate block types. A `{N}` definition cannot contain `[T]`, `[Q]`, `[W]`, or execution markers. A `{-}` definition cannot contain `%Native.*` metadata. Violating this is a compile error (PGE01028).

**`{T}`, `{Q}`, `{W}` subtypes** are IO-only by design — they declare IO ports and metadata only, with no execution body. Like `{N}`, they are bodyless, but unlike `{N}` they are user-extendable subtypes of `{-}`.

### `{N}` Metadata

`[%]` under `{N}` implicitly scopes to `%Native.*` — all fixed `.` fields (non-user-extendable):

| Field | Type | Description |
|-------|------|-------------|
| `.Kind` | `#NativeKind` | What subsystem role: Trigger, Queue, Wrapper, Execution, Intrinsic |
| `.<Language>` | `#string` | Native function name per supported language (`.Rust`, `.Cpp`, etc.) |
| `.description` | `#string` | Human-readable description |

### `#NativeKind` Enum

```polyglot
{#} #NativeKind
   .Trigger
   .Queue
   .Wrapper
   .Execution
   .Intrinsic
```

| Kind | What it does | Examples |
|------|-------------|---------|
| `.Trigger` | Fires pipeline execution | `-T.Call`, `-T.Folder.NewFiles`, `-T.Webhook` |
| `.Queue` | Manages job scheduling | `-Q.Default`, `-Q.Pause.Soft`, `-Q.Kill.Graceful` |
| `.Wrapper` | Setup/cleanup around execution | `-W.Polyglot`, `-W.DB.Connection`, `-W.RT.Python:3:14` |
| `.Execution` | Performs actual work (IO, compute) | `-File.Text.Read`, `-Math.Add`, `-DB.Query` |
| `.Intrinsic` | Compiler-internal operations | `-#.JSON.Parse`, `-DT.Now`, `-#.Validate` |

### Configuration

The Polyglot service configuration file selects which host language implements each native operation using **subsystem defaults with per-operation overrides**:

```yaml
native:
  defaults:
    tm: Rust           # default for all Trigger operations
    qh: Rust           # default for all Queue operations
    runner: Rust       # default for all Execution + Wrapper operations
    pgcompiler: Rust   # compiler implementation language

  overrides:
    "Math.Add": Go     # override specific operations by pipeline name
```

All `{N}` definitions must include a `.<Language>` binding for the language resolved for their subsystem. Future host languages add new `.<Language>` fields without changing pipeline definitions. See [[technical/spec/native-dispatch|native-dispatch]] for the full configuration spec.

### Examples

```polyglot
{ } Native definition — compiler primitive, no Polyglot body
{N} -File.Text.Read
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileTextRead"
   [%] .description << "Read text file contents"
   (-) <path#path
   (-) >content#string
   (-) !File.NotFound
   (-) !File.PermissionDenied

{ } Native trigger — fires when another pipeline calls this one
{N} -T.Call
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TriggerCall"
   [%] .description << "Pipeline invoked by another pipeline"
   (-) >IsTriggered#bool

{ } Derived pipeline — full Polyglot body, uses native definitions
{-} -ProcessData
   [T] -T.Call
   (-) <input#string
   (-) >result#string
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -File.Text.Read
      (-) <path << $input
      (-) >content >> $result
```

See [[pglib/types/NativeKind|#NativeKind enum]] for the full enum definition and [[technical/ebnf/09-definition-blocks#9.4c|EBNF §9.4c]] for the formal `{N}` grammar.

## Sub-Pages

| File | Covers |
|------|--------|
| [[metadata]] | Pipeline metadata, error trees |
| [[error-handling]] | Error handling |
| [[io-triggers]] | IO as implicit triggers, trigger configuration |
| [[permissions]] | Pipeline permissions |
| [[queue]] | Queue configuration |
| [[wrappers]] | Wrapper structure |
| [[execution]] | Execution body |
| [[chains]] | Chain execution |
| [[inline-calls]] | Inline calls, call site rules, compile rules |
