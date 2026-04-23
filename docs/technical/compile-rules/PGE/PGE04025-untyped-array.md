---
audience: developer
rule: "4.25"
code: PGE04025
name: Untyped Array
severity: error
status: grammar-enforced
---

# Rule 4.25 — Untyped Array (Grammar-Enforced)
`PGE04025`

<!-- @u:syntax/io -->

**Status:** Grammar-enforced as of #306. The `array_type` production now requires `element_type_param` — `#array` without an element type is a grammar error, not a semantic compile rule.

**Previous behavior:** The compiler checked that every `#array` type annotation included an element type. This is now enforced at the grammar level via `element_type_param ::= basic_type | user_type`.

**See:** [[technical/ebnf/04-type-system#4.1 Type Annotations]], EC-4.19, EC-4.21.
