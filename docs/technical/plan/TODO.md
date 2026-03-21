# Master TODO List

**This is the single central tracker for all outstanding work.** All TODOs, open issues, brainstorms, and documentation gaps live here. Do not create separate tracking lists elsewhere.

Last updated: 2026-03-20

## How to Use This File

### Adding Items

- **New compiler issue:** Add a row to [Compiler Issues](#compiler-issues-unresolved). Create a corresponding file in `docs/technical/compiler_issues/NNN-short-name.md` using the next available number. Set priority (High/Medium/Low).
- **New brainstorm:** Add a row to [Design Brainstorms](#design-brainstorms-open). Create a file in `docs/technical/brainstorming/topic-name.md` with `## Open Questions` section.
- **New doc gap:** Add a row to [Documentation Gaps](#documentation-gaps) with the area, what's missing, and where to fix it.
- **New algorithm spec:** Add a row to [Algorithm Specs](#algorithm-specs-deferred-implementation). File goes in `docs/technical/plan/TODO/NNN-short-name.md`.

### Resolving Items

- When a compiler issue is resolved, remove its row and add a line to [Completed](#completed-recent) with the date and summary.
- When a brainstorm is decided, move the decision to the relevant spec file, remove the row, and add to Completed.
- When a doc gap is filled, remove the row and add to Completed.
- Keep Completed section trimmed to the last ~20 items. Archive older entries by deleting them (git history preserves the record).

### Updating

- Update `Last updated` date whenever this file changes.
- When resolving compiler issues, also update the "Issues NNN–NNN resolved" note in the Compiler Issues header.
- Cross-reference: if a TODO here corresponds to a file, keep the file path accurate.

---

## Compiler Issues (Unresolved)

Issues 001–030 resolved. No remaining compiler issues.


## Compile Rule Open Points

Open points within existing compile rules that need future attention:

| Rule | Open Point | Priority |
|------|-----------|----------|
| ~~PGE-407~~ | ~~Path validation~~ — OS rules deferred to implementation; cross-platform inference mechanism added | ~~Medium~~ |
| ~~PGE-913~~ | ~~Reserved namespace list~~ — now maintained in `docs/user/stdlib/INDEX.md` | ~~Low~~ |

## Design Brainstorms (Open)

| Topic | File | Summary |
|-------|------|---------|
| ~~Serial → Struct matching~~ | ~~brainstorming/serial-to-struct-matching.md~~ | **Decided** — see completed items |
| ~~#DateTime PGE-501 fix~~ | ~~brainstorming/datetime-tree.md~~ | **Decided** — see completed items |
| ~~#DateTime open questions~~ | ~~brainstorming/datetime-tree.md §Open Questions~~ | **Decided** — see completed items |
| ~~Typed flex field inference~~ | ~~brainstorming/typed-flex-field-inference.md~~ | **Decided** — see completed items |
| ~~String RE subfield system~~ | ~~brainstorming/string-re-subfields.md~~ | **Decided** — see completed items |

## Algorithm Specs (Deferred Implementation)

| File | Summary |
|------|---------|
| plan/TODO/006-compound-exhaustiveness-algorithm.md | Boolean algebra for compound condition exhaustiveness |
| plan/TODO/007-overlap-detection-algorithm.md | Overlap detection for conditional branches |

## STDLIB Confirmation Needed

~50+ items in `docs/user/STDLIB.md` marked `(?)` (speculative/unconfirmed). These need formal confirmation or removal:

| Namespace | Speculative Items |
|-----------|-------------------|
| `=File.*` | `.Copy`, `.Move`, `.Delete`, `.List` |
| `=T.*` (Triggers) | `.Schedule.Cron`, `.HTTP.Webhook`, `.File.Created` |
| `=Q.*` (Queue) | `.Priority`, `.DispatchIf.*`, `.KillIf.*`, `.ReplaceReTriggeredIf.*` |
| `=W.*` (Wrappers) | `.Rust`, `.Node` |

## Documentation Gaps

| Area | Gap | Where to fix |
|------|-----|-------------|
| ~~Struct terminology~~ | ~~Updated user-facing docs~~ | ~~Done~~ |
| ~~Typed flexible fields `[:] :*;Type`~~ | ~~Added `typed_flex_wildcard` production to EBNF~~ | ~~Done~~ |
| ~~Typed flexible fields~~ | ~~EC-4.13 through EC-4.17 added to EDGE-CASES.md~~ | ~~Done~~ |
| ~~Typed flexible fields~~ | ~~No new PGE needed — PGE-401/PGE-402 already cover type/schema violations; PGE-409 added to reference table~~ | ~~Done~~ |
| ~~Serial always flexible~~ | ~~No rule needed — `.` vs `:` distinction is meaningless on serial (schema-free, any key accepted)~~ | ~~Done~~ |
| ~~String literal typing~~ | ~~Added to types.md Basic Types~~ | ~~Done~~ |
| ~~`struct` → `serial` conversion~~ | ~~Added to types.md Type Conversions~~ | ~~Done~~ |
| ~~`serial` → `struct` conversion~~ | ~~Added to types.md Type Conversions~~ | ~~Done~~ |
| ~~STDLIB struct references~~ | ~~stdlib/types.md labels them as structs~~ | ~~Done~~ |
| ~~Numeric RE patterns~~ | ~~int/float RE patterns specified in types.md and EBNF.md~~ | ~~Done~~ |
| ~~Future numeric types~~ | ~~`eng`/`sci` RE patterns proposed in string-re-subfields decision~~ | ~~Proposed~~ |
| ~~String RE subfield system~~ | ~~Decided: RawString primitive, #String struct, .re subfield~~ | ~~Done~~ |
| ~~Metadata Data Tree~~ | ~~Full `%#:String:*` path documentation~~ — **Decided**: see plan/decisions/metadata-data-tree.md | ~~Done~~ |

## Project Management

| Item | Status | Action |
|------|--------|--------|
| v0.1 milestone closure | Ready to close | Run `/paul:complete-milestone` |
| v0.2 Language Spec | Not started | Run `/paul:discuss-milestone` after v0.1 closes |
| Rebuild Polly as PAUL special flow | Deferred | Post-documentation phases |

## Completed (Recent)

- [x] 2026-03-21: Metadata Data Tree decided — %{type}:{ref}:{instance}.{fields} paths, String subtypes nested with alias, enum active-field-only, IO ports nested, all old %Data paths corrected
- [x] 2026-03-20: String RE subfield system decided — RawString primitive, #String struct (.string + .re), int/float as #String subtypes, PGE-410, custom string types
- [x] 2026-03-20: Numeric RE patterns specified — int `^-?[0-9]+$`, float `^-?[0-9]+\.[0-9]+$` (leading zeros allowed). Decision recorded in plan/decisions/
- [x] 2026-03-20: Serial `.` access — no compile rule needed; distinction is meaningless on schema-free type
- [x] 2026-03-20: Typed flex fields in COMPILE-RULES.md — no new PGE needed (PGE-401/402 cover violations); PGE-409 added to reference table
- [x] 2026-03-20: Typed flex field edge cases EC-4.13–EC-4.17 added to EDGE-CASES.md (inference, PGE-401 contradiction, multi-level, untyped, individual override)
- [x] 2026-03-20: Typed flex wildcard `[:] :*;Type` added to EBNF grammar as `typed_flex_wildcard` production; `flex_data_field` wired into `data_field`
- [x] 2026-03-20: Doc gaps resolved: struct terminology, string literal typing, struct↔serial conversions, STDLIB struct labels
- [x] 2026-03-20: stdlib/ folder created — STDLIB.md split into 14 per-namespace files, PGE-913 resolved
- [x] 2026-03-20: PGE-407 cross-platform inference added — compiler detects dual-OS proof, forces handling if absent
- [x] 2026-03-20: Typed flex field inference decided: compiler infers from [:] :*;Type, with multi-level resolution algorithm documented
- [x] 2026-03-20: #DateTime open questions decided: =DateTime"..." / =DT"..." inline notation, no BusinessWeek defaults, no projection caching
- [x] 2026-03-20: #DateTime PGE-501 fix — Calendar, Week, TimeUnit, Cultural levels now entirely flexible (`:`)
- [x] 2026-03-20: Serial→struct matching decided: superset allowed, PGE-402/PGE-409 (Murphy's Law — mandatory `[!]` + `*Continue >FallBack`)
- [x] 2026-03-20: Master TODO list created with usage instructions
- [x] 2026-03-20: Schema & type hierarchy documented (types.md, collections.md, identifiers.md, TYPE-IDENTITY.md)
- [x] 2026-03-20: `{#}` named "struct" across documentation
- [x] 2026-03-20: Typed flexible fields section added to types.md
- [x] 2026-03-20: PGE-501 violation flagged in datetime-tree.md
- [x] 2026-03-20: serial-to-struct-matching.md brainstorm created
- [x] 2026-03-20: Inline pipeline calls documented (pipelines.md, STDLIB.md, EBNF.md, EDGE-CASES.md)
- [x] 2026-03-19: `;path` type fully documented with PGE-408, PGW-408
- [x] 2026-03-19: Compiler issues 001–027 resolved (025: PGE-804/805, 026: PGE-405, 027: PGE-108/109)
- [x] 2026-03-19: `[<]/[>]` → all-`[*]` syntax migration complete
