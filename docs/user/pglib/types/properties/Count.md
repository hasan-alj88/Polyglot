---
audience: automation-builder
type: specification
updated: 2026-04-12
metadata_definition: "%##Count"
---

# %##Count / %##Count.Min

<!-- @c:syntax/types/schema-properties -->

`%##Count` sets the maximum number of children. `%##Count.Min` sets the minimum (defaults to 0 if absent). Together they define cardinality bounds.

## Allows (`%##Count << 3`, `%##Count.Min << 1`)

```
#TopThree
├── :0  -> "gold"         ← 1 to 3 children allowed
├── :1  -> "silver"
└── :2  -> "bronze"
                           3 children — within bounds

#TopThree
└── :0  -> "gold"
                           1 child — within bounds
```

## Allows (`%##Count << #Inf`)

```
#Items
├── :0  -> "apple"
├── :1  -> "banana"
├── ...
└── :999 -> "zucchini"
                           unlimited children
```

## Disallows

```
#TopThree [%##Count << 3, %##Count.Min << 1]
├── :0  -> "gold"
├── :1  -> "silver"
├── :2  -> "bronze"
└── :3  -> "copper"       ✗ 4 children — exceeds max of 3

#TopThree [%##Count << 3, %##Count.Min << 1]
                           ✗ 0 children — below min of 1
```

## Values

| Property | Type | Default | Meaning |
|----------|------|---------|---------|
| `%##Count` | `#Bound` | `#Inf` | Maximum children |
| `%##Count.Min` | `#uint` | `0` | Minimum children |

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
