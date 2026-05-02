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

For all generic DataTree collections (`##Array`, `##Map`, `##Serial`), use the universal `=ForEach` operator.

See [[concepts/collections/expand|Expand Operators]] for details on iterating DataTrees.

## Dataframes (Standard Type)

To accommodate the strictly 2D flat enum schema of `##Dataframe`, `jm3lib` provides domain-specific wrappers:

- [[jm3lib/expanders/ForEach/Row|=ForEach.Row]] -- iterate the Dataframe row-by-row (sugar for `<Depth << 1`)
- [[jm3lib/expanders/ForEach/Column|=ForEach.Column]] -- iterate the Dataframe column-by-column (sugar for Transpose + `<Depth << 1`)

## Text

- [[jm3lib/expanders/ForEach/Text.Lines|=ForEach.Text.Lines]] -- expand text into lines with index

## CSV

- [[jm3lib/expanders/ForEach/CSV.Rows|=ForEach.CSV.Rows]] -- expand CSV into rows with index

## Related

- [[jm3lib/expanders/INDEX|Expanders overview]]
- [[concepts/collections/expand|Expand Operators]]
