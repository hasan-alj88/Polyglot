---
audience: automation-builder
type: specification
updated: 2026-04-12
metadata_definition: "%##Gap"
---

# %##Gap

<!-- @c:syntax/types/schema-properties -->

`%##Gap` declares whether gaps are allowed in child keys. When `#False`, keys must be contiguous with no missing indices.

## Allows (`%##Gap << #False`)

```
#Scores
├── :0  -> 95
├── :1  -> 87
└── :2  -> 72
         ← contiguous: 0, 1, 2 — no gaps
```

## Allows (`%##Gap << #True`)

```
#SparseData
├── :0   -> 95
├── :5   -> 87
└── :12  -> 72
          ← gaps at 1-4 and 6-11 — allowed
```

## Disallows

```
#Scores [%##Gap << #False]
├── :0  -> 95
├── :2  -> 72             ✗ gap at :1 — compiler error
└── :5  -> 60             ✗ gaps at :3, :4
                           contiguous keys required
```

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[schemas/Array|##Array]] -- uses `%##Gap << #False`
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
