---
audience: design
rule: "2.13"
code: PGE02013
name: Write To Label Accessor
severity: error
type: spec
updated: 2026-04-09
---

### Rule 2.13 — Write To Label Accessor
`PGE02013`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/operators -->

**Statement:** Operation label accessors (`$Label>param`, `$Label<param`) are read-only. Writing to them (`$Label>param << value`) is a compile error because label outputs are Final — they capture the result of a completed operation and cannot be overwritten.
**Rationale:** Labels provide a read-only window into a completed operation's inputs and outputs. Allowing writes would violate the Final guarantee on operation results and break referential integrity for any other code that reads the same accessor. This enforces Polyglot's immutable-by-default data flow — once a value reaches Final, the compiler guarantees it cannot change, which is essential for safe concurrent access without locks or mutexes.
**Detection:** The compiler checks that label accessors never appear on the left-hand side of a push (`<<`) or as targets of `>>` wiring. Any such occurrence is rejected.

**VALID:**
```polyglot
[ ] ✓ reading from a label accessor
[-] -File.Text.Read"/data.txt"
   (-) $Read
(-) <data << $Read>content
```

**INVALID:**
```polyglot
[ ] ✗ PGE02013 — label accessor is read-only, cannot write
[-] -File.Text.Read"/data.txt"
   (-) $Read
[-] $Read>content << "override"
```

**Diagnostic:** "Cannot write to label accessor `$Read>content` — operation label outputs are Final and read-only"

**Related:** PGE02003 (Final Is Push-Once)
