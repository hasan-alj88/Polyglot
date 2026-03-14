---
type: audit-rules
scope: writing-conventions
updated: 2026-03-14
---

# Writing Conventions

<!-- @vision:Core Philosophy -->
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

| Rule ID | Rule |
|---------|------|
| dual-refs | Use both @-imports (Claude) and [[wikilinks]] (Obsidian) |
| no-duplication | Never duplicate content — reference it |
| relative-paths | Relative paths only within docs/ |
| vision-authority | Link to [[vision]] for philosophy justifications |
| glossary-authority | Link to [[glossary]] for term definitions |

## Frontmatter Template

```yaml
---
audience: user | developer | ai
type: spec | guide | reference | audit-rules
phase: NN (if part of a milestone phase)
updated: YYYY-MM-DD
---
```
