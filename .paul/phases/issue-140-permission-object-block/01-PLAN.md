---
phase: issue-140-permission-object-block
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - docs/user/concepts/permissions.md
  - docs/user/concepts/pipelines/permissions.md
  - docs/user/syntax/packages.md
  - docs/user/syntax/blocks.md
  - docs/user/syntax/identifiers.md
autonomous: true
---

<objective>
## Goal
Rewrite the core permission documentation to introduce {_} permission objects, the _/__/___ tier system, Ceiling vs Grant intent model, and per-category capability enums. This is the user-facing foundation that all subsequent plans build on.

## Purpose
The current permission system uses inline `[_]` declarations with flat `_Category.capability` identifiers. This plan elevates permissions to first-class `{_}` objects with structured schemas, matching the `#`/`##`/`###` data tier pattern. This enables reusable permission policies, compile-time Ceiling/Grant validation, and richer security modeling.

## Output
- Rewritten `docs/user/concepts/permissions.md` — complete {_} permission object system
- Updated `docs/user/concepts/pipelines/permissions.md` — [_] references {_} objects
- Updated `docs/user/syntax/packages.md` — {@} ceiling uses {_} objects
- Updated `docs/user/syntax/blocks.md` — {_} block type added
- Updated `docs/user/syntax/identifiers.md` — _/__/___ prefix tiers documented
</objective>

<context>
## Project Context
@.paul/PROJECT.md
@.paul/STATE.md

## Source Files
@docs/user/concepts/permissions.md
@docs/user/concepts/pipelines/permissions.md
@docs/user/syntax/packages.md
@docs/user/syntax/blocks.md
@docs/user/syntax/identifiers.md

