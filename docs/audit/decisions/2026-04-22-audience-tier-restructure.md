---
audience: ai-finder
type: audit-reference
scope: decision-record
category: audience
date: 2026-04-22
updated: 2026-04-22
---

# Audience Tier Restructure

## Summary

Aljam3's documentation audience system was restructured from a flat three-tier model (user/developer/ai) into a two-group, five-audience model: two external audiences (automation-builder, integrator) and three internal audiences (architect, designer, ai). This gives each audience a distinct tone, scope, and set of assumptions.

**Revision (2026-04-22):** Further restructured to support Agile development workflow — see #353. architect + designer merged into "design"; "ai" renamed to "ai-finder"; "product" added; "developer" added.

## Before (original)

Three audiences with broad, overlapping scope:

- **user** — anyone writing or reading Aljam3 code
- **developer** — anyone building Aljam3 itself
- **ai** — Claude and other AI agents consuming documentation

The "user" audience conflated two very different needs: developers writing `.aj3` files and developers integrating existing codebases via SDKs. The "developer" audience covered both architecture decisions and implementation details. "pg-coder" was used inconsistently as an alias for "user."

## After (first restructure)

Five audiences in two groups:

**External** (how to *use* Aljam3):

- **automation-builder** — developers who write `.aj3` files (renamed from pg-coder/user)
- **integrator** — developers who connect existing codebases via SDKs/APIs without writing `.aj3` code

**Internal** (how to *build* Aljam3):

- **architect** — core architecture, system design, infrastructure decisions
- **designer** — language spec, philosophy, syntax design, type system decisions
- **ai** — AI-facing documentation, indexes, retrieval metadata

## After (second restructure — #353)

Six audiences in two groups:

**External** (unchanged):

- **automation-builder** — developers who write `.aj3` files
- **integrator** — developers who connect existing codebases via SDKs/APIs

**Internal** (revised):

- **product** — PRDs, user stories, acceptance criteria, roadmap
- **design** — language spec, philosophy, architecture, design decisions (merges former architect + designer)
- **developer** — implementation, compile rules, pglib, tests
- **ai-finder** — discoverability layer: indexes, summaries, retrieval metadata for AI navigation

Each audience has a dedicated file in `docs/audit/audiences/` with tone rules, format assumptions, and content guidelines.

## Impact

- 5 audience files created/updated in `docs/audit/audiences/`
- `docs/audit/rules/conventions.md` frontmatter template updated
- `docs/audit/rules/checklist.md` audience checks reference all 5 audiences
- 218 files received audience frontmatter updates (issue #138)
- 3 index files split "For Contributors" into Designer/Architect sections (issue #159)

## Rationale

The original three audiences were too broad to drive useful writing rules:

- An **automation-builder** needs task-oriented guides with working examples — they want to solve problems, not understand internals
- An **integrator** needs SDK references and API contracts — they may never see `.aj3` syntax
- These two groups have fundamentally different vocabulary, assumptions, and success criteria

Splitting "developer" into architect and designer separates "how the system works" (architecture) from "how the language is designed" (language design). These require different levels of abstraction and different readers.

Alternatives considered:

- **Keep three audiences, add sub-categories** — rejected because sub-categories within a single audience file would bloat each file and make tone rules ambiguous
- **Six audiences with separate "product" role** — considered but deferred; product-level docs (PRDs, user stories) don't exist yet in the documentation set

## Related

- GitHub: #138 (audience frontmatter), #159 (audience routing fix)
- `docs/audit/audiences/` — individual audience definition files
