---
issue: 146
group: 1
group_name: "EBNF / Compiler Rule Gaps"
priority: P2-high
status: brief-ready
---

# Issue #146: Semicolon (`;`) type annotation separator missing from EBNF and identifier rules

## Inconsistency
The `;` character is used throughout stdlib type definitions and queue field examples as a separator between a field name and its type annotation (e.g., `.strategy;#QueueStrategy`, `.string;RawString`). However, the EBNF grammar defines `type_annotation ::= '#' type_expr` -- meaning annotations start directly with `#`, with no `;` separator. The `;` is not defined in the EBNF's lexical elements, identifier rules, field separators, or prefix tables. The decisions document for issue #88 explicitly states "`;` Retired -- `#` for Type Annotations", yet `;` continues to appear in active documentation.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/technical/ebnf/04-type-system.md` | `type_annotation ::= '#' type_expr` -- no `;` separator defined |
| `docs/technical/ebnf/03-identifiers.md` | Field separator rules list only `.`, `:`, `%` -- `;` absent |
| `docs/technical/ebnf/09-definition-blocks.md` | `value_field` grammar uses `type_annotation` (which starts with `#`), yet example at line ~375 uses `.strategy;#QueueStrategy` |
| `docs/user/stdlib/types/structs.md` | #Queue definition uses `;` notation: `.strategy;#QueueStrategy`, `.host;#String`, etc. |
| `docs/user/stdlib/pipelines/Q.md` | IO parameters use `;` notation in tables: `<mb;#Float`, `<duration;#String` |
| `docs/audit/reference/glossary.md` | Describes #String as "contains `.string;RawString`" using `;` notation |
| `docs/technical/plan/decisions/schema-properties.md` | Says "`;` Retired -- `#` for Type Annotations" and "`#` replaces `;`" but `;` examples persist within the same file |

## Example
**Source A** (`docs/technical/ebnf/04-type-system.md`, line ~14):
> `type_annotation     ::= '#' type_expr ;`

**Source B** (`docs/technical/ebnf/09-definition-blocks.md`, line ~375-376):
> `[.] .strategy;#QueueStrategy << #LIFO`
> `[.] .host;#String << "gpu-server-01"`

**Source C** (`docs/user/syntax/types/structs.md`, line ~40-44):
> `[.] .timeout#int`
> `[.] .server#ServerInfo`

Sources B and C show two different notations for the same construct -- `.name;#Type` vs `.name#type`. The EBNF grammar only supports the latter form (direct `#` annotation). The `;` form is not parseable under the current grammar.

## Prior Related Work
- Issue #88 -- Introduced schema properties; decision explicitly retired `;` in favor of `#` for type annotations
- Issues #99-#106 -- EBNF edge cases batch; did not address `;` remnants
- STATE.md note: "~30 technical/ files still use old package address format (migrate on touch)" -- suggests `;` remnants may fall into a similar "migrate on touch" pattern

## Recommendation
Two possible directions: (1) Treat `;` as fully retired and migrate all remaining `.name;#Type` occurrences to `.name#type`, aligning with the EBNF grammar and the user-facing docs which already use the `#`-only form. (2) If `;` serves a distinct purpose (e.g., separating field name from type when the type starts with `#` and would be ambiguous), formally define it in the EBNF as an optional separator: `type_annotation ::= [';'] '#' type_expr`. Given the decisions doc explicitly retired `;`, option (1) is recommended -- batch-replace all `;#` with `#` in field definitions and IO parameter tables.

## Discussion Prompts
1. Is `;` truly retired, or does it serve a disambiguation role that `#` alone cannot (e.g., `.string#RawString` vs `.string;RawString` where `.string` could be a dotted name)?
2. Should this be a batch find-and-replace, or should each file be migrated on touch per existing policy?
3. Does the glossary need updating to reflect the current `#`-only type annotation syntax?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 146*
