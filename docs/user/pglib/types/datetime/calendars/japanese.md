---
audience: automation-builder
type: reference
updated: 2026-04-09
metadata_definition: "%definition.#:JapaneseDate"
metadata_instance: "%#:JapaneseDate:N"
---

# Japanese Calendar Types

## #JapaneseDate

Uses Gregorian `#Month` for months.

```aljam3
{#} #JapaneseDate
   [%] .description << "Japanese calendar date with imperial era"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "japanesedate"
   [.] .year#int
   [.] .era#JapaneseEra
   [.] .eraYear#int
   [.] .month#Month
   [.] .day#int
```

## #JapaneseEra

Known modern eras are fixed enum fields. Users can add older historical eras via the flexible `:historical` field.

```aljam3
{#} #JapaneseEra
   [%] .description << "Japanese imperial era"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "japaneseera"
   [.] .Reiwa
   [.] .Heisei
   [.] .Showa
   [.] .Taisho
   [.] .Meiji
   [:] :historical
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:JapaneseDate` | Compile-time type template |
| Instance | `%#:JapaneseDate:N` | Runtime instance (N = instance number) |
| Definition | `%definition.#:JapaneseEra` | Compile-time type template |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
