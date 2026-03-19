---
issue: "007"
title: PGE-601 contains duplicated type-specific content from PGE-602/603
related: PGE-601 (Rule 6.1), PGE-602 (Rule 6.2), PGE-603 (Rule 6.3)
priority: cleanup
status: resolved
created: 2026-03-19
resolved: 2026-03-19
---

# 007 — PGE-601 contains duplicated type-specific content from PGE-602/603

## Resolution

**Two independent concepts identified — exhaustiveness and mutual exclusiveness must be separated.**

1. **Exhaustiveness** (PGE-601/602/603): "Does the union of all branches cover the full domain?" — algorithm in `docs/technical/plan/TODO/006-compound-exhaustiveness-algorithm.md`
2. **Mutual exclusiveness** (PGE-604/605): "Is the intersection of any two branches empty?" — algorithm in `docs/technical/plan/TODO/007-overlap-detection-algorithm.md`

### Decisions

- PGE-601 becomes a **dispatch rule** — keeps the summary table, routes to sub-rules, drops duplicated examples
- Overlap detection is **not a sub-rule of exhaustiveness** — it's an independent property with its own algorithm and error codes
- String/flexible field sections stay in PGE-601 until dedicated sub-rules are created (see Issue 014)
- Both checks share the same domain model and can run in a single pass, but report independently

## Original Problem

PGE-601 (Conditional Must Be Exhaustive) is both a standalone error and a parent meta-rule. It contains full type-specific sections — `#Boolean`, `int`/`float`, enum, string, flexible fields, compound conditions — with complete VALID/INVALID examples for each. The `#Boolean` section is a full duplicate of the `#Boolean` section in PGE-602. The `int`/`float` section duplicates PGE-603. The interaction table in PGE-602 explicitly shows PGE-601 and PGE-602 fire together on the same invalid code.

This duplication means:
1. Changes to enum exhaustiveness logic must be updated in both PGE-601 and PGE-602
2. Changes to numeric range logic must be updated in both PGE-601 and PGE-603
3. A developer reading PGE-601 may not realize the authoritative version is in PGE-602/603

## See also

- [014 — PGE-601 needs per-type sub-rules](014-pge601-restructure-type-sections.md) — proposes new PGE-605/606/607 for types currently only in PGE-601
- [006 — Compound Condition Exhaustiveness](006-compound-condition-exhaustiveness.md) — resolved, algorithm documented
