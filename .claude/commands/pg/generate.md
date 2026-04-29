---
name: pg:generate
description: Generate valid Aljam3 code from a description
allowed-tools: [Read, Glob, Grep, AskUserQuestion]
---

<objective>
Generate valid Aljam3 Code from a natural language description. Reads the full language spec and all accumulated training lessons before generating.

**Invoke with:** `/pg:generate <description of what to build>`
</objective>

<context>
@docs/user/SPEC-INDEX.md
@docs/audit/reference/glossary.md
</context>

<process>

<step name="learn">
1. Read `docs/user/SPEC-INDEX.md`
2. Read EVERY spec file listed in the index (all syntax/* and concepts/*)
3. Read `docs/audit/reference/glossary.md` for terminology
4. Glob for `memory/pg_lesson_*.md` and read any that exist
5. Internalize ALL rules and lessons before generating
</step>

<step name="clarify">
1. Parse `$ARGUMENTS` for the user's intent
2. If ambiguous, ask clarifying questions via AskUserQuestion
3. Identify which Aljam3 constructs are needed (packages, pipelines, data definitions, triggers, collections, etc.)
</step>

<step name="generate">
1. Generate code following ALL spec rules:
   - 3-space indentation, one expression per line
   - `{@ }` package declaration as first block
   - Correct block elements (`[r]`, `[p]`, `[=]`, etc.)
   - Correct identifier prefixes (`$`, `#`, `=`, `@`, `!`)
   - Correct separator usage (`.` for fixed, `:` for flexible)
   - Correct assignment operators (`<<` `>>` `<~` `~>`)
   - Correct variable lifecycle (declared → default → final → released)
2. Apply all lessons from `memory/pg_lesson_*.md`
3. Include `[ ]` comments explaining each section
4. Output the code in a ```aljam3 fenced block
5. Explain key design decisions below the code
</step>

</process>

<success_criteria>
- [ ] All spec files read before generation
- [ ] All memory lessons loaded and applied
- [ ] Generated code uses correct line structure (3-space indent)
- [ ] Package declaration is first block
- [ ] All identifiers have correct prefixes
- [ ] Assignment operators match variable lifecycle stage
- [ ] Block elements match their context
- [ ] No invented syntax — only what the spec defines
</success_criteria>
