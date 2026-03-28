---
phase: issue-88-schema-properties
plan: 01
status: complete
completed: 2026-03-28
---

# 88-01 Summary: Types and Data-is-Trees Updates

## Files Modified

### `docs/user/syntax/types.md`

1. **Three-Tier Prefix System** — new section documenting `#` (type), `##` (schema), `###` (field) with table and explanation
2. **`<` Operator: Definition vs Access** — new section documenting dual role (type param definition in `{#}` header, child access after `$var`), including type parameter default syntax `[#] <Param#Type << "default"`
3. **`%##` Schema Properties** — updated from old `%Key.*`/`%Ordered`/`%Depth.Max`/`%Alias` to full 9-property table with `%##` prefix: Depth.Max, Children.Type, Children.Gap, Children.Uniform, Children.Regular, Children.Min, Children.Max, Children.Ordered, Alias
4. **`%##Depth.Max` Inference Model** — updated to use `%##` prefix, added PGE-922 and PGW-906 references
5. **`###` Field Types** — new subsection documenting ###Value and ###Enum with inference rules, PGE-923 (contradiction), PGE-925 (mixed sibling kinds), examples from #Boolean and #String
6. **Approved `##` Schema Types** — new subsection with all 8 {#} definitions: ##Scalar, ##Flat, ##Deep, ##Homogeneous, ##Heterogeneous, ##Contiguous, ##Sparse, ##Rectangular
7. **#IndexString** — new Layer 2c subsection with {#} definition, regex `^[^\s.<>:]+$`, PGE-924 documentation
8. **#Dimension** — updated regex to `^[0-9]+$D` (allows 0D for scalars), updated description
9. **#String definition** — updated to use `[#] << ##Scalar` and `%##Alias`
10. **#Boolean definition** — updated to use `[#] << ##Scalar`, `[#] << ###Enum`, `%##Alias`
11. **All scalar subtype definitions** — updated `%Alias` to `%##Alias`
12. **[<] constraint examples** — updated to use `[<] << ##Scalar` instead of `[<] %Depth.Max << 0`
13. **Enum Fields vs Value Fields** — added `###` kind column, PGE-925 reference, cross-reference to field types section
14. **Type Hierarchy Summary** — added #IndexString, schema/field annotations, updated #Map/#Array descriptions
15. **Other Types list** — updated to reflect #Map rename, `<` accessor syntax, removed #Dataframe

### `docs/user/concepts/data-is-trees.md`

1. **Leaf-Only Values** — new section documenting the universal tree invariant: branch = namespace/enum (no value), leaf = RawString value (no children), no node has both
2. **Tree Shape and Leaf Content** — new section with brief `##` schema and `###` field type overview, `<` accessor mention, cross-references to types.md sections
3. All existing content preserved including Mermaid diagram

## Acceptance Criteria

- [x] AC-1: Three-Tier Prefix System documented
- [x] AC-2: All 8 approved ## schema types listed with {#} definitions
- [x] AC-3: ###Value and ###Enum documented with inference, PGE-923, PGE-925
- [x] AC-4: `<` operator dual role documented with type param defaults
- [x] AC-5: #IndexString added with {#} definition and PGE-924
- [x] AC-6: Leaf-only invariant in data-is-trees.md

## Boundaries Respected

- No changes to collections.md, metadata-tree.md, EBNF.md, COMPILE-RULES.md, or draft.md
