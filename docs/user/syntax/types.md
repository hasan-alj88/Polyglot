---
audience: user
type: specification
updated: 2026-03-27
status: complete
---

# Type System

<!-- @identifiers -->
<!-- @variable-lifecycle -->

## Type Annotation

Types are annotated on variables using the `#` character. The `#` starts a **type context** — it signals "what type this holds." See [[identifiers]] for prefix rules and [[variable-lifecycle]] for how typed variables move through lifecycle stages:

```polyglot
[r] $IntValue#int <~ 42
[r] $StringValue#string <~ "Hello World"
[r] $ArrayValue#array <~ {1, 2, 3, 4, 5}
```

Each special character has one job:

| Character | Role |
|-----------|------|
| `@#=$!_` | Identity (what it is) |
| `.` `:` | Navigation (fixed/flexible fields) |
| `#` (after identifier) | Annotation (what type it holds) |

### Nested Type References

`#` starts the type context. Within that context, nested type references separated by `:` **drop the `#` prefix** — the compiler resolves them:

```polyglot
[ ] #int resolves to #Int via alias
[r] $score#int <~ 0

[ ] :Person resolves to #Person — no # needed after the first
[r] $users#array:Person

[ ] :string → #String, :int → #Int
[r] $map#dict:string:int

[ ] :float → #Float, :2D → #Dimension (value 2)
[r] $matrix#array:float:2D
```

**Rule:** `#` always starts the type context. After the first `#`, type parameters separated by `:` drop the prefix.

## Basic Types

All Polyglot data is serialized strings. The type system is a schema layer on top of strings that constrains how each string is interpreted. Types are organized in layers — each built from the one below.

### Layer 0: RawString — The Compiler Intrinsic

`RawString` is the only compiler intrinsic — it has no `{#}` definition. It is a sequence of literal raw characters: no interpolation, no substitutions, no escaping. Every character is literal. All other types are built FROM `RawString` via `{#}` definitions.

`RawString` literals use inline pipeline syntax: `=RawString"..."` or the alias `=rs"..."`. Users can annotate variables as `#RawString` directly, though this is rare — most code works with `#string` instead.

### Layer 1: #String — The Foundation Type

<!-- @types -->
What `#string` refers to is `#String` — a struct built on `RawString`:

```polyglot
{#} #String
   [ ] #String and #string both resolve here
   [#] %Alias << "string"
   [ ] Scalar — no flexible children, no collection nesting
   [#] %Depth.Max << 0
   [ ] The actual string value
   [.] .string#RawString
   [ ] Regex constraint — default accepts all strings
   [ ] <~ allows subtypes to override once to specialize
   [.] .re#RawString <~ ".*"
```

- `.string` — the raw string value
- `.re` — a regular expression constraint. Defaults to `".*"` (accept any string). Subtypes override with `<~` (default assignment — overridable once). See [[variable-lifecycle]]
- `%Alias << "string"` — lets users write `#string` (lowercase) as shorthand for `#String`
- `%Depth.Max << 0` — formally marks this as a scalar (no flexible children)

A string literal (quoted text with `{$var}` interpolation) is always `#string`. When `.re` is set, the string value must match the pattern — violations are caught at compile time for literals (PGE-410) and at runtime for dynamic values (handled with `[!]` error blocks).

### Layer 2: Scalar Subtypes — Specialize `.re`

All scalar subtypes inherit `#String`'s schema via `[#] <~ #String` and override `.re` with a specific regex. The `<~` operator means "default schema, can be specialized further" — consistent with assignment semantics where `<~` is an overridable default.

| Type | Alias | `.re` pattern | Example values |
|------|-------|--------------|----------------|
| `#Int` | `int` | `^-?[0-9]+$` | `42`, `-7`, `007` |
| `#UnsignedInt` | `uint` | `^[0-9]+$` | `0`, `1`, `42` |
| `#Float` | `float` | `^-?[0-9]+\.[0-9]+$` | `3.14`, `-0.5`, `007.00` |
| `#Sci` | `sci` | `^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$` | `1e10`, `3.14e-2` |
| `#Eng` | `eng` | `^-?[1-9]\.[0-9]{0,2}[eE][+-]?(0\|[369]\|[1-9][0-9]*[0369])$` | `1.5e3`, `2.47e-6` |
| `#Dimension` | `dim` | `^[1-9][0-9]*$` | `1`, `2`, `3` |

