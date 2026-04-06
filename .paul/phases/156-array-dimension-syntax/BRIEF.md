---
issue: 156
group: 2
group_name: "Syntax Documentation Gaps"
priority: P2-high
status: brief-ready
---

# Issue #156: Array dimension :2D annotation vs access syntax unmapped -- three conflicting access notations

## Inconsistency
Three different array access syntaxes appear across the documentation with no reconciliation. User docs (arrays.md) show colon-separated indexing (`$matrix:0:1`), technical docs (PGE04017, edge-cases/04-type-system.md) show dot-separated indexing (`$matrix.0.1`), and the tree child accessor docs (prefix-system.md, array.md, data-is-trees.md, EBNF SS 4.4) show angle-bracket access (`$matrix<0<1`). There is no spec section that maps the `:2D` dimension annotation to any specific access operator, and the three notations are never cross-referenced or reconciled. This is not just a documentation gap -- the three forms appear to be genuinely contradictory.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/user/syntax/types/arrays.md` | Uses colon-separated access: `$matrix:0:1` for 2D arrays (line 38) |
| `docs/technical/compile-rules/PGE/PGE04017-array-dimension-access-mismatch.md` | Uses dot-separated access: `$matrix.0.1` for 2D arrays (line 28) |
| `docs/technical/edge-cases/04-type-system.md` | Uses dot-separated access: `$matrix.0.1` (line 302) |
| `docs/user/syntax/types/prefix-system.md` | Uses `<` tree child access: `$matrix<0<1` (line 34) |
| `docs/user/concepts/collections/array.md` | Uses `<` tree child access: `$matrix<1<2` (line 36) |
| `docs/user/concepts/data-is-trees.md` | Uses `<` tree child access: `$matrix<0<1` (line 37) |
| `docs/technical/ebnf/04-type-system.md` | EBNF defines `child_access ::= variable_id '<' name { '<' name }` -- only `<` form (line 162) |

## Example
**Source A** (`docs/user/syntax/types/arrays.md`, line ~38):
> `[r] $val << $matrix:0:1              [ ] 2 indices for :2D`

**Source B** (`docs/technical/compile-rules/PGE/PGE04017-array-dimension-access-mismatch.md`, line ~28):
> `[r] $val << $matrix.0.1`

**Source C** (`docs/user/syntax/types/prefix-system.md`, line ~34):
> | `$var<key<subkey` | Chained access for nested trees | `$matrix<0<1`, `$df<0<product` |

**Source D** (`docs/technical/ebnf/04-type-system.md`, line ~162):
> `child_access        ::= variable_id '<' name { '<' name } ;`

## Prior Related Work
- Issue #37 -- Multidimensional array via `:ND` (closed 2026-03-24). Unified array + tensor into `#array.<type>:<N>D`; removed `;tensor` concept. This is where the `:ND` dimension annotation was formalized.
- Issue #88 -- Add schema properties to `{#}` definitions (closed 2026-03-28). Established `##Contiguous` and `##Rectangular` for arrays, which implies the `<` tree-based access model (arrays as Maps with `#UnsignedInt` keys).

## Recommendation
This requires a design decision to pick one canonical access syntax for array elements and update all inconsistent files. Given that (a) the EBNF only defines `<` access, (b) `#Array` is documented as a `#Map` variant with `#UnsignedInt` keys using tree child access, and (c) prefix-system.md, array.md, and data-is-trees.md all agree on `<` -- the `<` form appears to be the authoritative syntax. The `:` form in arrays.md conflates flexible field notation (`:` is for user-defined keys) with integer indexing, and the `.` form in PGE04017 conflates fixed field notation (`.` is for schema-defined fields) with numeric access. Update arrays.md to use `$matrix<0<1`, update PGE04017 and edge-cases/04-type-system.md to use the same form, and add a "Multidimensional Access" section to arrays.md that explicitly maps `:ND` declarations to `<` access patterns.

## Discussion Prompts
1. Is `<` definitively the only valid array element access operator, or do `:` and `.` serve as alternative syntaxes for specific contexts (e.g., type annotations vs runtime access)?
2. If `<` is canonical, should arrays.md's `:0:1` examples be treated as a bug introduced during issue #37 (which predates the `<` operator formalization)?
3. Does `$matrix.0.1` (dot access) work because array indices could be interpreted as fixed fields on a `##Rectangular` schema, or is it simply an error in the PGE04017 examples?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 156*
