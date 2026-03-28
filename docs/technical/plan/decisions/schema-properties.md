# Decision: Schema Properties in `{#}` Definitions

**Date:** 2026-03-26
**Status:** Decided
**Issue:** #88

## Context

<!-- @types -->
<!-- @data-is-trees -->

All Polyglot data is trees of `#String` leaves (which have `RawString` leaves). The existing `.re;RawString` field on `#String` ([[types#Numeric Types — #String Subtypes]]) constrains **leaf values** via regex. However, **structural constraints** — key patterns, ordering, depth limits, openness — were implicit in compiler rules (PGE-401, PGE-501, PGE-502) with no declarative representation.

Collection types (`;array`, `;serial`, and the new `;dict`) need structural constraints that `.re` cannot express. Rather than hardcoding each collection's rules in the compiler, we need a declarative mechanism.

## Decisions

### Schema Properties via `[#] %` in `{#}` Blocks

`{#}` definitions gain **schema properties** declared with `[#] %Property`. These are compile-time metadata that describe structural constraints on the type's tree shape:

| Property | Type | Meaning |
|----------|------|---------|
| `%Key.Type` | type ref | Data type of keys at this level |
| `%Key.Gap` | `#Boolean` | Can keys have gaps? (`#False` = contiguous, `#True` = sparse) |
| `%Ordered` | `#Boolean` | Are keys ordered? |
| `%Open` | `#Boolean` | Can new keys be added at runtime? |
| `%Depth.Max` | `#Int` | Max tree depth (`1` = flat, `-1` = unlimited) |

Schema properties live in the metadata tree at `%definition.#:{TypeName}.{Property}`, making them introspectable at compile time.

### Generic Type Parameters via `<param`

`{#}` definitions support **generic type parameters** using the `<` prefix — consistent with IO input semantics (the type parameter is an "input" to the definition):

```polyglot
{#} #Array<type
   [#] <type << ;*           [ ] accepts any type

{#} #Map<keyType<valueType
   [#] <keyType << ;*        [ ] key type parameter
   [#] <valueType << ;*      [ ] value type parameter
```

Multiple type parameters chain with `<`: `#Name<param1<param2`. In usage, parameters bind via the existing separator conventions: `;array.int` (fixed `.` for array element type), `;map:string:int` (flexible `:` for map K:V types).

### No `.schema` Field on `#String`

The `{#}` definition at `%definition.#:{Type}` IS the schema. No separate `.schema;RawString` field is needed. The `#String` struct remains:

```polyglot
{#} #String
   [.] .string;RawString
   [.] .re;RawString
```

Schema properties are part of the definition metadata, not part of string instances.

### Built-In Collection Definitions

`#Array`, `#Map`, and `#Serial` are **stdlib-provided** `{#}` definitions. Users can create custom collection types using the same `[#] %` mechanism.

## Collection Type Definitions

### `#Array`

```polyglot
{#} #Array<type<dim
   [#] <type << ;*
   [#] <dim << ;int
   [#] %Key.Type << #Int
   [#] %Key.Gap << #False
   [#] %Ordered << #True
   [#] %Open << #True
   [#] %Depth.Max << dim
   [:] :*;type
```

- Keys are contiguous integers (`:0`, `:1`, `:2` ...) — flexible fields, not fixed
- No gaps — removing an element reindexes
- Ordered by insertion
- Open — elements can be added
- Depth from dimension parameter (default 1)
- **Note:** This changes arrays from `[.]` fixed to `[:]` flexible keys — access becomes `$arr:0` not `$arr.0`

### `#Map`

```polyglot
{#} #Map<keyType<valueType
   [#] <keyType << ;*
   [#] <valueType << ;*
   [#] %Key.Type << keyType
   [#] %Key.Gap << #True
   [#] %Ordered << #False
   [#] %Open << #True
   [#] %Depth.Max << 1
   [:] :*;valueType
```

- Keys typed by `keyType` parameter (must be a `#String` subtype)
- Gaps allowed (sparse keys)
- Unordered
- Open — keys can be added
- Flat only (depth = 1)

### `#Serial`

```polyglot
{#} #Serial
   [#] %Key.Gap << #True
   [#] %Ordered << #False
   [#] %Open << #True
   [#] %Depth.Max << -1
   [:] :*;*
```

- No type constraints on keys or values
- Gaps allowed
- Unordered
- Open
- Unlimited depth

### Regular Structs (No Schema Properties Needed)

```polyglot
{#} #Person
   [.] .name;string
   [.] .age;int
```

Structs with `[.]` fixed fields are self-describing — their field declarations ARE the schema. No `[#] %` properties needed. The compiler infers: closed, ordered by declaration, fixed depth, no gaps.

## Metadata Tree Impact

Schema properties become fields on the definition template:

```
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
| 1 | Generic param binding syntax | Positional via `:` — ALL type params are flexible (user-extensible). `;array.int` becomes `#array:int` |
| 2 | `#*` wildcard type + `[<]` constraints | `#*` is "any type" wildcard. `[<]` nested under `[#] <param` constrains via `%` properties (e.g., `[<] %Depth.Max << 0`) |
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

`[#] <~ #String` (not `<<`) for schema inheritance. `<~` = "default schema, can be specialized" — consistent with assignment semantics where `<~` is overridable default.

### `%Open` Property Removed

Collections are assembled at once via `*` collectors, not incrementally at runtime. `%Open` is not meaningful in Polyglot's async-collect model.

### `%Alias` Schema Property

New `[#] %Alias` property allows lowercase shorthand names (e.g., `%Alias << "int"` lets `#int` resolve to `#Int`). Multiple aliases allowed.

### `[<]` Type Parameter Constraints

New block marker nested under `[#] <param` declarations. Constrains type parameters via `%` schema properties:
```polyglot
[#] <ValueType << #*
   [<] %Depth.Max << 0
   [ ] ValueType must be scalar (depth 0)
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
- Layer 1: #String (foundation — .string + .re)
- Layer 2: Scalars (#Int, #UnsignedInt, #Float, #Sci, #Eng, #Dimension)
- Layer 2b: #Boolean (independent enum struct)
- Layer 3: Collections (#Array, #Map, #Dataframe, #Serial)

## Supersedes

- Issue #88 originally proposed `.schema;RawString` on `#String` — replaced by schema properties in `{#}` definitions
- No changes to `#String` struct (`.string` + `.re` remain as-is)
- `;` type annotation syntax — replaced by `#`
- `%Open` schema property — removed (collections use `*` collectors)
- Old array fixed `.` keys — replaced by flexible `:` keys

## Related

- [[types#Numeric Types — #String Subtypes]] — `.re` field for value constraints
- [[data-is-trees#Schema vs Instance]] — definition/instance split
- [[metadata-tree|spec/metadata-tree]] — `%definition` template paths
- Decision: [string-re-subfields](string-re-subfields.md) — `.re` as value schema
- `docs/draft.md` — complete ground-up type hierarchy definitions
