---
audience: developer
type: spec
updated: 2026-03-31
status: complete
---

# Metadata Tree Specification

<!-- @user/concepts/data-is-trees -->
<!-- @user/concepts/metadata -->
<!-- @EBNF -->

This document formally specifies the `%` metadata tree — the unified structure that organizes all Polyglot objects. For the user-friendly introduction, see [[data-is-trees|user/concepts/data-is-trees]]. For field listings and access patterns, see [[metadata|user/concepts/metadata]].

## Path Grammar

The general path patterns:

```ebnf
schema_path     ::= "%" "definition" "." type_prefix ":" ref
                   | "%" "definition" ".##:" ref
                   | "%" "definition" ".###:" ref
instance_path   ::= "%" type_prefix ":" ref ":" instance { "." field }
permission_path ::= "%" "_" { "." field }
error_path      ::= "%" "!" "." namespace { "." leaf }
                   | "%" "!" ".Error" { ":" user_path }
package_path    ::= "%" "@" ":" registry { ":" id_part } "::" name { ":" segment }
schema_prop     ::= "%" "##" property_name { "." sub_property }
field_type_prop ::= "%" "###" property_name
```

| Element | Rule |
|---------|------|
| `type_prefix` | One of: `#`, `##`, `###`, `=`, `W`, `Q`, `~`, `*`, `$`, `M`, `!`, `@`, `_` |
| `ref` | Object name — flexible field (`:`) |
| `instance` | Instance number — flexible field (`:`) |
| `field` | Fixed field path (`.`) within the instance |

**Branch-specific rules:**
- `%_` — all `.` fixed fields, no `:` anywhere. No ref or instance levels.
- `%!` — `.` for Polyglot-defined namespaces and leaves. `.Error` children use `:` (user-extensible).
- `%@` — `::` separates registry+ID from package name. `::` is treated as `:` in the tree.

### Shorthand in User Code

User code uses shorthand accessors that resolve to full instance paths:

| Shorthand | Resolves to |
|-----------|-------------|
| `=MyPipeline%status` | `%=:MyPipeline:<current>.status` |
| `$myVar%state` | `%$:myVar:<current>.state` |
| `#Record%lastModified` | `%#:Record:<current>.lastModified` |
| `=W.DB.Connection%status` | `%W:DB.Connection:<current>.status` |
| `#Queue:GPUQueue%activeCount` | `%Q:GPUQueue:<current>.activeCount` |

The `:<current>` segment is implicit — the runtime resolves it to the calling context's instance.

## Object Type Branches

The `%` root has fixed branches for each object type prefix:

| Branch | Objects | Name level | Description |
|--------|---------|------------|-------------|
| `%#` | Structs | Flexible (`:type`) | All `{#}` type definitions |
| `%=` | Pipelines | Flexible (`:name`) | All `{=}` pipeline definitions |
| `%~` | Expanders | Flexible (`:name`) | All `~ForEach.*` expand operators |
| `%*` | Collectors | Flexible (`:name`) | All `*Into.*`, `*Agg.*`, `*All`, `*First`, `*Nth` |
| `%$` | Variables | Flexible (`:name`) | All `$`-prefixed variables |
| `%W` | Wrappers | Flexible (`:name`) | All `{W}` wrapper definitions |
| `%Q` | Queues | Flexible (`:name`) | All `{Q}` queue definitions |
| `%M` | Macros | Flexible (`:name`) | All `{M}` macro definitions |
| `%!` | Errors | Fixed (`.namespace`) | Polyglot-defined namespaces; `.Error` has flexible `:` children |
| `%@` | Packages | Flexible (`:<registry>:<id>::<name>`) | All `@`-prefixed package addresses; `::` separates registry from name |
| `%_` | Permissions | All fixed (`.`) | All `_`-prefixed permission declarations; no instances, no `:` levels |

Plus `%definition` (fixed) for compile-time schema templates — including `%definition.#:{TypeName}` for type definitions, `%definition.=:{PipelineName}` for pipeline definitions, `%definition.W:{WrapperName}` for wrapper definitions, `%definition.Q:{QueueName}` for queue definitions, `%definition.##:{SchemaName}` for `##` schema definitions, and `%definition.###:{FieldTypeName}` for `###` field type definitions.

No `%Data` prefix exists — instance paths go directly to `%{type}:{ref}:{instance}.{fields}`.

## Instance Lifecycle

