---
rule: "6.12"
code: PGE-612
name: Unreachable Branch After Wildcard
severity: error
---

### Rule 6.12 — Unreachable Branch After Wildcard
`PGE-612`

**Statement:** The `[?] *?` wildcard catch-all must be the final branch in a conditional chain. Any `[?]` branch after `*?` is unreachable dead code and is a compile error.
**Rationale:** `*?` matches everything the preceding branches did not. Any branch declared after it can never execute — the wildcard already consumed all remaining cases. Enforcing `*?`-last prevents hidden dead code.
**Detection:** The compiler scans `[?]` branches in declaration order. If a `*?` branch is encountered and further `[?]` branches follow, PGE-612 fires on each subsequent branch.

**See also:** PGE-611 (duplicate wildcard catch-all — two `*?` in same chain), PGE-601 (conditional must be exhaustive)

**VALID:**
```polyglot
[ ] ✓ wildcard is the last branch
[?] $status =? "active"
   [r] =HandleActive
[?] $status =? "inactive"
   [r] =HandleInactive
[?] *?
   [r] =HandleUnknown
```

```polyglot
[ ] ✓ no wildcard — statically exhaustive enum
[?] $flag =? #Boolean.True
   [r] =DoSomething
[?] $flag =? #Boolean.False
   [r] =DoNothing
```

**INVALID:**
```polyglot
[ ] ✗ PGE-612 — branch after wildcard is unreachable
[?] $status =? "active"
   [r] =HandleActive
[?] *?
   [r] =HandleUnknown
[?] $status =? "inactive"                   [ ] ✗ PGE-612 — unreachable after *?
   [r] =HandleInactive
```

```polyglot
[ ] ✗ PGE-612 — multiple branches after wildcard
[?] $code >? 100
   [r] =HandleHigh
[?] *?
   [r] =HandleDefault
[?] $code =? 50                             [ ] ✗ PGE-612 — unreachable
   [r] =HandleFifty
[?] $code =? 0                              [ ] ✗ PGE-612 — unreachable
   [r] =HandleZero
```

**Open point:** None.
