# Project State

## Project Reference

See: .paul/PROJECT.md (updated 2026-03-24)

**Core value:** Building a new async programming language that can asynchronously compile other programming languages
**Current focus:** Language specification complete — next milestone TBD

## Current Position

Milestone: v0.2 Language Specification — COMPLETE
Phase: Issue #124 (QH vs DC glossary) — Complete
Plan: Complete — merged to main
Status: Merged to main, issue closed
Last activity: 2026-04-05 — Issue #124 complete, loop closed

Progress:
- v0.1 Language Spec: [██████████] 100%
- v0.1.1 Doc Audit: [██████████] 100%
- v0.2 Language Spec: [██████████] 100% (4/4 phases)
- Issue #82: [██████████] 100% (1/1 plans)
- Issue #83: [██████████] 100% (1/1 plans)
- Issue #84: [██████████] 100% (1/1 plans)
- Issue #85: [██████████] 100% (1/1 plans)
- Issue #86: [██████████] 100% (1/1 plans)
- Issue #64: [██████████] 100% (1/1 plans)
- Issue #65: [██████████] 100% (1/1 plans)
- Issue #66: [██████████] 100% (1/1 plans)
- Issue #61: [██████████] 100% (1/1 plans)
- Issue #62: [██████████] 100% (1/1 plans)
- Issue #63: [██████████] 100% (1/1 plans)
- Issue #67: [██████████] 100% (1/1 plans)
- Issues #68-#72: [██████████] 100% (batch — 5 Mermaid diagrams)
- Issue #88: [██████████] 100% (3/3 plans)
- Issue #92: [██████████] 100% (2/2 plans)
- Issue #90: [██████████] 100% (1/1 plans)
- Issue #94: [██████████] 100% (3/3 plans — also closes #93, #79, #91)
- Issue #97: [██████████] 100% (1/1 plans)
- Issue #98: [██████████] 100% (1/1 plans)
- Issues #99-#106: [██████████] 100% (batch — 8 EBNF edge cases)
- Issue #107: [██████████] 100% (1/1 plans)
- Issue #108: [██████████] 100% (1/1 plans)
- Issue #109: [██████████] 100% (1/1 plans)
- Issue #110: [██████████] 100% (1/1 plans)
- Issue #112: [██████████] 100% (1/1 plans)
- Issue #113: [██████████] 100% (1/1 plans)
- Issues #76-#78: [██████████] 100% (1/1 plans — RT runtime execution)
- Issues #95/#96: [██████████] 100% (3/3 plans — IC-005 fix, #DateTime types, =DT.* pipelines)
- Issue #118: [██████████] 100% (2/2 plans — ##Leaf/##Scalar redesign + propagation)
- Issue #116: [██████████] 100% (1/1 plans — PushLeft/PushRight operator rename)
- Issue #117: [██████████] 100% (1/1 plans — int/float coercion wording fix)
- Issue #119: [██████████] 100% (1/1 plans — ##Int schema vs #Int alias identity)
- Issue #120: [██████████] 100% (1/1 plans — IO perspective terminology fix)
- Issue #121: [██████████] 100% (3/3 plans — {N} native block type)
- Issue #122: [██████████] 100% (1/1 plans — *? wildcard standardization)
- Issue #123: [██████████] 100% (1/1 plans — Job vs Instance terminology)
- Issue #124: [██████████] 100% (1/1 plans — QH vs DC glossary)

## Active Issue

Issue: #125 — Docs: *All called 'Sync Barrier' in collect doc but 'Collector' in metadata spec
Branch: docs/issue-125-all-sync-barrier-vs-collector
Labels: docs, P2-high
Started: 2026-04-05
GitHub: https://github.com/hasan-alj88/Polyglot/issues/125

## Loop Position

