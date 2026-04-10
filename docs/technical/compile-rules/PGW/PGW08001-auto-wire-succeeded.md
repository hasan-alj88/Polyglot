---
audience: developer
rule: "8.1"
code: PGW08001
name: Auto-Wire Succeeded
severity: warning
---

### Rule 8.1 — Auto-Wire Succeeded
`PGW08001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** When auto-wire between adjacent chain steps succeeds (no PGE08001, PGE08002, or PGE08003), PGW08001 fires as a warning. Auto-wire is valid but explicit `(-)` wiring is preferred for clarity.
**Rationale:** Implicit wiring obscures data flow. Explicit `(-)` lines make the pipeline self-documenting and prevent surprises when a step's signature changes.
**Detection:** After all auto-wire checks pass, PGW08001 fires on each successfully auto-wired step pair.

**See also:** PGE08001 (type mismatch), PGE08002 (ambiguous type), PGE08003 (unmatched parameter)

**VALID:**
```polyglot
[ ] ✓ auto-wire: 1-to-1 type match (one output#string → one input#string)
[ ] Ignore PGW08001
[-] -File.Text.Read->-Text.Transform->-Text.Format
   (-) >0.path#path << $path
   (-) <2.formatted#string >> >output
```

```polyglot
[ ] ✓ auto-wire: multiple params, all types unique and matched
[ ] -Step.A outputs: >name#string, >count#int
[ ] -Step.B inputs:  <label#string, <total#int
[ ] Ignore PGW08001
[-] -Step.A->-Step.B
   (-) >0.query#string << $query
   (-) <1.result#string >> >output
```

```polyglot
[ ] ✓ explicit wiring — no warning, always preferred
[-] -Fetch.Data->-Process.Records
   (-) >0.url#string << $url
   (-) <0.results#array:string >> <1.items#array:string
   (-) <0.count#int >> <1.total#int
   (-) <1.output#string >> >report
```

**WARNING:**
```polyglot
[ ] ⚠ PGW08001 — auto-wire succeeded but explicit wiring preferred
[-] -File.Text.Read->-Text.Transform
   (-) >0.path#path << $path
   (-) <1.output#string >> >result
   [ ] ⚠ PGW08001 — step 0 → step 1 auto-wired (#string → #string)
```
