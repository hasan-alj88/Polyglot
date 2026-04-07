---
audience: pg-coder
type: specification
updated: 2026-03-28
status: stable
---

# ~ForEach.Array — Expand Array

Iterates over each item in an array, producing a mini-pipeline per item. The execution marker on the expand line controls parallelism: `[p]` for parallel, `[r]` for sequential.

No `[@]` import needed.

## IO Signature

| Input | Output |
|-------|--------|
| `<Array` | `>item` |

## Usage

```polyglot
[p] ~ForEach.Array
   [~] <Array << $myArray
   [~] >item >> $item
   ...
```

## Variants

- [Enumerate](Array/Enumerate.md) — adds positional `>index` output

See also: [[concepts/collections/expand#Expand Operators]]
