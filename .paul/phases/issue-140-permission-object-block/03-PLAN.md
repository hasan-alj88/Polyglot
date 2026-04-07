---
phase: issue-140-permission-object-block
plan: 03
type: execute
wave: 1
depends_on: ["02"]
files_modified:
  - docs/technical/compile-rules/PGE/PGE10003-unknown-permission-category.md
  - docs/technical/compile-rules/PGE/PGE10004-undeclared-permission.md
  - docs/technical/compile-rules/PGE/PGE10005-permission-output.md
  - docs/technical/compile-rules/PGE/PGE10006-duplicate-permission.md
  - docs/technical/compile-rules/PGW/PGW10001-unused-permission.md
  - docs/user/pglib/pipelines/DT.md
  - docs/user/pglib/pipelines/File.md
  - docs/user/pglib/pipelines/W.md
  - docs/user/pglib/pipelines/T.md
  - docs/user/pglib/pipelines/RT.md
  - docs/user/pglib/pipelines/Sys.md
  - docs/user/pglib/pipelines/#.md
autonomous: true
---

<objective>
## Goal
Update compile rules and pglib permission references to reflect the {_} named permission object system designed in Plans 01-02.

## Purpose
Plans 01-02 rewrote user-facing docs and technical spec (EBNF + metadata tree) for {_} objects. Compile rules and pglib files still use the old inline `[_] _Category.subfield` syntax. This plan brings them into alignment.

## Output
- 5 compile rule files rewritten for {_} object system
- 7 pglib pipeline files with updated permission sections
</objective>

<context>
## Prior Work
@.paul/phases/issue-140-permission-object-block/01-SUMMARY.md — user-facing docs (permissions.md, blocks.md, packages.md, identifiers.md, pipelines/permissions.md)
@.paul/phases/issue-140-permission-object-block/02-SUMMARY.md — EBNF grammar + metadata tree (§9.8, §3.1, §5, branches.md, path-grammar.md, object-types.md)

## Source Files
@docs/user/concepts/permissions.md — new {_} syntax reference (Plan 01 output)
@docs/technical/ebnf/09-definition-blocks.md — §9.8 {_} grammar (Plan 02 output)
@docs/technical/compile-rules/PGE/PGE10003-unknown-permission-category.md
@docs/technical/compile-rules/PGE/PGE10004-undeclared-permission.md
@docs/technical/compile-rules/PGE/PGE10005-permission-output.md
@docs/technical/compile-rules/PGE/PGE10006-duplicate-permission.md
@docs/technical/compile-rules/PGW/PGW10001-unused-permission.md
@docs/user/pglib/pipelines/DT.md
@docs/user/pglib/pipelines/File.md
@docs/user/pglib/pipelines/W.md
@docs/user/pglib/pipelines/T.md
@docs/user/pglib/pipelines/RT.md
@docs/user/pglib/pipelines/Sys.md
@docs/user/pglib/pipelines/#.md
</context>

<acceptance_criteria>

## AC-1: Compile rules use {_} named object syntax
```gherkin
Given compile rule files PGE10003, PGE10004, PGE10005, PGE10006, PGW10001
When reading each file
Then all statements, detection text, and code examples use {_} named permission objects (not inline [_] _Category.subfield declarations)
And all VALID/INVALID/WARNING examples use [_] _ObjectName (referencing {_} by name), {_} _Name blocks with [.] field lines, and {_} .intent << #Ceiling/#Grant
```

## AC-2: pglib permission tables use Category.Capability format
```gherkin
Given pglib files File.md, W.md, T.md, DT.md, Sys.md, RT.md, #.md
When reading each Permissions section
Then permission tables show required Category.Capability (e.g., File.Read, System.Env) — not old _Category.subfield format
And prose describes referencing named {_} objects (not inline declarations)
And the "Type" column (Inline/IO) is removed or replaced with "Category"
```

