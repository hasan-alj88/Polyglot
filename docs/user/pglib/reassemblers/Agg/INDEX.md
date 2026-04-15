---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# =*Agg — Aggregation Reassemblers

<!-- @c:concepts/collections/reassemble -->
Aggregation reassemblers expand a collection and reduce all items to a single scalar value in one atomic operation. They combine `=ForEach` expansion with `*Agg` collection — no intermediate variable or per-item logic.

Invocation uses `[-]` (sequential) or `[=]` (parallel) execution markers. IO lines use `(=)` for expander input and `(*)` for collector output.

No `[@]` import needed.

## Operators

- [[pglib/reassemblers/Agg/Sum|=*Agg.Sum]] -- sum numeric values
- [[pglib/reassemblers/Agg/Count|=*Agg.Count]] -- count items
- [[pglib/reassemblers/Agg/Average|=*Agg.Average]] -- arithmetic mean
- [[pglib/reassemblers/Agg/Max|=*Agg.Max]] -- maximum value
- [[pglib/reassemblers/Agg/Min|=*Agg.Min]] -- minimum value
- [[pglib/reassemblers/Agg/Concatenate|=*Agg.Concatenate]] -- join strings

## Related

- [[pglib/reassemblers/INDEX|Reassemble Operators overview]]
- [[pglib/collectors/Agg/INDEX|*Agg Collectors]] -- standalone collector equivalents
- [[concepts/collections/reassemble|Reassemble Operators (concept)]]
