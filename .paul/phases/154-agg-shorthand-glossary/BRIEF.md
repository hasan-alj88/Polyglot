---
issue: 154
group: 3
group_name: "Naming & Terminology Conflicts"
priority: P4-low
status: brief-ready
---

# Issue #154: *Agg shorthand not mentioned in glossary or vision

## Inconsistency
The `*Agg` namespace is explicitly called out in `Agg.md` as "The namespace is `*Agg`, NOT `*Aggregate`", and is used extensively across collection docs, compile rules, EBNF, and examples. However, neither the glossary (`docs/audit/reference/glossary.md`) nor the vision document (`docs/vision.md`) mention `*Agg`, collectors, or any collection operator. The glossary is described as the authoritative definition source that Claude must use, yet it contains no entries for any `*` operator, `~` operator, or collection concept. This means the naming warning in `Agg.md` has no glossary backing.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/audit/reference/glossary.md` | No entry for `*Agg`, `*Into`, `*All`, `*Continue`, or any collector/expander concept |
| `docs/vision.md` | No mention of collectors, expanders, or the `*` operator category |
| `docs/user/jm3lib/collectors/Agg.md` | Line 10: warns `*Agg NOT *Aggregate` but no glossary entry backs this |
| `docs/technical/spec/metadata-tree/object-types.md` | Line 19: `%*` branch lists `*Agg.*` — confirms it as a core namespace |

## Example
**Source A** (`docs/user/jm3lib/collectors/Agg.md`, line ~10):
> Aggregation collectors that reduce mini-pipeline outputs to a single value. The namespace is `*Agg`, NOT `*Aggregate`.

**Source B** (`docs/audit/reference/glossary.md`, full file):
> (No entry for *Agg, *Aggregate, collector, or any * operator)

**Source C** (`docs/technical/spec/metadata-tree/object-types.md`, line ~19):
> | `%*` | Collectors | Flexible (`:name`) | **Data:** `*Into.*`, `*Agg.*` · **Collect-all:** `*All` · **Race:** `*First`, `*Nth` |

## Prior Related Work
- Issue #125 — Closed: standardized `*All` terminology from "sync barrier" to "collect-all" and subcategorized `%*` into Data/Collect-all/Race. Did not add glossary entries for individual collector namespaces.

## Recommendation
Add glossary entries for at least the operator prefix categories: `*` (Collector), `~` (Expander), and specifically `*Agg` with the "NOT *Aggregate" warning. Vision.md likely does not need changes since it is a high-level philosophy document, but the glossary as the authoritative term reference should cover the jm3lib namespace shorthands. Consider whether `*Into`, `*All`, `*First`, `*Nth`, and `*Continue` also need entries.

## Discussion Prompts
1. Should the glossary cover all `*` operator namespaces or just `*Agg` since it has the naming trap?
2. Should vision.md mention collection operators at all, or is it intentionally high-level?
3. Is the `*Agg NOT *Aggregate` warning sufficient as inline documentation, or does it need glossary authority?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 154*
