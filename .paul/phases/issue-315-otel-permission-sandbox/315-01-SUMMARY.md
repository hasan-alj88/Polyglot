---
phase: issue-315-otel-permission-sandbox
plan: 01
type: summary
status: complete
completed: 2026-04-18
---

# Plan 01 Summary — OTel Permission Events Specification

## What Was Done

### Task 1: Created OTel permission events specification

Created `docs/technical/spec/otel-permission-events.md` with:

- **8 event types** fully specified: permission.sandbox.setup, .setup_failed, .violation, permission.resource.exceeded, .kill, permission.ast.suppressed, permission.opaque.activated, permission.compliance.generated
- **Per-event sections** with trigger condition, severity rationale, required/optional attributes, example OTel log record (JSON), and parent span assignment
- **Attribute registry** with 9 `polyglot.*` attributes (job.uid, pipeline.name, package.name, permission.category, sandbox.layer, sandbox.syscall, sandbox.resource, sandbox.action, sandbox.opaque)
- **Span hierarchy** showing sandbox spans nesting inside Job Span (which nests inside Pipeline Span from #318)
- **Runtime compliance appendix format** with JSON schema, storage model, and comparison table vs compile-time report
- **Exporter independence note** clarifying scope boundary with #318
- **Open questions** section (fallback logging, sampling, NATS trace context)

### Task 2: Updated cross-references in 3 files

1. **`docs/technical/spec/job-sandbox.md`** — Replaced "See the tracked sub-issue" with wikilink to [[otel-permission-events]]; added `@c:` cross-reference; added wikilink to Related line
2. **`docs/technical/compiler/compliance-report.md`** — Added Runtime Appendix section before Related; added `@c:` cross-reference; added wikilink to Related section
3. **`docs/technical/spec/behavior-contract.md`** — Added OTel paragraph to Permission Manifest section; added `@c:` cross-reference; added wikilink to Related line

## Files Modified

| File | Action |
|---|---|
| `docs/technical/spec/otel-permission-events.md` | Created (new) |
| `docs/technical/spec/job-sandbox.md` | Updated (cross-refs + Future Work) |
| `docs/technical/compiler/compliance-report.md` | Updated (Runtime Appendix + cross-refs) |
| `docs/technical/spec/behavior-contract.md` | Updated (Permission Manifest OTel note + cross-refs) |

## Decisions Made

- **Severity split for resource.exceeded:** WARN for throttle (job continues), ERROR for kill (job terminates). This follows the OTel severity model where WARN = degraded but operational.
- **permission.resource.kill as separate event:** Split from resource.exceeded because the kill happens after a grace period and attaches to the Job Span directly (not the Execution Span), reflecting different temporal context.
- **permission.ast.suppressed fires at job startup:** The Runner emits this when loading the compliance report, not during compilation — runtime operators need to see suppressed errors for jobs they are monitoring.
- **Runtime appendix keyed by job_uid:** Each execution produces its own appendix entry. The compile-time report remains immutable.

## Verification

- [x] All 8 events documented with trigger, severity, attributes, example, parent span
- [x] Attribute registry has 9 polyglot.* entries
- [x] Span hierarchy shows sandbox spans nesting inside Job Span
- [x] Runtime compliance appendix format defined with JSON schema
- [x] job-sandbox.md references otel-permission-events (no "tracked sub-issue")
- [x] compliance-report.md has Runtime Appendix section
- [x] behavior-contract.md references OTel in Permission Manifest
- [x] All files have valid frontmatter
- [x] All acceptance criteria (AC-1 through AC-5) met
