---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# `!Storage`

No `[@]` import needed — pglib built-in.

```polyglot
{!} !Storage
   [.] .Space#Error
      (-) .MessageTemplate << "Insufficient storage space: {required} needed"
      (-) .Info
         [:] :required#string
```
