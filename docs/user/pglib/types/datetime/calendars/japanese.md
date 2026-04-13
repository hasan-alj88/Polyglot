---
audience: automation-builder
type: reference
updated: 2026-04-09
---

# Japanese Calendar Types

## #JapaneseDate

Uses Gregorian `#Month` for months.

```polyglot
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

```polyglot
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
