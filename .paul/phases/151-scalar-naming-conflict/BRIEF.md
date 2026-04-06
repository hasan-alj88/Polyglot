---
issue: 151
group: 3
group_name: "Naming & Terminology Conflicts"
priority: P3-medium
status: brief-ready
---

# Issue #151: Scalar subtypes ##Int/##Float vs int/float naming conflict

## Inconsistency
Documentation uses `##Int`/`##Float` (schema property names) and `int`/`float` (alias names) interchangeably in prose, which creates confusion about whether they refer to the same concept. While the alias resolution table in `string-subtypes.md` clearly separates the two, other files blur the distinction by using `int`/`float` where the formal `##Int`/`##Float` is meant, or by describing them as "subtypes" without clarifying which naming layer is being referenced.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/audit/reference/glossary.md` | Describes `int`/`float` as "flexible subtypes of `#String`" without mentioning `##Int`/`##Float` schema names at all |
| `docs/user/syntax/operators.md` | Uses `#int`, `#float` (single-# alias) for type references in prose where `##Int`/`##Float` would be more precise |
| `docs/technical/spec/type-identity.md` | Uses bare `int` and `float` without `#` or `##` prefix in rule text |
| `docs/user/concepts/conditionals.md` | Uses `#int`, `#float` in prose for type references |
| `docs/technical/compile-rules/PGE/PGE04001-type-mismatch.md` | Uses bare `int` and `float` in statement text |
| `docs/technical/compile-rules/PGE/PGE04015-conditional-type-operator-mismatch.md` | Uses `#int` and `#float` in statement text |

## Example
**Source A** (`docs/audit/reference/glossary.md`, line ~34):
> #String | Struct type for `;string` — contains `.string;RawString` (value) and `.regex;RawString` (RE constraint; alias: `.re`). `int`/`float` are flexible subtypes of `#String` | Not the primitive — `RawString` is the primitive

**Source B** (`docs/technical/spec/metadata-tree/string-subtypes.md`, line ~39):
> The single-`#` names (`#Int`, `#Float`, etc.) are **user-facing aliases** — convenient shorthand you write in type annotations. The double-`##` names (`##Int`, `##Float`, etc.) are the **schema definitions** that specify regex constraints and live at `%##` on the metadata tree.

**Source C** (`docs/technical/spec/type-identity.md`, line ~20):
> **No implicit coercion** — `int` does not auto-promote to `float`, `string` does not coerce to `path`, etc.

## Prior Related Work
- Issue #119 — Closed: clarified `##Int` is schema, `#Int` is alias; added Schema column to alias table. Addressed the identity relationship but not the inconsistent usage of bare `int`/`float` vs prefixed forms across other docs.
- Issue #117 — Closed: changed "interoperate freely" to "comparable without conversion" for int/float coercion wording. Fixed coercion semantics but not the naming inconsistency.

## Recommendation
Establish a convention: (1) `##Int`/`##Float` in technical docs referring to schema definitions, (2) `#int`/`#float` (with `#` prefix) in user-facing docs referring to type annotations, (3) bare `int`/`float` only when referring to the alias string itself. Update the glossary to mention `##Int`/`##Float` explicitly. The `string-subtypes.md` alias table is the authoritative source for this distinction.

## Discussion Prompts
1. Should the glossary entry for `#String` explicitly list the three naming layers (bare alias, `#` alias, `##` schema)?
2. Should compile rule statements use `#int`/`#float` (user-facing alias) or `##Int`/`##Float` (formal schema name)?
3. Is this a documentation text fix or does it require a glossary expansion?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 151*
