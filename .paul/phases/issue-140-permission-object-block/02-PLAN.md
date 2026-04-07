---
phase: issue-140-permission-object-block
plan: 02
type: execute
wave: 2
depends_on: ["01"]
files_modified:
  - docs/technical/ebnf/03-identifiers.md
  - docs/technical/ebnf/05-block-elements.md
  - docs/technical/ebnf/09-definition-blocks.md
  - docs/technical/spec/metadata-tree/branches.md
  - docs/technical/spec/metadata-tree/object-types.md
autonomous: true
---

<objective>
## Goal
Update the technical specification files (EBNF grammar and metadata tree) to formally define the `{_}` permission object block, `_`/`__`/`___` identifier tiers, `[_]` block element, and restructured `%_` permission branch.

## Purpose
Plan 01 established the user-facing documentation for `{_}` permission objects. This plan formalizes the grammar and metadata tree so the technical spec is consistent with the user docs. Without this, the EBNF and metadata tree still describe the old flat `[_] _Category.capability` inline system.

## Output
- EBNF §3.1 updated with `permission_id`, `permission_descriptor_id`, `permission_constraint_id`
- EBNF §5 updated with `[_]` as a block element
- EBNF §9.x new section for `{_}` permission object definition block
- Metadata tree branches.md `%_` section updated for named `{_}` objects
- Metadata tree object-types.md `%_` row updated
</objective>

<context>
## Project Context
@.paul/PROJECT.md
@.paul/STATE.md

## Prior Work
@.paul/phases/issue-140-permission-object-block/01-SUMMARY.md

## Source Files
@docs/technical/ebnf/03-identifiers.md
@docs/technical/ebnf/05-block-elements.md
@docs/technical/ebnf/09-definition-blocks.md
@docs/technical/spec/metadata-tree/branches.md
@docs/technical/spec/metadata-tree/object-types.md

## Design Decisions (from issue #140)
1. {_} is a first-class block type for named, reusable permission objects
2. _/__/___ tiers mirror #/##/### (object / descriptor / constraint)
3. .intent field: #Ceiling allows globs, #Grant requires specific values
4. [_] in {@} and {=} always references a {_} object by name — no inline declarations
5. _ objects must be fully filled — no empty leaves
6. No instances — compile-time only, no :{instance} level
7. Per-category capability enums: #FileCapability, #WebCapability, etc.
8. __Permission is the schema; ___Unix, ___Sandboxed are constraint descriptors
</context>

<acceptance_criteria>

## AC-1: Permission Identifier Tiers in EBNF
```gherkin
Given the EBNF §3.1 identifier grammar
When a reader looks up permission identifiers
Then they find permission_id (_Name), permission_descriptor_id (__Name), and permission_constraint_id (___Name) as distinct productions
```

## AC-2: [_] Block Element in EBNF
```gherkin
Given the EBNF §5 block elements grammar
When a reader looks up permission-related block elements
Then they find [_] listed as a block element (permission reference)
```

## AC-3: {_} Definition Block in EBNF
```gherkin
Given the EBNF §9 definition blocks
When a reader looks up the {_} block
Then they find a new section (§9.x) defining permission_object_def grammar with .intent, per-category capability fields, and [.] field assignments
```

## AC-4: %_ Branch Updated in Metadata Tree
```gherkin
Given docs/technical/spec/metadata-tree/branches.md
When a reader examines the Permission Branch section
Then they see %_ structured around named {_} objects with .intent, .target, .grant, .resource, .audit subsections reflecting the __Permission schema
```

## AC-5: Object Types Table Updated
```gherkin
Given docs/technical/spec/metadata-tree/object-types.md
When a reader looks at the %_ row
Then they see updated description reflecting named {_} permission objects with _/__/___ tiers
```

</acceptance_criteria>

<tasks>

