---
phase: issue-88-schema-properties
plan: 03
status: complete
completed: 2026-03-28
---

# 88-03 Summary: Infrastructure Files (Metadata Tree, Compile Rules, EBNF)

## Files Modified

### `docs/technical/spec/metadata-tree.md`

1. **Path Grammar** — extended with `%definition.##:{ref}` and `%definition.###:{ref}` schema paths, plus `%##` and `%###` property path patterns
2. **Object Type Branches** — updated `%definition` description to mention `##` and `###` definition templates
3. **Schema Definition Templates (`%definition.##`)** — new section with full tree showing all 8 approved schemas (##Scalar, ##Flat, ##Deep, ##Homogeneous, ##Heterogeneous, ##Contiguous, ##Sparse, ##Rectangular) at their metadata paths
4. **Field Type Definition Templates (`%definition.###`)** — new section documenting `###Value` and `###Enum` at `%definition.###:{FieldTypeName}`
5. **Schema Properties in Type Definitions (`%##`)** — replaced old `%Key.*`/`%Ordered`/`%Depth.Max`/`%Alias` table with full 9-property `%##` prefix table
6. **Field Type Properties (`%###`)** — new section documenting `%###Value` and `%###Enum` with PGE-925 reference
7. **Complete Type Definition Examples** — `#Array` example showing all `%##` properties accumulated from composed schemas, `#Boolean` example showing `###Enum`
8. **Alias Resolution** — updated from `%Alias` to `%##Alias`

### `docs/technical/COMPILE-RULES.md`

Added 9 new rules in a "Schema Rules" section:

**Errors (PGE):**
- PGE-921 (Rule 9.21): Schema Property Scope Conflict — universal `[#]` vs branch-wise `[.]`/`[:]`
- PGE-922 (Rule 9.22): Unbounded Collection Nesting — collection value type without `%##Depth.Max`
- PGE-923 (Rule 9.23): Field Type Contradiction — explicit `###` mismatches field declarations
- PGE-924 (Rule 9.24): Invalid Key Type — `%##Children.Type` not inheriting `#IndexString`
- PGE-925 (Rule 9.25): Mixed Field Kinds — sibling fields mix typed and untyped
- PGE-926 (Rule 9.26): Schema Outside Type Definition — `##` used outside `{#}`

**Warnings (PGW):**
- PGW-904 (Rule 9.21w): Redundant Schema Property — `%##`/`%###` already inherited
- PGW-905 (Rule 9.22w): Contradicting Schema Override — overrides inherited value
- PGW-906 (Rule 9.23w): Unlimited Depth on User Type — `%##Depth.Max << -1` on non-Serial type

All rules include VALID, INVALID, and/or WARNING code examples matching the existing format.

### `docs/technical/EBNF.md`

1. **`schema_id` / `field_type_id`** — new identifier productions: `"##" name` and `"###" name`
2. **`identifier`** — extended to include `schema_id` and `field_type_id`
3. **`generic_param_typed`** — new production for typed params with defaults: `'<' name '#' type_expr [ "<<" value_expr ]`
4. **`schema_composition`** — new production: `"[#]" "<<" schema_id`
5. **`field_type_composition`** — new production: `"[#]" "<<" field_type_id`
6. **`schema_property`** — updated to use `"%##"` prefix: `"[#]" "%##" dotted_name assignment_op expression`
7. **`field_type_property`** — new production: `"[#]" "%###" dotted_name assignment_op expression`
8. **`type_constraint`** — updated to use schema composition: `"[<]" "<<" schema_id`
9. **Section 4.4 Tree Child Accessor** — new section with `child_access` production: `variable_id '<' name { '<' name }`
10. **`value_expr`** — extended to include `child_access`
11. **`data_def`** — restructured to use `generic_def_header` and `data_body_line` (which includes schema/field type composition, properties, and type parameter lines)
12. **`type_param_line`** — new production for `[#]` type parameter lines with nested `[<]` constraints

## Verification

- metadata-tree.md: 5 occurrences of `%definition.##`, 29 occurrences of `%##`/`%###` properties
- COMPILE-RULES.md: 32 occurrences of PGE-921 through PGE-926 and PGW-904 through PGW-906
- EBNF.md: 11 occurrences of new schema grammar productions
- No changes to types.md, collections.md, data-is-trees.md, or draft.md

## Acceptance Criteria

- AC-1: Satisfied — metadata-tree.md documents `%definition.##`, `%##` properties, `%###` field types, and complete examples
- AC-2: Satisfied — COMPILE-RULES.md contains all 9 rules with codes, conditions, messages, and examples
- AC-3: Satisfied — EBNF.md contains grammar for `##`, `###`, `<` child access, `%##`, `%###`, `[<]`, type param defaults
