---
audience: pg-coder
type: specification
updated: 2026-04-12
metadata_definition: "%##Schema"
---

# %##Schema

<!-- @c:syntax/types/schema-properties -->

`%##Schema` declares which structural schemas the children at this level must satisfy. Multiple schemas accumulate with AND semantics.

## Allows

```
#Matrix [%##Schema << [##Array]]
├── :0                          ← child satisfies ##Array
│   ├── :0  -> 1.0
│   ├── :1  -> 2.0
│   └── :2  -> 3.0
└── :1                          ← child satisfies ##Array
    ├── :0  -> 4.0
    ├── :1  -> 5.0
    └── :2  -> 6.0
```

## Disallows

```
#Matrix [%##Schema << [##Array]]
├── :0
│   ├── :0  -> 1.0
│   └── :1  -> 2.0
└── :1
    ├── .name  -> "row2"   ✗ fixed field — child must be ##Array (range-indexed)
    └── :0     -> 4.0
```

## Values

| Value | Effect |
|-------|--------|
| list of `##` | Children must satisfy all listed schemas (AND-composed) |

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[schemas/INDEX|## Schema Types]] -- schemas that can appear in the list
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
