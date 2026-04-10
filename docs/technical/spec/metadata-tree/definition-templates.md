---
audience: [architect, designer]
type: spec
updated: 2026-04-09
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
│
│   Value schemas
├── .##:Inf                     (composable .Inf variant)
│
│   Structure schemas
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
├── .##:Record                 <#Fields(##Enum), #ValueType(default #)>
├── .##:Array                  <#ValueType, Dim(default 1D)>
├── .##:Dataframe              <#Columns, #CellType(default #)>
│
│   Retired schemas (#275)
├── .##:Deep                   *(retired — use %##Depth.Max << #Inf)*
├── .##:Contiguous             *(retired — use %##Gap/%##Ordered directly)*
├── .##:Sparse                 *(retired — use %##Gap << #True)*
├── .##:Rectangular            *(retired — use %##Propagate directly)*
├── .##:Map                    *(retired — use ##Record)*
└── .##:Set                    *(retired — use ##Array + %###Unique)*
```

Schema definitions are immutable compile-time templates. When a `{#}` type composes a schema via `[#] ##Flat`, the schema's `%##` properties are inherited into the type's definition. Parameterized schemas accept `(#) <#param` / `(#) <param` bindings nested under the `[#]` line — the `:` separator in type annotations binds positionally to declared parameters.

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

The compiler infers `###Value` or `###Enum` from field declarations. Explicit `[#] ###Value` or `[#] ###Enum` is optional. A contradiction between explicit declaration and fields raises PGE11003.

## Schema Properties in Type Definitions (`%##`)

When a `{#}` definition includes `[#] %##Property` declarations or composes `##` schemas, the resolved properties appear as fixed fields under the type's definition template. Properties use the `%##` prefix to mark them as branch-level tree-structure metadata:

### Branch-level properties

| Property | Type | Meaning |
|----------|------|---------|
| `%##Fields` | `#FieldsDescriptor` or `##Enum` ref | `.Range` (integer-indexed) or enum ref (stamp children from variants) |
| `%##Schema` | list of `##` | Structural schemas children must satisfy (AND) |
| `%##Active` | `#ActiveKind` | `.All` (every branch present) / `.One` (exactly one) / `.Partial` (any non-zero subset) |
| `%##Ordered` | `#Boolean` | Insertion order preserved? |
| `%##Sorted` | `#Boolean` | Sorted by key? (order derived from key type) |
| `%##Gap` | `#Boolean` | Gaps allowed in keys? |
| `%##Count` | `#Bound` | Max children (#Inf = unlimited) |
| `%##Count.Min` | `#uint` | Min children (0 if absent) |
| `%##Propagate` | `#Boolean` | Apply these properties recursively to all levels down to Depth.Max |
| `%##Level.N` | scope | Per-level override when Propagate is true |
| `%##Depth.Max` | `#Bound` | Max depth (0, 1, N, #Inf) |
| `%##Alias` | `#NestedKeyString` | Lowercase shorthand name |

*Retired (#275):* `%##Flexible` (`#FlexKind`), `%##Key`, `%##Range`, `%##Regular`.

### Leaf-level properties (`%###`)

Field-level metadata uses the `%###` prefix. The `###` classification describes the nature of leaf nodes within a type:

| Property | Type | Meaning |
|----------|------|---------|
| `%###Kind` | `#FieldKind` | `###Value` (data) or `###Enum` (identity) |
| `%###Type` | type ref | Type all leaves must be. `#` = any. Absent = per-field |
| `%###Unique` | `#Boolean` | Leaf values must be distinct? |

All siblings must be the same `###` kind — mixing typed and untyped fields among siblings raises PGE05005.

### Native class metadata (`%Native.Class`)

`{#}` definitions backed by a native language class declare `[%] %Native.Class` with fixed fields naming the host class and its capabilities:

| Field | Type | Meaning |
|-------|------|---------|
| `.Rust` | `#string` | Rust class name backing this type |
| `.Validate` | `#Boolean` | Class provides a validation function |

The compiler uses `%Native.Class` to discover host-language functions for the type. Currently only `.Validate` is defined — additional capabilities will be added as needed.

```polyglot
%definition.#:String
├── %Native.Class
│   ├── .Rust                      -> "PgString"
│   └── .Validate                  -> #True
```

## Complete Type Definition Example

`#Array` definition template showing all metadata layers (generic type with `(#) <#ValueType` and `(#) <Dim` parameters):

```polyglot
%definition.#:Array
├── .%##Depth.Max              -> Dim (from ##Array parameter)
├── .%##Gap                    -> #False (from ##Array)
├── .%##Ordered                -> #True (from ##Array)
├── .%##Propagate              -> #True (from ##Array)
├── .%##Fields                 -> #Range (from ##Array)
├── .%##Alias                  -> "array"
├── .%###Kind                  -> ###Value (inferred from :*#ValueType)
└── :*#ValueType               <- flexible children (generic param)
```

The `%##` properties are accumulated from composed schemas: `##Array` provides `%##Gap << #False`, `%##Ordered << #True`, `%##Propagate << #True`, and `%##Fields << #Range`. Redundant properties raise PGW11001; contradicting overrides raise PGW11002.

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
