---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# `!Timeout`

No `[@]` import needed — pglib built-in.

```polyglot
{!} !Timeout
   [.] .Connection#Error
      (-) .MessageTemplate << "Connection timed out after {duration}"
      (-) .Info
         [:] :duration#string
   [.] .Read#Error
      (-) .MessageTemplate << "Read timed out after {duration}"
      (-) .Info
         [:] :duration#string
```
