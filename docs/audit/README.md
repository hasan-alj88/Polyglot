---
type: audit-index
scope: all-documentation
updated: 2026-03-30
---

# Audit — Claude's Documentation Ground Truth

**Consumer:** Claude (AI). Read this folder BEFORE writing or editing any documentation.
**Scope:** All documentation Claude writes — docs/, .paul/, CLAUDE.md, READMEs, any text a human or AI will read.
**Compatibility:** docs/ is an Obsidian vault. All files use YAML frontmatter and [[wikilinks]].

## File Index

| File | What it tells Claude | When to read |
|------|---------------------|--------------|
| [[audiences/user\|audiences/user]] | Tone, format, assumptions for user-facing docs | Before writing user docs |
| [[audiences/developer\|audiences/developer]] | Tone, format, assumptions for developer docs | Before writing dev docs |
| [[audiences/ai\|audiences/ai]] | Tone, format, assumptions for AI-facing docs | Before writing AI docs |
| [[rules/conventions\|rules/conventions]] | Style, structure, formatting rules | Before writing any doc |
| [[rules/checklist\|rules/checklist]] | Pre-publish quality checks | Before finalizing any doc |
| [[reference/glossary\|reference/glossary]] | Authoritative term definitions | When using project terminology |
| [[tracking/coverage-gaps\|tracking/coverage-gaps]] | Structural documentation gaps (UG/TG/MX/OR) | When logging or resolving coverage gaps |
| [[tracking/inconsistencies\|tracking/inconsistencies]] | Contradictions between docs (IC) | When logging or fixing inconsistencies |
| [[tracking/progress\|tracking/progress]] | What areas have been audited | Before starting audit of an area |
| [[tracking/decisions\|tracking/decisions]] | Decisions made during auditing | When making or reviewing audit decisions |
| [[rules/workflows\|rules/workflows]] | Fix, Sweep, Gate workflows for doc quality | Before starting any doc audit or fix work |

## Authority Chain

| Priority | Source | Governs |
|----------|--------|---------|
| 1 (highest) | [[vision]] | Product vision, philosophy, what Polyglot is |
| 2 | docs/audit/ (this folder) | How to write documentation |
| 3 | .paul/ | Project management process |

**Rule:** If any document contradicts [[vision]], vision.md wins. Always.

## Dual Smart Referencing

Every cross-reference uses both syntaxes:

| Purpose | Claude @-import (mandatory) | Obsidian [[wikilink]] |
|---------|----------------------------|----------------------|
| Vision section | `<!-- @vision:Core Philosophy -->` | `[[vision#Core Philosophy]]` |
| Glossary term | `<!-- @glossary:Polyglot Service -->` | `[[glossary#Polyglot Service]]` |
| Convention rule | `<!-- @rule:heading-hierarchy -->` | `[[conventions#heading-hierarchy]]` |
| Checklist item | `<!-- @checklist:contradiction-check -->` | `[[checklist#contradiction-check]]` |

### How @-imports work

`@` references are **mandatory imports**. When Claude encounters `<!-- @vision:Core Philosophy -->`, Claude MUST:
1. Read the referenced section from the source file
2. Hold that content in context
3. Only then proceed with writing

`[[wikilinks]]` are Obsidian navigation — clickable links, graph view edges, backlink tracking.

### Reference format in files

```markdown
<!-- @glossary:Polyglot Service -->
The Polyglot Service ([[glossary#Polyglot Service]]) consists of three components...
```
