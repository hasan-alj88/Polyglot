---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# =ForEach — Expand Operators

Expand operators iterate over collection elements, producing a mini-pipeline per item. The execution marker on the expand line controls parallelism: `[=]` for parallel, `[-]` for sequential. Expander IO lines use `(=)`.

No `[@]` import needed.

## Array

- [[pglib/expanders/ForEach/Array|=ForEach.Array]] -- iterate array items
- [[pglib/expanders/ForEach/Array/Enumerate|=ForEach.Array.Enumerate]] -- with positional index

## Map

- [[pglib/expanders/ForEach/Map|=ForEach.Map]] -- iterate key-value pairs

## Serial

- [[pglib/expanders/ForEach/Serial|=ForEach.Serial]] -- iterate all key-item pairs at all levels
- [[pglib/expanders/ForEach/Level|=ForEach.Level]] -- iterate siblings at a specific level (`.=` suffix)

## Dataframe

- [[pglib/expanders/ForEach/Dataframe|=ForEach.Dataframe]] -- iterate rows
- [[pglib/expanders/ForEach/Dataframe/Enumerate|=ForEach.Dataframe.Enumerate]] -- with row index
- ~~Column~~ -- **deprecated**. Use `-#.Column` pipeline instead.

## Related

- [[pglib/expanders/INDEX|Expanders overview]]
- [[concepts/collections/expand|Expand Operators]]
