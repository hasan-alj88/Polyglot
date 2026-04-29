---
audience: developer
rule: "8.3"
code: PGE08003
name: Auto-Wire Port Count Mismatch
severity: error
updated: 2026-04-23
---

# Rule 8.3 — Auto-Wire Port Count Mismatch
`PGE08003`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/io/auto-wire -->

**Statement:** In wildcard auto-wire (`(-) <* << $Label>*`), PGE08003 fires when `$Label`'s output count does not equal the target pipeline's input count. The bijective mapping must be **onto**: every output maps to exactly one input and every input receives exactly one output, which requires identical cardinality on both sides.
**Rationale:** Wildcard auto-wire resolves a complete, balanced pairing — not a partial one. A count mismatch indicates a signature mismatch that cannot be silently patched; the developer must explicitly wire ports (or fix the signatures).
**Detection:** When a `call_io_line` uses `<* << $Label>*`, the compiler counts `$Label`'s outputs and the target's inputs. If the counts differ, PGE08003 fires. Port-count check runs before type pairing.

**See also:** PGE08001 (type mismatch), PGE08002 (ambiguous type), PGW08001 (auto-wire succeeded)

**INVALID:**
```aljam3
[ ] ✗ PGE08003 — 3 outputs but only 2 inputs
[ ] -Fetch.Data outputs: >content#string, >count#int, >status#string  (3)
[ ] -Transform.Text inputs:  <text#string, <flag#int                  (2)
[-] -Fetch.Data
   (-) $A
   (-) <url#string << $url
   (-) >content#string
   (-) >count#int
   (-) >status#string

[-] -Transform.Text
   (-) <* << $A>*                  [ ] ✗ PGE08003 — 3 outputs, 2 inputs
   (-) >result#string >> >output
```

## See Also

- [[user/syntax/io/auto-wire|Wildcard Auto-Wire]] — user-facing explanation of `<* << $Label>*`
- [[technical/ebnf/07-io-parameters#7.4 Wildcard IO (Auto-Wire)]] — grammar productions
