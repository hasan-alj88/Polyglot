---
phase: 313-system-level-job-sandbox
plan: 02
subsystem: security
tags: [compile-rules, sandbox, permissions, PGE10015, PGE10016, PGW10007]

requires:
  - phase: 313-01
    provides: job-sandbox.md spec, enforcement.md updates, behavior-contract.md updates
provides:
  - PGE10015 compile rule (opaque binary without _Unsafe.SandboxOnly)
  - PGE10016 compile rule (missing mandatory metadata)
  - PGW10007 warning (sandbox-only enforcement active)
  - GitHub sub-issues for resource categories and OTel logging
affects: [issue-316-resource-categories, issue-317-opentelemetry]

key-files:
  created:
    - docs/technical/compile-rules/PGE/PGE10015-opaque-binary-without-sandbox-only.md
    - docs/technical/compile-rules/PGE/PGE10016-missing-unsafe-metadata.md
    - docs/technical/compile-rules/PGW/PGW10007-sandbox-only-active.md
  modified:
    - docs/technical/COMPILE-RULES.md

key-decisions:
  - "Sub-issues created as #316/#317 (not #314/#315 — other issues created between planning and execution)"

duration: ~10min
completed: 2026-04-18
---

# Issue #313 Plan 02: Compile Rules + Sub-Issues Summary

**Three sandbox compile rules (PGE10015, PGE10016, PGW10007) documented and registered; two sub-issues (#316, #317) created for deferred resource categories and OTel logging.**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~10min |
| Completed | 2026-04-18 |
| Tasks | 3 completed |
| Files created | 3 |
| Files modified | 1 |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: PGE10015 documented | Pass | Severity error, trigger/resolution/examples, cross-refs to job-sandbox.md |
| AC-2: PGE10016 documented | Pass | Severity error, missing metadata trigger, examples showing 1 and 3 missing fields |
| AC-3: PGW10007 documented | Pass | Severity warning, shows suppressed PGE10014 example |
| AC-4: COMPILE-RULES.md updated | Pass | PGE10015, PGE10016 in 10.x section; PGW10007 in warning table |
| AC-5: Sub-issues created | Pass | #316 (resource categories) and #317 (OTel) — not #314/#315 as anticipated |

## Accomplishments

- Documented PGE10015: forces developer acknowledgment when using opaque binaries
- Documented PGE10016: enforces accountability metadata (%Authors, %Description, %Version) on _Unsafe.SandboxOnly pipelines
- Documented PGW10007: informational warning surfacing suppressed PGE10014 findings in compliance report
- Created #316 for resource categories (#RAM, #CPU, #GPU, #IO, #Processes, #Duration) in {_}
- Created #317 for OpenTelemetry logging of permission/sandbox events

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| docs/technical/compile-rules/PGE/PGE10015-opaque-binary-without-sandbox-only.md | Created | Rule 9.29 — opaque binary must have [!] _Unsafe.SandboxOnly |
| docs/technical/compile-rules/PGE/PGE10016-missing-unsafe-metadata.md | Created | Rule 9.30 — mandatory %Authors/%Description/%Version with _Unsafe.SandboxOnly |
| docs/technical/compile-rules/PGW/PGW10007-sandbox-only-active.md | Created | Rule 9.8 — informational warning when _Unsafe.SandboxOnly active |
| docs/technical/COMPILE-RULES.md | Modified | Added 3 new entries to error/warning code tables |

## Deviations from Plan

| Type | Count | Impact |
|------|-------|--------|
| Numbering | 1 | No impact — sub-issues assigned #316/#317 instead of #314/#315 |

Sub-issues were created as #316 and #317 because other issues existed in the repository between planning and execution. The plan anticipated this possibility.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- Issue #313 fully complete (2/2 plans done)
- All compile rules documented and registered
- Sub-issues created for future work (#316, #317)

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 313-system-level-job-sandbox, Plan: 02*
*Completed: 2026-04-18*