Each subtype sets `.re` with `<<` (final — cannot be overridden further):

```polyglot
{#} #Int
   [ ] Inherits #String schema (.string, .re)
   [#] <~ #String
   [#] %Alias << "int"
   [ ] Matches: 42, -7, 0, 007
   [.] .re#RawString << "^-?[0-9]+$"

{#} #UnsignedInt
   [ ] Non-negative integers — array keys, dimensions
   [#] <~ #String
   [#] %Alias << "uint"
   [ ] Matches: 0, 1, 42, 007
   [.] .re#RawString << "^[0-9]+$"

{#} #Float
   [#] <~ #String
   [#] %Alias << "float"
   [ ] Matches: 3.14, -0.5, 007.00
   [.] .re#RawString << "^-?[0-9]+\.[0-9]+$"

{#} #Sci
   [#] <~ #String
   [#] %Alias << "sci"
   [ ] Scientific notation with optional decimal
   [ ] Matches: 1e10, 3.14e-2, -5E+3
   [.] .re#RawString << "^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$"

{#} #Eng
   [#] <~ #String
   [#] %Alias << "eng"
   [ ] Engineering notation: exponents are multiples of 3
   [ ] Matches: 1.5e3, 2.47e-6, 9.99e12
   [.] .re#RawString << "^-?[1-9]\.[0-9]{0,2}[eE][+-]?(0|[369]|[1-9][0-9]*[0369])$"

{#} #Dimension
   [ ] Positive integers only — array dimension parameters
   [ ] The "D" suffix in :2D usage is syntax sugar — :2D means dimension value = 2
   [#] <~ #String
   [#] %Alias << "dim"
   [ ] Matches: 1, 2, 3, 10
   [.] .re#RawString << "^[1-9][0-9]*$"
```

Users can define custom string subtypes with their own `.re`:

```polyglot
{#} #emailAddress
   [#] <~ #String
   [.] .re#RawString << "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$"
```

Literal numeric values always match their RE by construction — no error handling needed.

