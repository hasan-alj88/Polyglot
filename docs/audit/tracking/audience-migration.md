---
type: audit-tracking
scope: audience-migration
updated: 2026-04-03
---

# Audience Migration Tracker

<!-- @audit/README -->
Migration from 3-audience (user/developer/ai) to 5-audience system (pg-coder/integrator/architect/designer/ai). See [[audiences/pg-coder]], [[audiences/integrator]], [[audiences/architect]], [[audiences/designer]], [[audiences/ai]].

## Status Key

| Status | Meaning |
|--------|---------|
| Not Started | Work has not begun |
| In Progress | Partially complete |
| Done | Complete and verified |
| Blocked | Waiting on dependency |

## Phase Progress

| Phase | Description | Status | Notes |
|-------|-------------|--------|-------|
| 0 | Create this tracker | Done | — |
| 1 | Define audience system | Done | All files created, rules updated |
| 2 | Split oversized docs | Done | 4 split, 1 skipped (under limit) |
| 3 | Migrate frontmatter | Done | All files migrated |
| 4 | Verify | Done | All 9 checks passed |

## Phase 1 — Audience System

| Task | Status | File |
|------|--------|------|
| Create pg-coder.md | Done | audiences/pg-coder.md |
| Create integrator.md | Done | audiences/integrator.md |
| Create architect.md | Done | audiences/architect.md |
| Create designer.md | Done | audiences/designer.md |
| Update conventions.md | Done | rules/conventions.md |
| Update checklist.md | Done | rules/checklist.md |
| Update README.md | Done | audit/README.md |
| Delete user.md | Done | audiences/user.md |
| Delete developer.md | Done | audiences/developer.md |

## Phase 2 — Doc Splits

| File | Size | Target folder | Status |
|------|------|---------------|--------|
| user/SCENARIOS.md | 54KB | user/scenarios/ | Complete |
| technical/plan/queue-manager-architecture.md | 45KB | technical/plan/queue-manager/ | Complete |
| user/stdlib/types/datetime.md | 22KB | user/stdlib/types/datetime/ | Complete |
| technical/spec/metadata-tree.md | 21KB | technical/spec/metadata-tree/ | Complete |
| technical/edge-cases/24-datatype-defs.md | 21KB | — | Skipped (under 50KB, single H2) |

## Phase 3 — Frontmatter Migration

| Scope | Count | New value | Status |
|-------|-------|-----------|--------|
| docs/user/ | 83 | pg-coder | Done |
| technical/ebnf/ | 16 | designer | Done |
| technical/edge-cases/ | 25 | designer | Done |
| technical/compile-rules/ | 1 | designer | Done |
| technical/plan/ | 13 | architect | Done |
| technical/spec/ | 10 | [architect, designer] | Done |
| technical/INDEX.md | 1 | [architect, designer] | Done |
| Root (vision, INDEX) | 2 | [pg-coder, integrator, architect, designer] | Done |
| audit/ | 5 | per-audience | Done |
