---
rule: "6.6"
code: PGE-606
name: String Exhaustiveness
severity: error
split_from: PGE-601
---

### Rule 6.6 — String Exhaustiveness
`PGE-606`

**Statement:** A `[?]` conditional branching on a `string` value must include a `[?] *?` catch-all branch. String values are an open set — the compiler cannot know all possible values at compile time, so static exhaustiveness proof is impossible.
**Rationale:** Strings are unbounded. Unlike enums (PGE-602) or numeric ranges (PGE-603), the compiler cannot verify that all string values are covered by listed branches. The `*?` catch-all is the only way to guarantee exhaustiveness.
**Detection:** When a `[?]` block branches on a `string`-typed variable, the compiler checks for a `[?] *?` branch. If absent, PGE-606 fires.

**VALID:**
```polyglot
[ ] ✓ string conditional — known values + *? catch-all
[?] $status
   [?] "active"
      [r] =Process.Active
   [?] "paused"
      [r] =Process.Paused
   [?] *?
      [r] =Process.Unknown
```

**INVALID:**
```polyglot
[ ] ✗ PGE-606 — string without *?, no static proof possible
[?] $status
   [?] "active"
      [r] =Process.Active
   [?] "paused"
      [r] =Process.Paused
   [ ] ✗ PGE-606 — missing *?, string is open type
```

**See also:**
- [PGE-601 — Conditional Must Be Exhaustive](PGE-601-conditional-must-be-exhaustive.md) — parent rule
- [PGE-602 — Enum Exhaustiveness](PGE-602-enum-exhaustiveness.md) — closed-set counterpart

### See Also

- [[user/concepts/conditionals|Conditionals]] — string exhaustiveness rules reference PGE-606
