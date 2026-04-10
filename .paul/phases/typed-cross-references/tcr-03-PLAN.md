---
phase: typed-cross-references
plan: 03
type: execute
wave: 3
depends_on: ["tcr-02"]
files_modified:
  - "up to 226 files with existing @ refs"
  - "additional files needing new @u: coverage refs"
autonomous: true
---

<objective>
## Goal
Migrate all existing `@` references to `@c:` or `@u:` per the approved classification table, and add new `@u:` back-references for syntax constructs that appear without cross-references (coverage gaps).

## Purpose
Completes the typed cross-reference system. After this plan, every `@` import carries semantic context and every syntax construct appearance is traceable to its definition — enabling systematic change propagation across the documentation.

## Output
All 226+ files updated with typed references. Coverage gaps filled with new `@u:` refs.
</objective>

<context>
## Project Context
@.paul/PROJECT.md
@.paul/STATE.md

## Prior Work
@.paul/phases/typed-cross-references/tcr-01-PLAN.md (protocol)
@.paul/phases/typed-cross-references/tcr-02-PLAN.md (classification + gaps)
@docs/audit/tracking/ref-classification.md (approved classification table)

## Source Files
@docs/audit/README.md (protocol reference)
</context>

<acceptance_criteria>

## AC-1: Existing Refs Migrated
```gherkin
Given the approved classification table assigns @c: or @u: to each target
When migration runs across all 226 files
Then every classified @ ref is updated to @c: or @u: per the table
And refs classified as "untyped" remain unchanged
And no @ patterns are broken (every <!-- @ still has matching -->)
```

## AC-2: Coverage Gaps Filled
```gherkin
Given the coverage gap inventory identifies constructs appearing without @u: refs
When new @u: refs are added
Then high-priority gaps (files with 5+ unlinked constructs) are addressed first
And each new @u: ref points to the correct definition file per the gap inventory
```

## AC-3: Migration Verified
```gherkin
Given migration is complete
When grep validates the result
Then grep for '<!-- @[^cu]' in docs/ returns only intentionally untyped refs
And grep for '<!-- @[cu]:' returns all migrated refs
And total ref count (typed + untyped) matches or exceeds pre-migration count of 409
```

</acceptance_criteria>

<tasks>

<task type="auto">
  <name>Task 1: Migrate existing @ refs to @c:/@u:</name>
  <files>Up to 226 files per classification table</files>
  <action>
    1. Read the approved classification table from docs/audit/tracking/ref-classification.md
    2. For each classified target (non-untyped):
       - Find all files containing that @ pattern
       - Replace `<!-- @{target} -->` with `<!-- @c:{target} -->` or `<!-- @u:{target} -->` per classification
    3. Process targets in order of frequency (highest count first) for maximum impact early
    4. After all replacements, validate:
       - Every `<!-- @` still has a matching ` -->`
       - No double-prefixes (e.g., `@c:c:` or `@u:u:`)
    5. Commit in batches if the change set is large (group by docs subdirectory: user/, technical/, audit/)

    Avoid: Modifying anything between `<!-- @...` and `-->` other than adding the c:/u: prefix.
    Avoid: Touching wikilinks — only HTML comment refs are migrated in this plan.
  </action>
  <verify>
    grep -r '<!-- @[^cu!-]' docs/ --include='*.md' | grep -v '@c:' | grep -v '@u:' — should return only intentionally untyped refs
  </verify>
  <done>AC-1 satisfied: all classified refs migrated</done>
</task>

<task type="auto">
  <name>Task 2: Add @u: coverage refs for syntax gaps</name>
  <files>Files identified in coverage gap inventory</files>
  <action>
    1. Read the coverage gap section from docs/audit/tracking/ref-classification.md
    2. For each high-priority gap:
       - Open the file where the construct appears unexplained
       - Add `<!-- @u:{definition-file}#{section} -->` near the first appearance of the construct
       - Place the comment on its own line, before the paragraph or code block using the construct
    3. For code blocks: place the @u: ref before the code fence, not inside it
    4. Focus on high-priority files first (5+ unlinked constructs)
    5. Track new refs added for the summary

    Avoid: Adding refs inside code blocks or disrupting document flow.
    Avoid: Adding refs for constructs that ARE being defined/explained in that same file.
  </action>
  <verify>Spot-check 5 files from the gap inventory — each should have new @u: refs for previously unlinked constructs</verify>
  <done>AC-2 satisfied: high-priority coverage gaps filled</done>
</task>

<task type="auto">
  <name>Task 3: Validate migration completeness</name>
  <files>None (read-only validation)</files>
  <action>
    1. Count total @c: refs across docs/
    2. Count total @u: refs across docs/
    3. Count remaining untyped @ refs
    4. Verify: typed + untyped >= 409 (pre-migration count)
    5. Verify: no broken ref patterns (unclosed comments)
    6. Update docs/audit/tracking/ref-classification.md with a "Migration Results" section:
       - @c: count
       - @u: count
       - Untyped remaining count
       - New @u: refs added for coverage
  </action>
  <verify>
    grep -c '<!-- @c:' docs/ -r --include='*.md' && grep -c '<!-- @u:' docs/ -r --include='*.md' && grep -c '<!-- @[^cu]' docs/ -r --include='*.md'
  </verify>
  <done>AC-3 satisfied: migration verified with counts</done>
</task>

</tasks>

<boundaries>

## DO NOT CHANGE
- docs/audit/README.md (protocol — locked after Plan 01)
- docs/audit/rules/* (rules — locked after Plan 01)
- docs/vision.md (authority — never touch)
- .paul/* (project management — not documentation)

## SCOPE LIMITS
- Only modify @-import HTML comments — no prose changes
- Do not migrate wikilink display text (deferred to migrate-on-touch)
- Do not create new documentation files
- Do not restructure or reformat any document beyond adding/modifying @ comments
- Coverage gap refs: high-priority files only, not exhaustive 595-file sweep

</boundaries>

<verification>
Before declaring plan complete:
- [ ] grep for untyped @ refs returns only intentionally untyped targets
- [ ] grep for @c: and @u: returns expected counts
- [ ] Total ref count >= 409 (pre-migration baseline)
- [ ] No broken comment patterns
- [ ] ref-classification.md has Migration Results section
- [ ] At least high-priority coverage gaps addressed
</verification>

<success_criteria>
- All classified refs migrated to @c: or @u:
- High-priority coverage gaps filled with new @u: refs
- Migration validated with grep counts
- No documents broken or reformatted
</success_criteria>

<output>
After completion, create `.paul/phases/typed-cross-references/tcr-03-SUMMARY.md`
</output>
