---
name: paul:work-issue
description: Select issue, create branch, research context
argument-hint: "<issue-number>"
allowed-tools: [Read, Write, Bash, Glob, Grep, AskUserQuestion]
---

<objective>
Select a GitHub issue, create a feature branch, research the issue context, and prepare for planning.

**When to use:** After browsing issues with /paul:issues, or when you know which issue to work on.
</objective>

<execution_context>
@~/.claude/paul-framework/workflows/work-issue.md
</execution_context>

<context>
Issue number: $ARGUMENTS (required)

@.paul/STATE.md
@.paul/ROADMAP.md
</context>

<process>
Follow workflow: @~/.claude/paul-framework/workflows/work-issue.md
</process>

<success_criteria>
- [ ] Issue fetched and displayed
- [ ] Git branch created from main
- [ ] Codebase researched for related context
- [ ] STATE.md updated with Active Issue section
- [ ] User ready for /paul:plan or further discussion
</success_criteria>
