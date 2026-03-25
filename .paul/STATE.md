# Project State

## Project Reference

See: .paul/PROJECT.md (updated 2026-03-24)

**Core value:** Building a new async programming language that can asynchronously compile other programming languages
**Current focus:** Language specification complete — next milestone TBD

## Current Position

Milestone: v0.2 Language Specification — COMPLETE
Phase: 12 of 12 — Complete. All phases finished.
Plan: All plans complete
Status: Issue #86 APPLY+UNIFY complete, ready for merge
Last activity: 2026-03-25 — Issue #86 plan executed, SUMMARY created

Progress:
- v0.1 Language Spec: [██████████] 100%
- v0.1.1 Doc Audit: [██████████] 100%
- v0.2 Language Spec: [██████████] 100% (4/4 phases)
- Issue #82: [██████████] 100% (1/1 plans)
- Issue #83: [██████████] 100% (1/1 plans)
- Issue #84: [██████████] 100% (1/1 plans)
- Issue #85: [██████████] 100% (1/1 plans)
- Issue #86: [██████████] 100% (1/1 plans)

## Active Issue

**Issue #86**: Audit fixed (.) vs flexible (:) field usage across all tree branches
- **Labels**: design, P2-high
- **Branch**: `design/issue-86-audit-fixed-vs-flexible-field-usage`
- **Key rule**: `.` for Polyglot-defined fixed fields, `:` for user-extensible flexible fields
- **Scope**: All % tree branches — %_, %@, %=, %!, %$, %#, %~, %*, %Q, %M, %definition
- **Files**: data-is-trees.md, metadata.md, metadata-tree.md, permissions.md
- **Context**: Raised during #82 — permission categories are Polyglot-defined, should use `.` not `:`

## Loop Position

Current loop state:
```
PLAN ──▶ APPLY ──▶ UNIFY
  ✓        ✓        ✓     [ready for /paul:merge]
```

## Accumulated Context

