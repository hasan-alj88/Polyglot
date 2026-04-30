---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# *All / *First / *Nth — Collect-All & Race Collectors

<!-- @c:glossary#Reconciliation -->
Collect-all and race collectors operate outside expand scopes. They collect variables from parallel `[=]` pipeline calls using `(*) <<` (wait input) and `(*) >>` (collect output). Each collector type defines a different [[glossary#Reconciliation|c:reconciliation]] strategy — both for output selection and job lifecycle:

- `*All` — every associated job completes naturally; all variables stay accessible
- `*First` / `*Nth` — winner collected, remaining associated jobs are cancelled (unless another collector still holds a claim — see [[concepts/collections/collect#Compound Collector Strategies]])
- `*Ignore` — output discarded, job completes but output is released

No `[@]` import needed.

## Operators

- [[aj3lib/collectors/Sync/All|*All]] -- wait for all listed variables to reach Final
- [[aj3lib/collectors/Sync/First|*First]] -- race: take first arriving value
- [[aj3lib/collectors/Sync/Nth|*Nth]] -- race: take Nth arriving value (*Second is alias for n=2)
- [[aj3lib/collectors/Sync/Ignore|*Ignore]] -- explicit collector discard

## Inline Discard

`$*` is the inline discard syntax -- no variable created, output immediately released. This is a language construct, not a collector operator.

```aljam3
[=] -SomePipeline
   (-) >result >> $*
```

## Related

- [[aj3lib/collectors/INDEX|Collectors overview]]
- [[concepts/collections/collect|Collect Operators]]
- [[concepts/collections/collect#Reconciliation]] -- reconciliation model
- [[concepts/collections/collect#Compound Collector Strategies]] -- multiple collectors on same job
