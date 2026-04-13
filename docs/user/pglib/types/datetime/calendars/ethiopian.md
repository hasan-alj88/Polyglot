---
audience: automation-builder
type: reference
updated: 2026-04-09
---

# Ethiopian Calendar Types

## #EthiopianDate

```polyglot
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

```polyglot
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
