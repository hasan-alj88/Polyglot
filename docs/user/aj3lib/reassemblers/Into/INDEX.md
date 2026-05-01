---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# =*Into — Transformation Reassemblers

<!-- @c:concepts/collections/reassemble -->
Transformation reassemblers expand a collection and collect items into a different collection type in one atomic operation. They combine `=ForEach` expansion with `*Into` collection — no intermediate variable or per-item logic.

Use `=*Into` when each item passes through unchanged or with field extraction only. If you need per-item pipeline calls, conditionals, or error handling, use the full `=` ... `*Into` form instead.

Invocation uses `[-]` (sequential) or `[=]` (parallel) execution markers. IO lines use `(=)` for expander input and `(*)` for collector output.

No `[@]` import needed.

## Operators

- [[jm3lib/reassemblers/Into/Array|=*Into.Array]] -- collect into array
- [[jm3lib/reassemblers/Into/Map|=*Into.Map]] -- collect into map
- [[jm3lib/reassemblers/Into/Dataframe|=*Into.Dataframe]] -- collect into dataframe

## Related

- [[jm3lib/reassemblers/INDEX|Reassemble Operators overview]]
- [[jm3lib/collectors/Into/INDEX|*Into Collectors]] -- standalone collector equivalents
- [[concepts/collections/reassemble|Reassemble Operators (concept)]]
