---
rule: "2.7"
code: PGW02001
name: Default Pull Across State Change
severity: warning
---

### Rule 2.7 — Default Pull Across State Change
`PGW02001`

**Statement:** If a variable in Default state is pulled, and then pulled again after a subsequent push has promoted it to Final, the two pulls may return different values. The runtime emits PGW02001 as a warning on the second pull. The second pull succeeds with the Final value.
**Rationale:** Pulling a Default variable and later pulling the same variable after it becomes Final means two different pipeline steps observe different values — one the default, one the final. This inconsistency is usually unintentional and can cause subtle bugs.
**Detection:** At runtime, when a variable that was previously pulled in Default state is pulled again after transitioning to Final. The warning is emitted on the second pull — the pull succeeds but flags the inconsistency.

**VALID:**
```polyglot
[ ] ✓ Default pulled once, then promoted — no second pull before promotion
[=] >label#string
[r] >label <~ "pending"           [ ] Default
[r] >label << "confirmed"         [ ] Final
[r] =Display
   [=] <text << >label            [ ] ✓ only pull is after Final — no warning
```

```polyglot
[ ] ✓ Default pulled once, never promoted — consistent value
[=] >fallback#string
[r] >fallback <~ "N/A"            [ ] Default
[r] =Display
   [=] <text << >fallback         [ ] ✓ pull 1 — Default value
[r] =Log
   [=] <msg << >fallback          [ ] ✓ pull 2 — still Default, same value, no warning
```

**WARNING:**
```polyglot
[ ] ⚠ PGW02001 — two pulls straddle the Default→Final transition
[=] >label#string
[r] >label <~ "pending"           [ ] Default

[ ] first pull — sees "pending"
[r] =LogStatus
   [=] <status << >label

[ ] push promotes to Final
[r] >label << "confirmed"         [ ] Final

[ ] second pull — sees "confirmed" — PGW02001 warning
[r] =LogStatus
   [=] <status << >label          [ ] ⚠ PGW02001 — value differs from first pull
```
