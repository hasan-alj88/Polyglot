---
audience: user
type: specification
updated: 2026-03-29
status: stable
---

# ~ForEach.Dataframe — Expand Dataframe by Row

Iterates over each row in a dataframe, producing a mini-pipeline per row. The execution marker on the expand line controls parallelism: `[p]` for parallel, `[r]` for sequential.

No `[@]` import needed.

The `>row` output is a compiler-synthesized struct from the `ColumnEnum` type parameter, where each field is typed as `CellType`. For example, if `ColumnEnum` is `#SalesColumns` (`.product`, `.price`, `.quantity`) and `CellType` is `#string`, then `$row` has `.product#string`, `.price#string`, `.quantity#string`.

## IO Signature

| Input | Output |
|-------|--------|
| `<Dataframe` | `>row` |

## Usage

```polyglot
[p] ~ForEach.Dataframe
   [~] <Dataframe << $sales
   [~] >row >> $row
   [ ] $row has .product, .price, .quantity — all #string
   ...
```

## Variants

- [Enumerate](Dataframe/Enumerate.md) — adds positional `>index` output
- ~~Column~~ — **deprecated**. Use `=#.Column` pipeline instead (see [[#|stdlib/pipelines/#]])

See also: [[collections#Expand Operators]]
