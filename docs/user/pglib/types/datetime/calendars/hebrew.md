---
audience: automation-builder
type: reference
updated: 2026-04-09
metadata_definition: "%definition.#:HebrewDate"
metadata_instance: "%#:HebrewDate:N"
---

# Hebrew Calendar Types

## #HebrewDate

```aljam3
{#} #HebrewDate
   [%] .description << "Hebrew calendar date"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "hebrewdate"
   [.] .year#int
   [.] .month#HebrewMonth
   [.] .day#int
```

## #HebrewMonth

```aljam3
{#} #HebrewMonth
   [%] .description << "Hebrew calendar month"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "hebrewmonth"
   [.] .Tishrei
   [.] .Cheshvan
   [.] .Kislev
   [.] .Tevet
   [.] .Shevat
   [.] .AdarI
   [.] .AdarII
   [.] .Nisan
   [.] .Iyyar
   [.] .Sivan
   [.] .Tammuz
   [.] .Av
   [.] .Elul
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:HebrewDate` | Compile-time type template |
| Instance | `%#:HebrewDate:N` | Runtime instance (N = instance number) |
| Definition | `%definition.#:HebrewMonth` | Compile-time type template |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