## Design Decisions (from #131 brainstorm)
1. {_} is a first-class block type for named, reusable permission objects
2. _/__/___ tiers mirror #/##/### (object / descriptor / constraint)
3. .intent field: #Ceiling allows globs, #Grant requires specific values
4. [_] in {@} and {=} always references a {_} object by name — no inline declarations
5. Narrowing allowed (pipeline restricts policy subset), expanding not
6. _ objects must be fully filled — no empty leaves (compile error)
7. No instances — permissions are compile-time, apply across all pipeline instances
8. Per-category capability enums prevent nonsense (e.g., .Device.Read)
9. {@} first is compiler-enforced; {_} → {#} → {=} is recommended style
10. __Permission is the schema; ___Unix, ___Sandboxed are constraint descriptors
</context>

<acceptance_criteria>

## AC-1: {_} Block Type Defined
```gherkin
Given the blocks.md documentation
When a reader looks up {_}
Then they find a complete entry describing the permission object block, its purpose, and its tier system
```

## AC-2: Permission Object System Documented
```gherkin
Given docs/user/concepts/permissions.md
When a reader studies the permission system
Then they understand: {_} objects, _/__/___ tiers, Ceiling vs Grant, per-category enums, fully-filled requirement, and the narrowing-only rule
```

## AC-3: Pipeline Permission References Updated
```gherkin
Given docs/user/concepts/pipelines/permissions.md
When a reader checks how pipelines declare permissions
Then they see [_] referencing {_} objects by name, with narrowing syntax, and the Grant ⊆ Ceiling validation rule
```

## AC-4: Package Ceiling Uses {_} Objects
```gherkin
Given docs/user/syntax/packages.md
When a reader checks package permission ceiling syntax
Then they see {@} using [_] to reference {_} ceiling objects, not inline declarations
```

## AC-5: Identifier Tiers Documented
```gherkin
Given docs/user/syntax/identifiers.md
When a reader looks up the _ prefix
Then they find _/__/___ tier documentation parallel to #/##/###
```

</acceptance_criteria>

<tasks>

<task type="auto">
  <name>Task 1: Rewrite permissions.md with {_} object system</name>
  <files>docs/user/concepts/permissions.md</files>
  <action>
    Complete rewrite of the permission documentation:

    1. **Introduction** — Implicit-deny model unchanged, but permissions are now {_} objects
    2. **{_} Permission Object** — Named, reusable blocks containing permission declarations
       - Show syntax: `{_} _PolicyName` with [.] field assignments
       - .intent field: #Ceiling vs #Grant with examples
       - Ceiling allows globs ("data/*.csv"), Grant requires specifics ("data/reports/q1.csv")
       - Must be fully filled — every leaf must have a value
    3. **Tier System** — _/__/___ parallel to #/##/###
       - `_` = permission object (instance-level data)
       - `__` = permission descriptor (schema — __Permission)
       - `___` = constraint descriptor (leaf constraints — ___Unix, ___Sandboxed, ___ReadOnly)
    4. **Per-Category Capability Enums** — Replace flat string identifiers
       - #PermissionCategory enum: File, Web, Database, System, Crypto, IPC, Device, Memory
       - Per-category: #FileCapability (Read, Write, Execute, Delete, Create), #WebCapability (Request, Socket, Listen), etc.
    5. **Ceiling vs Grant** — How the two intents layer
       - {@} references ceiling {_} objects
       - {=} references grant {_} objects
       - Compiler validates: Grant ⊆ Ceiling
       - Narrowing allowed, expanding not
    6. **No Instances** — Permissions are compile-time, apply across all instances
    7. **Proposed __Permission Schema** — Full tree showing .target, .grant, .resource, .audit sections
    8. **Examples** — Complete package + pipeline example showing ceiling → grant flow

    Keep the 8 permission categories (File, Web, Database, System, Crypto, IPC, Device, Memory).
    Preserve wikilinks and @-import patterns. Update frontmatter date.
    Avoid: inventing new categories beyond the existing 8. Avoid: runtime permission concepts.
  </action>
  <verify>Read the file and confirm: {_} syntax shown, _/__/___ tiers explained, Ceiling/Grant documented, per-category enums listed, fully-filled rule stated, __Permission schema present</verify>
  <done>AC-2 satisfied: Complete {_} permission object system documented</done>
</task>

<task type="auto">
  <name>Task 2: Update blocks.md, packages.md, pipelines/permissions.md, identifiers.md</name>
  <files>docs/user/syntax/blocks.md, docs/user/syntax/packages.md, docs/user/concepts/pipelines/permissions.md, docs/user/syntax/identifiers.md</files>
  <action>
    **blocks.md:**
    - Add {_} to the definition block table alongside {#}, {=}, {M}, {W}, {Q}, {T}, {N}
    - Add description: "Permission object — named, reusable permission policy with Ceiling or Grant intent"
    - Update [_] block element entry to note it now references {_} objects by name

    **packages.md:**
    - Update the permission ceiling section (lines ~153-202)
    - Change syntax from inline [_] declarations to [_] referencing {_} ceiling objects
    - Show example: `{@} MyPackage` with `[_] _MyCeiling`
    - Keep PGE10001/PGE10002 compile rule references

    **pipelines/permissions.md:**
    - Update to show [_] referencing {_} grant objects
    - Document narrowing: pipeline can reference a {_} and use a subset
    - Keep the subset rule: pipeline [_] ⊆ package [_] ceiling
    - Keep the "no [_] = pure computation" rule

    **identifiers.md:**
    - Add _/__/___ to the prefix table
    - Show parallel with #/##/### tiers
    - _ = permission object, __ = permission descriptor, ___ = constraint descriptor

    Preserve existing wikilinks and cross-references. Update frontmatter dates.
    Avoid: changing any non-permission content in these files.
  </action>
  <verify>Read each file and confirm: {_} block type in blocks.md, {_} ceiling syntax in packages.md, {_} grant reference in pipelines/permissions.md, _/__/___ in identifiers.md</verify>
  <done>AC-1, AC-3, AC-4, AC-5 satisfied: All supporting docs updated for {_} objects</done>
</task>

</tasks>

<boundaries>

## DO NOT CHANGE
- docs/technical/ (technical spec updates are Plan 02)
- docs/technical/compile-rules/ (compile rule updates are Plan 03)
- docs/technical/ebnf/ (EBNF grammar updates are Plan 02)
- docs/user/pglib/ (pglib updates are Plan 03)
- .paul/ files other than STATE.md

## SCOPE LIMITS
- This plan covers user-facing documentation only
- No EBNF grammar changes (Plan 02)
- No compile rule updates (Plan 03)
- No metadata-tree changes (Plan 02)
- No pglib pipeline permission reference updates (Plan 03)

</boundaries>

<verification>
Before declaring plan complete:
- [ ] permissions.md contains {_} object syntax with examples
- [ ] _/__/___ tier system explained with parallel to #/##/###
- [ ] Ceiling vs Grant model documented with examples
- [ ] Per-category capability enums listed (8 categories)
- [ ] blocks.md has {_} entry
- [ ] packages.md shows {_} ceiling reference
- [ ] pipelines/permissions.md shows {_} grant reference
- [ ] identifiers.md has _/__/___ prefix entries
- [ ] All acceptance criteria met
</verification>

<success_criteria>
- All 5 files modified with consistent {_} object documentation
- No contradictions between files
- Existing wikilinks and cross-references preserved
- Frontmatter dates updated
</success_criteria>

<output>
After completion, create `.paul/phases/issue-140-permission-object-block/01-SUMMARY.md`
</output>
