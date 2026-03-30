---
type: spec-index
audience: developer
updated: 2026-03-30
---

# Polyglot Technical Documentation

<!-- @vision:Core Philosophy -->
<!-- @glossary:Polyglot Service -->
<!-- @audit/README -->

Internal specifications for Polyglot project contributors. These documents describe how Polyglot Code is parsed, validated, and processed.

## Specifications (docs/technical/)

| File | Covers |
|------|--------|
| [[ebnf/INDEX\|ebnf/]] | Complete formal grammar in Extended Backus-Naur Form (15 section files) |
| [[edge-cases/INDEX\|edge-cases/]] | 42+ edge cases organized by EBNF section for validation (24 section files) |
| COMPILE-RULES.md | Error/warning code lookup tables, legend, and rule format template |

## Formal Specifications (spec/)

| File | Covers |
|------|--------|
| spec/metadata-tree.md | Complete `%` metadata tree: path grammar, branches, instance rules, enum semantics, field expansion |
| spec/type-identity.md | Structural type matching rules (moved from compile-rules/) |

## Compiler Rules (compile-rules/)

Semantic constraints enforced at compile time — beyond what EBNF grammar captures. Each rule lives in its own file under `compile-rules/PGE/` (errors) or `compile-rules/PGW/` (warnings).

**Numbering convention:**
- Rules use `N.M` — category dot sequence (e.g., 3.5)
- Errors use `PGE-NMM` — category hundred + sequence (e.g., PGE-305)
- Warnings use `PGW-NMM` — same scheme, mirroring PGE ranges

**Categories:**

| Cat | Name | Code Range | Rules |
|-----|------|------------|-------|
| 1 | Execution Order & Structure | PGE-1xx | 1.1–1.4 |
| 2 | Variable Lifecycle | PGE-2xx, PGW-2xx | 2.1–2.7 |
| 3 | Parallel Execution | PGE-3xx, PGW-3xx | 3.1–3.6 |
| 4 | Type & Schema Rules | PGE-4xx | 4.1–4.4 |
| 5 | Identifier & Serialization | PGE-5xx | 5.1–5.2 |
| 6 | Conditional Exhaustiveness | PGE-6xx | 6.1–6.4 |
| 7 | Error Handling | PGE-7xx | 7.1 |
| 8 | Chain Execution | PGE-8xx, PGW-8xx | 8.1–8.2 |

## Issue Tracking

Compiler issues, design decisions, and project tracking are managed via [GitHub Issues](https://github.com/hasan-alj88/Polyglot/issues). Resolved design issues (001–030) are preserved in git history under `docs/technical/compiler_issues/`.

## Language Reference

See [[user/SPEC-INDEX|docs/user/SPEC-INDEX.md]] for the user-facing language reference (syntax, concepts, stdlib).
