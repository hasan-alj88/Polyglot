---
audience: developer
type: spec-index
updated: 2026-04-23
status: stable
---

<!-- @compiler/INDEX -->

# Aljam3 Compiler

Compiler internals — registries consulted during semantic analysis, foreign-code parsers, and the compliance-test matrix. These documents are referenced by compile rules (see [[../compile-rules/INDEX|Compile Rules]]) and by runtime specs.

## Contents

| File | Purpose |
|------|---------|
| [[io-registry\|IO Registry]] | Sink tables defining recognized IO sources (env vars, file reads, network calls) used by the compiler to validate foreign-code permission flows. |
| [[ast-invisible-registry\|AST-Invisible Functions Registry]] | Language-specific eval/exec-class functions banned by [[../compile-rules/PGE/PGE10014-ast-invisible-foreign-code\|PGE10014]] because AST analysis cannot reason about their runtime behavior. |
| [[foreign-code-parsers\|Foreign Code Parsers]] | tree-sitter-based parsers the compiler uses to read foreign-language blocks (`{C}`). |
| [[compliance-report\|Compliance Report]] | Canonical matrix showing which compile rules are implemented, planned, or deferred. |
