---
phase: 313-system-level-job-sandbox
plan: 01
subsystem: permissions
tags: [sandbox, landlock, seccomp, namespaces, cgroups, security]

requires:
  - phase: 311-foreign-code-permission-compliance
    provides: AST analysis foundation, PGE10011-14, compliance report
  - phase: 312-ast-invisible-functions-registry
    provides: PGE10014 banned list, registry format

provides:
  - Job sandbox specification (docs/technical/spec/job-sandbox.md)
  - Expanded enforcement docs with sandbox details
  - Permission Manifest in Behavior Contract (5th section)

affects: [313-02 compile rules, resource categories sub-issue, OTel sub-issue]

tech-stack:
  added: []
  patterns: [Permission Manifest as contract section, sandbox-as-defense-in-depth]

key-files:
  created: [docs/technical/spec/job-sandbox.md]
  modified: [docs/user/concepts/permissions/enforcement.md, docs/technical/spec/behavior-contract.md]

key-decisions:
  - "Rust-native sandbox implementation over bubblewrap"
  - "USER_NOTIF supervisor is optional enhancement, not default"
  - "Permission Manifest is 5th top-level Behavior Contract section"

patterns-established:
  - "_Unsafe.SandboxOnly uses [!] error handler position for opaque code acknowledgment"
  - "Sandbox inspection via polyglot inspect -sandbox CLI tool"

duration: ~15min
started: 2026-04-18
completed: 2026-04-18
---

# Issue #313 Plan 01: Job Sandbox Specification Summary

**OS-level job sandbox spec: Landlock + seccomp + namespaces + cgroups mapping from {_} permissions, _Unsafe.SandboxOnly for opaque code, Permission Manifest in Behavior Contract**

## Performance

| Metric | Value |
|--------|-------|
| Duration | ~15min |
| Started | 2026-04-18 |
| Completed | 2026-04-18 |
| Tasks | 3 completed |
| Files modified | 3 (1 created, 2 updated) |

## Acceptance Criteria Results

| Criterion | Status | Notes |
|-----------|--------|-------|
| AC-1: Job Sandbox Specification Exists | Pass | 11 sections, ~265 lines, all required content |
| AC-2: Enforcement Documentation Expanded | Pass | Category table, _Unsafe.SandboxOnly, inspection, cross-refs |
| AC-3: Behavior Contract Includes Permission Manifest | Pass | 5th section, manifest flow, contents table, registration entry |

## Accomplishments

- Created `docs/technical/spec/job-sandbox.md` — full implementer-facing sandbox spec covering setup sequence, category mapping, _Unsafe.SandboxOnly, -Run.* variations, runtime violations, inspection CLI, Rust-native implementation, supervisor architecture, and kernel requirements
- Expanded `docs/user/concepts/permissions/enforcement.md` — replaced 5-row sandbox table with full category mapping, _Unsafe.SandboxOnly user docs, and inspection mention
- Updated `docs/technical/spec/behavior-contract.md` — Permission Manifest as 5th contract section with manifest flow, contents table, and registration entry

## Files Created/Modified

| File | Change | Purpose |
|------|--------|---------|
| `docs/technical/spec/job-sandbox.md` | Created | Full sandbox specification for implementers |
| `docs/user/concepts/permissions/enforcement.md` | Modified | Expanded Foreign Code Sandbox section |
| `docs/technical/spec/behavior-contract.md` | Modified | Added Permission Manifest section + updated contract structure |

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| Rust-native over bubblewrap | bwrap lacks Landlock, cgroups, USER_NOTIF | Implementation uses landlock + seccompiler + nix crates |
| USER_NOTIF optional | Landlock + namespaces + nftables cover majority of cases | Start simple, add supervisor for high-security mode |
| Permission Manifest is 5th section | Natural extension of contract structure | Runner reads manifest from same NoSQL DB as signal map |

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered

None.

## Next Phase Readiness

**Ready:**
- job-sandbox.md provides foundation for compile rules (313-02)
- _Unsafe.SandboxOnly syntax and behavior fully specified for PGE10015/PGE10016/PGW10007
- Sub-issue scope clear from Future Work section

**Concerns:**
- None

**Blockers:**
- None

---
*Phase: 313-system-level-job-sandbox, Plan: 01*
*Completed: 2026-04-18*
