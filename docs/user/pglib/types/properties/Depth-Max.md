---
audience: pg-coder
type: specification
updated: 2026-04-12
metadata_definition: "%##Depth.Max"
---

# %##Depth.Max

<!-- @c:syntax/types/schema-properties -->

`%##Depth.Max` declares the maximum tree depth of a type's instances. Both fixed (`.`) and flexible (`:`) fields contribute to depth.

## Allows (`%##Depth.Max << 0`)

```
RawString                  ← atomic: no fields at all
  "hello"
```

## Allows (`%##Depth.Max << 1`)

```
#Person
├── .name   -> "Alice"     ← one level of fields
├── .age    -> 30
└── .email  -> "a@b.com"
```

## Allows (`%##Depth.Max << 2`)

```
#Dataframe
├── :0                      ← L1
│   ├── :product -> "W"     ← L2
│   └── :price   -> 9.99
└── :1
    ├── :product -> "G"
    └── :price   -> 19.99
```

## Allows (`%##Depth.Max << #Inf`)

```
#Serial
├── :0  -> "hello"
├── :1
│   ├── :0  -> "nested"    ← unlimited nesting
│   └── :1
│       └── :0  -> "deep"
└── :2  -> "world"
```

## Disallows

```
#Person [%##Depth.Max << 1]
├── .name  -> "Alice"
└── .address                ✗ nesting creates depth 2
    ├── .street -> "Main"     but max depth is 1
    └── .city   -> "London"

#Flag [%##Depth.Max << 0]
└── .value  -> #True        ✗ any field creates depth 1
                              but max depth is 0
```

## Compiler Inference

When a `{#}` definition does not set `%##Depth.Max` explicitly:

| Definition has | Inferred depth |
|----------------|----------------|
| `.` fixed fields only | `1` |
| `:` flexible fields | count of nested `:` levels (min 1) |
| No fields at all | `0` (requires explicit `##Leaf`) |

Explicit `[#] %##Depth.Max` overrides inference. Collections used as value types require explicit depth (PGE11002). `%##Depth.Max << #Inf` on user-defined types raises PGW11003.

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[schemas/Leaf|##Leaf]] -- `%##Depth.Max << 0`
- [[schemas/Scalar|##Scalar]] -- `%##Depth.Max << 1`
- [[schemas/Flat|##Flat]] -- `%##Depth.Max << 1`
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
