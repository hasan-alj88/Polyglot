---
audience: pg-coder
type: specification
updated: 2026-04-09
---

# Basic Types

<!-- @syntax/types/INDEX -->

## Basic Types

All Polyglot data is serialized strings. The type system is a schema layer on top of strings that constrains how each string is interpreted. Types are organized in layers — each built from the one below.

### Layer 0: RawString — The Compiler Intrinsic

`RawString` is the only compiler intrinsic — it has no `{#}` definition. It is a sequence of literal raw characters: no interpolation, no substitutions, no escaping. Every character is literal. All other types are built FROM `RawString` via `{#}` definitions.

`RawString` literals use inline pipeline syntax: `=RawString"..."` or the alias `=rs"..."`. Users can annotate variables as `#RawString` directly, though this is rare — most code works with `#string` instead.

### Layer 1: #String — The Foundation Type

<!-- @c:types -->
What `#string` refers to is `#String` — a struct built on `RawString`:

```polyglot
{#} #String
   [ ] #String and #string both resolve here
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "string"
   [ ] The actual string value
   [.] .string#RawString
   [ ] Regex constraint — default accepts all strings
   [ ] <~ allows subtypes to override once to specialize
   [.] .regex#RawString <~ ".*"
   [%] %alias
      [:] "re"
```

- `.string` — the raw string value
- `.regex` — a regular expression constraint (alias: `.re`). Defaults to `".*"` (accept any string). Subtypes override with `<~` (default assignment — overridable once). See [[variable-lifecycle]]
- `%##Alias << "string"` — lets users write `#string` (lowercase) as shorthand for `#String`
- `[#] ##Scalar` — applies the `##Scalar` schema (sets `%##Depth.Max << 1` — `#String` is a scalar type with fixed fields at one level of depth)
- `[#] ###ScalarValue` — marks leaf content as regex-validated string data (`#String:*` family)

A string literal (quoted text with `{$var}` interpolation) is always `#string`. When `.regex` is set, the string value must match the pattern — violations are caught at compile time for literals (PGE04010) and at runtime for dynamic values (handled with `[!]` error blocks).

### Layer 2: Scalar Subtypes

