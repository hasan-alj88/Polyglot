---
audience: pg-coder
type: specification
updated: 2026-03-28
status: stable
---

# ~ForEach.Level — Expand Level

Unlike `~ForEach.Serial` which iterates all keys, `~ForEach.Level` iterates only the siblings at a specific level of a serialized structure. The `.~` suffix on the input path marks the level iteration point — analogous to `.*` wildcard, `.~` means "expand siblings at this level."

The execution marker on the expand line controls parallelism: `[p]` for parallel, `[r]` for sequential.

No `[@]` import needed.

## IO Signature

| Input | Outputs |
|-------|---------|
| `<level` | `>key`, `>item` |

## Usage

```polyglot
[r] ~ForEach.Level
   [~] <level << #SomeData.SubField.~
   [~] >key >> $key
   [~] >item >> $item
   ...
```

The `.~` suffix on the input path marks the level iteration point: `<level << #SomeData.SubField.~`

See also: [[concepts/collections/expand#Expand Operators]]
