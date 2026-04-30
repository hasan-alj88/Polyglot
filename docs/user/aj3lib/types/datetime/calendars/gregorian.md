---
audience: automation-builder
type: reference
updated: 2026-04-09
metadata_definition: "%definition.#:GregorianDate"
metadata_instance: "%#:GregorianDate:N"
---

# #GregorianDate

```aljam3
{#} #GregorianDate
   [%] .description << "Gregorian calendar date"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "gregoriandate"
   [.] .year#int
   [.] .month#Month
   [.] .day#int
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:GregorianDate` | Compile-time type template |
| Instance | `%#:GregorianDate:N` | Runtime instance (N = instance number) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
