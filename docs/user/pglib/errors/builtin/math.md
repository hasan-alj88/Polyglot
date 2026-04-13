---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# `!Math`

No `[@]` import needed — pglib built-in.

```polyglot
{!} !Math
   [.] .DivideByZero#Error
      (-) .MessageTemplate << "Division by zero: {expression}"
      (-) .Info
         [:] :expression#string
```
