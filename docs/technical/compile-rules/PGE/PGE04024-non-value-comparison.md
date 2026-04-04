---
rule: "4.24"
code: PGE04024
name: Non-Value Comparison
severity: error
---

### Rule 4.24 — Non-Value Comparison
`PGE04024`

**Statement:** Comparison operators (`=?`, `>?`, `<?`, etc.) require both operands to be value types (`$` variables, literals, `<`/`>` IO params, field paths). Pipeline identifiers (`=Name`), collection operators (`*Into.*`), and expander operators (`~ForEach.*`) are not value types and cannot be compared.
**Rationale:** Pipelines are operations, not data values. Comparing two pipeline identifiers has no defined semantics — there is no ordering or equality relation between operations. Only data values can be meaningfully compared.
**Detection:** The compiler checks that both operands of a comparison expression resolve to value types. If either operand is a pipeline/collection/expander identifier, PGE04024 fires.

**VALID:**
```polyglot
[ ] ✓ comparing value types
[?] $count >? 10
   [r] =HandleOverflow

[ ] ✓ comparing enum values
[?] $status =? #Status.Active
   [r] =Process
```

**INVALID:**
```polyglot
[ ] ✗ PGE04024 — pipeline identifiers are not value types
[?] =Pipeline.A =? =Pipeline.B
   [r] $same << #Boolean.True
```

**Diagnostic:** "Cannot compare `=Name` — comparison requires value types, not operations"
