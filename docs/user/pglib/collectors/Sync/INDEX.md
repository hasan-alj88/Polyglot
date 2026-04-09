---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# *All / *First / *Nth — Collect-All & Race Collectors

Collect-all and race collectors operate outside expand scopes. They collect variables from parallel `[=]` pipeline calls using `(*) <<` (wait input) and `(*) >>` (collect output).

No `[@]` import needed.

## Operators

- [[pglib/collectors/Sync/All|*All]] -- wait for all listed variables to reach Final
- [[pglib/collectors/Sync/First|*First]] -- race: take first arriving value
- [[pglib/collectors/Sync/Nth|*Nth]] -- race: take Nth arriving value (*Second is alias for n=2)
- [[pglib/collectors/Sync/Ignore|*Ignore]] -- explicit collector discard

## Inline Discard

`$*` is the inline discard syntax -- no variable created, output immediately released. This is a language construct, not a collector operator.

```polyglot
[=] -SomePipeline
   (-) >result >> $*
```

## Related

- [[pglib/collectors/INDEX|Collectors overview]]
- [[concepts/collections/collect|Collect Operators]]
