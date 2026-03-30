---
rule: "8.1"
code: PGE-801
name: Auto-Wire Type Mismatch
severity: error
---

### Rule 8.1 — Auto-Wire Type Mismatch
`PGE-801`

**Statement:** In chain execution (`[r] =A=>=B=>=C`), auto-wire between adjacent steps requires a **1-to-1 type match** across all IO parameters per [TYPE-IDENTITY](../TYPE-IDENTITY.md). PGE-801 fires when an output schema has no matching input schema, or vice versa.

Entry IO (first step's inputs) and exit IO (last step's outputs) always require explicit `[=]` lines — auto-wire applies only between adjacent mid-chain steps.
**Rationale:** Auto-wire is a convenience for prototyping, but implicit wiring obscures data flow. Separate error codes let the developer quickly identify *why* auto-wire failed. PGE-801 covers the simplest failure: no type can be paired at all.
**Detection:** For each pair of adjacent steps without explicit `[=]` wires, the compiler attempts 1-to-1 type matching. PGE-801 fires when an output type has no corresponding input type (or vice versa).

**See also:** PGE-802 (ambiguous type), PGE-803 (unmatched parameter), PGW-801 (auto-wire succeeded)

**INVALID:**
```polyglot
[ ] ✗ PGE-801 — type mismatch, no 1-to-1 mapping
[r] =Count.Items=>=Format.Label
   [=] >0.list#array:string << $items
   [ ] =Count.Items output: >total#int
   [ ] =Format.Label input: <text#string
   [ ] ✗ PGE-801 — int ≠ string, no matching type
   [=] <1.label#string >> >output
```

### See Also

- [[user/concepts/pipelines/chains|Chains]] — references PGE-801 in auto-wire rules
- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE-801 in auto-wire table
