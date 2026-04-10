---
audience: ai
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
| [[audiences/pg-coder\|audiences/pg-coder]] | Tone, format, assumptions for .pg code writers | Before writing pg-coder docs |
| [[audiences/integrator\|audiences/integrator]] | Tone, format, assumptions for SDK/API integrators | Before writing integrator docs |
| [[audiences/architect\|audiences/architect]] | Tone, format, assumptions for core architecture docs | Before writing architect docs |
| [[audiences/designer\|audiences/designer]] | Tone, format, assumptions for language design docs | Before writing designer docs |
| [[audiences/ai\|audiences/ai]] | Tone, format, assumptions for AI-facing docs | Before writing AI docs |
| [[rules/conventions\|rules/conventions]] | Style, structure, formatting rules | Before writing any doc |
| [[rules/checklist\|rules/checklist]] | Pre-publish quality checks | Before finalizing any doc |
| [[reference/glossary\|reference/glossary]] | Authoritative term definitions | When using project terminology |
| [[tracking/coverage-gaps\|tracking/coverage-gaps]] | Structural documentation gaps (UG/TG/MX/OR) | When logging or resolving coverage gaps |
| [[tracking/inconsistencies\|tracking/inconsistencies]] | Contradictions between docs (IC) | When logging or fixing inconsistencies |
| [[tracking/progress\|tracking/progress]] | What areas have been audited | Before starting audit of an area |
| [[tracking/decisions\|tracking/decisions]] | Decisions made during auditing | When making or reviewing audit decisions |
| [[tracking/audience-migration\|tracking/audience-migration]] | Audience system migration progress | When continuing audience restructure work |
| [[rules/workflows\|rules/workflows]] | Fix, Sweep, Gate workflows for doc quality | Before starting any doc audit or fix work |

## Authority Chain

| Priority | Source | Governs |
|----------|--------|---------|
| 1 (highest) | [[vision]] | Product vision, philosophy, what Polyglot is |
| 2 | docs/audit/ (this folder) | How to write documentation |
| 3 | .paul/ | Project management process |

**Rule:** If any document contradicts [[vision]], vision.md wins. Always.

## Typed Smart Referencing

Every cross-reference carries a **type** that tells Claude *why* the import exists. This enables change propagation — when a concept changes, every doc that references it can be traced and updated.

### Reference Types

| Type | Prefix | Purpose | Points to |
|------|--------|---------|-----------|
| Concept | `@c:` | "What is this?" — load a definition before writing about it | Glossary, vision, design rationale, type definitions |
| Usage | `@u:` | "This construct appears here" — link to where it's defined | Syntax specs, EBNF grammar, pglib signatures, scenarios |
| Untyped | `@` | Legacy form — valid but deprecated for new docs | Any (migrate to `@c:` or `@u:` on touch) |

### Examples

**Concept references** (`@c:`) — understand before writing:

```markdown
<!-- @c:glossary#Trigger Monitor -->
The Trigger Monitor ([[glossary#Trigger Monitor|c:Trigger Monitor]]) owns collector logic...

<!-- @c:vision#Core Philosophy -->
This design follows the core philosophy ([[vision#Core Philosophy|c:Core Philosophy]])...
```

**Usage references** (`@u:`) — syntax appears here, defined elsewhere:

```markdown
<!-- @u:syntax/blocks#trigger -->
The pipeline uses [T] to declare its trigger:

<!-- @u:technical/ebnf/conditional#match-syntax -->
```polyglot
[?] $status
   =? "active" >> $result <~ "proceed"
   *?           >> $result <~ "skip"
```

### How @-imports work

Both `@c:` and `@u:` are **mandatory imports**. When Claude encounters either type, Claude MUST:
1. Read the referenced section from the source file
2. Hold that content in context
3. Only then proceed with writing

The type tag is **semantic** — it tells Claude the *purpose* of the import:
- `@c:` means "understand this concept before writing about it"
- `@u:` means "this construct is used here; if its definition changes, this doc needs updating"

Untyped `@` refs (legacy) behave the same as always — mandatory read-before-write. New docs must use `@c:` or `@u:`.

`[[wikilinks]]` are Obsidian navigation — clickable links, graph view edges, backlink tracking. The display text may carry a `c:` or `u:` prefix (e.g., `[[glossary#Trigger Monitor|c:Trigger Monitor]]`) to signal the reference type to readers. Wikilink prefixes are added on touch, not batch-migrated.

### Reference format in files

```markdown
<!-- @c:glossary#Polyglot Service -->
The Polyglot Service ([[glossary#Polyglot Service|c:Polyglot Service]]) consists of three components...

<!-- @u:syntax/operators#comparison -->
The conditional uses comparison operators defined in the operators spec.
```
