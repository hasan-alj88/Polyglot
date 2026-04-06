---
issue: 149
group: 1
group_name: "EBNF / Compiler Rule Gaps"
priority: P2-high
status: brief-ready
---

# Issue #149: PGE01001/PGE01002 pipeline ordering paradox with EBNF

## Inconsistency
PGE01001 states pipeline sections must appear in fixed order: `[T],[=]` then `[Q]` then setup then body then cleanup -- implying `[T]` comes first in the trigger/IO section. PGE01002 states IO input declarations (`[=] <param`) must appear **before** any `[T]` trigger line that pushes into them -- implying `[=]` comes before `[T]`. Read literally, PGE01001 says "T then =" while PGE01002 says "= then T". The EBNF resolves this with `trigger_io_section` which groups them into one unordered section, but the compile rule text does not reflect this.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/technical/compile-rules/PGE/PGE01001-pipeline-execution-order.md` | Statement says `[T],[=]` order (T before =), diagnostic message says `[T],[=]` as a unit |
| `docs/technical/compile-rules/PGE/PGE01002-io-before-trigger.md` | Statement says IO `[=]` must appear **before** `[T]` trigger that pushes into it |
| `docs/technical/ebnf/09-definition-blocks.md` | Correctly defines `trigger_io_section` as unordered: `{ indent ( trigger_line \| io_decl_line \| error_decl_line \| comment_line ) NEWLINE }` |

## Example
**Source A** (`docs/technical/compile-rules/PGE/PGE01001-pipeline-execution-order.md`, line ~12):
> **Statement:** A `{=}` pipeline's sections must appear in fixed order: `[T],[=]` -> `[Q]` -> setup -> execution body -> cleanup.

**Source B** (`docs/technical/compile-rules/PGE/PGE01002-io-before-trigger.md`, line ~11):
> **Statement:** IO input declarations (`[=] <param`) must appear **before** any `[T]` trigger line that pushes into them. A trigger cannot reference an undeclared IO parameter.

**Source C** (`docs/technical/ebnf/09-definition-blocks.md`, line ~86-88):
> `(* Trigger, IO, and error declarations form one section -- order between [T], [=], and error decls is not strict.`
> `   IO inputs are implicit triggers; some triggers produce inputs. Error declarations mark the pipeline as failable. *)`
> `trigger_io_section  ::= { indent ( trigger_line | io_decl_line | error_decl_line | comment_line ) NEWLINE } ;`

The EBNF comment explicitly says "order between [T], [=], and error decls is not strict" -- this contradicts PGE01001's `[T],[=]` ordering notation and PGE01002's "before" requirement.

## Prior Related Work
- Issue #109 -- Renamed `[t]` to `[T]` uppercase trigger element (339 replacements across 93 files)
- Issues #99-#106 -- EBNF edge cases batch; PGE01001 text may have been updated during this work
- STATE.md decision: "Inputs are always Final" -- input parameters reach Final before pipeline triggers; related to IO/trigger ordering semantics

## Recommendation
Clarify PGE01001 and PGE01002 to align with the EBNF's `trigger_io_section` definition. The `[T]` and `[=]` markers form **one declaration section** where internal order is flexible. PGE01002's actual constraint is narrower than stated: an IO parameter must be **declared** before a `[T]` line can **push into** it (forward reference), but this is a semantic dependency, not a positional ordering rule. Update PGE01001 to say `[T]/[=] section` (unordered) rather than `[T],[=]` (implying T-first order). Update PGE01002 to clarify it enforces declaration-before-use within the trigger/IO section, not positional ordering.

## Discussion Prompts
1. Which document is authoritative -- the EBNF (unordered section) or PGE01001 (T-first order)?
2. Should PGE01002 be reworded as a "declaration before reference" rule rather than a positional "before" rule?
3. Does this affect any existing valid Polyglot code that puts `[=]` before `[T]`?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 149*
