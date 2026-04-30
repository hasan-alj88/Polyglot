---
phase: typed-cross-references
plan: 02
type: research
wave: 2
depends_on: ["tcr-01"]
files_modified:
  - docs/audit/tracking/ref-classification.md
autonomous: false
---

<objective>
## Goal
Classify every unique `@` reference target as `@c:` (concept), `@u:` (usage), or "untyped" (leave as-is). Additionally, identify syntax constructs that appear in docs without any cross-reference — the coverage gaps that need new `@u:` refs.

## Purpose
Plan 03 (migration) needs a classification table to execute mechanically. Without it, each of the 226 files requires subjective judgment during migration. This plan front-loads that judgment into a reviewable artifact.

## Output
A classification table at docs/audit/tracking/ref-classification.md with every unique `@` target classified and a coverage gap inventory.
</objective>

<context>
## Project Context
@.paul/PROJECT.md
@.paul/STATE.md

## Prior Work
@.paul/phases/typed-cross-references/tcr-01-PLAN.md (protocol definition)

## Source Files
@docs/audit/README.md (updated protocol from Plan 01)
</context>

<acceptance_criteria>

## AC-1: All Unique Targets Classified
```gherkin
Given ~60-70 unique @ targets exist across 409 references in 226 files
When the classification table is created
Then every unique target has exactly one classification: @c:, @u:, or untyped
And each classification includes a one-line rationale
```

## AC-2: Classification Follows Protocol
```gherkin
Given the protocol from Plan 01 defines @c: and @u: semantics
When targets are classified
Then targets pointing to glossary, vision, design rationale → @c:
And targets pointing to EBNF, syntax specs, aj3lib signatures, scenarios → @u:
And targets that are pure navigation (INDEX files) → untyped
```

## AC-3: Coverage Gaps Identified
```gherkin
Given docs contain syntax constructs (block markers, operators, type prefixes) in prose and code blocks
When the coverage gap analysis completes
Then a list of syntax constructs that appear without @u: back-references is produced
And each gap entry names: the construct, the file(s) where it appears unexplained, and the definition file it should reference
```

## AC-4: User Reviews Classification
```gherkin
Given the classification table and coverage gap inventory are complete
When presented to the user
Then the user reviews and approves before Plan 03 proceeds
```

</acceptance_criteria>

<tasks>

<task type="auto">
  <name>Task 1: Extract and classify all unique @ targets</name>
  <files>docs/audit/tracking/ref-classification.md</files>
  <action>
    1. Grep all `<!-- @` patterns across docs/ to extract every unique target
    2. Group by target (e.g., @vision:Core Philosophy, @glossary:Polyglot Service, @types, @blocks, etc.)
    3. Count occurrences of each target
    4. Classify each using the protocol from Plan 01:
       - @c: if target is a definition, concept, design rationale, glossary entry, or vision section
       - @u: if target is syntax spec, EBNF grammar, aj3lib signature, compile rule, or scenario
       - untyped if target is an INDEX file or pure navigation
    5. Write classification table to docs/audit/tracking/ref-classification.md with columns:
       | Current Form | Classification | Count | Rationale |
    6. Add summary counts at bottom: total @c:, total @u:, total untyped

    Avoid: Guessing about ambiguous targets — flag them for user review.
  </action>
  <verify>Count of classified targets matches count of unique @ targets found by grep</verify>
  <done>AC-1 and AC-2 satisfied: all targets classified with rationale</done>
</task>

<task type="auto">
  <name>Task 2: Identify syntax coverage gaps</name>
  <files>docs/audit/tracking/ref-classification.md</files>
  <action>
    1. Identify the core syntax constructs that should be cross-referenced when used:
       - Block markers: [T], [Q], [W], [=], [~], [*], [!], [?], [#], [r], [p], [b], [C], etc.
       - Type prefixes: #, ##, ###
       - Identifier prefixes: @, =, $, !
       - Operators: <~, ~>, <<, >>, =>, [+], [|]
       - Other: {@ }, {# }, {= }, {N }, {_ }, {; }
    2. For each construct, scan docs/ for files where it appears in prose or code blocks WITHOUT an existing @ reference to its definition
    3. Append a "Coverage Gaps" section to ref-classification.md:
       | Construct | Appears In (sample files) | Definition File | Gap Count |
    4. Focus on the most-used constructs first; flag files with 5+ unlinked constructs as high-priority

    Avoid: Trying to list every single occurrence — sample representative files per construct.
  </action>
  <verify>Coverage gap section exists with at least 10 constructs analyzed</verify>
  <done>AC-3 satisfied: coverage gaps identified with construct, files, and definition targets</done>
</task>

<task type="checkpoint:human-verify" gate="blocking">
  <what-built>Classification table and coverage gap inventory in docs/audit/tracking/ref-classification.md</what-built>
  <how-to-verify>
    1. Open docs/audit/tracking/ref-classification.md
    2. Review the classification table — check that @c:/@u:/untyped assignments make sense
    3. Review the coverage gaps — check priorities and definition file mappings
    4. Flag any disagreements or reclassifications
  </how-to-verify>
  <resume-signal>Type "approved" to proceed to Plan 03 (migration), or describe changes needed</resume-signal>
</task>

</tasks>

<boundaries>

## DO NOT CHANGE
- docs/audit/README.md (updated in Plan 01 — locked)
- docs/audit/rules/* (updated in Plan 01 — locked)
- Any documentation files outside docs/audit/tracking/

## SCOPE LIMITS
- This plan produces a classification artifact only — no files are migrated
- Do not modify any @ references in any docs
- Coverage gap analysis samples representative files, not exhaustive scan of all 595

</boundaries>

<verification>
Before declaring plan complete:
- [ ] docs/audit/tracking/ref-classification.md exists
- [ ] Classification table covers all unique @ targets (60-70 expected)
- [ ] Each entry has: current form, classification, count, rationale
- [ ] Coverage gaps section has 10+ constructs with sample files and definition targets
- [ ] User has reviewed and approved the classification
</verification>

<success_criteria>
- All unique @ targets classified
- Coverage gap inventory produced
- User approval obtained before Plan 03
</success_criteria>

<output>
After completion, create `.paul/phases/typed-cross-references/tcr-02-SUMMARY.md`
</output>
