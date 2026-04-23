---
audience: developer
rule: "8.1"
code: PGE08001
name: Auto-Wire Type Mismatch
severity: error
updated: 2026-04-23
---

# Rule 8.1 — Auto-Wire Type Mismatch
`PGE08001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/io/auto-wire -->

**Statement:** In wildcard auto-wire (`(-) <* << $Label>*`), the compiler pairs every output of `$Label` with exactly one input of the target pipeline via [[type-identity|Type Identity]]. PGE08001 fires when an output type has no corresponding input type of the same type-identity (or vice versa), so no bijective mapping is possible.
**Rationale:** Wildcard auto-wire is a shorthand for "wire these matching ports automatically." If a type on one side has no counterpart on the other, the compiler cannot satisfy the bijection — the developer must either fix the signatures or fall back to explicit per-port `(-)` wiring.
**Detection:** When a `call_io_line` uses `<* << $Label>*`, the compiler collects the type-identity of every `$Label` output and every input of the target pipeline, then attempts 1-to-1 type matching. PGE08001 fires when an output type has no corresponding input type (or vice versa).

**See also:** PGE08002 (ambiguous type), PGE08003 (port count mismatch), PGW08001 (auto-wire succeeded)

**INVALID:**
```polyglot
[ ] ✗ PGE08001 — type mismatch, no bijective mapping possible
[ ] -Count.Items has one output >total#int
[ ] -Format.Label has one input  <text#string
[ ] #int ≠ #string — no bijection
[-] -Count.Items
   (-) $A
   (-) <list#array:string << $items
   (-) >total#int

[-] -Format.Label
   (-) <* << $A>*                  [ ] ✗ PGE08001 — >total#int has no #int input
   (-) >formatted#string >> >output
```

## See Also

- [[user/syntax/io/auto-wire|Wildcard Auto-Wire]] — user-facing explanation of `<* << $Label>*`
- [[technical/ebnf/07-io-parameters#7.4 Wildcard IO (Auto-Wire)]] — grammar productions
