---
rule: "8.2"
code: PGE-802
name: Auto-Wire Ambiguous Type
severity: error
---

### Rule 8.2 — Auto-Wire Ambiguous Type
`PGE-802`

**Statement:** In chain execution auto-wire, PGE-802 fires when two or more outputs (or inputs) share the same type (per [TYPE-IDENTITY](../TYPE-IDENTITY.md)), making it impossible for the compiler to determine which maps where.
**Rationale:** Auto-wire must be deterministic. When multiple parameters share a type, the compiler cannot infer which output feeds which input — the developer must wire explicitly.
**Detection:** After confirming all types have at least one match (no PGE-801), the compiler checks for duplicate types on either side. If found, PGE-802 fires.

**See also:** PGE-801 (type mismatch), PGE-803 (unmatched parameter), PGW-801 (auto-wire succeeded)

**INVALID:**
```polyglot
[ ] ✗ PGE-802 — two outputs share the same type, ambiguous
[r] =Fetch.Both=>=Process.Single
   [=] >0.url#string << $url
   [ ] =Fetch.Both has >name#string AND >label#string — two #string outputs
   [ ] =Process.Single has <text#string — one #string input
   [ ] ✗ PGE-802 — which #string output maps to <text?
   [=] <1.output#string >> >result
```

### See Also

- [[user/concepts/pipelines/chains|Chains]] — references PGE-802 in auto-wire rules
- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE-802 in auto-wire table