### Creation

An instance is created when:
- A pipeline is triggered (`%=:Name:N` where N is the next sequential number)
- A wrapper is invoked via `[W]` (`%W:Name:N`)
- A queue dispatches a pipeline (`%Q:Name:N`)
- A variable is declared (`%$:name:N`)
- An expand/collect operator begins execution (`%~:Name:N`, `%*:Name:N`)

### Numbering

Instances use sequential zero-based numbering: `:0`, `:1`, `:2`, etc. Numbers are scoped to the definition — each pipeline, variable, or operator tracks its own counter.

### Release

Instances are released when:
- A pipeline completes or fails (all stages resolved)
- A wrapper completes cleanup or fails
- A queue is destroyed (all assigned pipelines released)
- A variable leaves scope ([[variable-lifecycle#Released]])
- An operator finishes collection

Released instances are no longer addressable. Their `live` metadata is discarded.

## String Subtype Nesting

String subtypes live under `%#:String:*` at a flexible level:

```
%#:String
├── :int          ← .string#RawString + .regex = "^-?[0-9]+$"
├── :uint         ← .string#RawString + .regex = "^[0-9]+$"
├── :float        ← .string#RawString + .regex = "^-?[0-9]+\.[0-9]+$"
├── :sci          ← .string#RawString + .regex = scientific notation
├── :eng          ← .string#RawString + .regex = engineering notation
├── :dim          ← .string#RawString + .regex = "^[1-9][0-9]*$"
├── :emailAddress ← user-defined: .regex = custom pattern
└── :(any)        ← extensible — users define new subtypes
```

### Alias Resolution

User code `#int` is an alias for `#Int`. The `%##Alias` schema property enables this — each subtype declares `[#] %##Alias << "int"` (lowercase shorthand). The compiler resolves:

| User writes | Compiler resolves to | Tree path |
|-------------|---------------------|-----------|
| `#int` | `#Int` | `%#:String:int` |
| `#uint` | `#UnsignedInt` | `%#:String:uint` |
| `#float` | `#Float` | `%#:String:float` |
| `#string` | `#String` | `%#:String` |
| `#emailAddress` | `#String.emailAddress` | `%#:String:emailAddress` |

All subtypes share the `#String` schema (`.string#RawString` + `.regex#RawString`) with `.regex` pre-filled per subtype.

## Enum Instance Rules

### Active-Field-Only Invariant

An enum instance collapses to exactly ONE active field. The definition (`%definition.#:{EnumName}`) lists all valid branches, but a specific instance has only the active one:

```
%definition.#:Boolean       ← schema: .True, .False (both listed)
%#:Boolean:0.True           ← instance 0: .True is active
                              .False does NOT exist for this instance
```

### Architecture Safeguards

The runtime enforces exactly one active enum field per instance:

| Operation | Behavior |
|-----------|----------|
| Push new field | Atomically clears previous field, sets new one |
| Read active field | Returns the field's value/path |
| Read non-active field | Returns no path (field does not exist on this instance) |
| Multi-set same instance in same scope | Compiler rejects (PGE prevents) |

## IO Port Nesting

`.<` (inputs) and `.>` (outputs) are fixed typed data sections within each pipeline, expander, and collector instance:

```
%=:ProcessData:0
├── .<                      ← input ports
│   ├── .filepath#path
│   └── .options#serial
└── .>                      ← output ports
    └── .content#string
```

Parameter names within `.<` and `.>` are flexible — they follow the pipeline's `[=]` IO declarations.

Wrappers use `.[{]` (inputs) and `.[}]` (outputs) instead of `.<`/`.>`:

```
%W:DB.Connection:0
├── .[{]                     ← wrapper inputs
│   └── .connectionString#string
└── .[}]                     ← wrapper outputs
    └── .dbConn
```

Parameter names within `.[{]` and `.[}]` are flexible — they follow the wrapper's `[{]`/`[}]` declarations.

## Wrapper Branch

`%W` stores wrapper definitions (`{W}`). Wrappers provide setup/cleanup scope around pipeline execution bodies. Each `[W]` invocation in a pipeline creates a new wrapper instance.

### Structure

```
%W:DB.Connection:0
├── .[{]                     ← inputs from calling pipeline
│   └── .connectionString#string
├── .[}]                     ← outputs exposed to calling pipeline
│   └── .dbConn
├── .setup                   ← [\] setup phase
└── .cleanup                 ← [/] cleanup phase
```

### Key Properties

- **Flexible instances** — each `[W]` invocation creates `%W:Name:N` with sequential numbering, like pipelines.
- **IO via `[{]`/`[}]`** — wrapper inputs (`[{]`) and outputs (`[}]`) are fixed typed data sections, analogous to `.<`/`.>` in pipelines.
- **Composite wrappers** — a `{W}` definition can contain `[W]` references to other wrappers inside `[\]` or `[/]`, creating nested wrapper instances.
- **`live` fields** — wrapper instances report runtime state: `status`, `errors`, `setupDuration`. See [[metadata|user/concepts/metadata]].

## Queue Branch

`%Q` stores queue definitions (`{Q}`). Queues manage pipeline dispatch ordering and concurrency. Each queue dispatches pipelines and tracks active/pending counts.

### Structure

```
%Q:GPUQueue:0
├── .strategy#QueueStrategy        ← FIFO, LIFO, Priority
├── .maxInstances#int              ← max parallel instances
├── .retrigger#RetriggerStrategy   ← Disallow, Queue, Replace
└── .controls                      ← active queue controls
    ├── .pause
    ├── .resume
    └── .kill
```

### Key Properties

- **Flexible instances** — each queue use creates `%Q:Name:N` with sequential numbering.
- **Fields are fixed** — `.strategy`, `.maxInstances`, `.retrigger` are Polyglot-defined fixed fields.
- **Active controls** — nested `[Q]` lines within the definition set default pause/resume/kill behavior.
- **`live` fields** — queue instances report runtime state: `pendingCount`, `activeCount`, `totalProcessed`. See [[metadata|user/concepts/metadata]].

## Permission Branch

`%_` stores permission declarations. Unlike other branches, `%_` has **no `:{instance}` level** and **no `:` flexible fields** — permissions are compile-time declarations with an entirely fixed schema. All categories and capabilities are Polyglot-defined, not user-extensible. See [[permissions]] for the full permission system.

### Structure

```
%_
├── .File
│   ├── .read               #string  (glob pattern)
│   ├── .write              #string
│   ├── .execute            #string
│   └── .delete             #string
├── .Web
│   ├── .request
│   │   └── .<              (IO inputs)
│   └── .socket
│       └── .<
├── .Database
│   ├── .connect
│   │   └── .<
│   ├── .read               #string
│   └── .write              #string
├── .System
│   ├── .env                #string
│   ├── .process
│   │   └── .<
│   └── .signal             #string
├── .Crypto
│   ├── .key, .sign, .encrypt   #string
├── .IPC
│   ├── .send, .receive
│   │   └── .<
│   └── .subscribe          #string
├── .Device
│   ├── .camera, .microphone, .location, .bluetooth   #bool
└── .Memory
    ├── .allocate, .shared   #string
```

### Key Properties

- **All fixed (`.`)** — every level uses `.` fixed-field navigation. No `:` flexible fields anywhere in `%_`. Permission categories and capabilities are predefined by Polyglot.
- **No instances** — permissions are per-definition, resolved at compile time. No runtime metadata exists.
- **No `live` fields** — all permission data is static. The compiler resolves permissions entirely during compilation.
- **Nested under `%@` and `%=`** — permissions also appear as `._` subsections under package (`%@:<address>._`) and pipeline (`%=:<name>:<instance>._`) branches, representing the package ceiling and pipeline-level declarations respectively.
- **IO-form capabilities** — capabilities like `.request`, `.connect`, `.send` use `.<` for their IO input parameters, mirroring the IO form syntax in `[_]` declarations.

## Definition Templates

`%definition.{type}:{ref}` stores the compile-time structural template for each object. All instances `%{type}:{ref}:{n}` must conform to their definition's structure.

| Definition path | Ensures |
|-----------------|---------|
| `%definition.#:UserRecord` | All `%#:UserRecord:N` instances have `.name#string`, `.age#int` |
| `%definition.=:ProcessData` | All `%=:ProcessData:N` instances have the same IO ports and `live` fields |
| `%definition.W:DB.Connection` | All `%W:DB.Connection:N` instances have the same `[{]`/`[}]` IO and scope structure |
| `%definition.Q:GPUQueue` | All `%Q:GPUQueue:N` instances have the same fields and control defaults |

Definitions are immutable at runtime — they are resolved entirely at compile time.

### Schema Definition Templates (`%definition.##`)

`##` schema types live at `%definition.##:{SchemaName}` in the metadata tree. Each schema defines tree-structure properties using the `%##` prefix:

```
%definition
├── .##:Scalar
│   └── .%##Depth.Max          → 0
├── .##:Flat
│   └── .%##Depth.Max          → 1
├── .##:Deep
│   └── .%##Depth.Max          → -1
├── .##:Homogeneous
│   └── .%##Children.Uniform   → #True
├── .##:Heterogeneous
│   └── .%##Children.Uniform   → #False
├── .##:Contiguous
│   ├── .%##Children.Gap       → #False
│   └── .%##Children.Ordered   → #True
├── .##:Sparse
│   └── .%##Children.Gap       → #True
└── .##:Rectangular
    ├── .%##Children.Regular   → #True
    └── .%##Children.Uniform   → #True
```

Schema definitions are immutable compile-time templates. When a `{#}` type composes a schema via `[#] << ##Flat`, the schema's `%##` properties are inherited into the type's definition.

### Field Type Definition Templates (`%definition.###`)

`###` field types live at `%definition.###:{FieldTypeName}`:

```
%definition
├── .###:Value       ← leaf holds typed data (has #type annotation)
└── .###:Enum        ← leaf is variant selector (no #type annotation)
```

The compiler infers `###Value` or `###Enum` from field declarations. Explicit `[#] << ###Value` or `[#] << ###Enum` is optional. A contradiction between explicit declaration and fields raises PGE11003.

### Schema Properties in Type Definitions (`%##`)

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

### Field Type Properties (`%###`)

Field-level metadata uses the `%###` prefix. The `###` classification describes the nature of leaf nodes within a type:

| Property | Applies to | Meaning |
|----------|-----------|---------|
| `%###Value` | Types with `#type`-annotated fields | Leaves hold typed data |
| `%###Enum` | Types with unannotated enum fields | Leaves are variant selectors |

All siblings must be the same `###` kind — mixing typed and untyped fields among siblings raises PGE05005.

### Complete Type Definition Example

`#Array` definition template showing all metadata layers:

```
%definition.#:Array
├── .%##Depth.Max              → (from Dim parameter)
├── .%##Children
│   ├── .Type                  → #UnsignedInt
│   ├── .Gap                   → #Boolean (.False active)
│   ├── .Uniform               → #Boolean (.True active)
│   ├── .Regular               → #Boolean (.True active)
│   └── .Ordered               → #Boolean (.True active)
├── .%##Alias                  → "array"
├── .%###Value                 ← inferred from :*#ValueType (typed field)
└── :*#ValueType               ← flexible children
```

The `%##` properties are accumulated from composed schemas: `##Flat` provides `%##Depth.Max << 1`, `##Contiguous` provides `%##Children.Gap << #False` and `%##Children.Ordered << #True`, `##Rectangular` provides `%##Children.Regular << #True` and `%##Children.Uniform << #True`. Redundant properties raise PGW11001; contradicting overrides raise PGW11002.

`#Boolean` definition template showing `###Enum`:

```
%definition.#:Boolean
├── .%##Depth.Max              → 0 (from ##Scalar)
├── .%##Alias                  → "bool"
├── .%###Enum                  ← inferred from .True/.False (no #type)
├── .True                      ← enum field
└── .False                     ← enum field
```

Schema properties are introspectable at compile time and enforce structural invariants (e.g., `%##Children.Gap << #False` means the compiler rejects non-contiguous keys).

## Field Expansion

Any field typed `#string` expands to the full `#String` struct in the tree:

```
.description#string
  → .description
      .string#RawString     ← the raw value
      .regex#RawString       ← the regex constraint (default: ".*" = accept any)
```

This expansion applies recursively — `#array:string` expands each element's `.string` and `.regex` subfields.

## Related

- [[technical/ebnf/INDEX|EBNF]] — formal grammar productions for `[%]` blocks
- [[COMPILE-RULES]] — error/warning code reference
- [[type-identity|spec/type-identity]] — structural type matching rules
- [[data-is-trees|user/concepts/data-is-trees]] — user-facing tree overview
- [[metadata|user/concepts/metadata]] — user-facing `[%]` usage and `live` field accessors
- [Metadata Data Tree Decision](../plan/decisions/metadata-data-tree.md) — original design decision (2026-03-21)
