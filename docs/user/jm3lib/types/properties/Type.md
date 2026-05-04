---
audience: automation-builder
type: specification
updated: 2026-04-12
metadata_definition: "%###Type"
---

# %###Type

<!-- @c:syntax/types/schema-properties -->

`%###Type` constrains all leaf nodes to a single type. When set, every child must hold a value of the specified type. When absent, each field declares its own type annotation (per-field typing).

## Allows (`%###Type << #int`)

```
#Scores [%###Type << #int]
├── :0  -> 95#int          ← all leaves are #int
├── :1  -> 87#int
└── :2  -> 72#int
                            uniform type: every leaf is #int
```

## Allows (`%###Type` absent)

```
#Person                     ← no %###Type — per-field typing
├── .name   -> "Alice"#string
├── .age    -> 30#int
└── .active -> #True#bool
                            each field has its own #type
```

## Allows (`%###Type << #`)

```
#Bag [%###Type << #]
├── :0  -> "hello"#string   ← # means any type
├── :1  -> 42#int
└── :2  -> #True#bool
                            heterogeneous: any type allowed
```

## Disallows

```
#Scores [%###Type << #int]
├── :0  -> 95#int
├── :1  -> "eighty"#string  ✗ #string — must be #int
└── :2  -> 72#int
                            type mismatch: leaf is not #int
```

## Values

| Value | Meaning |
|-------|---------|
| specific `#Type` | All leaves must be that type |
| `#` | Any type (heterogeneous) |
| absent | Per-field annotation required |

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[schemas/Array|##Array]] -- uses `%###Type << <#ValueType` (parameterized)
- [[schemas/Record|##Record]] -- uses `%###Type << <#ValueType`
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
