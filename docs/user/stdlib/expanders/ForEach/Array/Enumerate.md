---
audience: user
type: specification
updated: 2026-03-28
status: stable
---

# ~ForEach.Array.Enumerate — Expand Array with Index

Iterates over each item in an array with its positional index. The execution marker on the expand line controls parallelism: `[p]` for parallel, `[r]` for sequential.

No `[@]` import needed.

## IO Signature

| Input | Outputs |
|-------|---------|
| `<Array` | `>index`, `>item` |

## Usage

```polyglot
[p] ~ForEach.Array.Enumerate
   [~] <Array << $myArray
   [~] >index >> $idx
   [~] >item >> $item
   ...
```

See also: [[collections#Expand Operators]]
