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

- [[aj3lib/reassemblers/Agg/Sum|=*Agg.Sum]] -- sum numeric values
- [[aj3lib/reassemblers/Agg/Count|=*Agg.Count]] -- count items
- [[aj3lib/reassemblers/Agg/Average|=*Agg.Average]] -- arithmetic mean
- [[aj3lib/reassemblers/Agg/Max|=*Agg.Max]] -- maximum value
- [[aj3lib/reassemblers/Agg/Min|=*Agg.Min]] -- minimum value
- [[aj3lib/reassemblers/Agg/Concatenate|=*Agg.Concatenate]] -- join strings

## Related

- [[aj3lib/reassemblers/INDEX|Reassemble Operators overview]]
- [[aj3lib/collectors/Agg/INDEX|*Agg Collectors]] -- standalone collector equivalents
- [[concepts/collections/reassemble|Reassemble Operators (concept)]]
