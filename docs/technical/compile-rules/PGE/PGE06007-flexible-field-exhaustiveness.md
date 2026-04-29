---
audience: developer
rule: "6.7"
code: PGE06007
name: Flexible Field Exhaustiveness
severity: error
split_from: PGE06001
---

# Rule 6.7 — Flexible Field Exhaustiveness
`PGE06007`

<!-- @u:syntax/blocks -->

**Statement:** A `[?]` conditional branching on a flexible field (`:`) must include a `[?] *?` catch-all branch. Flexible fields are an open set — new keys can be added at runtime, so the compiler cannot prove exhaustiveness statically.
**Rationale:** Flexible fields (`:`) allow arbitrary keys, unlike fixed fields (`.`) which form a closed set handled by PGE06002. The compiler cannot enumerate all possible keys at compile time, so `*?` is the only guarantee of exhaustiveness.
**Detection:** When a `[?]` block branches on a flexible field (`:` type), the compiler checks for a `[?] *?` branch. If absent, PGE06007 fires.

**VALID:**
```aljam3
[ ] ✓ flexible field — *? required
[?] $config:mode
   [?] "fast"
      [-] -Run.Fast
   [?] *?
      [-] -Run.Default
```

**INVALID:**
```aljam3
[ ] ✗ PGE06007 — flexible field without *?
[?] $config:mode
   [?] "fast"
      [-] -Run.Fast
   [?] "slow"
      [-] -Run.Slow
   [ ] ✗ PGE06007 — missing *?, flexible field is open type
```

**See also:**
- [PGE06001 — Conditional Must Be Exhaustive](PGE06001-conditional-must-be-exhaustive.md) — parent rule
- [PGE06002 — Enum Exhaustiveness](PGE06002-enum-exhaustiveness.md) — fixed field (`.`) uses enum rules

## See Also

- [[user/concepts/conditionals|Conditionals]] — flexible field exhaustiveness rules reference PGE06007