## AC-3: DT.md [_] lines removed from {N} definitions
```gherkin
Given DT.md with ~40 {N} native pipeline definitions
When reading each definition code block
Then [_] _None lines are removed entirely (pure computation needs no permission line)
And [_] _IO.Read on =DT.Now is removed (pglib {N} defs don't have [_] — they require capabilities implicitly)
And permission table correctly shows =DT.Now requires System.Env capability
```

## AC-4: RT.md example code uses named {_} objects
```gherkin
Given RT.md with 2 example code blocks containing {_} + [_]
When reading each example
Then {@} blocks reference named {_} objects via [_] _ObjectName
And a {_} _ObjectName definition block is shown with [.] .System.Process scope
```

</acceptance_criteria>

<tasks>

<task type="auto">
  <name>Task 1: Rewrite 5 compile rule files for {_} object system</name>
  <files>
    docs/technical/compile-rules/PGE/PGE10003-unknown-permission-category.md,
    docs/technical/compile-rules/PGE/PGE10004-undeclared-permission.md,
    docs/technical/compile-rules/PGE/PGE10005-permission-output.md,
    docs/technical/compile-rules/PGE/PGE10006-duplicate-permission.md,
    docs/technical/compile-rules/PGW/PGW10001-unused-permission.md
  </files>
  <action>
    Rewrite each compile rule to use {_} named permission objects:

    **PGE10003 — Unknown Permission Category:**
    - Statement: `{_}` field lines must use known category_name.capability_name from the 8 categories
    - Detection: compiler checks `[.] .Category.capability` lines in `{_}` blocks
    - Examples: show {_} blocks with valid/invalid category.capability field lines
    - Keep the 8 categories and their capabilities unchanged

    **PGE10004 — Undeclared Permission:**
    - Statement: pipeline calls IO pglib without matching `[_] _ObjectName` reference whose `{_}` object grants the needed capability
    - Detection: compiler traces call graph, checks referenced `{_}` objects cover all IO capabilities used
    - Examples: show {_} + {@} + {=} with [_] references, transitive enforcement

    **PGE10005 — Permission Output → rename to "Invalid Permission Block Marker":**
    - Old rule about `[_] >param` is obsolete (no IO direction in `[_]` anymore)
    - New rule: `{_}` blocks may only contain `[.]` field lines. Any other marker (`[r]`, `[=]`, `[p]`, etc.) inside `{_}` is an error.
    - Update rule name in frontmatter to "Invalid Permission Block Marker"
    - Examples: show valid {_} with only [.] lines vs invalid {_} with [r]/[=] lines

    **PGE10006 — Duplicate Permission:**
    - Statement covers TWO scopes:
      1. Same `[_] _ObjectName` referenced twice in one `{@}` or `{=}` scope
      2. Same `.Category.Capability` declared twice within one `{_}` block
    - Examples: show duplicate [_] references in {=}, and duplicate [.] field lines in {_}

    **PGW10001 — Unused Permission:**
    - Statement: `[_] _ObjectName` referenced in `{=}` but no IO call in call graph exercises any capability granted by that object
    - Examples: show {_} + {=} with unused reference

    For ALL rules:
    - Use permissions.md as syntax reference for {_} blocks
    - Keep frontmatter structure (rule number, code, name, severity)
    - Keep See also cross-references updated
    - Do NOT change rule numbers or PGE/PGW codes
    - Avoid: using old inline [_] _Category.subfield syntax anywhere
  </action>
  <verify>Grep all 5 files for old inline patterns like "_File.read\"" or "_Category.subfield" — should find zero matches</verify>
  <done>AC-1 satisfied: all compile rules use {_} named object syntax</done>
</task>