Scalar subtypes are `#` types that compose the `##String` parameterized schema -- regex constraints on `#String`. A `##` schema is a metadata descriptor that the compiler enforces on `#` data structs -- `##` describes `#` the way `###` describes leaf fields. When you write `#int` in a type annotation, it resolves to `#String` with `##String` applied. The lowercase form (`int`) is an alias. Data instances live at `%#:String:int` on the metadata tree. See [[metadata-tree/string-subtypes#Alias Resolution]] for the full resolution table.

Each scalar type composes `##String` with a specific `<regex` parameter:

| Type | Alias | `.regex` pattern | Example values |
|------|-------|--------------|----------------|
| `#Int` | `int` | `^-?[0-9]+$` | `42`, `-7`, `007` |
| `#UnsignedInt` | `uint` | `^[0-9]+$` | `0`, `1`, `42` |
| `#Float` | `float` | `^-?[0-9]+\.[0-9]+$` | `3.14`, `-0.5`, `007.00` |
| `#Sci` | `sci` | `^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$` | `1e10`, `3.14e-2` |
| `#Eng` | `eng` | `^-?[0-9]+(\.[0-9]+)?[kKMGTPE]?$` | `1.5e3`, `2.47e-6` |
| `#Dimension` | `dim` | `^[0-9]+D$` | `0D`, `1D`, `2D`, `3D` |

Each subtype composes `##String` directly:

```polyglot
{#} #Int
   [%] %alias << "int,integer,Integer"
   [#] ##String
      (#) <regex << "^-?[0-9]+$"

{#} #UnsignedInt
   [%] %alias << "uint"
   [#] ##String
      (#) <regex << "^[0-9]+$"

{#} #Float
   [%] %alias << "float"
   [#] ##String
      (#) <regex << "^-?[0-9]+\.[0-9]+$"

{#} #Dimension
   [%] %alias << "dim"
   [#] ##String
      (#) <regex << "^[0-9]+D$"
```

The `##String` parameterized schema provides `##Scalar`, `###ScalarValue`, `.string`, and `.regex` fields internally. See [[scalars]] for all subtypes.

Users can still define custom string subtypes with their own `.regex`:

```polyglot
{#} #emailAddress
   [#] ##String
      (#) <regex << "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$"
```

Literal numeric values always match their RE by construction — no error handling needed.

### Layer 2c: #KeyString — Key Type for Tree Access

`#KeyString` is a string subtype that excludes characters reserved by Polyglot syntax — whitespace, `.`, `:`, `<`, `>`. This makes it safe for use as tree child keys accessed via the `<` operator:

```polyglot
{#} #KeyString
   (#) <~ #String
   [#] %##Alias << "key"
   [ ] Excludes whitespace, dot, colon, angle brackets
   [.] .regex#RawString << "^[^\s.<>:]+$"
```

Any type used as flexible child keys must compose `#KeyString`. If it does not, the compiler raises PGE11004 -- keys must exclude syntax-reserved characters to avoid compile ambiguity.

### Layer 2d: #NestedKeyString — Key Type for Alias Paths

`#NestedKeyString` is a string subtype that allows `.` and `:` separators but still excludes whitespace, `<`, and `>`. This makes it safe for alias paths that reference nested definitions (e.g., `!File.Permission.Denied`):

```polyglot
{#} #NestedKeyString
   (#) <~ #String
   [#] %##Alias << "nestedkey"
   [ ] Allows dot and colon; excludes whitespace and angle brackets
   [.] .regex#RawString << "^[^\s<>]+$"
```

Used as the element type for `%alias` — alias values may contain `.` and `:` to reference paths in the definition tree.

> **Note:** The full metadata path for `int` is `%#:String:int` — String subtypes are nested under `:String` at a flexible level. `#int` is an alias for `#String:int`. See [[data-is-trees#String Subtypes — Nested Under `:String`]] for how subtypes connect to the unified tree, and [[metadata#String Subtypes in the Tree]] for the complete type registry structure.

### Layer 2b: #Boolean — Independent `##Enum` Type

`#Boolean` is intentionally NOT a `#String` subtype. It is a `##Enum` type — a struct whose fields are all enum fields (no `#type` annotation). `.True` and `.False` are enum fields — exactly one is active at a time. This is a separate type tree from `#String`.

```polyglot
{#} #Boolean
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "bool"
   [.] .True
   [.] .False
```

- `bool` — alias for the `#Boolean` struct. See [[pglib/types/boolean|#Boolean]].

### Other Types

- `record` — enum-keyed, flat structure with typed value fields. Child access uses `<` operator (`$myRecord<name`). See [[concepts/collections/INDEX|collections]].
- `array` — range-indexed, ordered collection with typed elements and N-dimensional support. Child access uses `<` operator (`$myArray<0`). See [[concepts/collections/INDEX|collections]].
- `serial` — unconstrained. Any keys, any types, any depth. No compile-time validation of shape. Child access uses `<` operator (`$data<key`). See [[concepts/collections/INDEX|collections]].
- `dataframe` — row-oriented table. Two-level schema: L1 range-indexed rows, L2 `##Record` columns. Row access uses `<` (array index), column access chains a second `<`: `$df<row<column`. See [[concepts/collections/INDEX|collections]].
- struct (`{#}`) — defined schema. Compile-time enforced field names and types. See [[structs]].

## See Also

- [[syntax/types/INDEX|Type System Overview]] -- ground truths and type annotation rules
- [[syntax/types/schema-properties|Schema Properties]] -- `%##` / `%###` property reference
- [[syntax/types/hierarchy|Type Hierarchy]] -- full hierarchy summary and namespaced types
