---
audience: pg-coder
type: specification
updated: 2026-04-11
status: stable
---

# Collector Operators (*)

<!-- @c:glossary#Reconciliation -->
Collectors gather outputs from expanded mini-pipelines or parallel pipeline calls back into a single value or collection. They are the [[glossary#Reconciliation|c:reconciliation]] layer — every parallel job must be reconciled, and the collector determines both what happens to the data (output reconciliation) and when jobs terminate (job reconciliation). See [[concepts/collections/collect#Reconciliation]] for the full model.

All collectors use the `*` prefix. Invocation uses `[-]` (sequential) or `[=]` (parallel) execution markers. Collector IO lines use `(*)`.

No `[@]` import needed.

## Data Collectors

Collect mini-pipeline outputs into a collection type.

- [[pglib/collectors/Into/INDEX|*Into.*]] -- collect into Array, Map, Serial, Level, Dataframe, Text, or CSV

## Aggregation Collectors

Reduce mini-pipeline outputs to a single value.

- [[pglib/collectors/Agg/INDEX|*Agg.*]] -- Sum, Count, Average, Max, Min, Concatenate

## Collect-All & Race Collectors

Collect variables from parallel `[=]` pipeline calls (outside expand scopes).

- [[pglib/collectors/Sync/INDEX|*All / *First / *Nth]] -- collect-all barrier, race collectors, discard

## Related

- [[pglib/INDEX|pglib Namespace Registry]]
- [[concepts/collections/collect|Collect Operators]] -- conceptual overview
- [[concepts/collections/collect#Reconciliation]] -- reconciliation model (output + job lifecycle)
- [[concepts/collections/collect#Compound Collector Strategies]] -- multiple collectors on the same job
