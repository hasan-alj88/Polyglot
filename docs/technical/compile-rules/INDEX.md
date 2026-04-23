---
audience: developer
type: spec-index
updated: 2026-04-23
status: stable
---

<!-- @compile-rules/INDEX -->

# Polyglot Compile Rules

This directory defines every compile-time rule enforced by the Polyglot compiler. Rules are organized into error codes (PGE, halting), warning codes (PGW, non-halting), and supporting algorithms (detection strategies).

## Code Scheme

| Prefix | Severity | Effect |
|--------|----------|--------|
| `PGE`  | Error    | Halts compilation. The compiler refuses to emit an executable until the violating construct is fixed. |
| `PGW`  | Warning  | Does not halt compilation. Surfaced in diagnostics; may be elevated to error by project policy. |

Codes use the form `PG{E,W}<range><ordinal>` — `<range>` is a two-digit category bucket (thousand-range), `<ordinal>` is the three-digit position within that range. Category descriptions for each range appear in the sub-indexes below.

## Contents

| Subtree | Purpose | Files | Index |
|---------|---------|-------|-------|
| `PGE/` | Compile errors (halt compilation) | 187 | [[PGE/INDEX\|PGE Index]] |
| `PGW/` | Compile warnings (non-halting) | 30 | [[PGW/INDEX\|PGW Index]] |
| `algorithms/` | Shared detection strategies used by multiple rules | 3 | [[algorithms/INDEX\|Algorithms Index]] |
