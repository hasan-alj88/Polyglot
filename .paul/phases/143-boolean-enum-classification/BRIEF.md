---
issue: 143
group: 3
group_name: "Naming & Terminology Conflicts"
priority: P3-medium
status: brief-ready
---

# Issue #143: #Boolean classified as 'enum struct' vs 'enum' inconsistently

## Inconsistency
`#Boolean` is described using three different classification terms across the documentation: "enum struct" (in hierarchy, types, and brainstorming docs), "enum" or "enum type" (in jm3lib enums.md and related links), and implicitly "struct" (via the type-identity.md rule that all `{#}` types are structs). The term "enum struct" is a compound label unique to `#Boolean` — no other enum type uses it. Meanwhile, enums.md describes `#Boolean` simply as "an enum, documented separately" without using the "enum struct" qualifier, and the glossary has no entry for `#Boolean` at all.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/user/syntax/types/hierarchy.md` | Line 56: `#Boolean (independent enum struct — NOT #String)` — uses "enum struct" |
| `docs/user/jm3lib/types/types.md` | Line 29: `#Boolean (independent enum struct — NOT #String)` — uses "enum struct" |
| `docs/user/syntax/types/basic-types.md` | Section header line 134: "Layer 2b: #Boolean — Independent Enum Struct" — uses "enum struct" |
| `docs/technical/ebnf/04-type-system.md` | Line 29: `bool (#Boolean): separate enum struct, not a #String subtype` — uses "enum struct" |
| `docs/user/jm3lib/types/enums.md` | Line 14: `See [[boolean]] for #Boolean (also an enum, documented separately)` — uses bare "enum" |
| `docs/user/jm3lib/types/datetime/related.md` | Line 13: `[[boolean]] -- #Boolean enum type` — uses "enum type" |
| `docs/technical/spec/type-identity.md` | Line 15: All `{#}` types are "structs" — implies #Boolean is a struct |

## Example
**Source A** (`docs/user/syntax/types/hierarchy.md`, line ~56):
> #Boolean (independent enum struct — NOT #String) [##Scalar, ###ScalarEnum]

**Source B** (`docs/user/jm3lib/types/enums.md`, line ~14):
> See [[boolean]] for `#Boolean` (also an enum, documented separately).

**Source C** (`docs/user/jm3lib/types/datetime/related.md`, line ~13):
> - [[boolean]] -- #Boolean enum type

## Prior Related Work
- IC-005 (inconsistencies.md) — Resolved: fixed `#PipelineStatus`, `#VarState`, `#Boolean` mislabeled as "jm3lib structs" in structs.md. Moved enum types to separate documentation but did not standardize the "enum struct" vs "enum" terminology for `#Boolean` itself.

## Recommendation
Settle on "enum struct" as the canonical term for `#Boolean` and document why: in Polyglot, ALL `{#}` types are structs (per type-identity.md), and `#Boolean` happens to be a struct whose fields are enum variants (no `#type` annotation). The term "enum struct" captures both facts. Update `enums.md` line 14 and `related.md` line 13 to use "enum struct" consistently. Consider adding a glossary entry for `#Boolean`.

## Discussion Prompts
1. Should "enum struct" be a defined glossary term, or should we just use "enum" for user-facing docs and "struct with enum fields" for technical docs?
2. Does the type-identity.md blanket rule ("all {#} types are structs") mean "enum struct" is redundant — or does it helpfully distinguish from value-field structs?
3. Should enums.md acknowledge the "enum struct" distinction or is the simpler "enum" label sufficient for the pg-coder audience?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 143*
