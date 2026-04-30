---
audience: automation-builder
type: specification
updated: 2026-04-11
status: stable
---

# =ForEach — Expand Operators

Expand operators iterate over collection elements, producing a mini-pipeline per item. The execution marker on the expand line controls parallelism: `[=]` for parallel, `[-]` for sequential. Expander IO lines use `(=)`.

No `[@]` import needed.

## Array

- [[aj3lib/expanders/ForEach/Array|=ForEach.Array]] -- iterate array items
- [[aj3lib/expanders/ForEach/Array/Enumerate|=ForEach.Array.Enumerate]] -- with positional index

## Map

- [[aj3lib/expanders/ForEach/Map|=ForEach.Map]] -- iterate key-value pairs

## Serial

- [[aj3lib/expanders/ForEach/Serial|=ForEach.Serial]] -- iterate all key-item pairs at all levels
- [[aj3lib/expanders/ForEach/Level|=ForEach.Level]] -- iterate siblings at a specific level (`.=` suffix)

## Dataframe

- [[aj3lib/expanders/ForEach/Dataframe|=ForEach.Dataframe]] -- iterate rows
- [[aj3lib/expanders/ForEach/Dataframe/Enumerate|=ForEach.Dataframe.Enumerate]] -- with row index
- ~~Column~~ -- **deprecated**. Use `-#.Column` pipeline instead.

## Text

- [[aj3lib/expanders/ForEach/Text.Lines|=ForEach.Text.Lines]] -- expand text into lines with index

## CSV

- [[aj3lib/expanders/ForEach/CSV.Rows|=ForEach.CSV.Rows]] -- expand CSV into rows with index

## Related

- [[aj3lib/expanders/INDEX|Expanders overview]]
- [[concepts/collections/expand|Expand Operators]]
