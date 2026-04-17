---
audience: automation-builder
type: specification
updated: 2026-04-17
status: complete
---

# Permissions

<!-- @c:identifiers -->
<!-- @c:blocks -->

Polyglot uses an implicit-deny permission system. Every pipeline starts with zero IO capabilities. To perform any IO — read a file, make a web request, access a database — the block must declare a named `{_}` permission object via its IO markers (`(#)` for `{#}` definitions, `(-)` for `{-}` pipelines). The `{_}` object carries both the capability grant and the resource locator (path, host, credentials, etc.).

## Sections

| Doc | Content |
|-----|---------|
| [[concepts/permissions/implicit-deny\|Implicit Deny]] | Zero-capability default, pure computation |
| [[concepts/permissions/permission-objects\|{_} Permission Objects]] | `{_}` blocks, intent (ceiling vs grant), instance vs template, IO-based references |
| [[concepts/permissions/permission-prefixes\|Permission Prefixes]] | `_` / `__` / `___` prefix system, generic permissions |
| [[concepts/permissions/capability-enums\|Capability Enums]] | Per-category enum types and resource locator fields |
| [[concepts/permissions/hierarchical-scoping\|Hierarchical Scoping]] | Package ceiling, pipeline grant, no inheritance |
| [[concepts/permissions/enforcement\|Enforcement]] | Parallel write exclusion, compile-time checks, file binding |
| [[concepts/permissions/permission-schema\|__Permission Schema]] | Full `__Permission` descriptor tree with `__ResourceLocator` |
| [[concepts/permissions/foreign-code\|Foreign Code]] | `[C]` block permission interaction |

## Complete Example

A full package showing the ceiling-to-grant flow:

```polyglot
[ ] Package declaration with permission ceiling
{@} @Local:999::DataProcessor:v1.0.0
   (@) _DataCeiling

[ ] Permission objects — decomposed fields
{_} _DataCeiling
   [.] .intent << #Ceiling
   [.] .category #File
   [.] .capability #Read
   [.] .scope "data/*.csv"
   [.] .path "data/*.csv"

{_} _ReportReader
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "data/reports/q1.csv"
   [.] .path "data/reports/q1.csv"

{_} _OutputWriter
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Write
   [.] .scope "output/summary.json"
   [.] .path "output/summary.json"
   [.] .format #JSON

[ ] Data definitions
{#} #Report
   [.] .name#string
   [.] .rows#int

[ ] Pipelines declare permissions via (-) IO
{-} -ReadReports
   (-) _ReportReader
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >reports#array.Report
   [-] -File.Serial.CSV.Parse
      (-) <path << _ReportReader
      (-) >data >> $reports
   [-] >reports << $reports

{-} -WriteOutput
   (-) _OutputWriter
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <data#Report
   (-) >result#string
   [-] -File.Serial.JSON.Serialize
      (-) <data << $data
      (-) <destination << _OutputWriter
      (-) >written >> $result
   [-] >result << $result
```

## File Ordering

`{@}` must appear first in every `.pg` file (compiler-enforced). The recommended stylistic ordering for the remaining blocks is:

```polyglot
{@}   <- mandatory first (compiler-enforced)
{_}   <- permission objects (recommended second)
{#}   <- data definitions
{-}   <- pipelines
```
