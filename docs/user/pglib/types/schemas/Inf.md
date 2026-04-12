---
audience: pg-coder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.##:Inf"
---

# ##Inf Schema

<!-- @c:types -->

`##Inf` is a composable schema that adds an `.Inf` variant to a type. It is not a standalone type -- it provides the "infinity" option when composed into concrete `#` types.

## Definition

```polyglot
{#} ##Inf
   [.] .Inf
```

## Allows

```
#Bound (with ##Inf composed)
├── .string -> "42"         ← concrete integer value (##Int active)
└── .Inf    ○ inactive

#Bound
├── .string ○ inactive
└── .Inf                    ← infinity variant active (##Inf active)
                             exactly one branch via %##Active << #One
```

## Disallows

```
#Bound (with ##Inf + %##Active << #One)
├── .string -> "42"         ✗ two branches active
└── .Inf                    ✗ #One requires exactly one
```

## Usage

Compose `##Inf` into types that need an unbounded option:

```polyglot
{#} #Bound
   [#] ##Int
   [#] ##Inf
   [#] %##Active << #ActiveKind.One
```

When `##Inf` is composed, the type gains an `.Inf` branch. Combined with `%##Active << #ActiveKind.One`, the type acts as a tagged union -- either a concrete integer or `.Inf`.

`#Inf` is a shorthand alias for `#Bound.Inf` -- the infinity variant of the `#Bound` type. Use `#Inf` in property assignments: `%##Depth.Max << #Inf`, `%##Count << #Inf`.

## Used By

<!-- @u:pglib/types/Bound -->

- [[Bound|#Bound]] composes `##Inf` for its `.Inf` variant

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.##:Inf` | Schema definition template |

Schemas are compile-time metadata constraints -- they have no runtime instances.

## Related

- [[schemas/INDEX|## Schema Types]] -- all schema definitions
- [[Bound]] -- `#Bound` type composing ##Inf
- [[syntax/types/schema-properties|Schema Properties]] -- `%##Depth.Max` uses `#Bound` (can be `.Inf`)
