---
type: handoff
phase: issue-338-regenerate-project-docs
plan: 338-01
created: 2026-04-23
reason: context exhausted mid-APPLY (parallel agents failed + manual writes consumed budget)
---

# HANDOFF — Issue #338 mid-APPLY

## Status

Loop position: PLAN ✓ → APPLY (in progress) → UNIFY ○ → MERGE ○
Plan: [.paul/phases/issue-338-regenerate-project-docs/338-01-PLAN.md](.paul/phases/issue-338-regenerate-project-docs/338-01-PLAN.md)
Branch: `issue/338-regenerate-project-docs`

## What was done this session

1. **AC-1 Triage decision resolved** — option `keep-six-rename-index`. Recorded in STATE.md Decisions row and appended as "Triage Outcome" heading in 338-01-PLAN.md.
2. **Parallel agent attempt failed** — spawned 6 `bmm-codebase-analyzer` agents; 3 hit output-size limits without writing, 1 reported tool sandbox issues, 1 falsely reported success, 1 stopped early. All 6 target files were missing on disk after agents finished. Do NOT retry this pattern — write files directly in main session.
3. **Repo structure surveyed and cached** — verified real paths: `docs/user/jm3lib/pipelines/{DT,File,Math,Q,RT,Run,Schema,T,Text,Variable,W}/`, `expanders/ForEach/`, `collectors/{Agg,Into,Sync}/`, `types/{datetime,field-types,properties,scalars,schemas}/`; `technical/compile-rules/PGE` = 188 files, `PGW` = 31, `algorithms` = 3; `technical/ebnf/` = 16 files + INDEX; `technical/spec/` = 12 files incl metadata-tree/; `technical/compiler/` = 4 files + INDEX; audiences/ = 6 files (ai-finder, automation-builder, design, developer, integrator, product); philosophy/ = 11 files.
4. **2 of 6 files written** (audit-clean, typed refs, no H4+, frontmatter correct):
   - `docs/project-overview.md` (product, type: reference) — ~6KB
   - `docs/development-guide.md` (developer, type: guide) — ~7KB

## What remains (in order)

**4 files to write** (use Write tool directly — do NOT spawn bmm-codebase-analyzer agents; they failed last time):

