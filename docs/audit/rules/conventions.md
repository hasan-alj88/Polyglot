---
audience: ai-finder
type: audit-rules
scope: writing-conventions
updated: 2026-04-22
---

# Writing Conventions

<!-- @c:vision:Core Philosophy -->
Authority: [[vision]]

## Structure Rules

| Rule ID | Rule | Detail |
|---------|------|--------|
| max-size | Max file size: 50KB | Split into parts if exceeded |
| frontmatter | YAML frontmatter required | audience, type, updated (minimum) |
| h1-title | H1 = document title | One per file, matches filename intent |
| heading-hierarchy | H2 sections, H3 subsections | No H4+. If you need H4, split the file |
| code-blocks | Language tag required | Always specify: \`\`\`polyglot, \`\`\`rust, etc. |
| obsidian-compat | Obsidian vault compatible | [[wikilinks]] for cross-refs, clean filenames |

## Style Rules

| Rule ID | Rule |
|---------|------|
| active-voice | Active voice, present tense |
| one-idea | One idea per paragraph |
| define-terms | Define terms on first use — link to [[glossary]] |
| no-marketing | No marketing language in specifications |
| naming | "Polyglot" = project/platform; "Polyglot code" / ".pg files" = the language |
| no-synonyms | Use [[glossary]] terms exactly. Don't invent synonyms |

## Cross-Reference Rules

| Rule ID | Rule | Detail |
|---------|------|--------|
| dual-refs | Use both @-imports (Claude) and [[wikilinks]] (Obsidian) | Every cross-reference needs both syntaxes |
| typed-refs | New docs must use `@c:`, `@u:`, or `@d:` — not bare `@` | `@c:` for concepts, `@u:` for syntax usage, `@d:` for deprecated content |
| usage-coverage | Syntax constructs appearing without explanation must have `@u:` to definition | Enables change propagation across docs |
| deprecated-refs | `@d:` marks superseded content with pointer to replacement | When archiving or deprecating docs, add `@d:` linking to current version |
| untyped-valid | Existing bare `@` refs valid until file is touched | Migrate to typed on touch |
| no-duplication | Never duplicate content — reference it | |
| relative-paths | Relative paths only within docs/ | |
| vision-authority | Link to [[vision]] for philosophy justifications | |
| glossary-authority | Link to [[glossary]] for term definitions | |

## Frontmatter Template

```yaml
---
audience: automation-builder | integrator | product | design | developer | ai-finder  # single value or YAML list
type: spec | guide | reference | tutorial | audit-rules
phase: NN (if part of a milestone phase)
updated: YYYY-MM-DD
---
```

Multi-audience documents use a YAML list. The first-listed audience drives the tone:

```yaml
audience: [automation-builder, integrator]  # tone follows automation-builder rules
```

## Document Type Definitions

| Type | Purpose | Structure |
|------|---------|-----------|
| tutorial | Step-by-step, learning-oriented. Progressive difficulty, clear learning goal, working result at end | Goal -> setup -> steps -> result -> next steps |
| guide | Task-oriented how-to. Solves a specific problem | Problem -> solution steps -> verification |
| reference | Lookup-oriented factual. Complete and precise | Definition -> syntax -> parameters -> examples |
| spec | Formal specification. Exhaustive and unambiguous | Problem -> design -> constraints -> behavior |
| audit-rules | Documentation quality standards (internal) | Rule -> rationale -> examples |
