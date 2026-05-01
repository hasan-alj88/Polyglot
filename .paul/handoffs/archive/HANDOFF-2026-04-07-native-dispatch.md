# PAUL Session Handoff

**Session:** 2026-04-07
**Phase:** Post v0.2 — Native Dispatch Architecture
**Context:** Designed native dispatch spec, created lib/ scaffold, renamed stdlib→jm3lib, began jm3lib .pg source files

---

## Session Accomplishments

- Discussed `{N}` native definitions: syntax, compiler dispatch, host-language bridging
- Wrote **native-dispatch.md** spec: subsystem architecture, YAML per-operation config, 6-step dispatch flow, JSON wire protocol, native function contract, #NativeKind routing, intrinsic catalog, failure modes, sequence diagrams
- Created **lib/ folder scaffold**: `lib/rust/` (integrator, tm, qh, runner, pgcompiler READMEs), `lib/go/`, `lib/python/` placeholders
- Created **lib/Polyglot/jm3lib/** with actual `.pg` source files organized by subsystem kind:
  - `tm/triggers.pg` — 5 trigger definitions
  - `qh/queue.pg` — 10 queue definitions
  - `runner/file.pg` — 11 file operations
  - `runner/math.pg` — 8 math operations
  - `runner/datetime.pg` — 37 datetime operations
  - `runner/wrappers.pg` — 13 wrapper definitions
  - `intrinsics/intrinsics.pg` — 11 compiler intrinsics
  - `types/NativeKind.pg` — #NativeKind enum
  - `errors/errors.pg` — Built-in error namespaces + new !RT.* errors
- Created **native-config-example.yaml** — per-operation language selection config
- **Fixed DT.md** — removed invalid `[T] =T.Manual`, `[Q] =Q.Default`, `[W] =W.Polyglot` from all 40 `{N}` blocks (PGE01028 violation)
- **Renamed stdlib → jm3lib** — physical folder move + ~190 text replacements across 70+ files, file renames (STDLIB.md → JM3LIB.md, PGE09012, phase folders, memory files)

---

## Decisions Made

| Decision | Rationale | Impact |
|----------|-----------|--------|
| JSON wire format for native boundary | Language-agnostic, aligns with serial type, human-readable | All native functions accept/return JSON |
| Per-operation language selection | Maximum flexibility — each `{N}` op can use different host language | YAML config replaces simple `base: Rust`; impacts PGE01028e, NativeKind.md, pipelines/INDEX.md |
| Subsystem-specific dispatch | TM dispatches triggers, QH dispatches queues, Runner dispatches execution+wrapper | Matches existing architecture; each subsystem owns its kind |
| Intrinsics = compiler-inlined, no dispatch | =DoNothing and =#.* have no host function | No `.<Language>` binding needed; compiler handles directly |
| Integrator SDK = bidirectional | Host can call Polyglot AND Polyglot can call host (=RT.*) | `lib/<lang>/integrator/` serves both directions |
| stdlib renamed to jm3lib | "jm3lib" = Polyglot library, clearer identity | 190+ occurrences updated; folder moved; all wikilinks updated |
| jm3lib .pg source files live in `lib/Polyglot/jm3lib/` | Organized by subsystem kind (tm/, qh/, runner/, intrinsics/) | Compiler reads these as source of truth for native definitions |
| Pipeline docs must include: inputs, outputs, errors, permissions, aliases, GitHub issues | Standardized format for all pipeline reference docs | Requires restructuring existing docs (next session task) |

---

## Gap Analysis with Decisions

### DT.md Too Long — Needs Split
**Status:** CREATE (next session)
**Notes:** User wants `docs/user/jm3lib/pipelines/DT.md` (692 lines, 37 pipelines) split into `DT/` folder with one `.md` per pipeline definition
**Effort:** Medium — 37 files + INDEX.md + wikilink updates
**Reference:** `@docs/user/jm3lib/pipelines/DT.md`

### Pipeline Doc Template Not Yet Applied
**Status:** CREATE (next session)
**Notes:** User defined a standard format for ALL pipeline docs: inputs (`<`), outputs (`>`), errors (`!`), permissions (`_`), aliases (`()`), GitHub issue links (pg + Rust impl). This applies to ALL pipeline files, not just DT.*
**Effort:** Large — affects DT.md, File.md, Math.md, Q.md, T.md, W.md, #.md, RT.md, Sys.md, Path.md
**Reference:** `@docs/user/jm3lib/pipelines/`

### GitHub Issues for Each Pipeline
**Status:** CREATE (next session)
**Notes:** Each pipeline needs two GitHub issues: one for .pg spec completion, one for Rust implementation. Needs appropriate labels.
**Effort:** Large — ~100+ pipelines × 2 issues each
**Reference:** GitHub Issues on `hasan aljamea` repo

### Existing Docs Reference Old `base: Rust` Config
**Status:** DEFER
**Notes:** native-dispatch.md introduces per-operation config that replaces `base: Rust` in NativeKind.md, pipelines/INDEX.md, PGE01028. Not updated this session.
**Effort:** Small — 3 files
**Reference:** `@docs/user/jm3lib/types/NativeKind.md:65-71`, `@docs/user/concepts/pipelines/INDEX.md:132-138`, `@docs/technical/compile-rules/PGE/PGE01028-base-derived-mutual-exclusion.md`

### DT.md `[%] .version` Lines in {N} Blocks
**Status:** REVIEW
**Notes:** The .pg files in `lib/Polyglot/jm3lib/runner/datetime.pg` don't include `.version` metadata. DT.md had `[%] .version << "1.0.0"` on every {N} block. Need to decide: is `.version` valid on {N} blocks? EBNF §9.4c `native_field` only lists `.Kind`, `.<Language>`, `.description`.
**Effort:** Trivial — remove from DT.md if invalid, or add to EBNF if valid

---

## Open Questions

1. **Pipeline doc template**: Should aliases appear in a dedicated section or inline with the {N} definition?
2. **GitHub issue labels**: What labels to use? (e.g., `jm3lib`, `native-impl`, `rust`, `spec`)
3. **Split granularity**: Should other long pipeline files (File.md, W.md) also be split into folders?
4. **`.version` on {N} blocks**: Valid metadata field or EBNF violation?

---

## Reference Files for Next Session

```
@docs/user/jm3lib/pipelines/DT.md              — file to split
@docs/user/jm3lib/pipelines/File.md             — template candidate
@docs/user/jm3lib/pipelines/Math.md             — template candidate
@docs/user/jm3lib/pipelines/Q.md                — template candidate
@docs/user/jm3lib/pipelines/T.md                — template candidate
@docs/user/jm3lib/pipelines/W.md                — template candidate
@docs/user/jm3lib/pipelines/#.md                — template candidate
@docs/user/jm3lib/pipelines/RT.md               — template candidate
@docs/technical/spec/native-dispatch.md         — dispatch spec (just written)
@lib/Polyglot/jm3lib/                           — .pg source files
@docs/audit/audiences/pg-coder.md              — writing rules for pipeline docs
@docs/audit/rules/conventions.md               — doc formatting rules
```

---

## Prioritized Next Actions

| Priority | Action | Effort |
|----------|--------|--------|
| 1 | Split DT.md into DT/ folder (37 individual .md files + INDEX.md) | Medium |
| 2 | Define pipeline doc template with all 6 required sections | Small |
| 3 | Apply template to DT/* files first (inputs, outputs, errors, permissions, aliases, issues) | Medium |
| 4 | Create GitHub issues for DT.* pipelines (37 × 2 = 74 issues) | Medium |
| 5 | Apply template to remaining pipeline files (File, Math, Q, T, W, #, RT, Sys, Path) | Large |
| 6 | Create GitHub issues for remaining pipelines | Large |
| 7 | Update NativeKind.md, pipelines/INDEX.md, PGE01028 for per-operation config | Small |
| 8 | Decide on `.version` validity in {N} blocks | Trivial |

---

## State Summary

**Current:** Post v0.2 — native dispatch architecture designed, lib/ scaffold created, jm3lib rename complete
**Next:** Split DT.md into folder, define pipeline doc template, create GitHub issues
**Resume:** `/paul:resume` then read this handoff

---

*Handoff created: 2026-04-07*
