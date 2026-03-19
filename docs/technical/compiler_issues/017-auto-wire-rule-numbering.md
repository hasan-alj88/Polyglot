---
issue: "017"
title: Auto-wire error codes share rule number 8.1
related: PGE-801 (Rule 8.1), PGE-802 (Rule 8.1), PGE-803 (Rule 8.1), PGE-804 (Rule 8.2)
priority: critical
status: resolved
created: 2026-03-19
---

# 017 — Auto-wire error codes share rule number 8.1

## Problem

Three distinct auto-wire error codes all share rule number "8.1" in COMPILE-RULES.md and in their frontmatter:

| Code | Current Rule | Name |
|------|-------------|------|
| PGE-801 | 8.1 | Auto-Wire Type Mismatch |
| PGE-802 | 8.1 | Auto-Wire Ambiguous Type |
| PGE-803 | 8.1 | Auto-Wire Unmatched Parameter |
| PGE-804 | 8.2 | Ambiguous Step Reference |

Each error code has its own file with distinct semantics, detection logic, and examples. Sharing a rule number makes it impossible to distinguish them in the COMPILE-RULES.md table and breaks the convention that each row has a unique rule number.

## Affected Rules

- `compile-rules/PGE/PGE-801-auto-wire-type-mismatch.md` — frontmatter says `rule: "8.1"`
- `compile-rules/PGE/PGE-802-auto-wire-ambiguous-type.md` — frontmatter says `rule: "8.1"`
- `compile-rules/PGE/PGE-803-auto-wire-unmatched-parameter.md` — frontmatter says `rule: "8.1"`
- `compile-rules/PGE/PGE-804-ambiguous-step-reference.md` — frontmatter says `rule: "8.2"`
- `COMPILE-RULES.md` — table lists all three as rule 8.1

## Proposed Resolution

Renumber to give each a distinct rule number:

| Code | New Rule | Name |
|------|---------|------|
| PGE-801 | 8.1 | Auto-Wire Type Mismatch |
| PGE-802 | 8.2 | Auto-Wire Ambiguous Type |
| PGE-803 | 8.3 | Auto-Wire Unmatched Parameter |
| PGE-804 | 8.4 | Ambiguous Step Reference |

Update frontmatter in each file and COMPILE-RULES.md table. Also update PGW-801 rule number if it references 8.1.

## See also

- [PGE-801 — Auto-Wire Type Mismatch](../compile-rules/PGE/PGE-801-auto-wire-type-mismatch.md)
- [PGE-802 — Auto-Wire Ambiguous Type](../compile-rules/PGE/PGE-802-auto-wire-ambiguous-type.md)
- [PGE-803 — Auto-Wire Unmatched Parameter](../compile-rules/PGE/PGE-803-auto-wire-unmatched-parameter.md)
- [PGE-804 — Ambiguous Step Reference](../compile-rules/PGE/PGE-804-ambiguous-step-reference.md)
