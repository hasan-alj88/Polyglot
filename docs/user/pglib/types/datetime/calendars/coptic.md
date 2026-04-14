---
audience: automation-builder
type: reference
updated: 2026-04-09
metadata_definition: "%definition.#:CopticDate"
metadata_instance: "%#:CopticDate:N"
---

# Coptic Calendar Types

## #CopticDate

```polyglot
{#} #CopticDate
   [%] .description << "Coptic calendar date"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "copticdate"
   [.] .year#int
   [.] .month#CopticMonth
   [.] .day#int
```

## #CopticMonth

```polyglot
{#} #CopticMonth
   [%] .description << "Coptic calendar month"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "copticmonth"
   [.] .Thout
   [.] .Paopi
   [.] .Hathor
   [.] .Koiak
   [.] .Tobi
   [.] .Meshir
   [.] .Paremhat
   [.] .Parmouti
   [.] .Pashons
   [.] .Paoni
   [.] .Epip
   [.] .Mesori
   [.] .PiKogiEnavot
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:CopticDate` | Compile-time type template |
| Instance | `%#:CopticDate:N` | Runtime instance (N = instance number) |
| Definition | `%definition.#:CopticMonth` | Compile-time type template |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.
