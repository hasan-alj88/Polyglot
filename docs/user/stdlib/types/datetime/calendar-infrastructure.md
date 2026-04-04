---
audience: pg-coder
type: reference
updated: 2026-04-04
---

# Calendar Infrastructure

<!-- @source:calendar-infrastructure -->

### #CalendarProjection

Defines how any calendar maps from epoch. Every calendar is a projection of `.Instant.epoch` (Unix epoch seconds). Converting between calendars always goes through epoch: CalendarA -> epoch -> CalendarB.

```polyglot
{#} #CalendarProjection
   [%] .description << "Epoch-to-calendar mapping definition"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###ScalarValue
   [#] %##Alias << "calendarprojection"
   [.] .epochOffset#int
   [.] .leapRule#LeapRule
   [.] .monthStructure#MonthStructure
```

### #LeapRule

`#LeapRule.FixedCycle` has nested value sub-fields for cycle configuration. `#LeapRule.Custom` uses flexible fields for user-defined leap logic.

```polyglot
{#} #LeapRule
   [%] .description << "Leap year calculation method"
   [%] .version << "1.0.0"
   [#] << ##Scalar
   [#] << ###ScalarEnum
   [#] %##Alias << "leaprule"
   [.] .None
   [.] .FixedCycle
      [.] .cycleYears#int
      [.] .leapYears#array.int
   [.] .Astronomical
   [.] .Tabular
   [.] .Custom
      [:] :rule
```

---

See also: [[calendar-date-types]], [[supporting-enums]], [[main-type]]