<task type="auto">
  <name>Task 1: Update EBNF files — identifiers, block elements, definition blocks</name>
  <files>docs/technical/ebnf/03-identifiers.md, docs/technical/ebnf/05-block-elements.md, docs/technical/ebnf/09-definition-blocks.md</files>
  <action>
    **03-identifiers.md (§3.1):**
    - Add three new identifier productions after `error_id`:
      ```
      permission_id            ::= '_' name ;
      permission_descriptor_id ::= "__" name ;
      permission_constraint_id ::= "___" name ;
      ```
    - Add all three to the `identifier` alternative list
    - Update frontmatter date

    **05-block-elements.md (§5):**
    - Add `[_]` as a new category (permission element) in the block_element alternatives
    - Add production: `permission_elem ::= "[_]" ;`
    - Update frontmatter date

    **09-definition-blocks.md (§9.x):**
    - Add new section §9.x (after §9.7 Array, number as §9.x to avoid renumbering) for Permission Object Definition
    - Grammar:
      ```
      permission_object_def ::= "{_}" permission_id NEWLINE
                                indent "[.]" ".intent" push_left ("#Ceiling" | "#Grant") NEWLINE
                                { indent permission_capability_line NEWLINE }
                                { indent comment_line NEWLINE } ;

      permission_capability_line ::= "[.]" "." category_name "." capability_name string_literal ;
      ```
    - Add rules: {_} must have .intent as first field, must be fully filled, no :{instance}
    - Add to cross-reference table at bottom
    - Update frontmatter date

    Avoid: Renumbering existing sections. Use §9.10 or similar for the new section.
    Avoid: Adding EBNF for __Permission schema internals (that's compiler-internal, not user syntax).
  </action>
  <verify>Read all three files and confirm: permission_id/descriptor/constraint in §3.1, [_] in §5, {_} grammar in §9, all productions syntactically valid EBNF</verify>
  <done>AC-1, AC-2, AC-3 satisfied: EBNF formally defines {_} permission objects</done>
</task>

<task type="auto">
  <name>Task 2: Update metadata tree — branches.md and object-types.md</name>
  <files>docs/technical/spec/metadata-tree/branches.md, docs/technical/spec/metadata-tree/object-types.md</files>
  <action>
    **branches.md — Permission Branch section (lines ~120-168):**
    - Rewrite the %_ structure to reflect named {_} objects instead of flat categories
    - New structure should show:
      ```
      %_
      +-- :{ObjectName}              <- named {_} permission objects (e.g., _DataCeiling)
          +-- .intent                #PermissionIntent (Ceiling | Grant)
          +-- .target                __PermissionTarget
          |   +-- .category          #PermissionCategory
          |   +-- .capability        per-category enum
          |   +-- .scope             __PermissionScope
          +-- .grant                 __PermissionGrant
          +-- .resource              __ResourceDescriptor
          +-- .audit                 __PermissionAudit
      ```
    - IMPORTANT: %_ now has `:` flexible field for the object name, but still NO :{instance} level
    - Update Key Properties to reflect: named objects, no instances, compile-time only, fully-filled requirement
    - Keep "Nested under %@ and %=" note
    - Update frontmatter date

    **object-types.md — %_ row (line 26):**
    - Update description to reflect named {_} permission objects with _/__/___ tiers
    - Change "All fixed (.)" to note that object names use `:` but all subfields use `.`
    - Update frontmatter date

    **IMPORTANT path-grammar.md consideration:**
    - The permission_path rule in path-grammar.md currently says `"%" "_" { "." field }` (all dots)
    - With named {_} objects, the path becomes `"%" "_" ":" name { "." field }` — the object name is a flexible `:` level
    - Update permission_path EBNF rule to reflect this
    - Update the "Exception branch rules" note accordingly

    Avoid: Changing pipeline, wrapper, queue, or trigger branch sections.
    Avoid: Modifying instance-lifecycle.md or other metadata-tree files.
  </action>
  <verify>Read branches.md, object-types.md, and path-grammar.md. Confirm: %_ shows named objects, object-types.md row updated, permission_path rule updated, no contradictions with Plan 01 user docs</verify>
  <done>AC-4, AC-5 satisfied: Metadata tree formally describes {_} permission objects</done>
</task>

</tasks>

<boundaries>

## DO NOT CHANGE
- docs/user/ (user-facing docs updated in Plan 01)
- docs/technical/compile-rules/ (compile rule updates are Plan 03)
- docs/technical/spec/metadata-tree/instance-lifecycle.md
- docs/technical/spec/metadata-tree/io-ports.md
- docs/technical/spec/metadata-tree/definition-templates.md
- docs/technical/ebnf/ files other than 03, 05, 09
- .paul/ files other than STATE.md

## SCOPE LIMITS
- This plan covers EBNF grammar and metadata tree only
- No compile rule updates (Plan 03)
- No pglib permission reference updates (Plan 03)
- No user-facing documentation changes (Plan 01 complete)

</boundaries>

<verification>
Before declaring plan complete:
- [ ] §3.1 has permission_id, permission_descriptor_id, permission_constraint_id
- [ ] §5 has [_] permission element
- [ ] §9.x has {_} permission_object_def grammar
- [ ] branches.md %_ shows named objects with __Permission schema
- [ ] object-types.md %_ row updated
- [ ] path-grammar.md permission_path updated for named objects
- [ ] No contradictions between EBNF and metadata tree
- [ ] No contradictions with Plan 01 user docs
- [ ] All acceptance criteria met
</verification>

<success_criteria>
- All 5 files (+ path-grammar.md) modified with consistent {_} technical spec
- EBNF productions are syntactically valid
- Metadata tree structure matches __Permission schema from Plan 01
- No contradictions between files
- Frontmatter dates updated
</success_criteria>

<output>
After completion, create `.paul/phases/issue-140-permission-object-block/02-SUMMARY.md`
</output>
