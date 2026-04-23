---
audience: developer
rule: "8.1"
code: PGW08001
name: Auto-Wire Succeeded
severity: warning
updated: 2026-04-23
---

# Rule 8.1 — Auto-Wire Succeeded
`PGW08001`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/io/auto-wire -->

**Statement:** When wildcard auto-wire (`(-) <* << $Label>*`) resolves successfully (no PGE08001, PGE08002, or PGE08003), PGW08001 fires as a warning. The wiring is valid, but explicit per-port `(-)` lines are preferred for clarity.
**Rationale:** Implicit wiring obscures data flow. Explicit per-port wiring makes the pipeline self-documenting and prevents surprises when an operation's signature changes and silently alters the resolved mapping.
**Detection:** After all wildcard auto-wire checks pass, PGW08001 fires on the `(-) <* << $Label>*` line.

**See also:** PGE08001 (type mismatch), PGE08002 (ambiguous type), PGE08003 (port count mismatch)

**VALID (warning):**
```polyglot
[ ] ⚠ PGW08001 — auto-wire succeeded, prefer explicit wiring
[ ] -File.Text.Read output: >content#string
[ ] -Text.Transform input:  <text#string
[-] -File.Text.Read
   (-) $Read
   (-) <path#path << $path
   (-) >content#string

[-] -Text.Transform
   (-) <* << $Read>*               [ ] ⚠ PGW08001 — one #string → one #string
   (-) >formatted#string >> >output
```

```polyglot
[ ] ⚠ PGW08001 — multiple ports, unique types, bijective
[ ] -Step.A outputs: >name#string, >count#int
[ ] -Step.B inputs:  <label#string, <total#int
[-] -Step.A
   (-) $A
   (-) <query#string << $query
   (-) >name#string
   (-) >count#int

[-] -Step.B
   (-) <* << $A>*                  [ ] ⚠ PGW08001 — bijective by type
   (-) >result#string >> >output
```

**VALID (no warning — always preferred):**
```polyglot
[ ] ✓ explicit per-port wiring — no warning
[-] -Fetch.Data
   (-) $A
   (-) <url#string << $url
   (-) >content#string
   (-) >count#int

[-] -Process.Records
   (-) <items#string << $A>content
   (-) <total#int << $A>count
   (-) >report#string >> >output
```
