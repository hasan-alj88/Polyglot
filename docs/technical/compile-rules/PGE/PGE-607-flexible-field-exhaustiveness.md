---
rule: "6.7"
code: PGE-607
name: Flexible Field Exhaustiveness
severity: error
split_from: PGE-601
---

### Rule 6.7 — Flexible Field Exhaustiveness
`PGE-607`

**Statement:** A `[?]` conditional branching on a flexible field (`:`) must include a `[?] *?` catch-all branch. Flexible fields are an open set — new keys can be added at runtime, so the compiler cannot prove exhaustiveness statically.
**Rationale:** Flexible fields (`:`) allow arbitrary keys, unlike fixed fields (`.`) which form a closed set handled by PGE-602. The compiler cannot enumerate all possible keys at compile time, so `*?` is the only guarantee of exhaustiveness.
**Detection:** When a `[?]` block branches on a flexible field (`:` type), the compiler checks for a `[?] *?` branch. If absent, PGE-607 fires.

**VALID:**
```polyglot
[ ] ✓ flexible field — *? required
[?] $config:mode
   [?] "fast"
      [r] =Run.Fast
   [?] *?
      [r] =Run.Default
```

**INVALID:**
```polyglot
[ ] ✗ PGE-607 — flexible field without *?
[?] $config:mode
   [?] "fast"
      [r] =Run.Fast
   [?] "slow"
      [r] =Run.Slow
   [ ] ✗ PGE-607 — missing *?, flexible field is open type
```

**See also:**
- [PGE-601 — Conditional Must Be Exhaustive](PGE-601-conditional-must-be-exhaustive.md) — parent rule
- [PGE-602 — Enum Exhaustiveness](PGE-602-enum-exhaustiveness.md) — fixed field (`.`) uses enum rules
