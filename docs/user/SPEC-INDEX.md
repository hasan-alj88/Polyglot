---
type: spec-index
audience: user
updated: 2026-03-17
---

# Polyglot Language Reference

Read the files below to learn how to write Polyglot Code.

## Syntax (docs/user/syntax/)

| File | Covers |
|------|--------|
| line-structure.md | 3-space indentation, one expression per line |
| types.md | Type system, ; annotation, basic/user-defined |
| blocks.md | {X} definitions, [X] block elements, full registry |
| identifiers.md | Prefixes (@#=$!), . fixed / : flexible separators |
| operators.md | Assignment (<<, >>, <~, ~>), comparison, range |
| io.md | < input / > output parameters, IO line patterns |
| comments.md | [ ] and { } single-line, [ ]< multi-line |
| packages.md | {@ } declaration, address format, imports |

## Concepts (docs/user/concepts/)

| File | Covers |
|------|--------|
| variable-lifecycle.md | Declared → Default → Final → Released |
| collections.md | array, serial, ~ expand, * collect |
| pipelines.md | {=} mandatory structure: trigger, IO, queue, wrapper, execution |

## Reference (docs/user/)

| File | Covers |
|------|--------|
| STDLIB.md | Standard library pipeline reference (File.*, T.*, Q.*, W.*) |
| SCENARIOS.md | 500 real-world automation scenarios |

## For Contributors

See [[technical/INDEX|docs/technical/INDEX.md]] for internal specifications (EBNF grammar, edge cases).

## Adding New Spec Files

When a new spec file is created, add it to the appropriate table above.
The `/pg:*` commands read this index dynamically — no command files need changing.
