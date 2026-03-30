---
rule: "2.10"
code: PGE02010
name: Discard Default Assignment
severity: error
---

### Rule 2.10 — Discard Default Assignment
`PGE02010`

**Statement:** The inline discard pattern `$*` only accepts final assignment operators (`<<`, `>>`). Using default operators (`<~`, `~>`) with `$*` is a compile error. A discard is immediately released — setting a default implies the value may be overridden later, which is meaningless for a pattern that is never read.
**Rationale:** `$*` exists to explicitly acknowledge and discard a value. Discards have no lifecycle beyond the assignment — they skip Declared, jump straight to Released. Default assignment implies the value sits in Default state awaiting possible promotion to Final, but `$*` has no state to promote. Allowing `<~`/`~>` on `$*` would mislead developers into thinking the value persists.
**Detection:** The compiler checks the assignment operator when `$*` appears as the target (left side) or source-side discard (right side via `>>`). If the operator is `<~` or `~>`, PGE02010 fires.

**VALID:**
```polyglot
[ ] ✓ Final push into discard — value explicitly thrown away
[r] $*#string << "discarded"

[ ] ✓ Final pull into discard via >>
[r] =Compute
   [=] >result >> $*                    [ ] ✓ explicit discard of output
```

**INVALID:**
```polyglot
[ ] ✗ PGE02010 — default push into discard
[r] $*#string <~ "never used"           [ ] ✗ $* is immediate release, default is meaningless

[ ] ✗ PGE02010 — default pull into discard
[r] =Compute
   [=] >result ~> $*                    [ ] ✗ $* cannot hold a default value
```

**Diagnostic:** "Discard `$*` does not support default assignment `<~`/`~>` — use final `<<`/`>>` instead (value is immediately released)"

### See Also

- [[concepts/variable-lifecycle|Variable Lifecycle]] — discard skips lifecycle, goes straight to Released
- [[technical/compile-rules/PGE/PGE02003-final-is-push-once|PGE02003]] — Final Is Push-Once (lifecycle rules for `<<`/`>>`)
- [[technical/compile-rules/PGW/PGW02002-unused-variable|PGW02002]] — Unused Variable (`$*` suppresses this warning)
