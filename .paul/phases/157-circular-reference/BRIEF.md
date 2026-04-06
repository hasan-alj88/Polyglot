---
issue: 157
group: 4
group_name: "Cross-Reference / Routing Errors"
priority: P3-medium
status: brief-ready
---

# Issue #157: PGE04009 / *Continue / conversions.md circular reference

## Inconsistency
Three documents form a circular dependency with no external ground truth breaking the cycle: `conversions.md` references PGE04009 for the serial-to-struct handling rule, PGE04009 references `*Continue` for the required error recovery mechanism, and `*Continue` references PGE04009 as the rule that mandates its usage. Each document defers authority to the next, creating a loop where no single document is the primary definition. A reader entering at any point is sent around the ring without reaching a self-contained explanation.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/user/syntax/types/conversions.md` | Defers to PGE04009 for the "cannot prove match" rule instead of stating it authoritatively; also references `*Continue >FallBack` without self-contained explanation |
| `docs/technical/compile-rules/PGE/PGE04009-unhandled-serial-struct-conversion.md` | References `*Continue` for the required recovery pattern and links back to `conversions.md` via See Also |
| `docs/user/stdlib/collectors/Continue.md` | References PGE04009 as the rule mandating `*Continue >FallBack` for serial-to-struct conversion, closing the loop |

## Example
**Source A** (`docs/user/syntax/types/conversions.md`, line ~58):
> **Cannot prove match** -- user must handle with `[!]` + `*Continue >FallBack`. If absent -> PGE04009

**Source B** (`docs/technical/compile-rules/PGE/PGE04009-unhandled-serial-struct-conversion.md`, lines ~102-103):
> - [[syntax/types/conversions|Type Conversions]] -- serial-to-struct conversion handling rules
> - [[stdlib/collectors/Continue|*Continue]] -- `*Continue >FallBack` usage for unproven conversions

**Source C** (`docs/user/stdlib/collectors/Continue.md`, line ~46):
> When used for serial->struct conversion, `[!]` + `*Continue >FallBack` is mandatory if the compiler cannot prove the match (PGE04009)

## Prior Related Work
- Issue #134 -- #Serial "schema-free" to "unconstrained" wording fix; touched the same serial conversion conceptual area
- Issue #125 -- *All collector terminology fix; established pattern for collector documentation authority
- IC-005 -- Type classification corrections; same category of type-system documentation consistency

## Recommendation
Establish PGE04009 as the primary authority for the serial-to-struct handling rule, since compile rules are the most formal specification layer. PGE04009 already contains the complete three-outcome table and valid/invalid examples. The fix: (1) make `conversions.md` a downstream consumer that summarizes and links to PGE04009 as authoritative, rather than restating the rule; (2) make `Continue.md` reference PGE04009 as a usage context without re-explaining the rule; (3) remove the See Also backlink from PGE04009 to `conversions.md` (or mark it as "user-facing summary" rather than a peer authority).

## Discussion Prompts
1. Should PGE04009 be the single source of truth, with conversions.md and Continue.md as downstream summaries, or should conversions.md own the user-facing rule and PGE04009 own only the compiler enforcement?
2. Does removing the backlink from PGE04009 to conversions.md break the Obsidian graph navigation pattern?
3. Is the three-outcome table (provably matches / provably mismatches / cannot prove) duplicated intentionally for different audiences, or should it live in only one place?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 157*
