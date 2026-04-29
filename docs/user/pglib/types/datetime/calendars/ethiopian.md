---
audience: automation-builder
type: reference
updated: 2026-04-09
metadata_definition: "%definition.#:EthiopianDate"
metadata_instance: "%#:EthiopianDate:N"
---

# Ethiopian Calendar Types

## #EthiopianDate

```aljam3
{#} #EthiopianDate
   [%] .description << "Ethiopian calendar date"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "ethiopiandate"
   [.] .year#int
   [.] .month#EthiopianMonth
   [.] .day#int
```

## #EthiopianMonth

```aljam3
{#} #EthiopianMonth
   [%] .description << "Ethiopian calendar month"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "ethiopianmonth"
   [.] .Meskerem
   [.] .Tikimt
   [.] .Hidar
   [.] .Tahsas
   [.] .Tir
   [.] .Yekatit
   [.] .Megabit
   [.] .Miyazya
   [.] .Ginbot
   [.] .Sene
   [.] .Hamle
   [.] .Nehase
   [.] .Pagume
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:EthiopianDate` | Compile-time type template |
| Instance | `%#:EthiopianDate:N` | Runtime instance (N = instance number) |
| Definition | `%definition.#:EthiopianMonth` | Compile-time type template |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
