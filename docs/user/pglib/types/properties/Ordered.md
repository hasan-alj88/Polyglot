---
audience: automation-builder
type: specification
updated: 2026-04-12
metadata_definition: "%##Ordered"
---

# %##Ordered

<!-- @c:syntax/types/schema-properties -->

`%##Ordered` declares whether insertion order is preserved among children. When `#True`, iterating children returns them in the order they were added.

## Allows (`%##Ordered << #True`)

```
#Tasks
├── :0  -> "design"       ← inserted first, iterated first
├── :1  -> "implement"    ← inserted second, iterated second
└── :2  -> "test"         ← inserted third, iterated third
                           order preserved
```

## Allows (`%##Ordered << #False`)

```
#Tags
├── :0  -> "rust"         ← iteration order
├── :1  -> "polyglot"       unspecified —
└── :2  -> "async"          may differ from insertion
                           order not preserved
```

## Disallows

```
#Tasks [%##Ordered << #True]
├── :0  -> "design"
├── :1  -> "test"         ✗ if "implement" was inserted at :1,
└── :2  -> "implement"      reordering violates preserved order
```

Order is a compiler/runtime guarantee, not a user-visible syntax constraint. The compiler ensures that operations on ordered types do not reorder children silently.

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[properties/Sorted|%##Sorted]] -- sorted implies ordered
- [[schemas/Array|##Array]] -- uses `%##Ordered << #True`
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
