---
audience: developer
rule: "8.3"
code: PGE08003
name: Auto-Wire Unmatched Parameter
severity: error
---

### Rule 8.3 — Auto-Wire Unmatched Parameter
`PGE08003`

**Statement:** In chain execution auto-wire, PGE08003 fires when, after successful type pairing, one or more parameters on either side have no counterpart.
**Rationale:** Every output must go somewhere and every input must come from somewhere. Leftover parameters indicate a signature mismatch that the developer must resolve with explicit `[=]` wiring.
**Detection:** After type matching succeeds (no PGE08001) and is unambiguous (no PGE08002), the compiler checks for leftover unmatched parameters. If any remain, PGE08003 fires.

**See also:** PGE08001 (type mismatch), PGE08002 (ambiguous type), PGW08001 (auto-wire succeeded)

**INVALID:**
```polyglot
[ ] ✗ PGE08003 — unmatched parameter after pairing
[r] =Fetch.Data=>=Transform.Text
   [=] >0.url#string << $url
   [ ] =Fetch.Data outputs: >content#string, >count#int
   [ ] =Transform.Text inputs: <text#string
   [ ] #string pairs, but >count#int has no match on step 1
   [ ] ✗ PGE08003 — >count#int unmatched
   [=] <1.output#string >> >result
```

### See Also

- [[user/concepts/pipelines/chains|Chains]] — references PGE08003 in auto-wire rules
- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE08003 in auto-wire table
