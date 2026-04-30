---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# =ForEach.Dataframe

Iterates over each row in a dataframe, producing a mini-pipeline per row. The execution marker on the expand line controls parallelism: `[=]` for parallel, `[-]` for sequential.

The `>row` output is a compiler-synthesized struct from the `ColumnEnum` type parameter, where each field is typed as `CellType`. For example, if `ColumnEnum` is `#SalesColumns` (`.product`, `.price`, `.quantity`) and `CellType` is `#string`, then `$row` has `.product#string`, `.price#string`, `.quantity#string`.

## Syntax

```aljam3
[=] =ForEach.Dataframe
   (=) <Dataframe << $sales
   (=) >row >> $row
   [ ] $row has .product, .price, .quantity — all #string
   ...
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<Dataframe` | `#Dataframe` | Dataframe to iterate by row |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>row` | row struct | Compiler-synthesized struct with column fields |

## Variants

- [[aj3lib/expanders/ForEach/Dataframe/Enumerate|=ForEach.Dataframe.Enumerate]] -- adds positional `>index` output
- ~~Column~~ -- **deprecated**. Use `-#.Column` pipeline instead.

## Errors

None.

## Permissions

None.

## Related

- [[aj3lib/expanders/ForEach/INDEX|=ForEach Expanders]]
- [[concepts/collections/expand|Expand Operators]]
