---
issue: 152
group: 3
group_name: "Naming & Terminology Conflicts"
priority: P3-medium
status: brief-ready
---

# Issue #152: *Continue classified as collector but functionally is error recovery

## Inconsistency
`*Continue` uses the `*` prefix (collector category) and appears in the EBNF grammar under `collect_operator`, but it functions as an error recovery mechanism inside `[!]` blocks, not as a data-gathering collector. True collectors (`*Into.*`, `*Agg.*`) operate inside `~ForEach` expand scopes and gather outputs from mini-pipelines. `*Continue` has no relationship to expand/collect — it signals the pipeline to continue after an error with a fallback value. The EBNF explicitly classifies it as `error_operator` within `collect_operator`, and the pglib INDEX lists it as "Error recovery with fallback value", yet its `*` prefix groups it with data collectors in the metadata tree (`%*`) and in the operator summary tables.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/technical/ebnf/12-collections.md` | Lines 58-61: `collect_operator` includes `error_operator ::= "Continue"` — correctly separated in grammar but still under collect umbrella |
| `docs/technical/ebnf/12-collections.md` | Lines 118-121: `*Continue` listed in Collect IO Signatures table alongside `*Into.*` and `*Agg.*` data collectors |
| `docs/user/concepts/collections/collect.md` | Does not mention `*Continue` at all — the collections concept page only covers `*Into`, `*Agg`, `*All`, `*First`, `*Nth`, `*Ignore` |
| `docs/user/pglib/collectors/Continue.md` | Title says "Error Recovery" — correct role, but file lives under `collectors/` directory |
| `docs/user/pglib/INDEX.md` | Line 53: lists `*Continue` under "Collector Operators (*)" section — grouped with data collectors |
| `docs/INDEX.md` | Line 164: `collectors/*.md` described as containing `*Into variants, *Agg, *Sync, *Continue` — all grouped |
| `docs/technical/spec/metadata-tree/object-types.md` | Line 19: `%*` branch lists only data/collect-all/race categories — `*Continue` is not even listed here |

## Example
**Source A** (`docs/technical/ebnf/12-collections.md`, lines ~58-61):
> collect_operator    ::= into_operator | agg_operator | sync_operator | race_operator
>                       | error_operator | discard_operator ;
> 
> error_operator      ::= "Continue" ;

**Source B** (`docs/user/pglib/collectors/Continue.md`, lines ~8-10):
> # *Continue — Error Recovery
> 
> Used inside `[!]` error blocks to continue the pipeline with a fallback value. Without `*Continue`, an `[!]` block terminates the pipeline on error (PGE02005).

**Source C** (`docs/technical/spec/metadata-tree/object-types.md`, line ~19):
> | `%*` | Collectors | Flexible (`:name`) | **Data:** `*Into.*`, `*Agg.*` · **Collect-all:** `*All` · **Race:** `*First`, `*Nth` |

## Prior Related Work
- Issue #125 — Closed: subcategorized `%*` into Data/Collect-all/Race categories. `*Continue` was not mentioned in the subcategorization, which is itself a gap.

## Recommendation
Add "Flow Control" or "Error Recovery" as a fourth subcategory of `%*` collectors alongside Data, Collect-all, and Race. Update the `object-types.md` `%*` description to include `*Continue` (and `*Ignore`) under this new subcategory. The `*` prefix is correct (it uses `[*]` IO markers and the `*` operator prefix), but the docs should explicitly distinguish flow-control collectors from data-gathering collectors. Consider adding a note to `collect.md` acknowledging `*Continue` and `*Ignore` exist but are documented elsewhere since they are not part of the expand/collect pattern.

## Discussion Prompts
1. Should `*Continue` and `*Ignore` be reclassified out of collectors entirely, or is the "flow-control collector" subcategory sufficient?
2. Should collect.md mention `*Continue` with a cross-reference, or is its absence from the collections concept intentional?
3. Does `object-types.md` need a fourth `%*` subcategory (e.g., "Flow Control: `*Continue`, `*Ignore`")?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 152*
