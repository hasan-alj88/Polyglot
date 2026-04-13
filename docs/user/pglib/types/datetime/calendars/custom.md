---
audience: automation-builder
type: reference
updated: 2026-04-09
---

# #CustomCalendar

User-extensible calendar type. Fixed fields define the basic structure; flexible fields allow user-defined month names, leap rules, and epoch offset.

```polyglot
{#} #CustomCalendar
   [%] .description << "User-defined calendar system"
   [%] .version << "1.0.0"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "customcalendar"
   [.] .name#string
   [.] .epochOffset#int
   [:] :months
   [:] :leapRule
```
