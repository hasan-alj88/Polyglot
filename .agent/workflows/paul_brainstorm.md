---
name: paul:brainstorm
description: Brainstorm a topic — research, recommend, refine, then draft
argument-hint: "<topic or idea>"
allowed-tools: [Read, Write, Edit, Bash, Glob, Grep, WebSearch, WebFetch, Task, AskUserQuestion]
---

<objective>
Iterative brainstorming loop. User provides a topic with research points, Claude researches and presents recommendations in docs/draft.md, user refines through feedback, then accepted content flows to actions: GitHub issues, /paul:draft, or shelved.

**When to use:** When exploring ideas that need research, synthesis, and concrete follow-up actions. Combines research + synthesis + iterative refinement + action planning.

**Invoke with:** `/paul:brainstorm <topic or idea>`
</objective>

<execution_context>
@~/.claude/paul-framework/workflows/brainstorm.md
@~/.claude/paul-framework/workflows/draft-loop.md
</execution_context>

<context>
$ARGUMENTS (topic and optional research points)

@.paul/STATE.md
@.paul/PROJECT.md
@docs/draft.md
</context>

<process>

<step name="start">
1. Parse topic and research points from $ARGUMENTS
2. If no arguments: show usage and exit
3. Follow workflow: @~/.claude/paul-framework/workflows/brainstorm.md
   - Research the topic and stated points
   - Write recommendations to docs/draft.md
   - Present summary and wait for feedback
   - Loop: amend on feedback, accept to proceed, shelf to stop
   - On accept: extract actions (doc updates, code work, design decisions)
   - Present actions → user chooses: GitHub issues, /paul:draft, or shelf
</step>

</process>

<success_criteria>
- [ ] Topic and research points parsed
- [ ] Research executed (codebase, web, or both)
- [ ] Recommendations written to docs/draft.md
- [ ] User feedback loop completed (accept, amend, or shelf)
- [ ] Actions extracted and presented with types and targets
- [ ] User chose disposition: GitHub issues created, handed off to /paul:draft, or shelved
</success_criteria>
