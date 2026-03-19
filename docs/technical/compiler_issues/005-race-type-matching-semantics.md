---
issue: "005"
title: Race collector type matching — exact vs structural
related: PGE-306 (Rule 3.6), PGE-401 (Rule 4.1)
priority: design-decision
status: resolved
created: 2026-03-18
resolved: 2026-03-19
---

# 005 — Race collector type matching: exact vs structural

## Question

Rule 3.6 requires all `[*] <<` inputs to race collectors be the "same type". Does this mean exact name match or structural compatibility?

## Resolution

**Schema matching.** All Polyglot data is serialized strings. "Same type" = "same schema" — same structure and field types, not same name. This is a universal principle defined canonically in PGE-401 (Rule 4.1).

For race collectors specifically:
- Each `[*] <<` input's schema must match the **target variable's schema** per PGE-401
- Two differently-named `{#}` types with identical field structures ARE the same type
- Comparison happens at the relevant section/subfield level — subfields of differently-named parents can match

This principle applies universally: IO wiring, race collectors, schema completeness, auto-wire, conditional matching, and any future type comparison.

## See also

- [PGE-401 — Type Mismatch](../compile-rules/PGE/PGE-401-type-mismatch.md) — canonical "same type" definition
- [PGE-306 — Race Collector Type Homogeneity](../compile-rules/PGE/PGE-306-race-collector-type-homogeneity.md)
