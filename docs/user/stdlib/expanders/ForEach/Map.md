---
audience: user
type: specification
updated: 2026-03-28
status: stable
---

# ~ForEach.Map — Expand Map

Iterates over each key-value pair in a `#Map`. The execution marker on the expand line controls parallelism: `[p]` for parallel, `[r]` for sequential.

No `[@]` import needed. No `.Enumerate` variant — Map is unordered (`%Ordered = #False`), so a positional index would be misleading.

## IO Signature

| Input | Outputs |
|-------|---------|
| `<Map` | `>key`, `>item` |

## Usage

```polyglot
[p] ~ForEach.Map
   [~] <Map << $prices
   [~] >key >> $ticker
   [~] >item >> $price
   ...
```

See also: [[concepts/collections/expand#Expand Operators]]
