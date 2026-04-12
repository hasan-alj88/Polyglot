---
audience: pg-coder
type: specification
updated: 2026-04-12
metadata_definition: "%###Unique"
---

# %###Unique

<!-- @c:syntax/types/schema-properties -->

`%###Unique` declares whether leaf values must be distinct. When `#True`, no two children can hold the same value -- this gives set-like semantics to an array.

## Allows (`%###Unique << #True`)

```
#Tags [%###Unique << #True]
├── :0  -> "rust"
├── :1  -> "async"
└── :2  -> "polyglot"
                     ← all values distinct ✓
```

## Allows (`%###Unique << #False` or absent)

```
#Rolls
├── :0  -> 4
├── :1  -> 6
├── :2  -> 4          ← duplicate value — allowed
└── :3  -> 1
```

## Disallows

```
#Tags [%###Unique << #True]
├── :0  -> "rust"
├── :1  -> "async"
├── :2  -> "polyglot"
└── :3  -> "rust"      ✗ duplicate "rust" — uniqueness violated
```

`##Set` was retired in favor of `##Array` + `%###Unique << #True`, making uniqueness a composable property rather than a separate schema.

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[schemas/Array|##Array]] -- combine with `%###Unique << #True` for set semantics
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
