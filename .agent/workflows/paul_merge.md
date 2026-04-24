---
name: paul:merge
description: User-controlled merge of issue branch to main
allowed-tools: [Read, Write, Edit, Bash, AskUserQuestion]
argument-description: Optional branch name to merge (if omitted, uses active issue branch or prompts for selection)
---

<objective>
Merge a branch back to main with user-controlled conflict resolution. Optionally closes the associated GitHub issue and cleans up state.

**When to use:** After /paul:unify completes the PAUL loop on an issue branch, or to merge any branch to main.

**Usage:**
- `/paul:merge` — merge active issue branch (from STATE.md) or select from list
- `/paul:merge <branch>` — merge the specified branch
</objective>

<execution_context>
@~/.claude/paul-framework/workflows/issue-merge.md
</execution_context>

<context>
@.paul/STATE.md
</context>

<process>
Follow workflow: @~/.claude/paul-framework/workflows/issue-merge.md
</process>

<success_criteria>
- [ ] Branch resolved (from argument, STATE.md, or user selection)
- [ ] UNIFY confirmed complete before merge (if issue merge)
- [ ] User selected merge strategy
- [ ] All conflicts resolved with user input
- [ ] GitHub issue closed (if issue merge)
- [ ] Branch optionally deleted
- [ ] STATE.md Active Issue section cleared (if issue merge)
- [ ] Does NOT push to origin (user action)
</success_criteria>
