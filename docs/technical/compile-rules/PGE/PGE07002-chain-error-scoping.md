---
audience: developer
rule: "7.2"
code: PGE07002
name: Chain Error Scoping
severity: error
status: retired
---

# Rule 7.2 — Chain Error Scoping (Retired)
`PGE07002`

**Retired:** 2026-04-22. The `->` chain operator has been removed from Aljam3. Labeled `[-]` calls with operation labels replace chains. Each `[-]` call uses standard `[!]` error blocks — no chain-specific `.N!ErrorName` syntax needed. See [[concepts/pipelines/chains]].
