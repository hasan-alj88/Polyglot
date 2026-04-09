---
audience: pg-coder
type: spec
status: complete
updated: 2026-04-09
---

<!-- @concepts/collections/INDEX -->

# Collections

<!-- @glossary:Polyglot Code -->
<!-- @operators -->
<!-- @blocks -->
Collections in Polyglot Code ([[glossary#Polyglot Code]]) are data structures that hold multiple items. They are processed using expand (`~`) and collect (`*`) operators — see [[operators#Collection Operators]] and [[blocks#Data Flow]] for block element reference. Expand operators live at `%~` and collect operators at `%*` in the metadata tree — see [[data-is-trees#How Concepts Connect]].

## Collection Hierarchy

Every collection in Polyglot is a tree. `#Map` is the universal flat key-value collection. `#Array` is a `#Map` variant where keys are contiguous integer enums. `#Serial` drops all schema constraints — it accepts any compilable tree.

| Type | Is a | Key type | Key structure |
|------|------|----------|--------------|
| `#Map:K:V` | Base collection (generic) | `K` (any key type) | Flat key-value |
| `#Array:V:Dim` | Contiguous collection (generic) | `#UnsignedInt` | Cartesian product of 0..n-1 per dimension |
| `#Dataframe:E:C` | Row-oriented table (generic) | `##Enum` columns | Array of Map -- each row is a map |
| `#Set:V` | Unique value collection (generic) | (sparse) | No duplicates |
| `#Serial` | Unconstrained tree | Any | Unlimited depth, no constraints |

User-defined structs (`{#}`) define fixed-field types with `.` accessor. Collections use `<` for flexible children. These two accessors can be combined: `$sales<0.product`.

For type annotations, type hierarchy, and schema properties, see [[syntax/types/INDEX|types]].

## Collection Types Summary

Collections are **assembled at once** using collect operators (`*` prefix) — not incrementally added to at runtime. Collections are populated via collectors (`*Into.Array`, `*Into.Map`, `*Into.Serial`, etc.) and are structurally complete after collection.

## Sub-Pages

| File | Covers |
|------|--------|
| [[map]] | #Map base collection |
| [[array]] | #Array map variant |
| [[serial]] | #Serial unconstrained tree |
| [[user-struct]] | User-defined struct as collection |
| [[dataframe]] | #Dataframe, nested safety |
| [[expand]] | ~ expand operators |
| [[collect]] | * collect operators, collect-all & race |
| [[examples]] | Expand/transform/collect examples, fallback |
