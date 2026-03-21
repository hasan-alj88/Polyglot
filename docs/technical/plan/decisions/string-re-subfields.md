# Decision: String RE Subfield System

**Date:** 2026-03-20
**Status:** Decided

## Context

All Polyglot data is serialized strings. `int` and `float` were already described as "matched via regular expression patterns" (see decision: numeric-re-patterns). The question was how to formalize the RE mechanism for all string-based types.

## Decisions

### RawString is the True Primitive

`RawString` is the most primitive datatype in Polyglot — a compiler intrinsic. It is a sequence of literal raw characters with no interpolation, no substitutions, no escaping. Every character is literal.

`RawString` literals use inline pipeline syntax: `=RawString"..."` or aliased `=rs"..."`.

### #String is a Struct

What `;string` refers to in type annotations is `#String` — a struct built on `RawString`:

```polyglot
{#} #String
   [.] .string;RawString
   [.] .re;RawString
```

- `.string` — the raw string value
- `.re` — the regular expression constraint (defaults to `""`, accept any string)

`.re` follows variable lifecycle: defaults to `""` via `<~`, can be pushed once to Final via `<<`. No concept of "mutable/immutable" — only lifecycle states.

### int/float are #String Subtypes

`int` and `float` are not separate struct types. They are flexible-level subtypes of `#String`:

```
#String
   [:] :*  ← flexible level for subtypes
      :int    (.re << "^-?[0-9]+$")
      :float  (.re << "^-?[0-9]+\.[0-9]+$")
      :eng    (deferred)
      :sci    (deferred)
      :customName  ← user-defined
```

Full metadata path for `int`: `%#:String:int` (`;int` is an alias for `;String.int`)

Users can define custom string subtypes:

```polyglot
{#} #String.emailAddress
   [.] .string;RawString
   [.] .re;RawString << "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$"
```

### bool is NOT a #String Subtype

`#Boolean` is a separate struct using enum fields (not RE-based value fields). Full metadata path: `%#:Boolean:{instance}.{True, False}`. Enum instances collapse to one active field — see [[Metadata Data Tree]].

### RE Declaration Syntax

No new grammar needed. RE is declared via standard subfield assignment:

```polyglot
{#} #Email
   [.] .address;string
      [.] .re << "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$"
```

### Validation Timing

- **Compile-time** — string literals and provable constants checked against RE (PGE-410)
- **Runtime** — dynamic values checked when pushed; failures handled with `[!]` error blocks
- Literal numeric values (int/float) always match their RE by construction — no error handling needed

### Proposed RE Patterns for eng/sci

| Type | Proposed RE | Description |
|------|------------|-------------|
| `sci` | `^-?[0-9]+(\.[0-9]+)?[eE][+-]?[0-9]+$` | Scientific notation (e.g., `1.23e-4`, `6.022e23`) |
| `eng` | `^-?[0-9]+(\.[0-9]+)?[eE][+-]?(0\|3\|6\|9\|12\|15\|18\|21)$` | Engineering notation — exponent restricted to multiples of 3 |

**Status:** Proposed, not yet decided. Syntax and exact patterns TBD.

## Deferred

- Metadata Data Tree full specification — will elaborate `%Data.#:*:String:*` paths
- RE composition (array elements inheriting RE from struct fields)
- WindowsPath / UnixPath RE patterns for `#path`

## Updated Files

- `docs/user/syntax/types.md` — Basic Types rewritten, RawString added
- `docs/technical/EBNF.md` — `RawString` added to `basic_type`
- `docs/technical/brainstorming/string-re-subfields.md` — marked decided
- `docs/technical/plan/TODO.md` — brainstorm resolved
- `docs/audit/reference/glossary.md` — RawString term added
