---
audience: automation-builder
type: specification
updated: 2026-04-11
status: stable
---

# *Into — Data Collectors

<!-- @c:glossary#Reconciliation -->
Data collectors gather mini-pipeline outputs back into a collection type, accessible one level up from the expand scope.

`*Into` collectors operate **inside** expand scopes. Their [[glossary#Reconciliation|c:reconciliation]] strategy is collection transformation — every expanded job completes naturally, and outputs are assembled into a new collection.

Collector invocation uses `[-]` (sequential) or `[=]` (parallel) execution markers. Collector IO lines use `(*)`.

No `[@]` import needed.

## Operators

- [[aj3lib/collectors/Into/Array|*Into.Array]] -- collect items into an array
- [[aj3lib/collectors/Into/Map|*Into.Map]] -- collect key-value pairs into a map
- [[aj3lib/collectors/Into/Serial|*Into.Serial]] -- collect key-value pairs into a serial
- [[aj3lib/collectors/Into/Level|*Into.Level]] -- collect into serialized siblings at a level
- [[aj3lib/collectors/Into/Dataframe|*Into.Dataframe]] -- collect rows into a dataframe

## Text Collectors

- [[aj3lib/collectors/Into/Text.Append|*Into.Text.Append]] -- concatenate text fragments with separator and ordering
- [[aj3lib/collectors/Into/Text.Merge|*Into.Text.Merge]] -- k-way merge text diffs against a base with conflict resolution

## CSV Collectors

- [[aj3lib/collectors/Into/CSV.Rows|*Into.CSV.Rows]] -- collect rows into CSV text with header
- [[aj3lib/collectors/Into/CSV.Merge|*Into.CSV.Merge]] -- k-way merge CSV diffs with header preservation

## Related

- [[aj3lib/collectors/INDEX|Collectors overview]]
- [[concepts/collections/collect|Collect Operators]]
- [[concepts/collections/collect#Reconciliation]] -- reconciliation model
