---
name: paul:draft
description: Iterative documentation loop — write in draft.md, review, place
allowed-tools: [Read, Write, Edit, Bash, Glob, Grep, AskUserQuestion]
---

<objective>
Iterative documentation authoring loop. User writes in docs/draft.md, Claude reviews for gaps/contradictions, appends questions to draft.md for inline answers, then places content into the correct files.

**Invoke with:** `/paul:draft` to start or continue the loop.
</objective>

<execution_context>
@~/.claude/paul-framework/workflows/draft-loop.md
</execution_context>

<context>
@docs/draft.md
@docs/audit/README.md
</context>

<process>

<step name="start_or_continue">
1. Read docs/draft.md
2. If draft has user content (not just the template):
   - Go to "review" step — user already wrote something
3. If draft is empty/template:
   - Clear docs/draft.md to fresh pad
   - Announce "Draft pad ready" and wait for user
</step>

<step name="review">
Follow workflow: @~/.claude/paul-framework/workflows/draft-loop.md

1. Read docs/draft.md
2. If draft has `## Review` section with answered questions → proceed to place
3. Otherwise: parse content — identify topic, type, target location
4. Search existing docs for contradictions, gaps, redundancy
5. Append `## Review` section with findings and questions to docs/draft.md
6. Wait for user to answer inline, then say "review" again
7. Place content into correct documentation files (with audit compliance)
8. Ask: [1] Next topic | [2] Done
   - "1" / "next" / "continue" → clear draft, loop back
   - "2" / "done" / "stop" → show session summary, exit
</step>

</process>

<success_criteria>
- [ ] Draft content reviewed for contradictions
- [ ] Clarifying questions appended to draft.md and answered inline
- [ ] Content placed in correct documentation files
- [ ] Draft cleared for next iteration (or session ended)
</success_criteria>
