# Context: Issue #309 — EBNF Token Ambiguity and Cross-Production IO Validation

## Goals

1. **X.45 — Accept + document lexer disambiguation rule**: The `?[` range token and `[?]` conditional block element never collide in practice. Document the rule explicitly: `[?]` is a three-character block element at line start; `?[`/`?(` are two-character range tokens in expression context; comparison operators consume `?` greedily (`=?[` → `=?` + `[`).

2. **X.46 — Accept, existing type system covers it**: Reassemble IO signature mismatch is already handled by the schema system. The expander input `<Collection.{Array|Serial|Dataframe}` is typed to a specific `##` schema. Schema mismatch on input is caught by standard type checking. Per-iteration output shape derives from the collection schema's `%##Active << #ActiveKind.One` property, and wiring to the collector is standard type checking. No new PGE rule needed.

## Decisions Made

- **X.45**: Accept — document lexer disambiguation rule in EBNF §2 (lexical) or §6 (operators)
- **X.46**: Accept — PGE04001 (type mismatch) covers reassemble IO incompatibility via desugar + typed IO wiring. The IO signature table in §12.3 is documentation of what schemas already enforce.

## Approach

- Add edge case entries (EC-6.6 for X.45, EC-12.17+ for X.46) with Accept decisions
- Add lexer disambiguation note to EBNF §2 or §6 (where `range_open` is defined)
- Add clarifying note to §12.3 reassemble section explaining that IO compatibility is schema-enforced, not a separate validation layer
- Update edge case INDEX.md coverage matrix

## Key Files

- `docs/technical/ebnf/02-lexical.md` — lexer rules (disambiguation note)
- `docs/technical/ebnf/06-operators.md` — range_open production
- `docs/technical/ebnf/12-collections.md` — reassemble IO signature table
- `docs/technical/edge-cases/06-operators.md` — X.45 edge case entry
- `docs/technical/edge-cases/12-collections.md` — X.46 edge case entry
- `docs/technical/edge-cases/INDEX.md` — coverage updates

## Open Questions

None — both decisions are Accept with documentation.

---
*Created: 2026-04-17 from /paul:discuss*