| File | Audience | Type | Target size | Notes |
|------|----------|------|-------------|-------|
| `docs/source-tree-analysis.md` | ai-finder | reference | 15–25KB | Repo shape map. Use ASCII box chars `├── └── │   ` (U+251C/2514/2502 — stable). Trees per major dir. Do NOT invent subdir names — real structure is cached in 338-01-PLAN triage section plus this handoff. |
| `docs/ai-retrieval-index.md` | ai-finder | reference | 20–30KB | Query-shaped lookup (concept → file). Differ from INDEX.md (audience-shaped) and component-inventory (inventory-shaped). Sections per PLAN task 2: by Construct, by Compile Rule Range, by jm3lib Namespace, by Type, by Error Namespace, by Philosophy, by Audit Area, Retrieval Hints, Cross-Reference Types Used. |
| `docs/architecture.md` | design | spec | 20–35KB | Consolidated architecture read. Sections: Overview, Language Layer, Compiler Layer, Service Layer (TM + QH + DC + Runner), Runtime Execution (-Run.*, -W.*), SDK & Cross-Language (polyglot-sdk.md, -Run.Bridge), Observability (otel-foundation, otel-permission-events), Authority Chain & Design Process. Cite technical/spec/*.md files verbatim; do not duplicate their prose. |
| `docs/component-inventory.md` | ai-finder | reference | 25–45KB | Flat categorical inventory. Sections: Block Types ({@} {#} {-} {=} {W} {T} {Q} {M} {N} {C} {!} {_} {$} {*} {;}), jm3lib Pipelines (by namespace), jm3lib Expanders (ForEach/), jm3lib Collectors (Agg/ Into/ Sync/), jm3lib Types (by folder), Error Namespaces, Compile Rule Groups (PGE01xxx–PGE14xxx + PGW), EBNF Sections (01–16), Philosophy Pages, Audit Rules. Don't enumerate every operator — cite folder INDEX.md files. |

**Then (Task 3 + finalize):**

5. Run verification greps on all 6 new files:
   - `grep -nP '@(?!c:|u:|d:)[a-zA-Z/]+' docs/{f}.md` → should return zero bare `@`-refs to doc paths
   - `grep -nP '^#### ' docs/{f}.md` → should return zero H4+
   - `grep -cP '\[\[[^|]+\|[cud]:' docs/{f}.md` → should return positive count
   - `ls -la` the files, verify all under 50KB
   - Visual-inspect `source-tree-analysis.md` box chars
6. Fix any audit issues found.
7. **Update `docs/INDEX.md`** (Task 3): add "For AI Tools" subsection under By Audience with rows for ai-retrieval-index/source-tree-analysis/component-inventory; add File Registry rows for all 6 new files under `### docs/`; set `updated: 2026-04-23` in frontmatter. Keep diff minimal — don't reorder existing rows, don't fix the pre-existing stale refs (audit/audiences/user.md, ai.md — those are out of scope per plan boundaries).
8. Create `.paul/phases/issue-338-regenerate-project-docs/338-01-SUMMARY.md` with per-file triage (all 6 kept), final filenames, sizes, any authority-chain issues found, follow-up issues (none expected).
9. Update STATE.md: loop position → PLAN ✓ APPLY ✓ UNIFY ○ MERGE ○; Session Continuity → "run /paul:unify .paul/phases/issue-338-regenerate-project-docs/338-01-PLAN.md".

## Authority chain reminders (already read this session; re-read if context is fresh)

- [docs/audit/README.md](docs/audit/README.md) — typed refs `@c:`/`@u:`/`@d:` paired with `[[wikilink]]`
- [docs/audit/rules/conventions.md](docs/audit/rules/conventions.md) — frontmatter (audience, type, updated), H1 + H2 only, language tags on all fences, 50KB max
- [docs/audit/rules/checklist.md](docs/audit/rules/checklist.md) — pre-publish checks
- [docs/audit/reference/glossary.md](docs/audit/reference/glossary.md) — use terms verbatim; key terms: Polyglot, Polyglot Code, Polyglot Service, Trigger Monitor, Queue Handler, Dispatch Coordinator, Runner, Instance, Job, Pipeline, Behavior Contract, Reconciliation, RawString, #String, *Agg, ##Record, Schema Bundle, Leaf Bundle, Constructor, Operation Label
- [docs/audit/audiences/ai-finder.md](docs/audit/audiences/ai-finder.md) — structured tables, no "may/might/usually", exact glossary terms
- [docs/audit/audiences/design.md](docs/audit/audiences/design.md) — precise, spec-oriented, EBNF/compile-rule refs, every failure mode named
- [docs/vision.md](docs/vision.md) — two pillars: Cross-Language Integration + Trigger-Driven Orchestration
- [docs/INDEX.md](docs/INDEX.md) — existing master index; new "By Audience / For AI Tools" subsection goes here

## Boundaries (from plan — do not violate)

- No edits to: `docs/vision.md`, `docs/philosophy/*`, `docs/audit/**/*`, `docs/user/**`, `docs/technical/**`, `docs/archive/**`, `.paul/*` (except STATE.md session-continuity + loop-position at finalize, and SUMMARY.md creation)
- Only `docs/INDEX.md` may be edited for link rows
- No new compile rules, EBNF edits, jm3lib additions
- No deletions

## Gotchas

- `docs/INDEX.md` already has stale refs (`audit/audiences/user.md`, `audit/audiences/ai.md` which no longer exist — real audiences are: ai-finder, automation-builder, design, developer, integrator, product). Plan scope says leave those alone; only add new rows.
- No `.paul/README.md` exists. Use [[.paul/PROJECT]] and [[.paul/STATE]] for PAUL-workflow refs.
- `docs/user/jm3lib/` has an `errors/` subdir AND a top-level `JM3LIB.md`; `jm3lib/INDEX.md` is the canonical entry.
- `docs/technical/` has `algorithms/` at the top AND `compile-rules/algorithms/`. Reference them distinctly.
- `docs/technical/brainstorming/` and `docs/technical/plan/` exist but are work-in-progress. Don't cite them as authoritative.

## Resume command

```
/paul:resume
```

Expected routing: "continue APPLY — 4 files remaining". Resume by Write-ing each file directly; do NOT spawn analyzer agents.
