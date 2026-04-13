---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# `!Alias`

No `[@]` import needed — pglib built-in.

```polyglot
{!} !Alias
   [.] .Clash#Error
      (-) .MessageTemplate << "Alias {alias} clashes with existing name in {namespace}"
      (-) .Info
         [:] :alias#string
         [:] :namespace#string
```

See [[pglib/errors/alias-clash|!Alias.Clash]] for the compile error behavior and `[<] !Alias.Clash` fallback chain pattern.
