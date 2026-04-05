---
audience: developer
rule: "6.14"
code: PGE06014
name: Wildcard-Only Match
severity: error
---

### Rule 6.14 — Wildcard-Only Match
`PGE06014`

**Statement:** A match expression must contain at least one non-wildcard arm before the `*?` catch-all. A match with only `[?] *?` always produces the same result regardless of input and is a compile error.
**Rationale:** A wildcard-only match is tautological — it adds indirection without branching. The developer should use direct assignment instead. This is related to PGE01018 (tautological trigger condition) and PGE06013 (tautological conditional).
**Detection:** The compiler checks that match expressions contain at least one `match_value` arm (literal, enum, or variable) before the `*?` wildcard arm.

**VALID:**
```polyglot
[ ] ✓ match with non-wildcard arm before wildcard
[r] $code >> $msg#string
   [?] 200 >> "OK"
   [?] 404 >> "Not Found"
   [?] *? >> "Unknown"
```

**INVALID:**
```polyglot
[ ] ✗ PGE06014 — wildcard-only match is tautological
[r] $code >> $msg#string
   [?] *? >> "always this"
```

**Diagnostic:** "Match expression has only a wildcard arm — use direct assignment instead"
