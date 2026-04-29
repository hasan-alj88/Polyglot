---
name: pg:pdfdocs
description: Generate all documentation PDFs (monolithic + per-audience)
allowed-tools: [Bash, Read]
---

<objective>
Generate all Aljam3 documentation PDFs: the monolithic book and all per-audience editions.

**When to use:** After editing docs, to regenerate everything in one go.
</objective>

<process>
1. Generate the monolithic PDF:
   `./scripts/generate-docs-pdf.sh`
2. Generate all per-audience PDFs:
   `./scripts/generate-docs-pdf.sh --by-audience`
3. Report results for both: file counts, sizes, and any failures
4. If there are failures, read the `.docs-build/book.typ` file to diagnose
</process>

<context>
Script: scripts/generate-docs-pdf.sh
Template: scripts/doc-template.typ

Outputs:
- Monolithic: Aljam3-Documentation.pdf (repo root)
- Per-audience: docs/pdf/{audience}.pdf (pg-coder, developer, designer, architect, integrator)

Audiences excluded: ai (machine-facing)
Directories excluded: docs/archive/ (deprecated)
</context>