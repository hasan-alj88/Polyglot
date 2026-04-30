---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# Expander Operators (=)

Expanders iterate over collection elements, producing a mini-pipeline per item. Each mini-pipeline runs independently and feeds results into a collector.

All expanders use the `=` prefix. Invocation uses `[=]` (parallel) or `[-]` (sequential) execution markers. Expander IO lines use `(=)`.

No `[@]` import needed.

## ForEach

- [[aj3lib/expanders/ForEach/INDEX|=ForEach.*]] -- iterate Array, Map, Serial, Level, or Dataframe

## Related

- [[aj3lib/INDEX|aj3lib Namespace Registry]]
- [[aj3lib/collectors/INDEX|Collector Operators]] -- pair with expanders to collect results
- [[concepts/collections/expand|Expand Operators]] -- conceptual overview
