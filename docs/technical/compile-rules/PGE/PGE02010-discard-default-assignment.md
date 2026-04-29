---
audience: developer
rule: "2.10"
code: PGE02010
name: Discard Default Assignment
severity: error
---

# Rule 2.10 — Discard Default Assignment
`PGE02010`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** The inline discard pattern `$*` only accepts final assignment operators (`<<`, `>>`). Using default operators (`<~`, `~>`) with `$*` is a compile error. A discard is immediately released — setting a default implies the value may be overridden later, which is meaningless for a pattern that is never read.
**Rationale:** `$*` exists to explicitly acknowledge and discard a value. Discards have no lifecycle beyond the assignment — they skip Declared, jump straight to Released. Default assignment implies the value sits in Default state awaiting possible promotion to Final, but `$*` has no state to promote. Allowing `<~`/`~>` on `$*` would mislead developers into thinking the value persists. This reflects Aljam3's "think with intent" philosophy — every operation must express clear, honest intent, and the compiler rejects patterns that mislead the developer about what the code actually does.
**Detection:** The compiler checks the assignment operator when `$*` appears as the target (left side) or source-side discard (right side via `>>`). If the operator is `<~` or `~>`, PGE02010 fires.

**VALID:**
```aljam3
[ ] ✓ PushLeft into discard — value explicitly thrown away
[-] $*#string << "discarded"

[ ] ✓ PushRight into discard via >>
[-] -Compute
   (-) >result >> $*                    [ ] ✓ explicit discard of output
```

**INVALID:**
```aljam3
[ ] ✗ PGE02010 — DefaultPushLeft into discard
[-] $*#string <~ "never used"           [ ] ✗ $* is immediate release, default is meaningless

[ ] ✗ PGE02010 — DefaultPushRight into discard
[-] -Compute
   (-) >result ~> $*                    [ ] ✗ $* cannot hold a default value
```

**Diagnostic:** "Discard `$*` does not support default assignment `<~`/`~>` — use final `<<`/`>>` instead (value is immediately released)"

## See Also

- [[concepts/variable-lifecycle|Variable Lifecycle]] — discard skips lifecycle, goes straight to Released
- [[technical/compile-rules/PGE/PGE02003-final-is-push-once|PGE02003]] — Final Is Push-Once (lifecycle rules for `<<`/`>>`)
- [[technical/compile-rules/PGW/PGW02002-unused-variable|PGW02002]] — Unused Variable (`$*` suppresses this warning)
