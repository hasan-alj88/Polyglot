---
name: pg:docs
description: Generate or update the Polyglot documentation PDF
argument-hint: "[section]"
allowed-tools: [Bash, Read]
---

<objective>
Generate the combined Polyglot documentation PDF from all markdown files in docs/.

**When to use:** After editing docs, adding new pages, or when you need a fresh PDF.
</objective>

<process>
1. Run the generation script:
   - No argument: `./scripts/generate-docs-pdf.sh` (full book)
   - With argument like "user" or "technical": `./scripts/generate-docs-pdf.sh docs/$ARGUMENTS`
2. Report the result: file count, size, and any failures
3. If there are failures, read the `.docs-build/book.typ` file to diagnose
</process>

<context>
Script: scripts/generate-docs-pdf.sh
Template: scripts/doc-template.typ
Output: Polyglot-Documentation.pdf (repo root)

The script:
- Combines all docs/ markdown into one PDF via Pandoc + Typst
- Orders content by canonical section (Vision → Syntax → Concepts → Stdlib → etc.)
- Includes title page, table of contents, part dividers
- Strips YAML frontmatter, HTML comments, and deduplicates Typst labels
</context>
