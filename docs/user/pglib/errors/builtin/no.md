---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# `!No`

No `[@]` import needed — pglib built-in.

```polyglot
{!} !No
   [.] .Input#Error
      (-) .MessageTemplate << "Missing required input: {name}"
      (-) .Info
         [:] :name#string
   [.] .Output#Error
      (-) .MessageTemplate << "Missing required output: {name}"
      (-) .Info
         [:] :name#string
```
