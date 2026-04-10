---
audience: developer
rule: "6.6"
code: PGE06006
name: String Exhaustiveness
severity: error
split_from: PGE06001
---

### Rule 6.6 — String Exhaustiveness
`PGE06006`

<!-- @u:syntax/blocks -->

**Statement:** A `[?]` conditional branching on a `string` value must include a `[?] *?` catch-all branch. String values are an open set — the compiler cannot know all possible values at compile time, so static exhaustiveness proof is impossible.
**Rationale:** Strings are unbounded. Unlike enums (PGE06002) or numeric ranges (PGE06003), the compiler cannot verify that all string values are covered by listed branches. The `*?` catch-all is the only way to guarantee exhaustiveness.
**Detection:** When a `[?]` block branches on a `string`-typed variable, the compiler checks for a `[?] *?` branch. If absent, PGE06006 fires.

**VALID:**
```polyglot
[ ] ✓ string conditional — known values + *? catch-all
[?] $status
   [?] "active"
      [-] -Process.Active
   [?] "paused"
      [-] -Process.Paused
   [?] *?
      [-] -Process.Unknown
```

**INVALID:**
```polyglot
[ ] ✗ PGE06006 — string without *?, no static proof possible
[?] $status
   [?] "active"
      [-] -Process.Active
   [?] "paused"
      [-] -Process.Paused
   [ ] ✗ PGE06006 — missing *?, string is open type
```

**See also:**
- [PGE06001 — Conditional Must Be Exhaustive](PGE06001-conditional-must-be-exhaustive.md) — parent rule
- [PGE06002 — Enum Exhaustiveness](PGE06002-enum-exhaustiveness.md) — closed-set counterpart

### See Also

- [[user/concepts/conditionals|Conditionals]] — string exhaustiveness rules reference PGE06006