### Decisions
| Decision | Date | Impact |
|----------|------|--------|
| Reset to documentation-first approach | 2026-03-12 | All Rust code deleted; focus on spec, architecture, and research before coding |
| Removed BMAD agent infrastructure | 2026-03-12 | Only PAUL remains as project management framework |
| Polly to be rebuilt as PAUL flow | 2026-03-12 | Language expert agent will return as a PAUL special flow |
| Archive docs before deleting (checkpoint) | 2026-03-12 | Created tarballs instead of permanent deletion |
| New documentation standards (v2.0) | 2026-03-12 | Replaced BMAD conventions with audience-based system (User/Tech/Audit), YAML indexes, 50KB limit, 7 templates |
| Archive Agile/ and v0.0.5/ | 2026-03-12 | Historical BMAD-era content archived to tarballs; v0.0.5 premature |
| uv environment for Python tooling | 2026-03-12 | PyYAML added to pyproject.toml; validation scripts use pydantic |
| Migrate docs on touch, not batch | 2026-03-12 | New files must comply; legacy files migrate when modified |
| Added Phase 5: Clean Slate Reset | 2026-03-12 | Extends milestone scope — commit all pending changes, remove tarballs, clean git state |
| Wiped docs/ to skeleton | 2026-03-12 | User wants to write spec from scratch via draft loop, not consolidate existing docs |
| Scrapped Plan 02-01 | 2026-03-12 | Old plan consolidated scattered docs; new approach: fresh writing via /paul:draft |
| Rescoped Plan 02-01 | 2026-03-12 | Core Principles → Project Vision & Philosophy; covers whole project not just language rules |
| Created milestone v0.1.1 Doc Audit Infrastructure | 2026-03-14 | docs/audit/ as Claude's ground-truth reference for doc writing; extracted audience meta-instructions from vision.md |
| docs/ as Obsidian vault | 2026-03-14 | All docs use YAML frontmatter + [[wikilinks]] for Obsidian compatibility |
| Dual smart referencing | 2026-03-14 | @-imports (Claude mandatory reads) + [[wikilinks]] (Obsidian navigation) on all cross-references |
| Audit scope: all documentation | 2026-03-14 | docs/audit/ rules apply to ALL documentation Claude writes, not just docs/ files |
| Restructured v0.1 to 2 phases | 2026-03-14 | Dropped Language Spec, Compiler Architecture, Prior Art Research phases; Language Spec moves to v0.2 |
| v0.2 milestone created with 4 phases | 2026-03-24 | Phases 9-12: Core Language & Type System, Operators & Control Flow, Pipelines & Concurrency, Package System & Stdlib |
| Arithmetic uses =Math.* stdlib, not raw operators | 2026-03-24 | PGE-410 confirms raw +,-,*,/ are compile errors; spec reflects pipeline-based arithmetic |
| Removed speculative stdlib items | 2026-03-24 | =T.Schedule/HTTP/File, =W.Rust/Node had zero usage; =T.Webhook confirmed from EDGE-CASES |
| Closed #36 — Pipeline call cycle detection algorithm | 2026-03-24 | Merged feat/issue-36-pipeline-call-cycle-detection to main |
| Closed #37 — Multidimensional array via :ND | 2026-03-24 | Unified array + tensor into ;array.<type>:<N>D; removed ;tensor concept |
| Closed #58 — Chain operator >> to => | 2026-03-24 | Chains use => with no spaces (=A=>=B=>=C); IO >> and [!] >> unchanged |
| Closed #26 — Confirm/remove speculative =T.* triggers | 2026-03-24 | Verified Phase 12 already resolved all three; issue closed |
| Closed #28 — Confirm/remove speculative =W.* wrappers | 2026-03-24 | Verified Phase 12 already removed =W.Rust/Node; issue closed |
| Closed #60 — Add Mermaid state diagram to variable-lifecycle.md | 2026-03-24 | First Mermaid diagram in docs; established pattern for remaining 12 issues |
| Closed #73 — Add match syntax for concise conditional assignment | 2026-03-24 | Merged feat/issue-73-add-match-syntax to main; reuses [?] in match context |
| Issue #74 — Add [+], [|], [c] block markers | 2026-03-24 | [+] reassigned from OR to line continuation; [|] new OR marker; [c] foreign code injection; 11 files updated |
| Issue #80 — Add _ permission prefix and [_] block marker | 2026-03-25 | Implicit-deny permission system; _ is 7th identifier prefix; [_] block marker; 8 permission categories; compile-time enforcement |
| Issue #81 — Add package permission ceiling in {@} block | 2026-03-25 | Ceiling syntax in {@}; pipeline-level [_] in pipelines.md; PGE-915/916 compile rules; two separate rules for pipeline vs import ceiling |
| Issue #82 — Add %_ metadata tree branch for permissions | 2026-03-25 | %_ in all 3 tree files; 8 categories; ._ under %@ and %=; no instances; #86 created for . vs : audit |
| Issue #83 — Add permission compile rules | 2026-03-25 | PGE-917/918/919/920 + PGW-903; issue's PGE-916 renumbered to PGE-920; extends 9.x range |
| Issue #84 — Add !Permission.* error tree | 2026-03-25 | 8 .Denied leaves; trimmed overlapping .NotFound/.Timeout; File IO pipeline associations; #87 created for multi-alias |
| Issue #85 — Document stdlib pipeline permission declarations | 2026-03-25 | [_] permissions in all 7 stdlib pipeline files; Permission column in INDEX.md; completes #80-#85 chain |
| Issue #86 — Audit fixed vs flexible field usage | 2026-03-25 | %_ all `.` fixed; %! namespaces `.` + new !Error with `:` children; %@ `::` separator + Company rename; path grammar updated |

### Deferred Issues
- ~~Rebuild Polly as PAUL special flow~~ — closed, redundant with pg:generate/pg:train
- 9 stdlib files remain status: draft despite Stable content (Math, Path, Sys, ForEach, collectors, types)
- EC-6.4 inconsistency: raw arithmetic in EDGE-CASES vs PGE-410
- ~30 technical/ files still use old package address format (migrate on touch)

### Blockers/Concerns
None.

## Boundaries (Active)

Protected elements for current milestone:
- docs/ directory (existing documentation to build on)
- .paul/ directory (project management)

## Session Continuity

Last session: 2026-03-25
Stopped at: Issue #86 UNIFY complete
Next action: /paul:merge
Resume file: .paul/phases/issue-86-audit-fixed-vs-flexible-field-usage/86-01-SUMMARY.md
Resume context:
- Issue #86 loop complete — %_ all fixed, %! namespaces fixed + !Error, %@ `::` separator
- 7 files modified, all verification passed
- Branch: design/issue-86-audit-fixed-vs-flexible-field-usage

---
*STATE.md — Updated after every significant action*
