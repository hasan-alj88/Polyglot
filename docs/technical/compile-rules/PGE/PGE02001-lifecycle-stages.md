---
audience: developer
rule: "2.1"
code: PGE02001
name: Lifecycle Stages
---

### Rule 2.1 — Lifecycle Stages
`PGE02001`

**Statement:** Every variable follows the lifecycle: **Declared** → (**Default** | **Final**) → **Released**. `<<` from Declared produces Final directly (bypasses Default). `<~` from Declared produces Default; one further push promotes to Final. **Failed** is a terminal stage reachable from any non-Released stage when the producing pipeline errors. Backwards transitions and invalid stage jumps are errors (PGE02001). Specific lifecycle violations fire PGE02002 through PGE02006.
**Rationale:** The lifecycle encodes write-once semantics and propagation guarantees. Each stage transition is intentional — accidental reassignment or reading an uninitialized variable are caught before they propagate through the pipeline.

**VALID:**
```polyglot
[ ] ✓ Declared → Final via <<
(-) >result#string
[-] -Compute
   (-) >result << "done"    [ ] Final — no further push allowed

[ ] ✓ Declared → Default via <~, then → Final via <<
(-) >label#string
[-] >label <~ "default"     [ ] Default — one more push allowed
[-] >label << "confirmed"   [ ] Final
```

**INVALID:**
```polyglot
[ ] ✗ PGE02001 — backwards transition: attempting to re-enter Default after Final
(-) >result#string
[-] -Compute
   (-) >result << "done"    [ ] Final
[-] >result <~ "retry"      [ ] ✗ PGE02001 — cannot move Final → Default
```
