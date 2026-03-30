---
audience: user
type: specification
updated: 2026-03-28
status: stable
---

# ~ForEach.Serial — Expand Serial

Iterates over all key-item pairs in a serial at all levels. The execution marker on the expand line controls parallelism: `[p]` for parallel, `[r]` for sequential.

No `[@]` import needed. No `.Enumerate` variant — Serial is unordered.

## IO Signature

| Input | Outputs |
|-------|---------|
| `<Serial` | `>key`, `>item` |

## Usage

```polyglot
[r] ~ForEach.Serial
   [~] <Serial << $data
   [~] >key >> $key
   [~] >item >> $item
   ...
```

See also: [[concepts/collections/expand#Expand Operators]]
