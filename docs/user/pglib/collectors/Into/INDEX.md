---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# *Into — Data Collectors

Data collectors gather mini-pipeline outputs back into a collection type, accessible one level up from the expand scope.

Collector invocation uses `[r]` (sequential) or `[p]` (parallel) execution markers. Collector IO lines use `[*]`.

No `[@]` import needed.

## Operators

- [[pglib/collectors/Into/Array|*Into.Array]] -- collect items into an array
- [[pglib/collectors/Into/Map|*Into.Map]] -- collect key-value pairs into a map
- [[pglib/collectors/Into/Serial|*Into.Serial]] -- collect key-value pairs into a serial
- [[pglib/collectors/Into/Level|*Into.Level]] -- collect into serialized siblings at a level
- [[pglib/collectors/Into/Dataframe|*Into.Dataframe]] -- collect rows into a dataframe

## Related

- [[pglib/collectors/INDEX|Collectors overview]]
- [[concepts/collections/collect|Collect Operators]]
