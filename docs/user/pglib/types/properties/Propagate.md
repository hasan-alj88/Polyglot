---
audience: pg-coder
type: specification
updated: 2026-04-12
metadata_definition: "%##Propagate"
---

# %##Propagate

<!-- @c:syntax/types/schema-properties -->

`%##Propagate` declares whether branch-level properties apply recursively to all nested levels down to `%##Depth.Max`. When `#True`, every level inherits the same constraints unless overridden by `%##Level.N`.

## Allows (`%##Propagate << #True`)

```
#Matrix [%##Gap << #False, %##Ordered << #True, %##Propagate << #True]
├── :0                     ← L1: ordered, no gaps ✓
│   ├── :0  -> 1.0         ← L2: ordered, no gaps ✓ (propagated)
│   ├── :1  -> 2.0
│   └── :2  -> 3.0
└── :1
    ├── :0  -> 4.0
    ├── :1  -> 5.0
    └── :2  -> 6.0
                            properties apply at every level
```

## Allows (`%##Propagate << #False`)

```
#Tree [%##Gap << #False, %##Propagate << #False]
├── :0                     ← L1: no gaps ✓
│   ├── :0  -> "a"
│   ├── :3  -> "d"         ← L2: gaps OK — property not propagated
│   └── :7  -> "h"
└── :1
    └── :0  -> "x"
                            only L1 enforces the constraint
```

## Disallows

```
#Matrix [%##Gap << #False, %##Propagate << #True]
├── :0
│   ├── :0  -> 1.0
│   ├── :2  -> 3.0         ✗ gap at L2 :1 — propagated constraint violated
│   └── :3  -> 4.0
└── :1
    ├── :0  -> 5.0
    └── :1  -> 6.0
```

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[properties/Level|%##Level.N]] -- per-level overrides when propagating
- [[schemas/Array|##Array]] -- uses `%##Propagate << #True`
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
