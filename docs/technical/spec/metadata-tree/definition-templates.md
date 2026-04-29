---
audience: design
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
| `%definition.W:DB.Connection` | All `%W:DB.Connection:N` instances have the same `(-)` IO and scope structure |
| `%definition.Q:GPUQueue` | All `%Q:GPUQueue:N` instances have the same fields and control defaults |

Definitions are immutable at runtime — they are resolved entirely at compile time.

## Constructor Definition Templates (`%definition.$`)

`{$}` constructor definitions live at `%definition.$:{ConstructorName}:{OverloadIndex}`. Each overload of a constructor name gets its own sequential index. Constructor definitions are compile-time-only — they have no runtime instances (constructor invocations produce `%$` variables).

| Definition path | Ensures |
|-----------------|---------|
| `%definition.$:DT:0` | "Today" keyword overload → `#DT.Date` |
| `%definition.$:DT:1` | "{hours}:{min}:{seconds}" string-parsing overload → `#DT.Time` |
| `%definition.$:Path:0` | "/{segments}" string-parsing overload → `#path` |

Each overload stores:

```aljam3
%definition.$:DT:1
├── .pattern                                <- compiled regex from pattern string
├── .targetType                             <- #DT.Time (from [$] binding)
├── .kind                                   <- #ConstructorKind (StringParsing, Keyword, NativePipeline)
└── .captures                               <- capture parameter definitions
    ├── :hours                              <- named capture
    │   └── .re                             <- regex pattern "[0-9][0-9]"
    ├── :min
    │   └── .re                             <- "[0-9][0-9]"
    └── :seconds
        └── .re                             <- "[0-9][0-9]"
```

Constructor definitions are immutable compile-time templates. The compiler uses them for overload resolution (regex matching), structural integrity checks (capture regex vs literal separators), and interpolation validation (only constructor-sourced `$var` allowed).

## Schema Definition Templates (`%definition.##`)

`##` schema types live at `%definition.##:{SchemaName}` in the metadata tree. Each schema defines tree-structure properties using the `%##` prefix. A `##` schema is a named bundle of `%##` properties — composing `[#] ##Flat` expands into the individual `%##` assignments stored at `%definition.##:Flat`. The `%##` properties are the ground truth; schemas are syntactic sugar over them:

```aljam3
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

```aljam3
%definition
├── .###:Value            <- leaf holds typed data (has #type annotation)
├── .###:Enum             <- leaf is variant selector (no #type annotation)
├── .###:ScalarValue      <- regex-validated string data (#String:* family, ##Scalar only)
├── .###:ScalarEnum       <- variant selector in scalar type (#Boolean, #NativeKind, ##Scalar only)
└── .###:None             <- nullable (empty string "")
```

The compiler infers `###Value` or `###Enum` from field declarations. Explicit `[#] ###Value` or `[#] ###Enum` is optional. A contradiction between explicit declaration and fields raises PGE11003.

## Schema Properties in Type Definitions (`%##`)

When a `{#}` definition includes `[#] %##Property` declarations or composes `##` schemas, the resolved properties appear as fixed fields under the type's definition template. Each `%##Property` written inside a `{#}` block is a relative metadata reference — the compiler resolves it to the absolute path `%definition.#:{TypeName}.%##Property`. The `%` prefix marks these as compile-time metadata entries, not runtime data:

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

```aljam3
%definition.#:String
├── %Native.Class
│   ├── .Rust                      -> "PgString"
│   └── .Validate                  -> #True
```

## Complete Type Definition Example

`#Array` definition template showing all metadata layers (generic type with `(#) <#ValueType` and `(#) <Dim` parameters):

```aljam3
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

```aljam3
%definition.#:Boolean
├── .%##Depth.Max              -> 1 (from ##Scalar)
├── .%##Alias                  -> "bool"
├── .%###ScalarEnum            <- inferred from .True/.False (no #type, ##Scalar context)
├── .True                      <- enum field
└── .False                     <- enum field
```

Schema properties are introspectable at compile time and enforce structural invariants (e.g., `%##Gap << #False` means the compiler rejects non-contiguous keys).

See also: [[object-types|Object Type Branches]], [[enum-rules|Enum Instance Rules]], [[string-subtypes|String Subtype Nesting]]
