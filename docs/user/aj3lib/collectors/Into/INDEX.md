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

## DataTrees (Unified)

For all DataTree collections (`##Array`, `##Map`, `##Serial`, `##Dataframe`), use the unified `*Collect` operator.

See [[concepts/collections/collect|Collect Operators]] for details on collecting into DataTrees.

## Text Collectors

- [[jm3lib/collectors/Into/Text.Append|*Into.Text.Append]] -- concatenate text fragments with separator and ordering
- [[jm3lib/collectors/Into/Text.Merge|*Into.Text.Merge]] -- k-way merge text diffs against a base with conflict resolution

## CSV Collectors

- [[jm3lib/collectors/Into/CSV.Rows|*Into.CSV.Rows]] -- collect rows into CSV text with header
- [[jm3lib/collectors/Into/CSV.Merge|*Into.CSV.Merge]] -- k-way merge CSV diffs with header preservation

## Related

- [[jm3lib/collectors/INDEX|Collectors overview]]
- [[concepts/collections/collect|Collect Operators]]
- [[concepts/collections/collect#Reconciliation]] -- reconciliation model
