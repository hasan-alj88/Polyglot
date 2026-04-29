---
audience: design
type: spec
updated: 2026-04-09
---

<!-- @ebnf/INDEX -->

## 9.11 Metadata Block

```ebnf
metadata_line       ::= "[%]" metadata_expr ;

metadata_expr       ::= metadata_fixed
                      | metadata_info
                      | metadata_alias
                      | metadata_live ;

(* Fixed schema fields — all use . fixed separator *)
metadata_fixed      ::= fixed_sep "description" [ type_annotation ] assignment_op string_literal
                      | fixed_sep "authors" [ type_annotation ] assignment_op inline_data
                      | fixed_sep "version" [ type_annotation ] assignment_op string_literal
                      | fixed_sep "license" [ type_annotation ] assignment_op string_literal
                      | fixed_sep "deprecated" assignment_op data_id ;

(* Info field — serial type, opens flexible scope *)
metadata_info       ::= fixed_sep "info" type_annotation NEWLINE
                         { indent "[%]" flex_sep name [ type_annotation ] assignment_op value_expr NEWLINE } ;

(* Alias — binds short names to parent definition or field *)
metadata_alias      ::= "%" "alias" NEWLINE
                         indent flex_sep string_literal NEWLINE
                         { indent flex_sep string_literal NEWLINE } ;
                      (* At least one alias name required — PGE12004 *)

(* Live fields — Aljam3-managed, read-only, implicit *)
metadata_live       ::= fixed_sep name ";" "live" type_expr ;
```

**Rules:**
- `[%]` appears inside any `{x}` definition (`{#}`, `{-}`, `{T}`, `{W}`, `{N}`).
- One definition = one metadata set (class-level, not instance-level).
- All top-level `[%]` fields use `.` fixed separator. Only `.info#serial` opens a `:` flexible scope underneath (sibling homogeneity preserved).
- `[%] %alias` declares shorthand names for definitions or fields. Each `[:]` child is a `#NestedKeyString` alias name. Multiple aliases per definition are allowed. All aliases must be globally unique (PGE12002).
- Aliases participate in exhaustiveness checking when the variable carries the parent type annotation.
- `live` fields are implicit on all `{-}`, `$`, and `{#}` definitions. The runtime populates them. Users read via `%` accessor (e.g., `-Pipeline%status`, `$var%state`) but never assign.
- Prefer reactive alternatives (error blocks, IO triggers) over polling `live` fields when possible.
- Native definitions use `{N}` — a separate block type (see [[definition-blocks/09-06-native|9.6]]). `{N}` metadata implicitly scopes to `%Native.*` with fixed fields `.Kind` (`#NativeKind`), `.<Language>` (native function binding), and `.description`. `{N}` and `{-}` are mutually exclusive block types — a definition cannot be both native and derived (PGE01028).

## Related User Documentation

| Section | User Doc |
|---------|----------|
| 9.11 `[%]` Metadata | [[concepts/metadata\|metadata]] |
