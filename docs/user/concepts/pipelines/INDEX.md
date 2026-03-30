---
audience: user
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
| 2 | Trigger / IO / Errors | `[t]`, `[=]` | `[t]` mandatory, `[=]` optional |
| 3 | Queue | `[Q]` | Mandatory |
| 4 | Wrapper | `[W]` | Mandatory |
| 5 | Execution | `[r]`, `[p]`, `[b]`, `[s]`, `[?]` | Yes |

Misordering these sections is a compile error (PGE01001).

**Metadata:** `[%]` lines declare description, version, authors, license, deprecation, and aliases. `.info#serial` holds custom metadata. Duplicate metadata field names are a compile error (PGE01015). See [[blocks#Metadata]].

**Note:** `[t]` triggers, `[=]` IO declarations, and `[=] !ErrorName` error declarations form one section. IO declarations must appear **before** any trigger that pushes into them — the variable must exist before assignment (PGE01002). Error declarations (`[=] !ErrorName`) appear alongside IO declarations. When a trigger produces outputs (e.g., `=T.Folder.NewFiles`), its `[=]` IO lines are indented under the `[t]` line and wire trigger outputs to pipeline inputs.

**Type inputs:** Pipelines can receive type definitions as data tree inputs using `[=] <#type` — the same `<#` syntax used in `{M}` macro type parameters. This extends GT-1 (all definitions are data trees) to runtime pipeline IO. See [[syntax/types/macro-types#`<#type` in Pipeline IO]] for details and [[#|stdlib/pipelines/#]] for the `=#.*` validation pipelines that use this pattern.

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
