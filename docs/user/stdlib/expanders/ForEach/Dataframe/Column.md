---
audience: user
type: specification
updated: 2026-03-29
status: stable
---

# ~ForEach.Dataframe.Column — Expand Dataframe by Column

Iterates over each column in a dataframe, producing a mini-pipeline per column. The execution marker on the expand line controls parallelism: `[p]` for parallel, `[r]` for sequential.

No `[@]` import needed.

`>key` is the `ColumnEnum` variant (the column name). `>column` is `#Array<CellType>` — the entire column as an array. Type-safe because all columns share the same `CellType` (homogeneous).

## IO Signature

| Input | Outputs |
|-------|---------|
| `<Dataframe` | `>key`, `>column` |

## Usage

```polyglot
[p] ~ForEach.Dataframe.Column
   [~] <Dataframe << $sales
   [~] >key >> $colName
   [~] >column >> $colValues
   [ ] $colName is #SalesColumns enum variant
   [ ] $colValues is #array:string — all rows for this column
   ...
```

See also: [[collections#Expand Operators]]
