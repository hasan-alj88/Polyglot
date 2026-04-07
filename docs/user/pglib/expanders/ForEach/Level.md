---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# ~ForEach.Level

Unlike `~ForEach.Serial` which iterates all keys, `~ForEach.Level` iterates only the siblings at a specific level of a serialized structure. The `.~` suffix on the input path marks the level iteration point -- analogous to `.*` wildcard, `.~` means "expand siblings at this level."

## Syntax

```polyglot
[r] ~ForEach.Level
   [~] <level << #SomeData.SubField.~
   [~] >key >> $key
   [~] >item >> $item
   ...
```

The `.~` suffix on the input path marks the level iteration point: `<level << #SomeData.SubField.~`

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<level` | path with `.~` | Level path to iterate siblings at |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>key` | `#KeyString` | Sibling key at target level |
| `>item` | any | Sibling value |

## Errors

None.

## Permissions

None.

## Related

- [[pglib/expanders/ForEach/Serial|~ForEach.Serial]] -- iterates all levels
- [[pglib/expanders/ForEach/INDEX|~ForEach Expanders]]
- [[concepts/collections/expand|Expand Operators]]
