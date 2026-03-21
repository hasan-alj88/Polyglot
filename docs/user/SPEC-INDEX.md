---
type: spec-index
audience: user
updated: 2026-03-21
---

# Polyglot Language Reference

Read the files below in order to learn how to write Polyglot Code. The progression builds from syntax fundamentals through core concepts to the unified tree that connects everything.

## Phase 1: Syntax Foundations

| # | File | Covers |
|---|------|--------|
| 1 | line-structure.md | 3-space indentation, one expression per line |
| 2 | comments.md | [ ] and { } single-line, [ ]< multi-line |
| 3 | identifiers.md | Prefixes (@#=$!%), . fixed / : flexible separators |
| 4 | blocks.md | {X} definitions, [X] block elements, full registry |
| 5 | types.md | Type system, RawString, #String, structs, enums |
| 6 | operators.md | Assignment (<<, >>, <~, ~>), comparison, range |
| 7 | io.md | < input / > output parameters, IO line patterns |

## Phase 2: Core Concepts

| # | File | Covers |
|---|------|--------|
| 8 | variable-lifecycle.md | Declared → Default → Final → Failed → Released |
| 9 | collections.md | array, serial, ~ expand, * collect |
| 10 | pipelines.md | {=} mandatory structure: trigger, IO, queue, wrapper, execution |
| 11 | errors.md | Error model, scoping, chain addressing, recovery |

## Phase 3: The Big Picture

| # | File | Covers |
|---|------|--------|
| 12 | data-is-trees.md | Everything is a tree — how all concepts connect via `%` |
| 13 | metadata.md | Full `%` tree field listings, live fields, access patterns |

## Phase 4: Packages & Standard Library

| # | File | Covers |
|---|------|--------|
| 14 | packages.md | {@ } declaration, address format, imports |
| 15 | stdlib/INDEX.md | Namespace registry → per-namespace reference files |

## Phase 5: Practice

| File | Covers |
|------|--------|
| SCENARIOS.md | 500 real-world automation scenarios |

## For Contributors

See [[technical/INDEX|docs/technical/INDEX.md]] for internal specifications (EBNF grammar, edge cases, compiler rules, metadata tree spec).

## Adding New Spec Files

When a new spec file is created, add it to the appropriate phase table above.
The `/pg:*` commands read this index dynamically — no command files need changing.
