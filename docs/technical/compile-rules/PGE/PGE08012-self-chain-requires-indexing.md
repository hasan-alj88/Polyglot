---
rule: "8.12"
code: PGE08012
name: Self-Chain Requires Numeric Indexing
severity: error
---

### Rule 8.12 — Self-Chain Requires Numeric Indexing
`PGE08012`

**Statement:** When a pipeline is chained to itself (`=A => =A`), all chain IO lines must use numeric step indexing (`>0.output`, `<1.input`) to disambiguate which instance of the pipeline is being addressed. Without numeric indexing, the compiler cannot determine which step's IO is referenced, and PGE08012 fires.
**Rationale:** Self-chains are valid — a pipeline can run twice sequentially (e.g., applying a transformation twice). However, because both steps have identical IO parameter names, numeric indexing is mandatory to avoid ambiguity. This is a special case of the general chain IO addressing rules.
**Detection:** The compiler detects when a chain references the same pipeline multiple times and checks that all `[=]` IO lines use numeric step prefixes (`>N.` / `<N.`).

**VALID:**
```polyglot
[ ] ✓ self-chain with numeric indexing
[r] =Process => =Process
   [=] >0.input << $data
   [=] <1.result >> >output
```

**INVALID:**
```polyglot
[ ] ✗ PGE08012 — self-chain without numeric indexing
[r] =Process => =Process
   [=] >input << $data
   [=] <result >> >output
```

**Diagnostic:** "Self-chain `=Name => =Name` requires numeric step indexing (`>0.`, `<1.`) to disambiguate IO"
