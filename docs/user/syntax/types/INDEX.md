---
audience: automation-builder
type: specification
updated: 2026-04-08
status: complete
---

# Type System

<!-- @syntax/types/INDEX -->
<!-- @c:identifiers -->
<!-- @c:variable-lifecycle -->
<!-- @u:technical/ebnf/04-type-system -->
<!-- @u:technical/edge-cases/04-type-system -->
<!-- @u:data-is-trees -->

## Ground Truths

These axioms define Polyglot's data model. All type system design follows from these facts:

1. **GT-1: ALL Polyglot objects are data trees with `#RawString` leafs** — types, definitions, pipelines, expanders, collectors, errors — everything is a tree
2. **GT-2: ALL data has a `%` metadata address** — both compile-time and runtime data lives on the unified metadata tree (future: backed by NoSQL DB)
3. **GT-3: `##` schemas are data trees that describe tree structure and constraints** — shape, depth, key patterns, ordering, uniformity, regex patterns. A `#` struct may have multiple `##` schema specifications. `##` schemas are NOT structs — they describe properties of structs
4. **GT-4: `#RawString` is the only primitive** — its limitations match the compiler host language; currently assumed theoretically unlimited
5. **GT-5: `#String` extends `#RawString` with regex enforcement** — from it derive all `##Scalar` types (`#Int`, `#Float`, `#Dimension`, `#KeyString`, etc.)
6. **GT-6: Not all tree properties can be expressed via regex** — hence `%` metadata properties (`##` schemas) describe tree shapes the compiler can identify and enforce
7. **GT-7: `#`, `##`, `###` are user-friendly aliases for `%` metadata** — users should never need to use `%` directly; the prefix tiers are the user-facing alternative
8. **GT-8: `{x}` = compile-time definitions; `-` pipelines = runtime definitions** — clean separation
9. **GT-9: `##` schemas can be parameterized** — parameterized schemas use `(#) <param` inputs to generate structural constraints at compose time
10. **GT-10: `#` types can be generic** — generic types use `(#) <param` inputs with `:` positional binding; compiler resolves all params to produce fully realized types

## Type Annotation

Types are annotated on variables using the `#` character. The `#` starts a **type context** — it signals "what type this holds." See [[identifiers]] for prefix rules and [[variable-lifecycle]] for how typed variables move through lifecycle stages:

```polyglot
[-] $IntValue#int <~ 42
[-] $StringValue#string <~ "Hello World"
[-] $ArrayValue#array << #Array
   (#) << 1
   (#) << 2
   (#) << 3
```

Each special character has one job:

| Character | Role |
|-----------|------|
| `@#-$!_` | Identity (what it is) |
| `.` `:` | Navigation (fixed/flexible fields) |
| `#` (after identifier) | Annotation (what type it holds) |

### Nested Type References

`#` starts the type context. Within that context, nested type references separated by `:` **drop the `#` prefix** — the compiler resolves them:

```polyglot
[ ] #int resolves to #Int via alias
[-] $score#int <~ 0

[ ] :Person resolves to #Person — no # needed after the first
[-] $users#array:Person

[ ] :string → #String, :int → #Int
[-] $map#dict:string:int

[ ] :float → #Float, :2D → #Dimension (value 2)
[-] $matrix#array:float:2D
```

**Rule:** `#` always starts the type context. After the first `#`, type parameters separated by `:` drop the prefix.

## Sub-Pages

| File | Covers |
|------|--------|
| [[prefix-system]] | Three-tier prefix system, < operator |
| [[basic-types]] | RawString, #String, int, float, scalars |
| [[schema-properties]] | ## schema properties |
| [[generic-types]] | Generic types and parameterized schemas |
| [[arrays]] | Element-typed and multidimensional arrays |
| [[structs]] | Struct types, inline data, enum vs value fields |
| [[flexible-fields]] | Typed flexible fields |
| [[strings]] | String interpolation, path type |
| [[conversions]] | Type identity (structural matching) and conversions |
| [[hierarchy]] | Namespaced types, hierarchy summary, live modifier |
