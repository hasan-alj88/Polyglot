---
audience: automation-builder
type: specification
updated: 2026-04-11
status: stable
---

# =ForEach — Expand Operators

Expand operators iterate over collection elements, producing a mini-pipeline per item. The execution marker on the expand line controls parallelism: `[=]` for parallel, `[-]` for sequential. Expander IO lines use `(=)`.

No `[@]` import needed.

## DataTrees (Unified)

For all DataTree collections (`##Array`, `##Map`, `##Serial`, `##Dataframe`), use the unified `=ForEach` operator.

See [[concepts/collections/expand|Expand Operators]] for details on iterating DataTrees.

## Text

- [[jm3lib/expanders/ForEach/Text.Lines|=ForEach.Text.Lines]] -- expand text into lines with index

## CSV

- [[jm3lib/expanders/ForEach/CSV.Rows|=ForEach.CSV.Rows]] -- expand CSV into rows with index

## Related

- [[jm3lib/expanders/INDEX|Expanders overview]]
- [[concepts/collections/expand|Expand Operators]]
