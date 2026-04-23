---
audience: developer
rule: "6.12"
code: PGE06012
name: Unreachable Branch After Wildcard
severity: error
---

# Rule 6.12 — Unreachable Branch After Wildcard
`PGE06012`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/types -->

**Statement:** The `[?] *?` wildcard catch-all must be the final branch in a conditional chain. Any `[?]` branch after `*?` is unreachable dead code and is a compile error.
**Rationale:** `*?` matches everything the preceding branches did not. Any branch declared after it can never execute — the wildcard already consumed all remaining cases. Enforcing `*?`-last prevents hidden dead code.
**Detection:** The compiler scans `[?]` branches in declaration order. If a `*?` branch is encountered and further `[?]` branches follow, PGE06012 fires on each subsequent branch.

**See also:** PGE06011 (duplicate wildcard catch-all — two `*?` in same chain), PGE06001 (conditional must be exhaustive)

**VALID:**
```polyglot
[ ] ✓ wildcard is the last branch
[?] $status =? "active"
   [-] -HandleActive
[?] $status =? "inactive"
   [-] -HandleInactive
[?] *?
   [-] -HandleUnknown
```

```polyglot
[ ] ✓ no wildcard — statically exhaustive enum
[?] $flag =? #Boolean.True
   [-] -DoSomething
[?] $flag =? #Boolean.False
   [-] -DoNothing
```

**INVALID:**
```polyglot
[ ] ✗ PGE06012 — branch after wildcard is unreachable
[?] $status =? "active"
   [-] -HandleActive
[?] *?
   [-] -HandleUnknown
[?] $status =? "inactive"                   [ ] ✗ PGE06012 — unreachable after *?
   [-] -HandleInactive
```

```polyglot
[ ] ✗ PGE06012 — multiple branches after wildcard
[?] $code >? 100
   [-] -HandleHigh
[?] *?
   [-] -HandleDefault
[?] $code =? 50                             [ ] ✗ PGE06012 — unreachable
   [-] -HandleFifty
[?] $code =? 0                              [ ] ✗ PGE06012 — unreachable
   [-] -HandleZero
```

**Open point:** None.

## See Also

- [[user/concepts/conditionals|Conditionals]] — unreachable branch rules reference PGE06012
