---
audience: design
rule: "2.12"
code: PGE02012
name: Duplicate Operation Label
severity: error
type: spec
updated: 2026-04-09
---

### Rule 2.12 — Duplicate Operation Label
`PGE02012`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->

**Statement:** Two `($)` operation labels with the same `$Name` in the same scope produce a compile error. Each label must be unique within its enclosing scope so that downstream accessors (`$Name>param`, `$Name<param`) resolve unambiguously.
**Rationale:** Operation labels serve as named handles for accessing inputs and outputs of a specific call. Duplicate names would make accessor resolution ambiguous — the compiler cannot determine which labeled operation `$Name>output` refers to. Polyglot requires explicit, unambiguous intent — every reference must resolve to exactly one target so the compiler can verify the complete data flow at compile time.
**Detection:** The compiler collects all `($)` labels within each scope and checks for name collisions. A collision is reported at the second occurrence.

**VALID:**
```polyglot
[ ] ✓ distinct label names in the same scope
[-] -File.Text.Read"/input.txt"
   (-) $Read
[-] -Serial.JSON.Parse
   (-) $Parse
   (-) <raw << $Read>content
```

**INVALID:**
```polyglot
[ ] ✗ PGE02012 — duplicate operation label "$Read" in same scope
[-] -File.Text.Read"/input.txt"
   (-) $Read
[-] -File.Text.Read"/other.txt"
   (-) $Read
```

**Diagnostic:** "Duplicate operation label `$Read` — each `($)` label must be unique within its scope"

**Related:** PGE09007 (Duplicate Definition), PGE05003 (Duplicate Data Field Name)
