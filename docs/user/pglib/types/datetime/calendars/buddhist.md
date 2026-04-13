---
audience: automation-builder
type: reference
updated: 2026-04-09
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
