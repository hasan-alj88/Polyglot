---
audience: automation-builder
type: specification
updated: 2026-04-12
metadata_definition: "%##Sorted"
---

# %##Sorted

<!-- @c:syntax/types/schema-properties -->

`%##Sorted` declares whether children are sorted by key. Sort order derives from the key type: numeric for `#Range`, alphabetical for string keys, declaration order for enum keys.

## Allows (`%##Sorted << #True`)

```
#Leaderboard
├── :0  -> 100            ← key 0 < 1 < 2
├── :1  -> 85               sorted by integer key
└── :2  -> 72
```

## Allows (`%##Sorted << #False`)

```
#Responses
├── :0  -> "pong"         ← no sort guarantee —
├── :1  -> "ack"            keys may appear
└── :2  -> "hello"          in any order
```

## Disallows

```
#Leaderboard [%##Sorted << #True]
├── :2  -> 72
├── :0  -> 100            ✗ key 0 appears after key 2
└── :1  -> 85             ✗ key 1 appears after key 2
                           keys must be in ascending order
```

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[properties/Ordered|%##Ordered]] -- `%##Sorted << #True` implies `%##Ordered << #True`
- [[schemas/Sorted|##Sorted]] -- bundles `%##Sorted` + `%##Ordered`
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