Current loop state:
```
PLAN ──▶ APPLY ──▶ UNIFY ──▶ MERGE
  ○        ○        ○        ○     [Issue #125 active]
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
| Arithmetic uses =Math.* stdlib, not raw operators | 2026-03-24 | PGE04010 confirms raw +,-,*,/ are compile errors; spec reflects pipeline-based arithmetic |
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
| Issue #81 — Add package permission ceiling in {@} block | 2026-03-25 | Ceiling syntax in {@}; pipeline-level [_] in pipelines.md; PGE10001/916 compile rules; two separate rules for pipeline vs import ceiling |
| Issue #82 — Add %_ metadata tree branch for permissions | 2026-03-25 | %_ in all 3 tree files; 8 categories; ._ under %@ and %=; no instances; #86 created for . vs : audit |
| Issue #83 — Add permission compile rules | 2026-03-25 | PGE10003/918/919/920 + PGW10001; issue's PGE10002 renumbered to PGE10006; extends 9.x range |
| Issue #84 — Add !Permission.* error tree | 2026-03-25 | 8 .Denied leaves; trimmed overlapping .NotFound/.Timeout; File IO pipeline associations; #87 created for multi-alias |
| Issue #85 — Document stdlib pipeline permission declarations | 2026-03-25 | [_] permissions in all 7 stdlib pipeline files; Permission column in INDEX.md; completes #80-#85 chain |
| Issue #86 — Audit fixed vs flexible field usage | 2026-03-25 | %_ all `.` fixed; %! namespaces `.` + new !Error with `:` children; %@ `::` separator + Company rename; path grammar updated |
| Issue #64 — Add Mermaid tree diagram to data-is-trees.md | 2026-03-25 | graph TD diagram showing %definition schema → instances for #Boolean, =ProcessData, $myVar |
| Issue #65 — Add Mermaid flowchart to SPEC-INDEX.md | 2026-03-25 | flowchart LR showing 5-phase learning path with file counts |
| Issue #66 — Add Mermaid diagrams to pipelines.md auto-wire | 2026-03-26 | Two sequence diagrams: simple chain + multi-IO auto-wire with type matching |
| Issue #61 — Add Mermaid flowchart to pipelines.md execution order | 2026-03-26 | flowchart LR showing 5-stage sequence: Trigger/IO → Queue → Setup → Body → Cleanup |
| Issue #62 — Add Mermaid flowchart to errors.md error resolution | 2026-03-26 | flowchart TD decision tree: [!] match → replacement → fallback chain → Failed |
| Issue #63 — Add Mermaid flowchart to collections.md expand/collect | 2026-03-26 | flowchart LR fan-out/fan-in: ~ForEach.Array → items → *Into.Array + *Agg.Sum collectors |
| Issue #67 — Add Mermaid flowchart to pipelines.md parallel forking | 2026-03-26 | flowchart TD: [\] setup splits to sequential + [p] fork concurrent with body, [/] collects |
| Closed #68, #69, #70, #71, #72 — Batch Mermaid diagrams | 2026-03-26 | Merged docs/issue-68-72-mermaid-batch to main |
| Closed #88 — Add schema properties to {#} definitions | 2026-03-28 | Three-tier prefix (#/##/###), #Map/#Array/#Serial hierarchy, 9 compile rules, 26 design decisions |
| Closed #87 — Support multiple [%] .alias declarations per definition | 2026-03-28 | #IndexString→#KeyString, #NestedKeyString, flexible %alias, PGE12002 |
| Closed #89 — Add ~ForEach.Map and *Into.Map collection operators | 2026-03-28 | #Dict→#Map rename, ForEach/ and Into/ folder restructure, dot=folder convention |
| Closed #92 — Edge-case audit for all datatype definitions | 2026-03-28 | 92-01 audit + restructure, 92-02 resolved 4 edge cases: PGE11005, PGE04021, ###None, 0D array |
| Closed #90 — Add #Dataframe type + expand/collect operators | 2026-03-29 | Column-oriented #Dataframe<E<C, ##EnumLeafs, %##Leafs.Kind, #FieldKind, field expansion, 3 expanders + 1 collector, PGE04022/929/930 |
| Plan 94-01 — Macro-for-generics redesign | 2026-03-29 | Replaced generic <param with {M} macros; {M}/{W} split; [M] invocation; scalars as ## schemas; row-oriented Dataframe; 9 ground truths; 12 files updated |
| Dataframe uses ## composition not <~ inheritance | 2026-03-29 | Array ##Scalar constraint incompatible with Map elements; Dataframe composes ##Contiguous/##Rectangular/##Ordered directly |
| [M] merge = identity rule | 2026-03-29 | Outer {#} names result, macro fills body, additional [#] lines extend |
| =#.Column pipeline for column extraction | 2026-03-29 | Row-oriented Dataframe loses $df.column accessor; =#.Column replaces ~ForEach.Dataframe.Column |
| Plan 94-02 — Serial file loading + schema validation | 2026-03-30 | 10 =#.* pipelines, 3 =File.Serial.* pipelines, <#type pipeline IO pattern, !Validation.Schema/.Type/.Regex + !Field.* + !File.ParseError; 7 files |
| Base parsers as compiler intrinsics | 2026-03-30 | =#.JSON/YAML/TOML.Parse are compiler-internal, not user-definable |
| Validation pipelines are non-failable | 2026-03-30 | =#.Match/Validate/Describe/Coerce report via outputs (>errors, >dropped), not [!] errors |
| <#type extends <# to pipeline IO | 2026-03-30 | Same mechanism as {M} macro type inputs, now available at runtime in {=} pipelines; works with #/##/### tiers |
| Plan 94-03 — Expand/collect audit | 2026-03-30 | All operators compatible with macro-generated types; 2 stale Column references removed; no new operators needed |
| Closed #94 — =Schema.* runtime validation pipelines | 2026-03-30 | 3 plans: macro-for-generics redesign, validation pipelines, expand/collect audit; also closes #93, #79, #91 |
| Issue #97 — EBNF bare literals/non-pipeline identifiers | 2026-03-30 | PGE01020 error + exec_expr tightened; X.3 auto-resolves; 1 plan |
| Closed #97 — EBNF bare literals/non-pipeline identifiers | 2026-03-30 | Merged design/issue-97 to main |
| Issue #98 — EBNF discard defaults and self-assignment | 2026-03-30 | PGE02010 (discard default), PGE08011 (self-assignment); 3 edge cases |
| Closed #98 — EBNF discard defaults and self-assignment | 2026-03-30 | Merged design/issue-98 to main |
| Issues #99-#106 — EBNF edge cases batch | 2026-03-30 | 8 issues, 26 edge cases, 16 new compile rules (PGE01021-27, PGE02011, PGE03011, PGE04024-25, PGE06014, PGE08012, PGE12004, PGW01003, PGW04002) |
| Multiple triggers = AND semantics | 2026-03-30 | Multiple [t] lines use AND (all must fire); [⏐] for OR |
| Operations declare allowed markers | 2026-03-30 | PGE01024 — each operation declares compatible block markers; compiler validates |
| PGW01002 superseded by PGE01021 | 2026-03-30 | Empty {#} upgraded from warning to error; EBNF tightened |
| Self-chains require numeric indexing | 2026-03-30 | PGE08012 — =A => =A valid but must use >0./<1. notation |
| Array element type mandatory | 2026-03-30 | PGE04025 — untyped #array is compile error |
| Inputs are always Final | 2026-03-30 | Input parameters reach Final before pipeline triggers; write = PGE02003 |
| Closed #99-#106 — EBNF edge cases batch | 2026-03-30 | Merged design/issue-99-106-ebnf-edge-cases to main |
| Closed #107 — Document object type hierarchy | 2026-03-31 | Merged design/issue-107-object-type-hierarchy to main |
| Closed #108 — Marker declaration syntax {=}[exe] | 2026-03-31 | Merged design/issue-108-marker-declaration-syntax to main; no warning for implicit default |
| Closed #109 — [t] → [T] uppercase trigger element | 2026-03-31 | Merged design/issue-109; 339 replacements across 93 files; archive untouched |
| Closed #110 — Document base pipelines and #BaseCode enum | 2026-03-31 | Merged design/issue-110; BaseCode.md, EBNF .baseCode, metadata.md, INDEX.md Base vs Derived, PGE01028 |
| Unified [c] → [C] uppercase foreign code element | 2026-03-31 | #112: [C] is block element for inline foreign code passed to =RT.*; #Code header removed; language from =RT.* pipeline |
| Closed #112 — Reconcile [C] inline code element | 2026-03-31 | Merged design/issue-112 to main; 6 files updated; EBNF simplified |
| Closed #114 — %T trigger branch already done | 2026-04-01 | Already implemented by #107; closed as duplicate |
| Closed #113 — Unify {Q} dual-purpose documentation | 2026-04-01 | blocks.md expanded, queue.md new section, EBNF §9.5 dual-purpose note; 3 files |
| Closed #76-#78 — RT runtime execution subsystem | 2026-04-01 | =W.RT wrapper, !RT errors, =RT.* pipelines (7 modes), #Code/#PyEnv/#RsEnv types; 2 new + 7 edited files |
| Script vs Bind by binding origin | 2026-04-01 | Script: Polyglot injects vars (<Bind/>Bind); Bind: foreign code calls pull()/push() |
| .Inline/.File mode split for all =RT modes | 2026-04-01 | 7 total variants; .CLI inherently file-based (no .Inline) |
| .CLI uses =W.Polyglot, not =W.RT | 2026-04-01 | No language runtime needed for compiled binaries |
| Compiler validates Function/Script only | 2026-04-01 | <func name checked in .Function; <Bind names checked in .Script; .CLI and .Bind opaque |
| IC-005 fix — enums mislabeled as structs | 2026-04-01 | structs.md lists #path/#Queue/#DateTime; type-identity.md adds #Queue; tracking corrected |
| Formalized #DateTime as stdlib type | 2026-04-01 | 55 {#} types in datetime.md (22KB); multi-calendar, time units, cultural extensions |
| #WeekSystem Option A restructure | 2026-04-01 | Enum fields with nested .config#BusinessWeek; fixes PGE05005 violation |
| =DT.* stdlib pipelines | 2026-04-01 | 40 {=} pipelines in DT.md; construction, conversion, arithmetic, comparison, formatting, business |
| PGE04026-04028 compile rules | 2026-04-01 | Invalid IANA timezone, missing DateTime epoch, out-of-range epoch |
| Closed #118 — #String Depth.Max=0 contradiction | 2026-04-04 | ##Leaf new, ##Scalar Depth.Max=1, ###ScalarValue/###ScalarEnum; merged to main |
| Closed #116 — PushLeft/PushRight operator rename | 2026-04-04 | Renamed 6 EBNF symbols + display names; eliminated Push/Pull ambiguity; merged to main |
| Closed #117 — int/float coercion wording fix | 2026-04-04 | "interoperate freely" → "comparable without conversion"; merged to main |
| Closed #119 — ##Int schema vs #Int alias identity | 2026-04-04 | Alias table now has Schema column; clarifies #Int is alias, ##Int is schema; merged to main |
| Closed #120 — IO perspective terminology fix | 2026-04-04 | "caller perspective" → "pipeline perspective"; one-line scoping rule added; merged to main |
| Closed #121 — {N} native block type | 2026-04-04 | {N} block, #NativeKind enum, NativeKind.md, PGE01028 rewritten, 44 stdlib defs converted; merged to main |
| Issue #122 — Standardize *? wildcard catch-all | 2026-04-04 | Bare `*` removed from match syntax; `*?` is the only wildcard form in both verbose and match conditionals; 5 files updated |
| Issue #123 — Job vs Instance terminology | 2026-04-05 | {#} #Job struct (9 fields), glossary entries, Pipeline Branch %=:Pipeline:N.jobs:UID, data-is-trees clarification; Instance=sequential run, Job=UID-keyed work unit |
| Closed #124 — Queue Handler vs Dispatch Coordinator glossary | 2026-04-05 | Added DC glossary entry; narrowed QH "never makes decisions" to trigger/business logic scope |

### Deferred Issues
- ~~Rebuild Polly as PAUL special flow~~ — closed, redundant with pg:generate/pg:train
- 9 stdlib files remain status: draft despite Stable content (Math, Path, Sys, ForEach, collectors, types)
- EC-6.4 inconsistency: raw arithmetic in EDGE-CASES vs PGE04010
- ~30 technical/ files still use old package address format (migrate on touch)

### Blockers/Concerns
None.

## Boundaries (Active)

Protected elements for current milestone:
- docs/ directory (existing documentation to build on)
- .paul/ directory (project management)

## Session Continuity

Last session: 2026-04-05
Stopped at: Merged #124 to main, issue closed
Next action: Pick next issue or push to origin
Resume context:
- Issue #124 complete and merged to main
- Branch docs/issue-124-queue-handler-dispatch-coordinator-glossary deleted

---
*STATE.md — Updated after every significant action*
