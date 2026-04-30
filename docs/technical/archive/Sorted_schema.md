---
audience: automation-builder
type: specification
updated: 2026-04-08
status: complete
metadata_definition: "%definition.##:Sorted"
---

# ##Sorted Schema

<!-- @c:types -->

`##Sorted` enforces that children are sorted by key. Order is derived from the key type: numeric keys sort numerically, string keys sort alphabetically (lexicographic), enum keys sort by declaration order.

## Allows

```
#Leaderboard [##Sorted]
├── :0  -> 100             ← keys in ascending order
├── :1  -> 85
└── :2  -> 72
                            sorted by integer key
```

## Disallows

```
#Leaderboard [##Sorted]
├── :2  -> 72              ✗ key 2 before key 0
├── :0  -> 100             ✗ out of order
└── :1  -> 85
                            keys must be in ascending order
```

## Properties

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Sorted` | `#True` | Children sorted by key |
| `%##Ordered` | `#True` | Order preserved (implied by sorting) |

## Used By

Available for user-defined types requiring sorted access patterns. Not used by any built-in aj3lib types by default.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Sorted` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Contiguous|##Contiguous]] -- ordered + no gaps (different from sorted)
- [[syntax/types/schema-properties|Schema Properties]] -- `%##Sorted` property
