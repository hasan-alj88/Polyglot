---
issue: 158
group: 1
group_name: "EBNF / Compiler Rule Gaps"
priority: P3-medium
status: brief-ready
---

# Issue #158: Schema property prefix `##` missing from EBNF Section 05

## Inconsistency
EBNF Section 05 (Block Elements) lists `[#]` as an execution element but does not mention `##` or `###` as schema/field-type prefixes usable within `[#]` lines. Meanwhile, EBNF Section 04 defines `schema_id ::= "##" name` and `field_type_id ::= "###" name`, and EBNF Section 09 defines `schema_property ::= "[#]" "%##" ...`. The block element taxonomy in Section 05 is incomplete -- it classifies `[#]` only as an execution element and does not account for the schema composition and schema property contexts that use `[#]` with `##`/`###` prefixes.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/technical/ebnf/05-block-elements.md` | `[#]` listed only as `execution_elem`; no mention of schema composition (`[#] << ##`) or schema property (`[#] %##`) contexts |
| `docs/technical/ebnf/04-type-system.md` | Defines `schema_id ::= "##" name` and `field_type_id ::= "###" name` -- correctly present, but not cross-referenced from Section 05 |
| `docs/technical/ebnf/09-definition-blocks.md` | Defines `schema_composition`, `schema_property`, `field_type_composition` all starting with `[#]` -- not reflected in Section 05 taxonomy |

## Example
**Source A** (`docs/technical/ebnf/05-block-elements.md`, line ~37):
> `execution_elem      ::= "[r]" | "[p]" | "[b]" | "[#]" ;`

**Source B** (`docs/technical/ebnf/04-type-system.md`, line ~18):
> `schema_id           ::= "##" name ;               (* e.g., ##Scalar, ##Flat, ##Contiguous *)`

**Source C** (`docs/technical/ebnf/09-definition-blocks.md`, line ~83):
> `schema_property     ::= "[#]" "%##" dotted_name assignment_op expression ;`

Section 05 classifies `[#]` solely under "Execution" with `[r]`, `[p]`, `[b]`, ignoring that `[#]` also introduces schema lines (`[#] << ##Scalar`, `[#] %##Depth.Max`, `[#] << ###ScalarEnum`).

## Prior Related Work
- Issue #88 -- Introduced schema properties (`%##`) and the three-tier `#`/`##`/`###` prefix system
- Issue #118 -- Redesigned `##Leaf`/`##Scalar` schema types; expanded `##`/`###` usage
- Issue #133 -- Removed `##`/`###` from `instance_path` `type_prefix` row; confirmed they belong in `schema_prop`/`field_type_prop` grammar rules

## Recommendation
Add a comment or rule note in EBNF Section 05 clarifying that `[#]` serves dual roles: (1) execution element for data loading (`[#] <param` in pipeline body), and (2) schema/type declaration element in `{#}` definitions (for `[#] << ##`, `[#] << ###`, `[#] %##`, `[#] <~`). Alternatively, split `[#]` into the execution and data-definition categories in the taxonomy, with a cross-reference to Section 04 and Section 09.

## Discussion Prompts
1. Should `[#]` be reclassified from pure "execution element" to a dual-purpose marker, or should Section 05 add a separate "schema element" category?
2. Does this gap affect compiler implementation -- would a naive parser treating `[#]` only as execution fail to parse `{#}` definition bodies?
3. Is this a text-level documentation fix or does the EBNF grammar itself need restructuring?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 158*
