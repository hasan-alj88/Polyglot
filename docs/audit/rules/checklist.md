---
audience: ai
type: audit-rules
scope: quality-checklist
updated: 2026-04-10
---

# Documentation Quality Checklist

Run before finalizing any new or modified documentation.

## Contradiction Check

<!-- @c:vision:Core Philosophy -->
| ID | Check | Source |
|----|-------|--------|
| contradiction-check | Does not contradict [[vision]] (two pillars, integration evolution) | [[vision]] |
| glossary-check | Uses [[glossary]] terms correctly — no redefinitions | [[glossary]] |
| philosophy-check | Aligns with [[vision#Core Philosophy]] principles | [[vision#Core Philosophy]] |

## Structure Check

<!-- @c:rule:frontmatter -->
<!-- @c:rule:heading-hierarchy -->
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
| audience-declared | One or more audiences from allowed set declared in frontmatter | [[conventions#frontmatter]] |
| tone-match | Tone matches first-listed audience rules | [[audiences/pg-coder]] \| [[audiences/integrator]] \| [[audiences/architect]] \| [[audiences/designer]] \| [[audiences/ai]] |
| no-leak | Does not expose concerns outside declared audience set | Audience files |

## Completeness Check

| ID | Check |
|----|-------|
| no-stubs | No TODO/TBD stubs without a tracking reference |
| valid-links | All [[wikilinks]] resolve to existing files |
| examples-valid | Examples compile/run (or clearly marked pseudocode) |
| dual-refs | @-imports paired with [[wikilinks]] |

## Cross-Reference Check

| ID | Check | Source |
|----|-------|--------|
| typed-ref-check | New @-imports use `@c:`, `@u:`, or `@d:` type prefix | [[conventions#typed-refs]] |
| usage-coverage-check | Syntax constructs used without explanation have `@u:` back-reference | [[conventions#usage-coverage]] |
| deprecated-target-check | `@d:` refs point to a valid replacement doc (the current version) | [[conventions#deprecated-refs]] |
