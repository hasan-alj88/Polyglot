---
audience: pg-coder
type: specification
updated: 2026-04-08
status: complete
metadata_definition: "%definition.##:Rectangular"
---

# ##Rectangular Schema (Parameterized)

<!-- @types -->

`##Rectangular` enforces a regular shape where all sub-branches at each level have the same child count. It is parameterized with `<Dim` to set depth and propagation scope.

## Definition

```polyglot
{#} ##Rectangular
   [#] <Dim <~ "1D"
   [#] %##Regular << #True
   [#] %##Depth.Max << <Dim
   [#] %##Flexible << #FlexKind.Range
   [#] %##Propagate << #True
```

## Properties Set

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Regular` | `#True` | Same child count at each level |
| `%##Depth.Max` | `<Dim` | Dimension count from parameter |
| `%##Flexible` | `#FlexKind.Range` | Compiler-generated indices |
| `%##Propagate` | `#True` | Properties apply recursively to all levels |

When `%##Propagate` is `#True`, the properties set by `##Rectangular` apply to every level down to `%##Depth.Max`. Use `%##Level.N` for per-level overrides.

## Used By

- `#Array` (via `##Array`)
- `#Dataframe` (via `##Dataframe`)

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Rectangular` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[schemas/Contiguous|##Contiguous]] -- no gaps, ordered (often paired with Rectangular)
- [[schemas/Array|##Array]] -- array schema composing ##Rectangular
- [[FlexKind]] -- `#FlexKind` enum used by `%##Flexible`
