---
phase: 280-inline-pipeline-syntax
plan: 01
subsystem: docs
tags: [inline-calls, %InlineString, template, compiler-rules, jm3lib]

requires:
  - phase: 274-generic-input-marker
    provides: (#) generic input syntax merged to main
provides:
  - "%InlineString template-based inline pipeline call syntax"
  - "7 new compiler rules for template validation (PGE12005-PGE12010, PGW12002)"
  - "Rewritten PGE12003 and PGW12001 for new system"
  - "3 jm3lib pipelines updated to use %InlineString"
affects: [compile-rules, jm3lib, inline-calls]

tech-stack:
  added: []
  patterns: ["%InlineString template with {name}/{name?} placeholders"]

key-files:
  created:
    - docs/technical/compile-rules/PGE/PGE12005-inline-format-mismatch.md
    - docs/technical/compile-rules/PGE/PGE12006-unresolved-template-placeholder.md
    - docs/technical/compile-rules/PGE/PGE12007-required-input-not-in-template.md
    - docs/technical/compile-rules/PGE/PGE12008-duplicate-template-placeholder.md
    - docs/technical/compile-rules/PGE/PGE12009-template-type-coercion-failure.md
    - docs/technical/compile-rules/PGE/PGE12010-optional-placeholder-without-default.md
    - docs/technical/compile-rules/PGW/PGW12002-optional-placeholder-never-provided.md
  modified:
    - docs/user/concepts/pipelines/inline-calls.md
    - docs/technical/ebnf/08-expressions.md
    - docs/technical/ebnf/09-definition-blocks.md
    - docs/technical/edge-cases/04-type-system.md
    - docs/technical/compile-rules/PGE/PGE12003-invalid-inline-pipeline-argument.md
    - docs/technical/compile-rules/PGW/PGW12001-missing-inline-format-metadata.md
    - docs/technical/COMPILE-RULES.md
    - docs/user/jm3lib/pipelines/Path.md
    - docs/user/jm3lib/pipelines/T/Daily.md
    - docs/user/jm3lib/pipelines/T/Webhook.md

key-decisions:
  - "Shifted new rule numbering to PGE12005-PGE12010 because PGE12004 already existed (Empty Metadata Alias)"

patterns-established:
  - "%InlineString << \"{template}\" declares inline call template in (-) IO section"
  - "{name} = required placeholder, {name?} = optional placeholder"

duration: ~15min
started: 2026-04-12
completed: 2026-04-12
---

# Issue #280 Plan 01: %InlineString Template System Summary

**Replaced opaque `<InlineStringLiteral` parameter with structured `%InlineString` template-based inline pipeline syntax — compiler extracts named values from placeholders and wires them to declared inputs.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Started | 2026-04-12 |
| Completed | 2026-04-12 |
| Tasks | 3 completed |
| Files modified | 17 (7 created, 10 modified) |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: %InlineString Template Syntax Documented | Pass | inline-calls.md fully rewritten with template declaration, mechanism, optional placeholders, dual-mode |
| AC-2: EBNF Grammar Updated | Pass | 08-expressions.md comment updated, 09-definition-blocks.md has inline_template_decl production |
| AC-3: Compiler Rules Complete | Pass | PGE12003 rewritten, PGE12005-12010 created, PGW12001 rewritten, PGW12002 created, index updated |
| AC-4: jm3lib Pipelines Use %InlineString | Pass | -Path, -T.Daily, -T.Webhook all declare %InlineString with named inputs |
| AC-5: No Stale =Pipeline References | Pass | Zero results for InlineStringLiteral, =Path", =Pipeline", inlineFormat, inlineExamples in docs/ |

## Accomplishments

- Rewrote inline-calls.md with complete %InlineString template system: `{name}` required and `{name?}` optional placeholders, 5-step mechanism, updated Mermaid diagram
- Created 6 new compiler rules (PGE12005-PGE12010) + PGW12002 covering template validation, format mismatch, placeholder resolution, type coercion, and optional placeholder semantics
- Updated 3 jm3lib pipelines (-Path, -T.Daily, -T.Webhook) and 3 edge case entries (EC-4.9, EC-4.11, EC-4.12) to use new syntax

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/user/concepts/pipelines/inline-calls.md | Modified | Full rewrite: %InlineString template, mechanism, optional placeholders, dual-mode |
| docs/technical/ebnf/08-expressions.md | Modified | EBNF comment updated for %InlineString |
| docs/technical/ebnf/09-definition-blocks.md | Modified | Added inline_template_decl grammar production |
| docs/technical/edge-cases/04-type-system.md | Modified | EC-4.9, 4.11, 4.12 updated to %InlineString |
| docs/technical/compile-rules/PGE/PGE12003-*.md | Modified | Rewritten: Undefined Inline Template |
| docs/technical/compile-rules/PGE/PGE12005-*.md | Created | Inline Format Mismatch |
| docs/technical/compile-rules/PGE/PGE12006-*.md | Created | Unresolved Template Placeholder |
| docs/technical/compile-rules/PGE/PGE12007-*.md | Created | Required Input Not In Template |
| docs/technical/compile-rules/PGE/PGE12008-*.md | Created | Duplicate Template Placeholder |
| docs/technical/compile-rules/PGE/PGE12009-*.md | Created | Template Type Coercion Failure |
| docs/technical/compile-rules/PGE/PGE12010-*.md | Created | Optional Placeholder Without Default |
| docs/technical/compile-rules/PGW/PGW12001-*.md | Modified | Rewritten: Template With No Placeholders |
| docs/technical/compile-rules/PGW/PGW12002-*.md | Created | Optional Placeholder Never Provided |
| docs/technical/COMPILE-RULES.md | Modified | Index updated with all new rules |
| docs/user/jm3lib/pipelines/Path.md | Modified | %InlineString << "{path}", <path#path |
| docs/user/jm3lib/pipelines/T/Daily.md | Modified | %InlineString << "{time}", <time#string |
| docs/user/jm3lib/pipelines/T/Webhook.md | Modified | %InlineString << "{endpoint}", <endpoint#string |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Shift new rules to PGE12005-PGE12010 | PGE12004 already existed as "Empty Metadata Alias" | Plan specified PGE12004-PGE12009; actual is PGE12005-PGE12010. No functional impact. |

## Deviations from Plan

### Summary

| Type | Count | Impact |
|------|-------|--------|
| Auto-fixed | 1 | Rule numbering shifted by 1 |
| Scope additions | 0 | — |
| Deferred | 0 | — |

**Total impact:** Minimal — numbering shifted, all rules created with correct content.

### Auto-fixed Issues

**1. Rule numbering collision**
- **Found during:** Task 2 (compiler rules)
- **Issue:** Plan specified PGE12004 for "Inline Format Mismatch" but PGE12004 already existed as "Empty Metadata Alias"
- **Fix:** Shifted all new rules by +1 (PGE12005-PGE12010 instead of PGE12004-PGE12009)
- **Files:** All new PGE rule files and COMPILE-RULES.md index
- **Verification:** All 8 rule files exist with correct numbering, no collisions

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- %InlineString template system fully documented
- Compiler rules provide complete validation coverage
- jm3lib pipelines demonstrate the new syntax
- Phase is single-plan — ready for transition

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 280-inline-pipeline-syntax, Plan: 01*
*Completed: 2026-04-12*
