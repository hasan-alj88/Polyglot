---
audience: ai-finder
type: audit-index
scope: all-documentation
updated: 2026-04-22
---

# Audit — Claude's Documentation Ground Truth

**Consumer:** Claude (AI). Read this folder BEFORE writing or editing any documentation.
**Scope:** All documentation Claude writes — docs/, .paul/, CLAUDE.md, READMEs, any text a human or AI will read.
**Compatibility:** docs/ is an Obsidian vault. All files use YAML frontmatter and [[wikilinks]].

## File Index

| File | What it tells Claude | When to read |
|------|---------------------|--------------|
| [[audiences/automation-builder\|audiences/automation-builder]] | Tone, format, assumptions for .aj3 code writers (external) | Before writing automation-builder docs |
| [[audiences/integrator\|audiences/integrator]] | Tone, format, assumptions for SDK/API integrators (external) | Before writing integrator docs |
| [[audiences/product\|audiences/product]] | Tone, format, assumptions for product docs (internal) | Before writing product docs |
| [[audiences/design\|audiences/design]] | Tone, format, assumptions for language + architecture design docs (internal) | Before writing design docs |
| [[audiences/developer\|audiences/developer]] | Tone, format, assumptions for implementation docs (internal) | Before writing developer docs |
| [[audiences/ai-finder\|audiences/ai-finder]] | Tone, format, assumptions for AI discoverability docs (internal) | Before writing AI-finder docs |
| [[rules/conventions\|rules/conventions]] | Style, structure, formatting rules | Before writing any doc |
| [[rules/checklist\|rules/checklist]] | Pre-publish quality checks | Before finalizing any doc |
| [[reference/glossary\|reference/glossary]] | Authoritative term definitions | When using project terminology |
| [[tracking/coverage-gaps/INDEX\|tracking/coverage-gaps/INDEX]] | Structural documentation gaps (UG/TG/MX/OR) | When logging or resolving coverage gaps |
| [[tracking/inconsistencies/INDEX\|tracking/inconsistencies/INDEX]] | Contradictions between docs (IC) | When logging or fixing inconsistencies |
| [[tracking/progress\|tracking/progress]] | What areas have been audited | Before starting audit of an area |
| [[tracking/decisions\|tracking/decisions]] | Decisions made during auditing | When making or reviewing audit decisions |
| [[decisions/README\|decisions/]] | Decision history with before/after impact | When making or reviewing design decisions |
| [[tracking/audience-migration\|tracking/audience-migration]] | Audience system migration progress | When continuing audience restructure work |
| [[rules/workflows\|rules/workflows]] | Fix, Sweep, Gate workflows for doc quality | Before starting any doc audit or fix work |

## Authority Chain

| Priority | Source | Governs |
|----------|--------|---------|
| 1 (highest) | [[vision]] | Product vision, philosophy, what Aljam3 is |
| 2 | docs/audit/ (this folder) | How to write documentation |
| 3 | .paul/ | Project management process |

**Rule:** If any document contradicts [[vision]], vision.md wins. Always.

## Typed Smart Referencing

Every cross-reference carries a **type** that tells Claude *why* the import exists. This enables change propagation — when a concept changes, every doc that references it can be traced and updated.

### Reference Types

| Type | Prefix | Purpose | Points to |
|------|--------|---------|-----------|
| Concept | `@c:` | "What is this?" — load a definition before writing about it | Glossary, vision, design rationale, type definitions |
| Usage | `@u:` | "This construct appears here" — link to where it's defined | Syntax specs, EBNF grammar, aj3lib signatures, scenarios |
| Deprecated | `@d:` | "This is superseded" — mark content as deprecated with pointer to replacement | Archived docs, retired constructs, superseded designs |
| Untyped | `@` | Legacy form — valid but deprecated for new docs | Any (migrate to `@c:`, `@u:`, or `@d:` on touch) |

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
```aljam3
[?] $status
   =? "active" >> $result <~ "proceed"
   *?           >> $result <~ "skip"
```

**Deprecated references** (`@d:`) — content superseded, kept for history:

```markdown
<!-- @d:archive/old-pipeline-spec -->
This document described the original pipeline syntax. See [[pipelines|u:pipelines]] for the current spec.

<!-- @d:archive/old-type-system#enums -->
The enum approach described here was replaced by [[basic-types#enums|c:enums]].
```

### How @-imports work

`@c:` and `@u:` are **mandatory imports**. When Claude encounters either type, Claude MUST:
1. Read the referenced section from the source file
2. Hold that content in context
3. Only then proceed with writing

`@d:` is an **informational marker**, not a mandatory import. When Claude encounters `@d:`, Claude should:
1. Note that the referenced content is deprecated
2. Do NOT hold deprecated content in context or propagate its patterns
3. If writing about the topic, reference the current replacement instead

The type tag is **semantic** — it tells Claude the *purpose* of the import:
- `@c:` means "understand this concept before writing about it"
- `@u:` means "this construct is used here; if its definition changes, this doc needs updating"
- `@d:` means "this content is deprecated; note the deprecation but use the replacement instead"

Untyped `@` refs (legacy) behave the same as always — mandatory read-before-write. New docs must use `@c:`, `@u:`, or `@d:`.

`[[wikilinks]]` are Obsidian navigation — clickable links, graph view edges, backlink tracking. The display text may carry a `c:`, `u:`, or `d:` prefix (e.g., `[[glossary#Trigger Monitor|c:Trigger Monitor]]`) to signal the reference type to readers. Wikilink prefixes are added on touch, not batch-migrated.

### Reference format in files

```markdown
<!-- @c:glossary#Aljam3 Service -->
The Aljam3 Service ([[glossary#Aljam3 Service|c:Aljam3 Service]]) consists of three components...

<!-- @u:syntax/operators#comparison -->
The conditional uses comparison operators defined in the operators spec.
```
