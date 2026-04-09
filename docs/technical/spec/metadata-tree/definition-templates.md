---
audience: [architect, designer]
type: spec
updated: 2026-04-04
---

# Definition Templates

<!-- @source:metadata-tree/INDEX -->

`%definition.{type}:{ref}` stores the compile-time structural template for each object. All instances `%{type}:{ref}:{n}` must conform to their definition's structure.

| Definition path | Ensures |
|-----------------|---------|
| `%definition.#:UserRecord` | All `%#:UserRecord:N` instances have `.name#string`, `.age#int` |
| `%definition.=:ProcessData` | All `%-:ProcessData:N` instances have the same IO ports and `live` fields |
| `%definition.T:Folder.NewFiles` | All `%T:Folder.NewFiles:N` instances have the same IO ports and `live` fields |
| `%definition.W:DB.Connection` | All `%W:DB.Connection:N` instances have the same `[{]`/`[}]` IO and scope structure |
| `%definition.Q:GPUQueue` | All `%Q:GPUQueue:N` instances have the same fields and control defaults |

Definitions are immutable at runtime — they are resolved entirely at compile time.

## Schema Definition Templates (`%definition.##`)

`##` schema types live at `%definition.##:{SchemaName}` in the metadata tree. Each schema defines tree-structure properties using the `%##` prefix:

```polyglot
%definition
│   Depth schemas
├── .##:Leaf
│   └── .%##Depth.Max          -> 0
├── .##:Scalar
│   └── .%##Depth.Max          -> 1
├── .##:Flat
│   └── .%##Depth.Max          -> 1
├── .##:Deep
│   └── .%##Depth.Max          -> .Inf
│
│   Value schemas
├── .##:Inf                     (composable .Inf variant)
│
│   Structure schemas
├── .##:Contiguous
│   ├── .%##Gap                -> #False
│   └── .%##Ordered            -> #True
├── .##:Sparse
│   └── .%##Gap                -> #True
├── .##:Rectangular            <Dim(default 1D)>
│   ├── .%##Regular            -> #True
│   ├── .%##Depth.Max          -> Dim
│   ├── .%##Flexible           -> .Range
│   └── .%##Propagate          -> #True
├── .##:Sorted
│   ├── .%##Sorted             -> #True
│   └── .%##Ordered            -> #True
│
│   Classification schemas
├── .##:Enum
│   ├── (composes ##Flat)
│   ├── .%##Active             -> .One
│   └── .%###Kind              -> #FieldKind.Enum
│
│   Parameterized schemas
├── .##:Fields                 <#Type(##Enum)>
├── .##:Nullable               <#ValueType>
├── .##:Result                 <#OkType, #ErrType>
├── .##:String                 <regex>
├── .##:Map                    <#KeyType, #ValueType(default #)>
├── .##:Array                  <#ValueType, Dim(default 1D)>
├── .##:Set                    <#ValueType>
└── .##:Dataframe              <#Columns, #CellType(default #)>
```

Schema definitions are immutable compile-time templates. When a `{#}` type composes a schema via `[#] << ##Flat`, the schema's `%##` properties are inherited into the type's definition. Parameterized schemas accept `[#] <#param` / `[#] <param` bindings nested under the `[#] <<` line — the `:` separator in type annotations binds positionally to declared parameters.

## Field Type Definition Templates (`%definition.###`)

`###` field types live at `%definition.###:{FieldTypeName}`:

```polyglot
%definition
├── .###:Value            <- leaf holds typed data (has #type annotation)
├── .###:Enum             <- leaf is variant selector (no #type annotation)
├── .###:ScalarValue      <- regex-validated string data (#String:* family, ##Scalar only)
├── .###:ScalarEnum       <- variant selector in scalar type (#Boolean, #NativeKind, ##Scalar only)
└── .###:None             <- nullable (empty string "")
```

The compiler infers `###Value` or `###Enum` from field declarations. Explicit `[#] << ###Value` or `[#] << ###Enum` is optional. A contradiction between explicit declaration and fields raises PGE11003.

