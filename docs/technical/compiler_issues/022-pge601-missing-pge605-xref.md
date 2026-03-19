---
issue: "022"
title: PGE-601 dispatch table missing PGE-605 cross-reference
related: PGE-601 (Rule 6.1), PGE-605 (Rule 6.5)
priority: cleanup
status: resolved
created: 2026-03-19
---

# 022 — PGE-601 dispatch table missing PGE-605 cross-reference

## Problem

PGE-601 was rewritten as a dispatch table referencing per-type exhaustiveness rules (PGE-602 through PGE-608). However, PGE-605 (Compound Condition Overlap) is not listed in the dispatch table or "See also" section.

PGE-605 is an overlap detection rule (not an exhaustiveness rule), so it may be intentionally omitted from the exhaustiveness matrix. However, it's closely related — compound condition overlap is the counterpart to compound condition exhaustiveness (PGE-608). A cross-reference would help developers find it.

## Affected Rules

- `compile-rules/PGE/PGE-601-conditional-must-be-exhaustive.md`

## Proposed Resolution

Add PGE-605 to the "See also" section of PGE-601 (not to the exhaustiveness matrix, since it's an overlap rule not an exhaustiveness rule). Note: this depends on Issue 016 — PGE-605 file must exist first.

## See also

- [PGE-601 — Conditional Must Be Exhaustive](../compile-rules/PGE/PGE-601-conditional-must-be-exhaustive.md)
- [Issue 016 — PGE-605 rule file missing](016-pge605-missing-file.md)
