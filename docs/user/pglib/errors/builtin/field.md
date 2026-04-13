---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# `!Field`

No `[@]` import needed — pglib built-in.

```polyglot
{!} !Field
   [.] .NotFound#Error
      (-) .MessageTemplate << "Field not found: {field}"
      (-) .Info
         [:] :field#string
   [.] .PathError#Error
      (-) .MessageTemplate << "Invalid field path: {path}"
      (-) .Info
         [:] :path#string
```
