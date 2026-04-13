---
audience: automation-builder
type: reference
updated: 2026-04-09
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