<task type="auto">
  <name>Task 2: Update 7 pglib files' permission sections</name>
  <files>
    docs/user/pglib/pipelines/DT.md,
    docs/user/pglib/pipelines/File.md,
    docs/user/pglib/pipelines/W.md,
    docs/user/pglib/pipelines/T.md,
    docs/user/pglib/pipelines/RT.md,
    docs/user/pglib/pipelines/Sys.md,
    docs/user/pglib/pipelines/#.md
  </files>
  <action>
    Update each pglib file's Permissions section:

    **All files — Permission table format change:**
    - Old: `| Pipeline | Permission | Type |` with `_File.read | Inline`
    - New: `| Pipeline | Required Capability | Category |` with `File.Read | File`
    - Drop the "Inline"/"IO" Type column (concept doesn't apply to {_} objects)
    - Use PascalCase capability names matching EBNF §9.8 category_name: File, Web, Database, System, Crypto, IPC, Device, Memory
    - Capabilities use PascalCase: Read, Write, Execute, Delete, Request, Socket, Connect, Env, Process, Signal, Key, Sign, Encrypt, Send, Receive, Subscribe, Camera, Microphone, Location, Bluetooth, Allocate, Shared

    **All files — Prose update:**
    - Replace "require `[_]` permission declarations" with "require a `{_}` permission object granting the listed capabilities"
    - Replace "require `[_] _Category.subfield`" with "require `{_}` object granting `Category.Capability`"
    - Keep [[permissions]] cross-references

    **DT.md — specific changes:**
    - Remove ALL `[_] _None` lines from {N} definition code blocks (~40 occurrences)
    - Remove `[_] _IO.Read` from =DT.Now definition
    - Fix permission table: =DT.Now requires System.Env (not _IO.Read — IO is not a category)
    - Update prose: pure computation pipelines need no permission reference, not "_None"

    **RT.md — specific changes:**
    - Two example code blocks have `{@} ... [_] _System.process`
    - Change to: add a `{_} _RuntimeProcess` block with `[.] .intent << #Ceiling` and `[.] .System.Process "*"`, then reference via `[_] _RuntimeProcess` in `{@}`

    **File.md, W.md, T.md, Sys.md — table-only changes:**
    - Just update permission tables and prose; no code block changes needed

    **#.md — minimal change:**
    - Already says no permissions needed; update wording to reference {_} objects

    Avoid:
    - Changing anything outside the Permissions section
    - Changing pipeline definitions, IO, errors, or descriptions
    - Adding {_} definition blocks to pglib files (these are reference docs, not example files)
    - Using old inline permission syntax
  </action>
  <verify>Grep all 7 files for "_None", "_IO.Read", "_File.read\"", "_System.process" (old inline patterns) — should find zero matches. Verify DT.md has no [_] lines in {N} definitions.</verify>
  <done>AC-2, AC-3, AC-4 satisfied: pglib files use new permission terminology and syntax</done>
</task>

</tasks>

<boundaries>

## DO NOT CHANGE
- docs/user/concepts/permissions.md (Plan 01 output — done)
- docs/technical/ebnf/ (Plan 02 output — done)
- docs/technical/spec/metadata-tree/ (Plan 02 output — done)
- COMPILE-RULES.md index (name change for PGE10005 will need updating but is minor)

## SCOPE LIMITS
- Do not update PGE10001/PGE10002 (planned rules — out of scope for this issue)
- Do not update PGE01024 open point about [_] marker compatibility (separate concern)
- Do not add new compile rules beyond updating existing ones
- Do not change pglib pipeline definitions, IO, errors, or descriptions — only Permissions sections

</boundaries>

<verification>
Before declaring plan complete:
- [ ] All 5 compile rule files use {_} named object syntax in statements + examples
- [ ] All 7 pglib files use Category.Capability format in permission tables
- [ ] DT.md has zero [_] lines inside {N} definition code blocks
- [ ] RT.md examples show {_} definition + [_] reference pattern
- [ ] Grep for old patterns (_File.read, _None, _IO.Read, _System.process inline) returns zero hits in modified files
- [ ] COMPILE-RULES.md index updated if PGE10005 name changed
</verification>

<success_criteria>
- All tasks completed
- All verification checks pass
- No old inline permission syntax remains in modified files
- Compile rules and pglib are fully aligned with {_} object system from Plans 01-02
</success_criteria>

<output>
After completion, create `.paul/phases/issue-140-permission-object-block/03-SUMMARY.md`
</output>
