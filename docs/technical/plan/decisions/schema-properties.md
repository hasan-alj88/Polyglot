---
audience: developer
type: decision
updated: 2026-04-09
---

# Decision: Schema Properties in `{#}` Definitions

**Date:** 2026-03-26
**Status:** Decided
**Issue:** #88

## Context

<!-- @types -->
<!-- @data-is-trees -->

All Polyglot data is trees of `#String` leaves (which have `RawString` leaves). The existing `.regex#RawString` field on `#String` ([[syntax/types/basic-types#Numeric Types — #String Subtypes]]) constrains **leaf values** via regex. However, **structural constraints** — key patterns, ordering, depth limits, openness — were implicit in compiler rules (PGE04001, PGE05001, PGE05002) with no declarative representation.

Collection types (`;array`, `;serial`, and the new `;dict`) need structural constraints that `.regex` cannot express. Rather than hardcoding each collection's rules in the compiler, we need a declarative mechanism.

## Decisions

### Schema Properties via `[#] %##` in `{#}` Blocks

> **Note:** The original #88 property names below (`%Key.Type`, `%Key.Gap`, etc.) were superseded by the `%##` prefix system in #272, then further refined in #275 where `%##Key` became `%##Fields`, `%##Range` became `%##Count`, and `%##Flexible`/`#FlexKind` were retired. See the current property table in [[definition-templates]].

`{#}` definitions gain **schema properties** declared with `[#] %##Property << value`. These are compile-time metadata that describe structural constraints on the type's tree shape. Original #88 property names (historical):

| Property | Type | Meaning |
|----------|------|---------|
| `%Key.Type` | type ref | *(now `%##Fields`)* Data type of keys at this level |
| `%Key.Gap` | `#Boolean` | *(now `%##Gap`)* Can keys have gaps? (`#False` = contiguous, `#True` = sparse) |
| `%Ordered` | `#Boolean` | *(now `%##Ordered`)* Are keys ordered? |
| `%Open` | `#Boolean` | *(removed — see below)* Can new keys be added at runtime? |
| `%Depth.Max` | `#Int` | *(now `%##Depth.Max`)* Max tree depth (`1` = flat, `-1` = unlimited) |

Schema properties live in the metadata tree at `%definition.#:{TypeName}.{Property}`, making them introspectable at compile time.

### Parameterized Types via Generic `{#}` Definitions

> **Note:** This section originally described type macros. As of Issue #272, macros are retired. Parameterized types now use generic `{#}` definitions with `(#) <#param` type inputs and `(#) <param` value inputs directly.

Generic `{#}` definitions declare parameters with `(#) <#Param` (type input) and `(#) <Param` (value input). The `:` separator in type annotations binds positionally to parameters:

```polyglot
{#} #Array
   (#) <#ValueType
   (#) <Dim##Dimension <~ "1D"
   [#] ##Array
      (#) <#ValueType << <#ValueType
      (#) <Dim << <Dim
```

`[#] ##Schema` sets `%##` properties — two schemas that agree on a property value produce no error; conflicting values produce PGE11001.

### No `.schema` Field on `#String`

The `{#}` definition at `%definition.#:{Type}` IS the schema. No separate `.schema#RawString` field is needed. The `#String` struct remains:

```polyglot
{#} #String
   [.] .string#RawString
   [.] .regex#RawString
```

Schema properties are part of the definition metadata, not part of string instances.

### Built-In Collection Definitions

`#Array`, `#Map`, and `#Serial` are **pglib-provided** `{#}` definitions. Users can create custom collection types using the same `[#] %` mechanism.

## Collection Type Definitions

### `#Array`

Generic `{#}` definition:

```polyglot
{#} #Array
   (#) <#ValueType
   (#) <Dim##Dimension <~ "1D"
   [#] ##Array
      (#) <#ValueType << <#ValueType
      (#) <Dim << <Dim
   [#] %##Alias << "array"
   [#] %##Fields << #Range
   [:] :*#<#ValueType
```

- Keys are contiguous unsigned integers — flexible fields, not fixed
- No gaps — removing an element reindexes
- Ordered by insertion
- Depth from dimension parameter (default 1D)
- Composed from ##Array (sets %##Gap, %##Ordered, %##Propagate)

### `#Map` *(Retired #275 — replaced by ##Record)*

> **Note:** As of Issue #275, `#Map`/`##Map` are retired. Use `##Record` for enum-keyed flat collections.

### `##Record` (replaces `##Map`)

Parameterized schema for enum-keyed flat collections:

```polyglot
{#} ##Record
   (#) <#Fields << ##Enum
   (#) <#ValueType <~ #
   [#] ##Flat
   [#] %##Fields << <#Fields
   [#] %##Active << #ActiveKind.All
   [#] %###Type << <#ValueType
   [#] %###Kind << #FieldKind.Value
```

- Fields keyed by an enum type (compile-time known)
- Flat (depth = 1)
- All fields active by default
- Value type uniform across all fields

### `#Serial`

```polyglot
{#} #Serial
   [#] %##Gap << #True
   [#] %##Ordered << #False
   [#] %##Depth.Max << #Inf
   [#] %##Count << #Inf
   [#] %##Fields << #Range
   [:] :*#*
```

- No type constraints on keys or values
- Gaps allowed
- Unordered
- Unlimited depth and count

### Regular Structs (No Schema Properties Needed)

```polyglot
{#} #Person
   [.] .name#string
   [.] .age;int
```

Structs with `[.]` fixed fields are self-describing — their field declarations ARE the schema. No `[#] %` properties needed. The compiler infers: closed, ordered by declaration, fixed depth, no gaps.

## Metadata Tree Impact

Schema properties become fields on the definition template:

```polyglot
%definition.#:Array
├── .Key
│   ├── .Type          → #Int
│   └── .Gap           → #Boolean (.False active)
├── .Ordered           → #Boolean (.True active)
├── .Open              → #Boolean (.True active)
└── .Depth
    └── .Max           → #Int (value = 1)
```

## Gap Resolutions (2026-03-27)

Design gaps identified during session review, all resolved:

| # | Gap | Resolution |
|---|-----|-----------|
| 1 | Generic param binding syntax | Generic `{#}` definitions with `(#) <#param`. Type annotations use `:` for positional binding (e.g., `#array:int`) |
| 2 | `#*` wildcard type + `[<]` constraints | `#*` is "any type" wildcard. `[<]` nested under `(#) <param` in `{#}` constrains via `##` schemas (e.g., `[<] << ##Scalar`) |
| 3 | Enum vs Value field kind | Implicit from syntax — no `%Kind` property. Document that no `#type` = enum field |
| 4 | Key uniqueness | Universal tree invariant — duplicate keys always error, including deserialized data. No `%Key.Unique` property |
| 5 | `[#]` overloading | Keep — prefix after `[#]` disambiguates (`.` field, `<` type param, `%` schema prop) |
| 6 | User-defined collection syntax | Resolved by Gap 1 — always `:` for type params |
| 7 | Array `.` → `:` migration | Execute during spec updates. `%Key.Gap` is compiler-enforced |

## Additional Decisions (2026-03-27)

These emerged during gap resolution and significantly evolve the original design:

### `;` Retired — `#` for Type Annotations

`#` replaces `;` as the type annotation character. Rationale: `#` = schema = datatype in Polyglot. Using a different symbol caused disconnect. Within `#type` context, nested type refs drop the `#` prefix (e.g., `#dict:string:int`).

### Schema Inheritance via `<~`

`(#) <~ #String` (not `<<`) for schema inheritance. `<~` = "default schema, can be specialized" — consistent with assignment semantics where `<~` is overridable default.

### `%Open` Property Removed

Collections are assembled at once via `*` collectors, not incrementally at runtime. `%Open` is not meaningful in Polyglot's async-collect model.

### `%Alias` Schema Property

New `[#] %Alias` property allows lowercase shorthand names (e.g., `%Alias << "int"` lets `#int` resolve to `#Int`). Multiple aliases allowed.

### `[<]` Type Parameter Constraints

Block marker nested under `(#) <param` declarations in generic `{#}` definitions. Constrains parameters via `##` schema references:
```polyglot
(#) <#ValueType
   [<] ##Scalar
   [ ] ValueType must satisfy ##Scalar schema
```

### #UnsignedInt Type

New `#String` subtype (alias `uint`, regex `^[0-9]+$`). Used for `%Key.Type` in #Array and as basis for #Dimension.

### #Dimension Type

New `#String` subtype (alias `dim`, regex `^[1-9][0-9]*$`). Used as #Array's second type parameter. `:2D` syntax sugar strips `D` suffix.

### #Sci and #Eng Types

Both approved (not deferred). #Sci regex: `^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$`. #Eng regex: `^-?[1-9]\.[0-9]{0,2}[eE][+-]?(0|[369]|[1-9][0-9]*[0369])$`.

### #Dataframe Type

New collection: array of dicts (tabular data). `%Depth.Max << 2` (row + column level). Usage: `$df#dataframe:string:float`, access: `$df:1:price`.

### %Depth.Max Inference Model

Depth counts flexible `:` levels only. Fixed `.` fields don't add depth. Compiler infers when not explicit:
- All `.` fields → depth 0 (record/scalar — eligible as array/dict element)
- Has `:` fields → depth ≥ 1 (collection — NOT eligible)
- Explicit `[#] %Depth.Max` → overrides inference

### Ground-Up Type Hierarchy

Complete hierarchy defined in `docs/draft.md`:
- Layer 0: RawString (compiler intrinsic)
- Layer 1: #String (foundation — .string + .regex)
- Layer 2: Scalars (#Int, #UnsignedInt, #Float, #Sci, #Eng, #Dimension)
- Layer 2b: #Boolean (##Enum type)
- Layer 3: Collections (#Array, #Map, #Dataframe, #Serial)

## Supersedes

- Issue #88 originally proposed `.schema#RawString` on `#String` — replaced by schema properties in `{#}` definitions
- No changes to `#String` struct (`.string` + `.regex` remain as-is)
- `;` type annotation syntax — replaced by `#`
- `%Open` schema property — removed (collections use `*` collectors)
- Old array fixed `.` keys — replaced by flexible `:` keys

## Related

- [[syntax/types/basic-types#Numeric Types — #String Subtypes]] — `.regex` field for value constraints
- [[data-is-trees#Schema vs Instance]] — definition/instance split
- [[metadata-tree/INDEX|spec/metadata-tree]] — `%definition` template paths
- Decision: [string-re-subfields](string-re-subfields.md) — `.regex` as value schema
- `docs/draft.md` — complete ground-up type hierarchy definitions

## Issue #275 Updates (2026-04-09)

- `#Map`/`##Map` retired — replaced by `##Record` (enum-keyed flat collection)
- `#Set`/`##Set` retired — replaced by `#Array` + `%###Unique << #True`
- `##Contiguous`, `##Sparse`, `##Rectangular`, `##Deep` retired — properties stated directly
- `%##Key` → `%##Fields` (`#FieldsDescriptor` or `##Enum` ref)
- `%##Range` → `%##Count` (`#Bound`)
- `%##Flexible`/`#FlexKind` → `%##Fields`
- `%##Regular` retired — consequence of `%##Propagate` + `%##Count`
- Schema composition syntax: `[#] ##Name` (drop `<<` for schemas); properties keep `<<`: `[#] %##Prop << value`
