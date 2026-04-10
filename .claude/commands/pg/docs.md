---
name: pg:docs
description: Generate or update the Polyglot documentation PDF
argument-hint: "[section | --by-audience | --audience=NAME]"
allowed-tools: [Bash, Read]
---

<objective>
Generate Polyglot documentation PDFs from all markdown files in docs/.

**When to use:** After editing docs, adding new pages, or when you need fresh PDFs.
</objective>

<process>
1. Run the generation script based on arguments:
   - No argument: `./scripts/generate-docs-pdf.sh` (full monolithic book)
   - Section filter: `./scripts/generate-docs-pdf.sh docs/$ARGUMENTS`
   - `--by-audience`: `./scripts/generate-docs-pdf.sh --by-audience` (one PDF per audience in docs/pdf/)
   - `--audience=NAME`: `./scripts/generate-docs-pdf.sh --audience=NAME` (single audience PDF)
2. Report the result: file count, size, and any failures
3. If there are failures, read the `.docs-build/book.typ` file to diagnose
</process>

<context>
Script: scripts/generate-docs-pdf.sh
Template: scripts/doc-template.typ

Outputs:
- Monolithic: Polyglot-Documentation.pdf (repo root)
- Per-audience: docs/pdf/{audience}.pdf (pg-coder, developer, designer, architect, integrator)

The script:
- Combines docs/ markdown into PDFs via Pandoc + Typst
- Orders content by canonical section (Vision → Syntax → Concepts → pglib → etc.)
- Includes title page, table of contents, part dividers
- Strips YAML frontmatter, HTML comments, and deduplicates Typst labels
- Per-audience mode filters files by frontmatter `audience:` field
- Files without audience frontmatter are guessed by directory path
- `ai` audience and `docs/archive/` are excluded from per-audience PDFs
</context>