## Schema Properties in Type Definitions (`%##`)

When a `{#}` definition includes `[#] %##Property` declarations or composes `##` schemas, the resolved properties appear as fixed fields under the type's definition template. Properties use the `%##` prefix to mark them as branch-level tree-structure metadata:

### Branch-level properties

| Property | Type | Meaning |
|----------|------|---------|
| `%##Flexible` | `#FlexKind` | `.Fixed` (. fields), `.Flexible` (: user-controlled), `.Range` (: compiler-generated from range) |
| `%##Key` | type ref | Type of flexible `:` child names. Only for Flexible/Range |
| `%##Range` | range expr | Valid key interval (numeric keys only) |
| `%##Schema` | list of `##` | Structural schemas children must satisfy (AND) |
| `%##Active` | `#ActiveKind` | `.All` (every branch present) / `.One` (exactly one) / `.Partial` (any non-zero subset) |
| `%##Ordered` | `#Boolean` | Insertion order preserved? |
| `%##Sorted` | `#Boolean` | Sorted by key? (order derived from key type) |
| `%##Gap` | `#Boolean` | Gaps allowed in keys? |
| `%##Regular` | `#Boolean` | Same child count per sub-branch? |
| `%##Count` | `#Bound` | Max children (.Inf = unlimited) |
| `%##Count.Min` | `#uint` | Min children (0 if absent) |
| `%##Propagate` | `#Boolean` | Apply these properties recursively to all levels down to Depth.Max |
| `%##Level.N` | scope | Per-level override when Propagate is true |
| `%##Depth.Max` | `#Bound` | Max depth (0, 1, N, .Inf) |
| `%##Alias` | `#NestedKeyString` | Lowercase shorthand name |

### Leaf-level properties (`%###`)

Field-level metadata uses the `%###` prefix. The `###` classification describes the nature of leaf nodes within a type:

| Property | Type | Meaning |
|----------|------|---------|
| `%###Kind` | `#FieldKind` | `###Value` (data) or `###Enum` (identity) |
| `%###Type` | type ref | Type all leaves must be. `#` = any. Absent = per-field |
| `%###Unique` | `#Boolean` | Leaf values must be distinct? |

All siblings must be the same `###` kind — mixing typed and untyped fields among siblings raises PGE05005.

## Complete Type Definition Example

`#Array` definition template showing all metadata layers (generic type with `[#] <#ValueType` and `[#] <Dim` parameters):

```polyglot
%definition.#:Array
├── .%##Depth.Max              -> Dim (from ##Rectangular parameter)
├── .%##Gap                    -> #False (from ##Contiguous)
├── .%##Ordered                -> #True (from ##Contiguous)
├── .%##Regular                -> #True (from ##Rectangular)
├── .%##Propagate              -> #True (from ##Rectangular)
├── .%##Flexible               -> .Range (from ##Rectangular)
├── .%##Key                    -> #uint (numeric indices)
├── .%##Alias                  -> "array"
├── .%###Kind                  -> ###Value (inferred from :*#ValueType)
└── :*#ValueType               <- flexible children (generic param)
```

The `%##` properties are accumulated from composed schemas: `##Contiguous` provides `%##Gap << #False` and `%##Ordered << #True`, `##Rectangular` provides `%##Regular << #True` and `%##Propagate << #True`. Redundant properties raise PGW11001; contradicting overrides raise PGW11002.

`#Boolean` definition template showing `###ScalarEnum`:

```polyglot
%definition.#:Boolean
├── .%##Depth.Max              -> 1 (from ##Scalar)
├── .%##Alias                  -> "bool"
├── .%###ScalarEnum            <- inferred from .True/.False (no #type, ##Scalar context)
├── .True                      <- enum field
└── .False                     <- enum field
```

Schema properties are introspectable at compile time and enforce structural invariants (e.g., `%##Gap << #False` means the compiler rejects non-contiguous keys).

See also: [[object-types|Object Type Branches]], [[enum-rules|Enum Instance Rules]], [[string-subtypes|String Subtype Nesting]]
