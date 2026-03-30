---
rule: "8.3"
code: PGE-803
name: Auto-Wire Unmatched Parameter
severity: error
---

### Rule 8.3 — Auto-Wire Unmatched Parameter
`PGE-803`

**Statement:** In chain execution auto-wire, PGE-803 fires when, after successful type pairing, one or more parameters on either side have no counterpart.
**Rationale:** Every output must go somewhere and every input must come from somewhere. Leftover parameters indicate a signature mismatch that the developer must resolve with explicit `[=]` wiring.
**Detection:** After type matching succeeds (no PGE-801) and is unambiguous (no PGE-802), the compiler checks for leftover unmatched parameters. If any remain, PGE-803 fires.

**See also:** PGE-801 (type mismatch), PGE-802 (ambiguous type), PGW-801 (auto-wire succeeded)

**INVALID:**
```polyglot
[ ] ✗ PGE-803 — unmatched parameter after pairing
[r] =Fetch.Data=>=Transform.Text
   [=] >0.url#string << $url
   [ ] =Fetch.Data outputs: >content#string, >count#int
   [ ] =Transform.Text inputs: <text#string
   [ ] #string pairs, but >count#int has no match on step 1
   [ ] ✗ PGE-803 — >count#int unmatched
   [=] <1.output#string >> >result
```

### See Also

- [[user/concepts/pipelines/chains|Chains]] — references PGE-803 in auto-wire rules
- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE-803 in auto-wire table
