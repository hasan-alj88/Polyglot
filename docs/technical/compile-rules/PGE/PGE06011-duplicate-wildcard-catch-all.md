---
rule: "6.11"
code: PGE06011
name: Duplicate Wildcard Catch-All
severity: error
---

### Rule 6.11 — Duplicate Wildcard Catch-All
`PGE06011`

**Statement:** A `[?]` conditional chain must contain at most one `[?] *?` wildcard catch-all branch. A second `*?` is unreachable dead code and is a compile error.
**Rationale:** Only the first `*?` is reachable — it catches everything the preceding branches did not. Any subsequent `*?` can never execute. Rejecting duplicates early prevents hidden dead code.
**Detection:** The compiler scans all `[?]` branches in a conditional chain. If more than one branch uses the `*?` wildcard pattern, PGE06011 fires on the second (and any subsequent) occurrence.

**See also:** PGE06001 (conditional must be exhaustive — requires `*?` for open types), PGE06009 (conditional missing comparison operator)

**VALID:**
```polyglot
[ ] ✓ single wildcard catch-all
[?] $status =? "active"
   [r] =HandleActive
[?] $status =? "inactive"
   [r] =HandleInactive
[?] *?
   [r] =HandleUnknown
```

```polyglot
[ ] ✓ no wildcard needed — enum is statically exhaustive
[?] $severity =? #Severity.Critical
   [r] =AlertCritical
[?] $severity =? #Severity.Error
   [r] =AlertError
[?] $severity =? #Severity.Info
   [r] =LogInfo
[?] $severity =? #Severity.Other
   [r] =LogOther
```

**INVALID:**
```polyglot
[ ] ✗ PGE06011 — two wildcard catch-alls
[?] $status =? "active"
   [r] =HandleActive
[?] *?
   [r] =HandleUnknown
[?] *?                                       [ ] ✗ PGE06011 — second *? is unreachable
   [r] =NeverReached
```

```polyglot
[ ] ✗ PGE06011 — wildcard duplicated even with branches between
[?] $code >? 200
   [r] =HandleHigh
[?] *?
   [r] =HandleLow
[?] $code =? 0
   [r] =HandleZero
[?] *?                                       [ ] ✗ PGE06011 — second *? is unreachable
   [r] =NeverReached
```

**Open point:** None.

### See Also

- [[user/concepts/conditionals|Conditionals]] — wildcard catch-all rules reference PGE06011
