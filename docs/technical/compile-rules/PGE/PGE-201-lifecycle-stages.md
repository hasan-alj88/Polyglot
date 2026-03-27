---
rule: "2.1"
code: PGE-201
name: Lifecycle Stages
---

### Rule 2.1 — Lifecycle Stages
`PGE-201`

**Statement:** Every variable follows the lifecycle: **Declared** → (**Default** | **Final**) → **Released**. `<<` from Declared produces Final directly (bypasses Default). `<~` from Declared produces Default; one further push promotes to Final. **Failed** is a terminal stage reachable from any non-Released stage when the producing pipeline errors. Backwards transitions and invalid stage jumps are errors (PGE-201). Specific lifecycle violations fire PGE-202 through PGE-206.
**Rationale:** The lifecycle encodes write-once semantics and propagation guarantees. Each stage transition is intentional — accidental reassignment or reading an uninitialized variable are caught before they propagate through the pipeline.

**VALID:**
```polyglot
[ ] ✓ Declared → Final via <<
[=] >result#string
[r] =Compute
   [=] >result << "done"    [ ] Final — no further push allowed

[ ] ✓ Declared → Default via <~, then → Final via <<
[=] >label#string
[r] >label <~ "default"     [ ] Default — one more push allowed
[r] >label << "confirmed"   [ ] Final
```

**INVALID:**
```polyglot
[ ] ✗ PGE-201 — backwards transition: attempting to re-enter Default after Final
[=] >result#string
[r] =Compute
   [=] >result << "done"    [ ] Final
[r] >result <~ "retry"      [ ] ✗ PGE-201 — cannot move Final → Default
```
