---
rule: "8.2"
code: PGE08002
name: Auto-Wire Ambiguous Type
severity: error
---

### Rule 8.2 — Auto-Wire Ambiguous Type
`PGE08002`

**Statement:** In chain execution auto-wire, PGE08002 fires when two or more outputs (or inputs) share the same type (per [TYPE-IDENTITY](../TYPE-IDENTITY.md)), making it impossible for the compiler to determine which maps where.
**Rationale:** Auto-wire must be deterministic. When multiple parameters share a type, the compiler cannot infer which output feeds which input — the developer must wire explicitly.
**Detection:** After confirming all types have at least one match (no PGE08001), the compiler checks for duplicate types on either side. If found, PGE08002 fires.

**See also:** PGE08001 (type mismatch), PGE08003 (unmatched parameter), PGW08001 (auto-wire succeeded)

**INVALID:**
```polyglot
[ ] ✗ PGE08002 — two outputs share the same type, ambiguous
[r] =Fetch.Both=>=Process.Single
   [=] >0.url#string << $url
   [ ] =Fetch.Both has >name#string AND >label#string — two #string outputs
   [ ] =Process.Single has <text#string — one #string input
   [ ] ✗ PGE08002 — which #string output maps to <text?
   [=] <1.output#string >> >result
```

### See Also

- [[user/concepts/pipelines/chains|Chains]] — references PGE08002 in auto-wire rules
- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE08002 in auto-wire table
