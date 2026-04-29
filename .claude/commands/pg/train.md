---
name: pg:train
description: Training loop — generate code, correct, infer mistakes, update docs, memorize
allowed-tools: [Read, Write, Edit, Glob, Grep, AskUserQuestion]
---

<objective>
Iterative training loop for the Aljam3 language agent. Generate Aljam3 Code, let the user correct it in docs/draft.md, infer what rules were violated, update spec docs, and memorize the lesson.

**Invoke with:** `/pg:train` to start or `/pg:train <prompt>` to generate code for a specific scenario.
</objective>

<context>
@docs/user/SPEC-INDEX.md
@docs/audit/reference/glossary.md
</context>

<process>

<step name="learn">
## Step 1: LEARN — Read the full spec + past lessons

1. Read `docs/user/SPEC-INDEX.md`
2. Read EVERY file listed in the index:
   - `docs/user/syntax/line-structure.md`
   - `docs/user/syntax/types.md`
   - `docs/user/syntax/blocks.md`
   - `docs/user/syntax/identifiers.md`
   - `docs/user/syntax/operators.md`
   - `docs/user/syntax/io.md`
   - `docs/user/syntax/comments.md`
   - `docs/user/syntax/packages.md`
   - `docs/user/concepts/variable-lifecycle.md`
   - `docs/user/concepts/collections.md`
3. Read `docs/audit/reference/glossary.md` for terminology
4. Glob for `memory/pg_lesson_*.md` files and read any that exist — these are lessons from previous training sessions
5. Internalize ALL rules before generating any code
</step>

<step name="generate">
## Step 2: GENERATE — Produce Aljam3 Code

1. If `$ARGUMENTS` provided, use it as the generation prompt
2. If no arguments, ask the user what to generate via AskUserQuestion
3. Generate Aljam3 Code following ALL spec rules:
   - 3-space indentation, one expression per line
   - `{@ }` package declaration as first block
   - Correct block elements (`[r]`, `[p]`, `[=]`, etc.)
   - Correct identifier prefixes (`$`, `#`, `=`, `@`, `!`)
   - Correct separator usage (`.` for fixed, `:` for flexible)
   - Correct assignment operators (`<<` `>>` `<~` `~>`)
   - Correct variable lifecycle (declared → default → final → released)
   - Apply all lessons from `memory/pg_lesson_*.md`
4. Save generated code internally (keep a copy for diffing later)
5. Write generated code to `docs/draft.md` with a header:
   ```
   ## Generated Aljam3 Code

   <!-- pg:train — edit corrections below, then tell Claude "done" -->

   ```aljam3
   (generated code here)
   ```
   ```
6. Display the code to the user and say:
   > "Code written to `docs/draft.md`. Edit your corrections there, then say **done**.
   > If the code is already correct, say **correct**."
7. Wait for user response.
</step>

<step name="correct">
## Step 3: CORRECT — Read user's corrections

1. If user says "correct" or "good" or "looks right" → skip to step 6 (LOOP)
2. Otherwise, read `docs/draft.md` to get the corrected code
3. Store the corrected version for comparison
</step>

<step name="infer">
## Step 4: INFER — Identify what went wrong

1. Compare the original generated code with the corrected version
2. For EACH difference, determine:
   - **What changed**: exact before/after
   - **Which rule was violated**: map to a specific spec file and section
   - **Root cause** — one of:
     - `spec_ambiguous` — the spec doc can be read multiple ways
     - `spec_incomplete` — the spec doc doesn't cover this case
     - `spec_correct_but_misread` — the spec is clear, agent made an error
     - `spec_wrong` — the spec itself has an error
3. Present the analysis to the user:
   > "Here's what I got wrong and why:"
   > - Change 1: [before] → [after] — Rule: [spec file, section] — Cause: [root cause]
   > - Change 2: ...
4. Ask user to confirm or adjust the analysis
</step>

<step name="update">
## Step 5a: UPDATE — Fix spec docs if needed

For each difference with root cause `spec_ambiguous`, `spec_incomplete`, or `spec_wrong`:

1. Read the relevant spec file
2. Draft the proposed edit (show old text → new text)
3. Present ALL proposed doc changes to the user:
   > "I'd like to update these spec files:"
   > - `docs/user/syntax/X.md`: [proposed change]
4. Wait for user approval
5. Apply approved edits to the spec files
6. If no spec changes needed (all `spec_correct_but_misread`), say so and skip to memorize
</step>

<step name="memorize">
## Step 5b: MEMORIZE — Save lesson to persistent memory

For each mistake identified:

1. Write a memory file:
   - Path: `memory/pg_lesson_{topic}.md` (where topic is a short slug like `indent_depth` or `operator_order`)
   - If a lesson file for this topic already exists, UPDATE it instead of creating a duplicate

2. Memory file format:
   ```markdown
   ---
   name: pg_lesson_{topic}
   description: {one-line description of the lesson}
   type: feedback
   ---

   **Rule:** {the correct rule, stated clearly}
   **Why:** {what the agent got wrong and why}
   **How to apply:** {concrete instruction for future code generation}
   **Spec file:** {which docs/user/ file governs this}
   **Spec updated:** {yes/no}
   ```

3. Update `memory/MEMORY.md` index with a pointer to the new lesson file

4. Check if 3+ lessons share a common pattern → if so, suggest adding a CARL rule:
   > "I've made this type of mistake 3+ times. Suggest adding CARL rule:
   > `ALJAM3_RULE_N=...`
   > Want me to add it to `~/.carl/aljam3`?"
</step>

<step name="loop">
## Step 6: LOOP — Continue or stop

Ask the user via AskUserQuestion:
- **[1] Another round** → go back to GENERATE step (lessons are now loaded)
- **[2] Done** → show session summary and exit

Session summary format:
```
## Training Session Summary

**Rounds:** N
**Lessons learned:** N
**Spec docs updated:** [list of files edited]
**Memory files created/updated:** [list]
**CARL rules suggested:** [list, if any]
```
</step>

</process>

<success_criteria>
- [ ] All spec files read before generating code
- [ ] All existing memory lessons loaded before generating
- [ ] Generated code written to docs/draft.md
- [ ] User corrections read from docs/draft.md
- [ ] Each difference mapped to a specific spec rule and root cause
- [ ] Spec docs updated when ambiguous/incomplete (with user approval)
- [ ] Lesson saved to memory after each correction round
- [ ] Recurring patterns flagged for CARL rule creation
</success_criteria>
