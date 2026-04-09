---
audience: developer
rule: "8.4"
code: PGE08004
name: Ambiguous Step Reference
severity: error
---

### Rule 8.4 — Ambiguous Step Reference
`PGE08004`

**Statement:** In chain execution, step references in `(-)` IO lines and `[!]` error blocks must unambiguously identify a single step. When two or more steps share the same leaf name, using that leaf name alone is ambiguous and is a compile error. Two solutions: (1) extend the reference upward through the dotted name until it uniquely identifies the step, or (2) use numeric indices. Non-ambiguous references (unique leaf names) can use the short form. Mixing numeric and name-based references in the same chain is allowed.
**Rationale:** Step references are a readability convenience, but they must resolve to exactly one step. The issue is not that leaf names happen to match — it's that the reference is ambiguous. Extending the reference path or using numeric indices both eliminate the ambiguity while keeping chains readable.
**Detection:** The compiler resolves each step reference in `(-)` and `[!]` lines against the chain's step list. If a reference matches more than one step, PGE08004 fires.

**VALID:**
```polyglot
[ ] ✓ unique leaf names — short reference is unambiguous
[-] -File.Text.Read->-Text.Parse.CSV->-Report.Format
   (-) >Read.path#path << $path
   (-) <Format.result#string >> >output
```

```polyglot
[ ] ✓ shared leaf name — extended reference disambiguates
[-] -Text.Transform->-Data.Transform
   (-) >Text.Transform.input#string << $text   [ ] ✓ "Text.Transform" is unique
   (-) <Data.Transform.output#string >> >result [ ] ✓ "Data.Transform" is unique
```

```polyglot
[ ] ✓ shared leaf name — numeric indices disambiguate
[-] -Text.Transform->-Data.Transform
   (-) >0.input#string << $text
   (-) <1.output#string >> >result
```

```polyglot
[ ] ✓ mixed: extended name for duplicates, leaf name for unique
[-] -Text.Transform->-Data.Transform->-Report.Format
   (-) >Text.Transform.input#string << $text
   (-) <Data.Transform.output#string >> >Format.input
   (-) <Format.result#string >> >output
```

**INVALID:**
```polyglot
[ ] ✗ PGE08004 — "Transform" matches two steps
[-] -Text.Transform->-Data.Transform
   (-) >Transform.input#string << $text   [ ] ✗ PGE08004 — ambiguous reference
   (-) <Transform.output#string >> >result [ ] ✗ PGE08004 — ambiguous reference
```

```polyglot
[ ] ✗ PGE08004 — "Read" matches two steps in error reference
[-] -Text.Read->-Data.Read
   (-) >0.path#string << $path
   (-) <1.output#string >> >result
   [!] .Read!NotFound                     [ ] ✗ PGE08004 — which Read?
      [-] >result << "error"
```

**See also:**
- [PGE07002 — Chain Error Scoping](PGE07002-chain-error-scoping.md) — chain error addressing syntax
- [[user/concepts/pipelines/chains|Chains]] — references PGE08004 in step addressing rules
- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE08004 in step addressing table
