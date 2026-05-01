---
phase: typed-cross-references
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - docs/audit/README.md
  - docs/audit/rules/conventions.md
  - docs/audit/rules/checklist.md
autonomous: true
---

<objective>
## Goal
Define the typed cross-reference protocol (`@c:` concept and `@u:` usage) in the audit infrastructure so all future documentation follows the new convention.

## Purpose
Cross-references currently carry no semantic context — a reader can't tell if a link explains a concept or shows where syntax is used. Typed references enable change propagation: when a concept changes, all docs referencing it can be traced and updated.

## Output
Three updated audit files defining the new protocol, with backward compatibility for existing untyped `@` refs.
</objective>

<context>
## Project Context
@.paul/PROJECT.md
@.paul/ROADMAP.md
@.paul/STATE.md

## Phase Context
@.paul/phases/typed-cross-references/CONTEXT.md

## Source Files
@docs/audit/README.md
@docs/audit/rules/conventions.md
@docs/audit/rules/checklist.md
</context>

<acceptance_criteria>

## AC-1: Protocol Defined
```gherkin
Given docs/audit/README.md "Dual Smart Referencing" section exists
When the section is updated
Then it defines three reference forms:
  - @c: (concept) — points to definitions, theory, design rationale
  - @u: (usage) — points to syntax specs, EBNF, jm3lib signatures, scenarios where a construct appears
  - @ (untyped) — legacy form, valid but deprecated for new docs
And each form has at least two real examples from existing docs (not hypothetical)
And behavioral guidance explains both types are mandatory read-before-write
```

## AC-2: Convention Rules Updated
```gherkin
Given docs/audit/rules/conventions.md "Cross-Reference Rules" table exists
When the table is updated
Then it includes rules for:
  - typed-refs: New docs must use @c: or @u: (not bare @)
  - usage-coverage: Every syntax construct appearing without explanation must have @u: to its definition
  - untyped-valid: Existing bare @ refs remain valid until migrated
```

## AC-3: Checklist Updated
```gherkin
Given docs/audit/rules/checklist.md exists
When updated
Then a new check "typed-ref-check" exists under a "Cross-Reference Check" section
And the check validates: new @-imports use @c: or @u: type prefix
And the existing "dual-refs" check is preserved
```

</acceptance_criteria>

<tasks>

<task type="auto">
  <name>Task 1: Update audit/README.md protocol definition</name>
  <files>docs/audit/README.md</files>
  <action>
    Replace the "Dual Smart Referencing" section (lines 43-68) with an expanded section covering:

    1. Rename to "Typed Smart Referencing" (or similar)
    2. Three-row table: @c: (concept), @u: (usage), @ (untyped/legacy)
    3. For @c: — explain it points to definitions, theory, design rationale. Examples:
       - `<!-- @c:glossary#Trigger Monitor -->` — loading a term definition
       - `<!-- @c:vision#Core Philosophy -->` — loading philosophy context
    4. For @u: — explain it marks where a syntax construct appears and points back to its definition. Examples:
       - `<!-- @u:syntax/blocks#trigger -->` — a doc uses [T] trigger syntax without explaining it
       - `<!-- @u:technical/ebnf/conditional#match-syntax -->` — match syntax appears in a code example
    5. Both are mandatory read-before-write imports for Claude
    6. The semantic tag tells Claude WHY the import exists:
       - @c: means "understand the concept before writing about it"
       - @u: means "this construct is used here; if its definition changes, this doc needs updating"
    7. Untyped @ remains valid for backward compatibility, deprecated for new docs
    8. Update the wikilink convention to show c:/u: display prefix (deferred migration)
    9. Keep the "How @-imports work" subsection, updated for typed refs

    Avoid: Removing existing wikilink examples. Do NOT invent guide/tutorial references.
  </action>
  <verify>Read docs/audit/README.md and confirm: three ref types defined, real examples used, backward compat stated</verify>
  <done>AC-1 satisfied: Protocol defines @c:, @u:, and untyped @ with real examples and behavioral guidance</done>
</task>

<task type="auto">
  <name>Task 2: Update convention cross-reference rules</name>
  <files>docs/audit/rules/conventions.md</files>
  <action>
    In the "Cross-Reference Rules" table (lines 36-43), add three new rows:

    | typed-refs | New docs must use @c: or @u: — not bare @ | Bare @ is deprecated for new files |
    | usage-coverage | Syntax constructs appearing without explanation must have @u: to definition | Enables change propagation |
    | untyped-valid | Existing bare @ refs valid until file is touched | Migrate-on-touch policy |

    Keep existing rows (dual-refs, no-duplication, relative-paths, vision-authority, glossary-authority).

    Avoid: Changing any other section of conventions.md.
  </action>
  <verify>Read docs/audit/rules/conventions.md and confirm: three new rows present, existing rows unchanged</verify>
  <done>AC-2 satisfied: Convention rules include typed-refs, usage-coverage, untyped-valid</done>
</task>

<task type="auto">
  <name>Task 3: Add typed-ref-check to checklist</name>
  <files>docs/audit/rules/checklist.md</files>
  <action>
    Add a new "Cross-Reference Check" section after the existing "Completeness Check" section:

    ## Cross-Reference Check

    | ID | Check | Source |
    |----|-------|--------|
    | typed-ref-check | New @-imports use @c: or @u: type prefix | [[conventions#typed-refs]] |
    | usage-coverage-check | Syntax constructs used without explanation have @u: back-reference | [[conventions#usage-coverage]] |

    Keep the existing "dual-refs" check in the Completeness section as-is.

    Avoid: Modifying any existing checks. Do NOT remove or reorder existing sections.
  </action>
  <verify>Read docs/audit/rules/checklist.md and confirm: new section exists with two checks, existing dual-refs check preserved</verify>
  <done>AC-3 satisfied: typed-ref-check and usage-coverage-check added</done>
</task>

</tasks>

<boundaries>

## DO NOT CHANGE
- docs/vision.md (authority chain — untouched)
- docs/audit/reference/glossary.md (no new terms needed for this plan)
- docs/audit/audiences/* (audience rules unaffected)
- Any files outside docs/audit/

## SCOPE LIMITS
- This plan defines the protocol only — no classification or migration
- Do not add @c:/@u: refs to any docs outside the audit folder
- Do not create guide/tutorial documents
- Wikilink c:/u: display prefixes: define syntax but do not migrate existing wikilinks

</boundaries>

<verification>
Before declaring plan complete:
- [ ] docs/audit/README.md defines @c:, @u:, and untyped @ with examples
- [ ] docs/audit/rules/conventions.md has three new cross-ref rules
- [ ] docs/audit/rules/checklist.md has typed-ref-check and usage-coverage-check
- [ ] No existing checks, rules, or sections removed
- [ ] All examples use real doc paths from the existing codebase
</verification>

<success_criteria>
- All three tasks completed
- Protocol is self-consistent across all three files
- Backward compatibility explicitly stated
- No files outside docs/audit/ modified
</success_criteria>

<output>
After completion, create `.paul/phases/typed-cross-references/tcr-01-SUMMARY.md`
</output>
