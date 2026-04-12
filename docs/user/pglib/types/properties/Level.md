---
audience: pg-coder
type: specification
updated: 2026-04-12
metadata_definition: "%##Level.N"
---

# %##Level.N

<!-- @c:syntax/types/schema-properties -->

`%##Level.N` provides per-level overrides when `%##Propagate` is `#True`. It scopes different constraints to specific nesting depths, overriding the propagated defaults at that level.

## Allows

```
#Dataframe [%##Propagate << #True, %##Fields << #Range]
│
│  %##Level.2 applies ##Record:
│
├── :0                              ← L1: #Range (propagated)
│   ├── :product  -> "Widget"       ← L2: ##Record (overridden)
│   ├── :price    -> 9.99
│   └── :stock    -> 42
└── :1
    ├── :product  -> "Gadget"
    ├── :price    -> 19.99
    └── :stock    -> 7
                                     L1 = range rows, L2 = record columns
```

## Disallows

```
#Dataframe [%##Level.2 applies ##Record]
├── :0
│   ├── :0  -> "Widget"            ✗ L2 must be ##Record (enum-keyed)
│   ├── :1  -> 9.99                  not range-indexed
│   └── :2  -> 42
```

## Usage

`%##Level.N` appears inside a `{#}` definition as a scoped block:

```polyglot
{#} ##Dataframe
   (#) <#Columns << ##Enum
   (#) <#CellType <~ #
   [#] %##Depth.Max << 2
   [#] %##Fields << #Range
   [#] %##Ordered << #True
   [#] %##Gap << #False
   [#] %##Level.2 ##Record
      (#) <#Fields << <#Columns
      (#) <#ValueType << <#CellType
```

## Related

- [[properties/INDEX|Properties]] -- all `%##` and `%###` properties
- [[properties/Propagate|%##Propagate]] -- enables recursive application that Level.N overrides
- [[schemas/Dataframe|##Dataframe]] -- uses `%##Level.2` for column schema
- [[syntax/types/schema-properties|Schema Properties]] -- full specification
