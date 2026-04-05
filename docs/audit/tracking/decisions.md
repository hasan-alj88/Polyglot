---
audience: ai
type: audit-tracking
scope: audit-decisions
updated: 2026-03-30
---

# Audit Decisions

<!-- @audit/README -->
Decisions made during documentation auditing and fixing. These are NOT language design decisions (those live in [[technical/plan/decisions/INDEX|technical/plan/decisions/]]).

## Decisions

| ID | Date | Decision | Rationale |
|----|------|----------|-----------|
| AD-001 | 2026-03-30 | Split IC items out of [[tracking/coverage-gaps]] into dedicated [[tracking/inconsistencies]] | coverage-gaps.md tracks structural gaps (UG/TG/MX/OR); inconsistencies are a different concern needing severity levels and tags |
| AD-002 | 2026-03-30 | No separate audience-issues file; audience problems tracked as IC items with `audience` tag | Low volume; [[rules/checklist]] already validates audience; separate file would be premature |
| AD-003 | 2026-03-30 | Flat files in tracking/ — no subdirectories | Matches existing audit pattern (audiences/ has 3 files, rules/ has 2); subdirectories would over-engineer |
| AD-004 | 2026-03-30 | All doc workflows route through GitHub Issues (`docs` label) with umbrella issue #95 | Single source of truth for progress; checklist items on the issue make status visible without reading markdown files |
| AD-005 | 2026-03-30 | metadata.md is a technical doc — split into thin user stub + full technical spec | Metadata is architecture-level (NoSQL tree, `%` paths, instance addressing); users should not need to interact with it directly. User docs get a minimal "query with `%`" page; deep content moves to `technical/spec/` |
