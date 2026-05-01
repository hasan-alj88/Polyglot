---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# Reassemble Operators (=*)

<!-- @c:concepts/collections/reassemble -->
Reassemble operators combine an expander and collector into a single atomic operation. The `=*` prefix reads as "expand, then collect" — fan-out followed by fan-in with no intermediate body logic. See [[concepts/collections/reassemble|c:Reassemble Operators]] for the conceptual overview.

All reassemblers use the `=*` prefix. Invocation uses `[-]` (sequential) or `[=]` (parallel) execution markers. IO lines use `(=)` for expander inputs and `(*)` for collector outputs.

No `[@]` import needed.

## Aggregation Reassemblers

Expand a collection and reduce to a single scalar value.

- [[jm3lib/reassemblers/Agg/INDEX|=*Agg.*]] -- Sum, Count, Average, Max, Min, Concatenate

## Transformation Reassemblers

Expand a collection and collect into a different collection type.

- [[jm3lib/reassemblers/Into/INDEX|=*Into.*]] -- Array, Map, Dataframe

## Related

- [[jm3lib/INDEX|jm3lib Namespace Registry]]
- [[jm3lib/expanders/INDEX|Expander Operators]] -- `=` fan-out operators
- [[jm3lib/collectors/INDEX|Collector Operators]] -- `*` fan-in operators
- [[concepts/collections/reassemble|Reassemble Operators (concept)]]
