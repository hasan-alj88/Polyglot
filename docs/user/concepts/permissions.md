---
audience: automation-builder
type: specification
updated: 2026-04-05
status: complete
---

# Permissions

<!-- @c:identifiers -->
<!-- @c:blocks -->

Polyglot uses an implicit-deny permission system. Every pipeline starts with zero IO capabilities. To perform any IO — read a file, make a web request, access a database — the package or pipeline must reference a named `{_}` permission object. The `{_}` definition block and `[_]` block element are registered in [[blocks#Permissions]].

## Sections

| Doc | Content |
|-----|---------|
| [[concepts/permissions/implicit-deny\|Implicit Deny]] | Zero-capability default, pure computation |
| [[concepts/permissions/permission-objects\|{_} Permission Objects]] | `{_}` blocks, intent (ceiling vs grant), fully-filled requirement |
| [[concepts/permissions/permission-prefixes\|Permission Prefixes]] | `_` / `__` / `___` prefix system, generic permissions |
| [[concepts/permissions/capability-enums\|Capability Enums]] | Per-category enum types (`#FileCapability`, `#WebCapability`, etc.) |
| [[concepts/permissions/hierarchical-scoping\|Hierarchical Scoping]] | Package ceiling, pipeline grant, no inheritance |
| [[concepts/permissions/enforcement\|Enforcement]] | Parallel write exclusion, compile-time checks, file binding |
| [[concepts/permissions/permission-schema\|__Permission Schema]] | Full `__Permission` descriptor tree |
| [[concepts/permissions/foreign-code\|Foreign Code]] | `[C]` block permission interaction |

## Complete Example

A full package showing the ceiling-to-grant flow:

```polyglot
{ } Package declaration
{@} @Local:999::DataProcessor:v1.0.0
   [_] _DataCeiling

{ } Permission objects
{_} _DataCeiling
   [.] .intent << #Ceiling
   [.] .File.Read "data/*.csv"
   [.] .File.Write "output/*.json"
   [.] .Database.Read "*.postgres"

{_} _ReportReader
   [.] .intent << #Grant
   [.] .File.Read "data/reports/q1.csv"
   [.] .File.Read "data/reports/q2.csv"

{_} _OutputWriter
   [.] .intent << #Grant
   [.] .File.Read "data/reports/q1.csv"
   [.] .File.Write "output/summary.json"

{ } Data definitions
{#} #Report
   [.] .name#string
   [.] .rows#int

{ } Pipelines
{-} -ReadReports
   [_] _ReportReader
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >reports#array.Report
   [-] >reports << -File.Serial.CSV.Parse "data/reports/q1.csv"

{-} -WriteOutput
   [_] _OutputWriter
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <data#Report
   (-) >result#string
   [-] >result << -File.Serial.JSON.Serialize $data >> "output/summary.json"
```

## File Ordering

`{@}` must appear first in every `.pg` file (compiler-enforced). The recommended stylistic ordering for the remaining blocks is:

```polyglot
{@}   <- mandatory first (compiler-enforced)
{_}   <- permission objects (recommended second)
{#}   <- data definitions
{-}   <- pipelines
```
