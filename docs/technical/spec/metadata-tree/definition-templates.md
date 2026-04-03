---
audience: [architect, designer]
type: spec
updated: 2026-04-03
---

# Definition Templates

<!-- @source:metadata-tree/INDEX -->

`%definition.{type}:{ref}` stores the compile-time structural template for each object. All instances `%{type}:{ref}:{n}` must conform to their definition's structure.

| Definition path | Ensures |
|-----------------|---------|
| `%definition.#:UserRecord` | All `%#:UserRecord:N` instances have `.name#string`, `.age#int` |
| `%definition.=:ProcessData` | All `%=:ProcessData:N` instances have the same IO ports and `live` fields |
| `%definition.T:Folder.NewFiles` | All `%T:Folder.NewFiles:N` instances have the same IO ports and `live` fields |
| `%definition.W:DB.Connection` | All `%W:DB.Connection:N` instances have the same `[{]`/`[}]` IO and scope structure |
| `%definition.Q:GPUQueue` | All `%Q:GPUQueue:N` instances have the same fields and control defaults |

Definitions are immutable at runtime — they are resolved entirely at compile time.

## Schema Definition Templates (`%definition.##`)

`##` schema types live at `%definition.##:{SchemaName}` in the metadata tree. Each schema defines tree-structure properties using the `%##` prefix:

```
%definition
├── .##:Scalar
│   └── .%##Depth.Max          -> 0
├── .##:Flat
│   └── .%##Depth.Max          -> 1
├── .##:Deep
│   └── .%##Depth.Max          -> -1
├── .##:Homogeneous
│   └── .%##Children.Uniform   -> #True
├── .##:Heterogeneous
│   └── .%##Children.Uniform   -> #False
├── .##:Contiguous
│   ├── .%##Children.Gap       -> #False
│   └── .%##Children.Ordered   -> #True
├── .##:Sparse
│   └── .%##Children.Gap       -> #True
└── .##:Rectangular
    ├── .%##Children.Regular   -> #True
    └── .%##Children.Uniform   -> #True
```

Schema definitions are immutable compile-time templates. When a `{#}` type composes a schema via `[#] << ##Flat`, the schema's `%##` properties are inherited into the type's definition.

## Field Type Definition Templates (`%definition.###`)

`###` field types live at `%definition.###:{FieldTypeName}`:

```
%definition
├── .###:Value       <- leaf holds typed data (has #type annotation)
└── .###:Enum        <- leaf is variant selector (no #type annotation)
```

The compiler infers `###Value` or `###Enum` from field declarations. Explicit `[#] << ###Value` or `[#] << ###Enum` is optional. A contradiction between explicit declaration and fields raises PGE11003.

## Schema Properties in Type Definitions (`%##`)

When a `{#}` definition includes `[#] %##Property` declarations or composes `##` schemas, the resolved properties appear as fixed fields under the type's definition template. Properties use the `%##` prefix to mark them as tree-structure metadata:

| Property | Type | Meaning |
|----------|------|---------|
| `%##Depth.Max` | `#int` | Max tree depth (`0` = scalar, `1` = flat, `-1` = unlimited) |
| `%##Children.Type` | type ref | Data type of child keys (must inherit from `#KeyString`) |
| `%##Children.Gap` | `#Boolean` | Gaps allowed in child keys? |
| `%##Children.Uniform` | `#Boolean` | All children same schema? |
| `%##Children.Regular` | `#Boolean` | All branches at same depth have same child count? |
| `%##Children.Min` | `#uint` | Minimum child count |
| `%##Children.Max` | `#int` | Max child count (`-1` = unlimited) |
| `%##Children.Ordered` | `#Boolean` | Are children ordered? |
| `%##Alias` | `#NestedKeyString` | Lowercase shorthand name |

## Field Type Properties (`%###`)

Field-level metadata uses the `%###` prefix. The `###` classification describes the nature of leaf nodes within a type:

| Property | Applies to | Meaning |
|----------|-----------|---------|
| `%###Value` | Types with `#type`-annotated fields | Leaves hold typed data |
| `%###Enum` | Types with unannotated enum fields | Leaves are variant selectors |

All siblings must be the same `###` kind — mixing typed and untyped fields among siblings raises PGE05005.

## Complete Type Definition Example

`#Array` definition template showing all metadata layers:

```
%definition.#:Array
├── .%##Depth.Max              -> (from Dim parameter)
├── .%##Children
│   ├── .Type                  -> #UnsignedInt
│   ├── .Gap                   -> #Boolean (.False active)
│   ├── .Uniform               -> #Boolean (.True active)
│   ├── .Regular               -> #Boolean (.True active)
│   └── .Ordered               -> #Boolean (.True active)
├── .%##Alias                  -> "array"
├── .%###Value                 <- inferred from :*#ValueType (typed field)
└── :*#ValueType               <- flexible children
```

The `%##` properties are accumulated from composed schemas: `##Flat` provides `%##Depth.Max << 1`, `##Contiguous` provides `%##Children.Gap << #False` and `%##Children.Ordered << #True`, `##Rectangular` provides `%##Children.Regular << #True` and `%##Children.Uniform << #True`. Redundant properties raise PGW11001; contradicting overrides raise PGW11002.

`#Boolean` definition template showing `###Enum`:

```
%definition.#:Boolean
├── .%##Depth.Max              -> 0 (from ##Scalar)
├── .%##Alias                  -> "bool"
├── .%###Enum                  <- inferred from .True/.False (no #type)
├── .True                      <- enum field
└── .False                     <- enum field
```

Schema properties are introspectable at compile time and enforce structural invariants (e.g., `%##Children.Gap << #False` means the compiler rejects non-contiguous keys).

See also: [[object-types|Object Type Branches]], [[enum-rules|Enum Instance Rules]], [[string-subtypes|String Subtype Nesting]]
