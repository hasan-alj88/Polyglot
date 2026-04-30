---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# *Agg — Aggregation Collectors

<!-- @c:glossary#Reconciliation -->
Aggregation collectors reduce mini-pipeline outputs to a single value. The namespace is `*Agg` -- the canonical shorthand for `*Aggregate`. Always use `*Agg` in code.

`*Agg` collectors operate **inside** expand scopes. Their [[glossary#Reconciliation|c:reconciliation]] strategy is aggregation — every expanded job completes naturally, and outputs are reduced into one scalar result.

Collector invocation uses `[-]` (sequential) or `[=]` (parallel) execution markers. Collector IO lines use `(*)`.

No `[@]` import needed.

## Operators

- [[aj3lib/collectors/Agg/Sum|*Agg.Sum]] -- sum numeric values
- [[aj3lib/collectors/Agg/Count|*Agg.Count]] -- count items
- [[aj3lib/collectors/Agg/Average|*Agg.Average]] -- arithmetic mean
- [[aj3lib/collectors/Agg/Max|*Agg.Max]] -- maximum value
- [[aj3lib/collectors/Agg/Min|*Agg.Min]] -- minimum value
- [[aj3lib/collectors/Agg/Concatenate|*Agg.Concatenate]] -- join strings

## Related

- [[aj3lib/collectors/INDEX|Collectors overview]]
- [[concepts/collections/collect|Collect Operators]]
- [[concepts/collections/collect#Reconciliation]] -- reconciliation model
