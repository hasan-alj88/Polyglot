---
type: audit-rules
scope: quality-checklist
updated: 2026-03-14
---

# Documentation Quality Checklist

Run before finalizing any new or modified documentation.

## Contradiction Check

<!-- @vision:Core Philosophy -->
| ID | Check | Source |
|----|-------|--------|
| contradiction-check | Does not contradict [[vision]] (two pillars, integration evolution) | [[vision]] |
| glossary-check | Uses [[glossary]] terms correctly — no redefinitions | [[glossary]] |
| philosophy-check | Aligns with [[vision#Core Philosophy]] principles | [[vision#Core Philosophy]] |

## Structure Check

<!-- @rule:frontmatter -->
<!-- @rule:heading-hierarchy -->
| ID | Check | Source |
|----|-------|--------|
| frontmatter-check | Has YAML frontmatter (audience, type, updated) | [[conventions#frontmatter]] |
| size-check | Under 50KB | [[conventions#max-size]] |
| heading-check | No H4+ headings | [[conventions#heading-hierarchy]] |
| code-check | Code blocks have language tags | [[conventions#code-blocks]] |
| obsidian-check | [[wikilinks]] used for cross-refs, clean filenames | [[conventions#obsidian-compat]] |

## Audience Check

| ID | Check | Source |
|----|-------|--------|
| audience-declared | Single audience declared in frontmatter | [[conventions#frontmatter]] |
| tone-match | Tone matches audience rules | [[audiences/user]] or [[audiences/developer]] or [[audiences/ai]] |
| no-leak | Does not leak cross-audience concerns | Audience files |

## Completeness Check

| ID | Check |
|----|-------|
| no-stubs | No TODO/TBD stubs without a tracking reference |
| valid-links | All [[wikilinks]] resolve to existing files |
| examples-valid | Examples compile/run (or clearly marked pseudocode) |
| dual-refs | @-imports paired with [[wikilinks]] |
