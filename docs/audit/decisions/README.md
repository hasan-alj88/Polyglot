---
audience: ai-finder
type: audit-reference
scope: decision-history
updated: 2026-04-22
---

# Decision History

## Purpose

This folder records design decisions with structured before/after impact tracking. Each record captures what changed, why, and what it affected — giving the project a traceable decision log beyond what git history alone provides.

Git history shows *what* changed. Decision records explain *why* and document the alternatives considered, trade-offs accepted, and downstream impact.

## When to Create a Record

Create a decision record when a change affects any of these:

- **Syntax** — new or removed operators, markers, block types, or grammar productions
- **Compiler rules** — new PGE/PGW codes, retired rules, changed severity
- **Compiler operations** — new compiler phases, changed compilation behavior
- **Type system** — new types, schemas, retired types, changed type relationships
- **Runtime** — changed execution model, new runtime constructs
- **Audience** — changed audience definitions, documentation routing
- **Process** — changed project workflows, tooling, conventions

PAUL workflow integration: the UNIFY and MERGE phases check STATE.md for `[DR]`-flagged decisions and prompt for records in this folder.

## File Naming

```text
YYYY-MM-DD-short-title.md
```

- Date is when the decision was made (not when the record was written)
- Short title uses lowercase kebab-case
- One decision per file

Examples:
- `2026-04-22-retire-chain-operator.md`
- `2026-04-22-constructor-blocks.md`
- `2026-03-12-documentation-first-approach.md`

## Record Template

```yaml
---
audience: ai-finder
type: audit-reference
scope: decision-record
category: syntax | compiler-rule | compiler-operation | type-system | runtime | audience | process
issue: "#NNN"
date: YYYY-MM-DD
updated: YYYY-MM-DD
---
```

```markdown
# Decision Title

## Summary

One-paragraph description of the decision and its outcome.

## Before

What existed before this decision. Describe the previous state: syntax, behavior, rules, or conventions that were in place.

## After

What exists now. Describe the new state: what replaced the old approach, new syntax, new rules.

## Impact

What changed as a result:
- Files modified (count and scope)
- Rules added/removed (PGE/PGW codes)
- EBNF productions affected
- Downstream effects on other systems

## Rationale

Why this decision was made. Include:
- The problem or tension that prompted it
- Alternatives considered (and why they were rejected)
- Trade-offs accepted

## Related

- Issue links (GitHub #NNN)
- Related decision records
- Affected documentation
```

## Categories

| Category | Scope |
|----------|-------|
| syntax | Operators, markers, block types, grammar |
| compiler-rule | PGE/PGW compile rules |
| compiler-operation | Compiler phases, compilation behavior |
| type-system | Types, schemas, type relationships |
| runtime | Execution model, runtime constructs |
| audience | Audience definitions, doc routing |
| process | Workflows, tooling, conventions |

## Guidelines

- Write records after the decision is implemented, not during deliberation
- Reference specific issue numbers and file paths
- Keep the Before/After sections concrete — show syntax or rules, not abstract descriptions
- The Rationale section is the most valuable part; invest effort there
- Link to related decisions when choices interact or build on each other
