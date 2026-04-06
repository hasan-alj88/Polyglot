---
issue: 148
group: 3
group_name: "Naming & Terminology Conflicts"
priority: P4-low
status: brief-ready
---

# Issue #148: #Array vs #array — case inconsistency across docs

## Inconsistency
The Array type appears in two casings: `#Array` (PascalCase, the formal type/macro definition name) and `#array` (lowercase, the alias used in type annotations). Polyglot's alias system intentionally supports both — `#Array` is the macro-generated type name, `#array` is the `%##Alias`-registered shorthand. However, some docs mix the two in annotation contexts where only the lowercase alias should appear, or use PascalCase in field annotations where lowercase is the convention. The metadata tree branch specification uses `#Array:ResourceTag` in a field annotation (line 102) instead of the expected lowercase `#array:ResourceTag`.

## Affected Files
| File | What's Wrong |
|------|-------------|
| `docs/technical/spec/metadata-tree/branches.md` | Line 102: `.resourceTags#Array:ResourceTag` uses PascalCase `#Array` in a field annotation where lowercase `#array` is the convention |
| `docs/technical/COMPILE-RULES.md` | Lines 335, 342, 348: `#Array<#Array<#float` uses PascalCase `#Array` in inheritance syntax |
| `docs/user/stdlib/types/types.md` | Line 32: `#Array:ValueType:Dim` uses PascalCase in type hierarchy display |
| `docs/user/stdlib/types/collections.md` | Uses both `#Array` (section headers, macro defs) and `#array` (usage examples) — correct but potentially confusing without explanation |
| `docs/technical/spec/metadata-tree/field-expansion.md` | Line 20: `#array:string` uses lowercase — correct for annotation context |

## Example
**Source A** (`docs/technical/spec/metadata-tree/branches.md`, line ~102):
> ├── .resourceTags#Array:ResourceTag <- resource constraint tags

**Source B** (`docs/user/syntax/types/arrays.md`, line ~28):
> [=] <items#array:string              [ ] 1D array (default)

**Source C** (`docs/user/stdlib/types/collections.md`, line ~105):
> [#] %##Alias
>    [:] << "array:{$ValueType%name}:{$dim}"

## Prior Related Work
- No directly related closed issues. The alias system was established during Issue #88 (schema properties) and Issue #94 (macro-for-generics redesign).

## Recommendation
This is mostly working as intended — `#Array` is the formal definition name and `#array` is the alias for type annotations. The fix is narrow: (1) change `branches.md` line 102 from `#Array:ResourceTag` to `#array:ResourceTag` to match annotation conventions, (2) ensure COMPILE-RULES.md inheritance examples use `#Array` correctly since `<~` operates on definition names not aliases, (3) optionally add a note to `collections.md` explaining why both casings appear.

## Discussion Prompts
1. Is `#Array` vs `#array` in field annotations a genuine error or an acceptable alternate form?
2. Should the inheritance syntax (`<~ #Array`) always use PascalCase while type annotations (`$var#array:int`) always use lowercase?
3. Does this need a compile rule or is it purely a documentation consistency fix?

---
*Brief prepared: 2026-04-06*
*Start work: /paul:work-issue 148*
