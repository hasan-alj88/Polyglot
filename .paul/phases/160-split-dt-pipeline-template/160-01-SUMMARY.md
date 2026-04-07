---
phase: 160-split-dt-pipeline-template
plan: 01
subsystem: docs
tags: [pglib, DT, datetime, pipeline-docs, template]

requires: []
provides:
  - DT/ folder with 40 individual pipeline docs + INDEX.md
  - Pipeline doc template pattern (frontmatter, definition, inputs, outputs, errors, permissions, related)
affects: [issue-161-apply-template-to-all-pipelines]

tech-stack:
  added: []
  patterns:
    - "Pipeline doc template: frontmatter → description → {N} definition → Inputs table → Outputs table → Errors → Permissions → Related"
    - "Pipeline folder structure: namespace/INDEX.md + one .md per pipeline"

key-files:
  created:
    - docs/user/pglib/pipelines/DT/INDEX.md
    - docs/user/pglib/pipelines/DT/*.md (40 pipeline files)
  modified:
    - docs/user/pglib/INDEX.md
    - docs/technical/compile-rules/PGE/PGE04026-invalid-iana-timezone.md
    - docs/technical/compile-rules/PGE/PGE04027-missing-required-datetime-subfield.md
    - docs/technical/compile-rules/PGE/PGE04028-invalid-epoch-value.md

key-decisions:
  - "Pipeline count is 40 not 37 — original issue undercount corrected"
  - ".version metadata removed from all {N} blocks — EBNF §9.4c violation"
  - "Wikilinks point to specific pipelines where context warrants (not just INDEX)"

patterns-established:
  - "Pipeline doc template with 6 standard sections"
  - "DT/ folder pattern for namespace-based pipeline organization"

duration: ~15min
started: 2026-04-07
completed: 2026-04-07
---

# Issue #160 Plan 01: Split DT.md into DT/ folder with pipeline doc template

**Split 692-line DT.md into 41-file DT/ folder (INDEX + 40 pipelines), each with standardized doc template; removed EBNF-invalid .version metadata.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Tasks | 2 completed |
| Files created | 41 |
| Files modified | 4 |
| Files deleted | 1 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: DT/ folder replaces DT.md | Pass | 41 files in DT/, DT.md deleted |
| AC-2: INDEX.md preserves overview | Pass | Intro, inline notation, permissions table, categorized listing |
| AC-3: Each file follows template | Pass | All 40 pipeline files have frontmatter + 6 sections |
| AC-4: .version metadata removed | Pass | grep confirms zero matches |
| AC-5: Wikilinks updated | Pass | 4 files updated, grep confirms no broken refs |

## Accomplishments

- Split monolithic DT.md (692 lines) into 41 individually-linkable files
- Established pipeline doc template pattern for reuse in #161
- Removed invalid `.version` metadata from all {N} blocks (EBNF §9.4c compliance)
- Updated wikilinks to point to specific pipelines (e.g., PGE04026 → DT/Zone.Set, DT/Zone.Convert)

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/user/pglib/pipelines/DT/INDEX.md` | Created | Overview, inline notation, permissions, categorized pipeline listing |
| `docs/user/pglib/pipelines/DT/*.md` (40 files) | Created | Individual pipeline docs with template |
| `docs/user/pglib/pipelines/DT.md` | Deleted | Replaced by DT/ folder |
| `docs/user/pglib/INDEX.md` | Modified | DT.md → DT/INDEX.md link |
| `docs/technical/compile-rules/PGE/PGE04026-invalid-iana-timezone.md` | Modified | Wikilink → DT/Zone.Set, DT/Zone.Convert |
| `docs/technical/compile-rules/PGE/PGE04027-missing-required-datetime-subfield.md` | Modified | Wikilink → DT/From.Parts |
| `docs/technical/compile-rules/PGE/PGE04028-invalid-epoch-value.md` | Modified | Wikilink → DT/From.Epoch |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| 40 pipelines not 37 | Recount from DT.md (4+11+3+3+4+7+2+3+3=40) | Issue body had undercount; corrected |
| .version removed | EBNF §9.4c native_field only allows .Kind, .<Language>, .description | All DT {N} blocks now EBNF-compliant |
| Specific pipeline wikilinks in PGE files | PGE04026 references Zone.Set/Zone.Convert specifically | More precise cross-references |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 1 | Pipeline count corrected (40 not 37) |
| Deferred | 0 | — |

**Total impact:** Minor correction, no scope creep.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Pipeline doc template pattern established and proven on 40 files
- DT/ folder structure ready as reference for #161 (apply template to remaining pipeline files)

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 160-split-dt-pipeline-template, Plan: 01*
*Completed: 2026-04-07*
