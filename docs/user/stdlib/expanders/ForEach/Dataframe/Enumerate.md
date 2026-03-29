---
audience: user
type: specification
updated: 2026-03-29
status: stable
---

# ~ForEach.Dataframe.Enumerate — Expand Dataframe with Row Index

Iterates over each row in a dataframe with its positional index. The execution marker on the expand line controls parallelism: `[p]` for parallel, `[r]` for sequential.

No `[@]` import needed.

## IO Signature

| Input | Outputs |
|-------|---------|
| `<Dataframe` | `>index`, `>row` |

## Usage

```polyglot
[p] ~ForEach.Dataframe.Enumerate
   [~] <Dataframe << $sales
   [~] >index >> $idx
   [~] >row >> $row
   ...
```

See also: [[collections#Expand Operators]]
