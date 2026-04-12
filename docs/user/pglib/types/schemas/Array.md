---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.##:Array"
---

# ##Array Schema (Parameterized)

<!-- @c:types -->

`##Array` is a parameterized schema that describes a contiguous, rectangular, N-dimensional structure. It takes value type and dimension parameters.

## Definition

```polyglot
{#} ##Array
   (#) <#ValueType
   (#) <Dim <~ "1D"
   [#] %##Gap << #False
   [#] %##Ordered << #True
   [#] %##Regular << #True
   [#] %##Depth.Max << <Dim
   [#] %##Propagate << #True
   [#] %##Fields << #Range
   [#] %###Type << <#ValueType
```

## Allows

```
#Scores;Array:int
├── :0  -> 95              ← contiguous integer indices
├── :1  -> 87                ordered, no gaps
└── :2  -> 72                all elements #int

#Matrix;Array:float:"2D"
��── :0                     ← L1: range-indexed rows
│   ├── :0  -> 1.0         ← L2: range-indexed cols (propagated)
│   ├── :1  -> 2.0
│   └── :2  -> 3.0
└── :1
    ├── :0  -> 4.0
    ├��─ :1  -> 5.0
    └── :2  -> 6.0
```

## Disallows

```
#Scores;Array:int
├── :0  -> 95
├── :2  -> 72              ✗ gap at :1 — %##Gap << #False
└── :3  -> 60

#Scores;Array:int
├── :0  -> 95
├── :1  -> "high"#string   ✗ #string — %###Type requires #int
└── :2  -> 72

#Scores;Array:int
├── :name -> 95            ✗ string key — %##Fields << #Range
└── :0    -> 72              requires integer indices
```

## Properties Set

| Property | Value | Meaning |
|----------|-------|---------|
| `%##Gap` | `#False` | No gaps in indices |
| `%##Ordered` | `#True` | Insertion order preserved |
| `%##Depth.Max` | `<Dim` | Dimension count |
| `%##Propagate` | `#True` | Properties apply to all levels |
| `%##Fields` | `#Range` | Compiler-generated integer indices |
| `%###Type` | `<#ValueType` | Element type constraint |

## Used By

<!-- @u:pglib/types/Array -->

- [[Array|#Array]] type composes this schema

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Array` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[Array]] -- `#Array` generic type composing ##Array
- [[syntax/types/schema-properties|Schema Properties]] -- property definitions