> **Note:** The full metadata path for `int` is `%#:String:int` — String subtypes are nested under `:String` at a flexible level. `#int` is an alias for `#String:int`. See [[data-is-trees#String Subtypes — Nested Under `:String`]] for how subtypes connect to the unified tree, and [[metadata#String Subtypes in the Tree]] for the complete type registry structure.

### Layer 2b: #Boolean — Independent Enum Struct

`#Boolean` is intentionally NOT a `#String` subtype. Booleans are enum fields (no `#type`), not string values with regex. `.True` and `.False` are enum fields — exactly one is active at a time. This is a separate type tree from `#String`.

```polyglot
{#} #Boolean
   [#] %Alias << "bool"
   [ ] Scalar — leaf node, NOT a #String subtype
   [#] %Depth.Max << 0
   [.] .True
   [.] .False
```

- `bool` — alias for the `#Boolean` struct. See [[STDLIB#Boolean]].

### Other Types

- `array` — ordered, contiguous collection with typed elements and N-dimensional support. Element access uses `:` flexible fields (`:0`, `:1`, `:2`, ...). See [[collections]].
- `dict` — unordered, sparse key-value pairs with typed keys and values. See [[collections]].
- `dataframe` — array of dicts (tabular data). See [[collections]].
- `serial` — schema-free. Always uses flexible fields (`:`), even if dot notation is used in access. Any keys, any types, any depth. No compile-time validation of shape. Easily converts to/from JSON-like formats. See [[collections]].
- struct (`{#}`) — defined schema. Compile-time enforced field names and types. See [[#Struct Types]].

## Schema Properties

`{#}` definitions gain **schema properties** declared with `[#] %Property`. These are compile-time metadata that describe structural constraints on the type's tree shape:

| Property | Type | Meaning |
|----------|------|---------|
| `%Key.Type` | type ref | Data type of keys at this level |
| `%Key.Gap` | `#Boolean` | Can keys have gaps? (`#False` = contiguous, `#True` = sparse) |
| `%Ordered` | `#Boolean` | Are keys ordered? |
| `%Depth.Max` | `#Int` | Max tree depth (`0` = scalar, `1` = flat, `-1` = unlimited) |
| `%Alias` | `#String` | Lowercase shorthand name for the type |

Schema properties live in the metadata tree at `%definition.#:{TypeName}.{Property}`, making them introspectable at compile time.

### %Depth.Max — Inference Model

`%Depth.Max` describes how many levels of **flexible** (`:`) nesting a type supports. Fixed (`.`) fields define static schema structure and do NOT count as depth.

| Value | Meaning | Examples |
|-------|---------|---------|
| `0` | Scalar/record — no flexible children | #String, #Int, #Boolean, #Person (all `.` fields) |
| `1` | One level of flexible children | #Array (1D), #Dict |
| `N` | N levels of flexible nesting | #Array with `:ND` dimension |
| `-1` | Unlimited flexible nesting | #Serial |

**Compiler inference:** When a `{#}` definition does not explicitly set `%Depth.Max`, the compiler infers it:
- **All `.` fixed fields** → `%Depth.Max = 0` (record/scalar)
- **Has `:` flexible fields** → `%Depth.Max` = count of nested `:` levels
- **Explicit `[#] %Depth.Max`** → overrides inference

This means structs like `#Person` (with `.name#string`, `.age#int`) are automatically depth 0 and CAN be used as array/dict elements. A struct with `[:] :*#Handler` has depth 1 and CANNOT.

## Generic Type Parameters

`{#}` definitions support **generic type parameters** using the `<` prefix — consistent with IO input semantics (the type parameter is an "input" to the definition):

```polyglot
{#} #Array<ValueType<Dim
   [#] <ValueType << #*
   [#] <Dim << #Dimension

{#} #Dict<KeyType<ValueType
   [#] <KeyType << #*
   [#] <ValueType << #*
```

Multiple type parameters chain with `<`: `#Name<param1<param2`. In usage, parameters bind positionally via `:` separators: `#array:int`, `#dict:string:int`, `#array:float:2D`.

### `#*` Wildcard Type

`#*` is the "any type" wildcard. In type parameter defaults, `<ValueType << #*` means "accepts any type." In field declarations, `:*#*` means "any key, any value type."

### `[<]` Type Parameter Constraints

`[<]` blocks nested under `[#] <param` declarations constrain type parameters via `%` schema properties:

```polyglot
{#} #Array<ValueType<Dim
   [#] <ValueType << #*
      [ ] ValueType must be scalar/record (depth 0)
      [<] %Depth.Max << 0
   [#] <Dim << #Dimension
      [ ] Dimension must be scalar
      [<] %Depth.Max << 0
```

The `[<]` constraint declares that any type passed as `ValueType` must have `%Depth.Max = 0` — preventing nested collections like `#array:#array:#int`.

## Element-Typed Arrays

Arrays specify their element type using `:` (flexible field) notation:

```polyglot
[r] $files#array:path <~ {}
[r] $names#array:string <~ {}
[r] $scores#array:int <~ {}
```

This constrains the array to hold only elements of the specified type.

## Multidimensional Arrays

Arrays support a dimension specifier using an `<N>D` suffix. Omitting the dimension defaults to 1D:

```polyglot
[=] <items#array:string              [ ] 1D array (default)
[=] <matrix#array:float:2D           [ ] 2D matrix of floats
[=] <cube#array:int:3D               [ ] 3D cube of ints
[=] <hyper#array:float:4D            [ ] 4D hypercube of floats
```

Element access uses colon-separated integer indices. The number of indices must match the declared dimension count:

```polyglot
[r] $val << $items:0                 [ ] 1 index for 1D
[r] $val << $matrix:0:1              [ ] 2 indices for :2D
[r] $val << $cube:2:3:0              [ ] 3 indices for :3D
```

The compiler enforces access depth — too many or too few indices triggers PGE-417. Nested array types (`#array:#array:X`) remain banned (PGE-412) — use `:ND` instead.

## Struct Types

<!-- @blocks -->
<!-- @identifiers -->
A **struct** is any type with a defined schema, declared with `{#}` (see [[blocks]]). The term "struct" refers to the type category — `{#}` is the declaration syntax. Every struct definition creates a schema template at `%definition.#:{StructName}` and instances at `%#:{StructName}:{n}` — see [[data-is-trees#Schema vs Instance]].

Stdlib structs include `path`, `#Boolean`, `#OS`, `#PipelineStatus`, `#VarState`, and `#DateTime`. User-defined structs follow the same rules.

In type annotations (after `#`), nested type refs drop the `#` prefix — the compiler knows `#` starts a type context:

```polyglot
[r] $user#UserRecord <~ ...
[r] $users#array:UserRecord <~ {}
[=] <incoming#Alert
```

The `#` prefix is only used when **referencing** a struct outside of type annotations (e.g., `#Boolean.True`, `@alias#DataName.EnumField`).

### Struct Level Rules

Each level in a struct must be homogeneous in two ways (see [[identifiers#Serialization Rules]]):

1. **Separator homogeneity** — all siblings at a level must be all fixed (`.`) or all flexible (`:`) — never mixed (PGE-501)
2. **Kind homogeneity** — all siblings at a level must be all enum fields or all value fields — never mixed (PGE-502)

A field whose type is itself a struct defines the later levels. It is invalid to declare sub-levels after a field typed as a struct — the struct's definition already specifies those levels.

```polyglot
{#} #Config
   [.] .timeout#int
   [.] .server#ServerInfo      [ ] later levels defined by #ServerInfo

{#} #ServerInfo
   [.] .host#string
   [.] .port#int
```

## Inline Data Shorthand

Curly braces with comma-separated values create inline data:

```polyglot
[r] $values#array <~ {1, 2, 3, 4, 5}
```

This is shorthand for explicit field assignment: `#data:1 << 1`, `#data:2 << 2`, etc.

## Enum Fields vs Value Fields

In `{#}` struct definitions, fields are either **enum fields** or **value fields**:

| Field Type | Has `#type`? | Has assignment? | Example |
|------------|-------------|-----------------|---------|
| Enum | No | No | `[.] .Critical` |
| Value | Yes | Optional | `[.] .code#int <~ 500` |

**Rules:**
- No type annotation (`#type`) implies an **enum field**
- Enum fields always use `[.]` fixed fields
- All siblings at the same level must be the same kind (all enum or all value)
- Enum fields can nest value sub-fields

```polyglot
{#} #Severity
   [.] .Critical
   [.] .Error
   [.] .Info
   [.] .Other
      [ ] Enum field with nested value sub-fields
      [.] .code#int <~ 500
```

Reference enum values cross-package: `@alias#DataName.EnumField` (e.g., `@Alert#Severity.Critical`). See [[packages#Usage]] for import syntax and [[identifiers#Serialized Identifiers]] for `.` fixed-field navigation.

## Typed Flexible Fields

A struct can have levels that use flexible (`:`) fields. At such a level, the `[:] :*#Type` syntax declares that ALL `:` siblings share the same type — a typed dictionary. The `:*` wildcard means "collectively, every key at this level has this type."

```polyglot
{#} #Registry
   [.] .builtins
      [.] .http#Handler
      [.] .grpc#Handler
   [.] .plugins
      [:] :*#Handler
```

Here `.plugins` has flexible children. Every `:key` under `.plugins` must be `#Handler`. Users can push `:myPlugin`, `:anotherPlugin`, etc. — all constrained to `#Handler`.

### Schema Enforcement on New Keys

When a typed flexible level references a struct type, all new keys inherit that struct's schema:

```polyglot
{#} #SubStruct
   [.] .level4#string

{#} #Example
   [.] .level1
      [.] .level2
         [:] :*#SubStruct
```

To create a new key `:new` alongside existing `:level3`: push to `#Example.level1.level2:new.level4` — the compiler knows `:new` is `#SubStruct`, so `.level4#string` is enforced.

### Constraints

- **No extra levels** — a flexible field's children are fully defined by its type annotation. You cannot insert additional levels between the flexible field and its typed children.
- **Multi-level flexibility** — a struct can have multiple flexible levels (each level independently homogeneous):

```polyglot
{#} #DeepFlex
   [.] .config
      [:] :*#Section

{#} #Section
   [:] :*#Setting

{#} #Setting
   [.] .value#string
   [.] .default#string
```

## String Interpolation

Strings support variable interpolation using `{$variable}` inside string literals. Any `$`-prefixed identifier inside `{...}` within a double-quoted string is expanded to its value:

```polyglot
[r] $greeting#string << "Hello {$name}, you are {$age} years old"
[r] $path#string << "/users/{$userId}/profile"
```

Interpolation works with any `$`-prefixed variable, including flexible-field paths:

```polyglot
[r] $msg#string << "User {$user:name} logged in from {$user:location}"
```

For literal curly braces inside strings, use `{{` and `}}`.

## Live Type Modifier

`live` is a type modifier reserved for `[%]` metadata fields managed by the Polyglot runtime. Users can read `live` fields via the `%` accessor but never assign to them. The type uses dot notation: `#live.#PipelineStatus`, `#live.int`, `#live.array:error`.

`live` fields are **implicit** on every `{=}` pipeline, `$` variable, and `{#}` struct. They do not need to be declared — the runtime populates them automatically and updates them in real-time.

See [[metadata]] for the full metadata tree, all `live` field listings, and access patterns.

## Path Type

`path` is a stdlib struct with OS-specific subfields:

```polyglot
{#} #path
   [.] .Unix#string
   [.] .Windows#string
```

### Explicit Subfield Assignment

Assign both subfields so code works cross-platform:

```polyglot
[r] $AppDir#path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"
```

At runtime, the Polyglot runtime resolves `$AppDir` to the correct subfield based on the current OS (see `=Sys.OS` in [[STDLIB]]).

Assigning only one subfield triggers a portability warning (PGW-408). If the missing subfield is for the current OS, the compiler raises an error (PGE-408).

A plain string cannot be assigned to a `#path` variable — `[r] $dir#path << "/tmp"` is a type mismatch (PGE-401). Use `=Path"..."` instead.

### `=Path"..."` Inline Notation

`=Path"..."` is an inline pipeline call ([[STDLIB#=Path]], [[pipelines#Inline Pipeline Calls]]) that creates a `#path` value from a string:

```polyglot
[r] $LogDir#path << =Path"/tmp/MyApp/logs"
[r] $AppDir#path << =Path"{.}/MyApp"
```

Both `/` and `\` are treated as path separators and normalized to the correct separator for the current OS. These two lines produce identical results:

```polyglot
[r] $a#path << =Path"{.}\MyApp\logs"
[r] $b#path << =Path"{.}/MyApp/logs"
[ ] $a and $b resolve to the same path on any OS
```

`{$var}` interpolation works inside `=Path"..."` strings — interpolated variables must be `#path` values with both OS subfields defined (e.g., `{.}`, `{..}`, or a user-defined `#path` variable). `{{` and `}}` produce literal brace characters, same as in regular string interpolation.

### Path Roots and Interpolation

Define a root path, then build on it with interpolation:

```polyglot
[r] $Root#path
   [.] .Unix << "/tmp"
   [.] .Windows << "C:"

[ ] renders as "/tmp/MyApp" on Unix, "C:\MyApp" on Windows
[r] $AppDir#path << =Path"{$Root}/MyApp"
```

Path interpolation with `{$pathVar}` inside `=Path"..."` resolves the path variable to its OS-appropriate subfield.

### File Path Shorthands

- `{.}` — current file's directory (`#path` value, defined for all OS)
- `{..}` — parent directory (`#path` value, defined for all OS)

These are equivalent to a built-in `$cfd` variable and are available in any path context, including `=Path"..."` calls and `[@]` multi-file package references (see [[packages#Multi-File Packages]]).

### Trigger Path Strings

Trigger inline string arguments that contain file paths parse as path strings:

```polyglot
[t] =T.Folder.NewFiles"/inbox/"
```

The `"/inbox/"` argument is parsed as a path string — separators are normalized per OS, same as `=Path"..."`.

### Related

- `=Path` — stdlib pipeline for creating `#path` values from strings. See [[STDLIB#=Path]]
- `#OS` — stdlib enum with `.Unix` and `.Windows` variants. See [[STDLIB]]
- `=Sys.OS` — stdlib pipeline that yields `>os#OS`. See [[STDLIB]]
- PGE-407 — invalid path string (compile error)
- PGE-408 — missing path platform subfield (compile error)
- PGW-408 — single-platform path (warning)

## Type Conversions

### Struct → Serial

Always allowed. A struct's fixed (`.`) fields are converted to flexible (`:`) fields in the serial. The struct is always a valid subset of serial's openness.

```polyglot
{#} #UserRecord
   [.] .name#string
   [.] .age#int

[r] $user#UserRecord
   [r] $user.name << "Alice"
   [r] $user.age << 30

[ ] struct → serial is always safe
[r] $data#serial << $user
[ ] $data now has :name and :age as flexible fields
```

### Serial → Struct

Allowed only if the serial's fields satisfy the struct's schema. Extra fields in the serial are ignored; missing fields are an error.

The compiler performs best-effort static analysis:
- **Provably matches** — no handling needed
- **Provably mismatches** — PGE-402 (schema mismatch)
- **Cannot prove match** — user must handle with `[!]` + `*Continue >FallBack`. If absent → PGE-409

```polyglot
{#} #UserRecord
   [.] .name#string
   [.] .age#int

[ ] handled — *Continue provides fallback if serial doesn't match
[r] $defaultUser#UserRecord
   [r] $defaultUser.name << "Unknown"
   [r] $defaultUser.age << 0

[r] $user#UserRecord << $dynamicSerial
   [!] !SchemaMismatch
      [*] *Continue >FallBack << $defaultUser
```

See [TYPE-IDENTITY](../../technical/compile-rules/TYPE-IDENTITY.md) rules 5 and 6, [PGE-409](../../technical/compile-rules/PGE/PGE-409-unhandled-serial-struct-conversion.md).

## Namespaced Types

Types use dot notation for namespaces — these are fixed schema fields. Namespacing is optional for basic types but available when needed (e.g., referencing enumeration fields from `{#}` definitions).

```polyglot
[ ] Direct type annotation — most common
[r] $score#int <~ 0

[ ] Fully qualified — equivalent to the above
[r] $score#String:int <~ 0

[ ] Struct enum field — must use # outside type annotations
[r] $severity << #Severity.Critical

[ ] Cross-package reference — @alias#DataName.Field
[r] $status << @alerts#Severity.Error
```

In type annotations (after `#`), nested type refs drop the `#` prefix — the compiler knows `#` starts a type context. Outside annotations, struct references keep the `#` prefix. See [[identifiers#Serialized Identifiers]] for the full prefix rules.

## Type Hierarchy Summary

```
RawString (compiler intrinsic)
└── #String (foundation — .string + .re)
    ├── #Int (.re = signed integers)
    ├── #UnsignedInt (.re = non-negative integers)
    ├── #Float (.re = decimals)
    ├── #Sci (.re = scientific notation)
    ├── #Eng (.re = engineering notation)
    ├── #Dimension (.re = positive integers — array dimensions)
    └── (user-defined: #emailAddress, #phoneNumber, etc.)

#Boolean (independent enum struct — NOT #String)

#Array<ValueType<Dim (ordered, contiguous, typed elements, N-dimensional)
#Dict<KeyType<ValueType (unordered, sparse, typed K-V pairs)
#Dataframe<KeyType<ValueType (array of dicts — tabular data)
#Serial (schema-free, unlimited depth)
```
