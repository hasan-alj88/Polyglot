---
audience: developer
rule: "1.2w"
code: PGW01002
name: Empty Data Definition
severity: warning
status: retired
superseded_by: PGE01021
---

### Rule 1.2w — Empty Data Definition *(retired)*
`PGW01002`

**Retired.** This warning has been superseded by [PGE01021](../PGE/PGE01021-empty-data-definition.md) — empty `{#}` data definitions are now a compile error, not a warning. The EBNF grammar requires at least one body line.
