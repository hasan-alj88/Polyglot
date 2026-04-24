---
name: paul:issues
description: Load open GitHub issues grouped by label with dependency-aware recommendations
argument-hint: "[--label <label>] [--recalc]"
allowed-tools: [Read, Write, Edit, Bash, AskUserQuestion]
---

<objective>
Fetch all open GitHub issues, group by type label, cross-reference against the issue resolution order, and recommend the next unblocked issue to work on.

**When to use:** Starting issue-driven work, browsing backlog, or deciding what to work on next.

**Resolution order:** The file `docs/audit/tracking/issue-resolution-order.md` defines wave-based execution order with dependency chains. This skill uses it to identify which issues are unblocked (all dependencies closed) and recommends the highest-priority unblocked issue.

**Recalculation:** Pass `--recalc` to re-fetch all open issues and update the resolution order document (e.g., after creating or closing issues).
</objective>

<execution_context>
@~/.claude/paul-framework/workflows/issue-dashboard.md
</execution_context>

<context>
$ARGUMENTS

@.paul/STATE.md
@docs/audit/tracking/issue-resolution-order.md
</context>

<process>
Follow workflow: @~/.claude/paul-framework/workflows/issue-dashboard.md
</process>

<success_criteria>
- [ ] All open issues fetched from GitHub
- [ ] Issues grouped by type label
- [ ] Priority sub-sorting applied within groups
- [ ] Resolution order cross-referenced — unblocked issues identified
- [ ] Recommended next issue displayed with rationale
- [ ] Dashboard displayed with counts per group
- [ ] User routed to /paul:work-issue <number>
- [ ] If --recalc: resolution order document updated
</success_criteria>
