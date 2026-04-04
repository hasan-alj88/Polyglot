---
type: spec-index
audience: pg-coder
updated: 2026-03-30
---

# Polyglot Language Reference

<!-- @vision:Core Philosophy -->
<!-- @glossary:Polyglot Code -->
<!-- @audit/README -->

Read the files below in order to learn how to write Polyglot Code. The progression builds from syntax fundamentals through core concepts to the unified tree that connects everything.

```mermaid
flowchart LR
    P1["Syntax Foundations\n(7 files)"]
    P2["Core Concepts\n(5 files)"]
    P3["The Big Picture\n(2 files)"]
    P4["Packages & Stdlib\n(2 files)"]
    P5["Practice"]

    P1 --> P2 --> P3 --> P4 --> P5
```

## Phase 1: Syntax Foundations

| # | File | Covers |
|---|------|--------|
| 1 | line-structure.md | 3-space indentation, one expression per line |
| 2 | comments.md | [ ] and { } single-line, [ ]< multi-line |
| 3 | identifiers.md | Prefixes (@#=$!%), . fixed / : flexible separators |
| 4 | blocks.md | {X} definitions, [X] block elements, full registry |
| 5 | types/INDEX.md | Type system, RawString, #String, structs, enums (read INDEX then sub-pages) |
| 6 | operators.md | Assignment (<<, >>, <~, ~>), comparison, negation, range, arithmetic |
| 7 | io.md | < input / > output parameters, IO line patterns |

## Phase 2: Core Concepts

| # | File | Covers |
|---|------|--------|
| 8 | variable-lifecycle.md | Declared → Default → Final → Failed → Released |
| 9 | collections/INDEX.md | array, serial, ~ expand, * collect (read INDEX then sub-pages) |
| 10 | conditionals.md | [?] chains, exhaustiveness, logical operators, nesting |
| 11 | pipelines/INDEX.md | {=} mandatory structure: trigger, IO, queue, wrapper, execution (read INDEX then sub-pages) |
| 12 | errors.md | Error model, scoping, chain addressing, recovery |

## Phase 3: The Big Picture

| # | File | Covers |
|---|------|--------|
| 13 | data-is-trees.md | Everything is a tree — how all concepts connect via `%` |
| 14 | metadata.md | Full `%` tree field listings, live fields, access patterns |

## Phase 4: Packages & Standard Library

| # | File | Covers |
|---|------|--------|
| 15 | packages.md | {@ } declaration, address format, imports |
| 16 | stdlib/INDEX.md | Namespace registry → per-namespace reference files |

## Phase 5: Practice

| File | Covers |
|------|--------|
| scenarios/INDEX.md | 500 real-world automation scenarios (6 thematic files) |

## For Contributors

See [[technical/INDEX|docs/technical/INDEX.md]] for internal specifications (EBNF grammar, edge cases, compiler rules, metadata tree spec).

## Adding New Spec Files

When a new spec file is created, add it to the appropriate phase table above.
The `/pg:*` commands read this index dynamically — no command files need changing.
