---
audience: designer
type: spec
updated: 2026-03-30
status: draft
---

<!-- @ebnf/INDEX -->

# Polyglot Code — EBNF Grammar

<!-- @line-structure -->
<!-- @blocks -->
<!-- @identifiers -->
<!-- @types -->
<!-- @operators -->
<!-- @io -->
<!-- @packages -->
<!-- @comments -->
<!-- @pipelines -->
<!-- @collections -->
<!-- @variable-lifecycle -->
This document defines the complete formal grammar for Polyglot Code (`.pg` files) using Extended Backus-Naur Form (EBNF). Each section maps to spec files: [[line-structure]], [[blocks]], [[identifiers]], [[syntax/types/INDEX|types]], [[operators]], [[io]], [[packages]], [[comments]], [[concepts/pipelines/INDEX|pipelines]], [[concepts/collections/INDEX|collections]], [[variable-lifecycle]]. For edge case testing, see [[technical/edge-cases/INDEX|EDGE-CASES]].

## Notation Conventions

| Symbol | Meaning |
|--------|---------|
| `::=` | Definition |
| `\|` | Alternative |
| `[ ... ]` | Optional (0 or 1) |
| `{ ... }` | Repetition (0 or more) |
| `( ... )` | Grouping |
| `" ... "` | Terminal string literal |
| `'...'` | Terminal character literal |
| `(* ... *)` | EBNF comment |

---

## Sections

| # | File | Covers |
|---|------|--------|
| 1 | [[01-file-structure]] | File structure |
| 2 | [[02-lexical]] | Lexical elements |
| 3 | [[03-identifiers]] | Identifiers |
| 4 | [[04-type-system]] | Type system |
| 5 | [[05-block-elements]] | Block elements |
| 6 | [[06-operators]] | Operators |
| 7 | [[07-io-parameters]] | IO parameters |
| 8 | [[08-expressions]] | Expressions |
| 9 | [[09-definition-blocks]] | Definition blocks ({=}, {#}, {T}, {W}, {N}, {Q}, {!}, {_}, {@}) |
| 10 | [[10-execution]] | Execution statements |
| 11 | [[11-control-flow]] | Control flow |
| 12 | [[12-collections]] | Collection operations |
| 13 | [[13-comments]] | Comments |
| 14 | [[14-lifecycle]] | Variable lifecycle constraints |
| 15 | [[15-example]] | Complete file example |
