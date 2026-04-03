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
Every pipeline definition `{=}` (see [[blocks]]) must contain these elements in order. IO lines use [[io]] parameters with [[operators]] for assignment. Variable states follow [[variable-lifecycle]].

| Order | Element | Marker | Required |
|-------|---------|--------|----------|
| 0 | Metadata | `[%]` | Optional |
| 1 | Permissions | `[_]` | Optional |
| 2 | Trigger / IO / Errors | `[T]`, `[=]` | `[T]` mandatory, `[=]` optional |
| 3 | Queue | `[Q]` | Mandatory |
| 4 | Wrapper | `[W]` | Mandatory |
| 5 | Execution | `[r]`, `[p]`, `[b]`, `[s]`, `[?]` | Yes |

Misordering these sections is a compile error (PGE01001).

**Metadata:** `[%]` lines declare description, version, authors, license, deprecation, and aliases. `.info#serial` holds custom metadata. Duplicate metadata field names are a compile error (PGE01015). See [[blocks#Metadata]].

**Note:** `[T]` triggers, `[=]` IO declarations, and `[=] !ErrorName` error declarations form one section. IO declarations must appear **before** any trigger that pushes into them — the variable must exist before assignment (PGE01002). Error declarations (`[=] !ErrorName`) appear alongside IO declarations. When a trigger produces outputs (e.g., `=T.Folder.NewFiles`), its `[=]` IO lines are indented under the `[T]` line and wire trigger outputs to pipeline inputs.

**Type inputs:** Pipelines can receive type definitions as data tree inputs using `[=] <#type` — the same `<#` syntax used in `{M}` macro type parameters. This extends GT-1 (all definitions are data trees) to runtime pipeline IO. See [[syntax/types/macro-types#`<#type` in Pipeline IO]] for details and [[#|stdlib/pipelines/#]] for the `=#.*` validation pipelines that use this pattern.

## Marker Declarations

<!-- @blocks -->
A marker declaration on `{=}` specifies the pipeline's invocation context — which execution markers (`[r]`, `[p]`, `[b]`) can invoke it. See [[blocks#Marker declarations on `{=}`]] for the definition-level summary.

| Declaration | Invocable via | Restriction |
|-------------|---------------|-------------|
| `{=}[exe]` | `[r]`, `[p]`, `[b]` | None — full execution pipeline (default) |
| `{=}[r]` | `[r]` only | Sequential only |
| `{=}[p]` | `[p]` only | Parallel only |
| `{=}[b]` | `[b]` only | Background only — no outputs allowed (fire-and-forget) |
| `{=}[rp]` | `[r]`, `[p]` | Sequential or parallel (no background) |
| `{=}[rb]` | `[r]`, `[b]` | Sequential or background (no parallel) |
| `{=}[pb]` | `[p]`, `[b]` | Parallel or background (no sequential) |

**Default:** `{=}` without a marker is equivalent to `{=}[exe]` — no warning, no error.

**Subtypes have fixed markers:** `{T}` = `{=}[T]`, `{W}` = `{=}[W]`, `{Q}` = `{=}[Q]`. These cannot take additional `marker_decl`.

**Examples:**

```polyglot
{ } Default — same as {=}[exe]
{=} =ProcessData
   [T] =T.Call
   ...

{ } Explicit execution marker — identical to above
{=}[exe] =ProcessData
   [T] =T.Call
   ...

{ } Background-only — no outputs, fire-and-forget
{=}[b] =LogEvent
   [T] =T.Call
   [=] <message#string
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =File.Text.Append"{$logPath}"
      [=] <text << $message
```

See [[technical/ebnf/09-definition-blocks#9.3|EBNF §9.3]] for the formal `marker_decl` grammar.

## Base vs Derived

<!-- @stdlib/types/BaseCode -->
Every pipeline definition is either **base** or **derived**. The distinction determines whether execution is handled by native code or by a Polyglot body.

| Property | Base | Derived |
|----------|------|---------|
| Execution body | None — bodyless | Full Polyglot body (`[T]`, `[Q]`, `[W]`, `[r]`/`[p]`/`[b]`) |
| `.baseCode` metadata | Required — `[%] .baseCode << #BaseCode.*` | Forbidden |
| Where defined | Stdlib `.pg` files | Stdlib or user `.pg` files |
| Implementation | Native compiler code (Rust) | Polyglot pipelines |

**Mutual exclusion:** `.baseCode` and an execution body cannot coexist. Violating this is a compile error (PGE01028).

**Exception:** `{T}` triggers and `{Q}` queue pipelines are IO-only by design — they may be bodyless without `.baseCode` (they declare IO ports and metadata only).

### Configuration

The Polyglot config file selects the active base language:

```
base: Rust
```

All `.baseCode` references must use the configured language. Future base languages expand the `#BaseCode` enum without changing pipeline definitions.

### Examples

```polyglot
{ } Base pipeline — bodyless, native implementation
{=}[exe] =File.Text.Read
   [%] .baseCode << #BaseCode.Rust.File.Text.Read
   [=] <path#path
   [=] >content#string
   [=] !File.NotFound
   [=] !File.PermissionDenied

{ } Derived pipeline — full Polyglot body, no .baseCode
{=} =ProcessData
   [T] =T.Call
   [=] <input#string
   [=] >result#string
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =File.Text.Read
      [=] <path << $input
      [=] >content >> $result
```

See [[stdlib/types/BaseCode|#BaseCode enum]] for the full variant tree and [[technical/ebnf/09-definition-blocks#9.9|EBNF §9.9]] for the formal `.baseCode` grammar.

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
