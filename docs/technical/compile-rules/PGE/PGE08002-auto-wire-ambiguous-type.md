---
audience: developer
rule: "8.2"
code: PGE08002
name: Auto-Wire Ambiguous Type
severity: error
updated: 2026-04-23
---

# Rule 8.2 — Auto-Wire Ambiguous Type
`PGE08002`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/io/auto-wire -->

**Statement:** In wildcard auto-wire (`(-) <* << $Label>*`), PGE08002 fires when two or more outputs (or inputs) share the same type-identity (per [TYPE-IDENTITY](../TYPE-IDENTITY.md)), making the bijective mapping non-unique — the compiler cannot determine which output feeds which input.
**Rationale:** Wildcard auto-wire must be deterministic. When multiple ports share a type on either side, any of several bijections would satisfy the types, so the choice becomes implementation-defined. The developer must wire explicitly to disambiguate.
**Detection:** After confirming every type has at least one match (no PGE08001), the compiler checks for duplicate type-identities on either side. If any duplicates exist, PGE08002 fires.

**See also:** PGE08001 (type mismatch), PGE08003 (port count mismatch), PGW08001 (auto-wire succeeded)

**INVALID:**
```polyglot
[ ] ✗ PGE08002 — two outputs share #string, ambiguous mapping
[ ] -Fetch.Both outputs: >name#string, >label#string  (two #string)
[ ] -Process.Single inputs:  <text#string             (one #string)
[-] -Fetch.Both
   (-) $A
   (-) <url#string << $url
   (-) >name#string
   (-) >label#string

[-] -Process.Single
   (-) <* << $A>*                  [ ] ✗ PGE08002 — which #string maps to <text?
   (-) >result#string >> >output
```

## See Also

- [[user/syntax/io/auto-wire|Wildcard Auto-Wire]] — user-facing explanation of `<* << $Label>*`
- [[technical/ebnf/07-io-parameters#7.4 Wildcard IO (Auto-Wire)]] — grammar productions
