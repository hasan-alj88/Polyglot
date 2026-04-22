---
audience: design
type: spec
updated: 2026-04-03
---

# Field Expansion

<!-- @source:metadata-tree/INDEX -->

Any field typed `#string` expands to the full `#String` struct in the tree:

```polyglot
.description#string
  -> .description
      .string#RawString     <- the raw value
      .regex#RawString       <- the regex constraint (default: ".*" = accept any)
```

This expansion applies recursively — `#array:string` expands each element's `.string` and `.regex` subfields.

## Related

- [[technical/ebnf/INDEX|EBNF]] — formal grammar productions for `[%]` blocks
- [[COMPILE-RULES]] — error/warning code reference
- [[type-identity|spec/type-identity]] — structural type matching rules
- [[data-is-trees|user/concepts/data-is-trees]] — user-facing tree overview
- [[metadata|user/concepts/metadata]] — user-facing `[%]` usage and `live` field accessors
- [Metadata Data Tree Decision](../../plan/decisions/metadata-data-tree.md) — original design decision (2026-03-21)

See also: [[string-subtypes|String Subtype Nesting]], [[definition-templates|Definition Templates]]
