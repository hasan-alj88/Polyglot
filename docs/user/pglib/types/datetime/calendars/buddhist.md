---
audience: automation-builder
type: reference
updated: 2026-04-09
metadata_definition: "%definition.#:BuddhistDate"
metadata_instance: "%#:BuddhistDate:N"
---

# #BuddhistDate

Uses Gregorian `#Month` for months (Buddhist Era = Gregorian + 543).

```polyglot
{#} #BuddhistDate
   [%] .description << "Buddhist calendar date"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "buddhistdate"
   [.] .year#int
   [.] .month#Month
   [.] .day#int
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:BuddhistDate` | Compile-time type template |
| Instance | `%#:BuddhistDate:N` | Runtime instance (N = instance number) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
