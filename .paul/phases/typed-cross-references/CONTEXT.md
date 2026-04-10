# Phase Context: Typed Cross-References

## Vision

Add semantic types to the cross-referencing system so every `@` import carries context about *why* it's referenced. This turns cross-references into change propagation infrastructure — when a concept changes, every doc that uses it can be traced and updated.

## Goals

1. **Typed references** — Two types replace the current flat `@` imports:
   - `@c:` (concept) — "what is X?" Points to definitions, theory, design rationale
   - `@u:` (usage) — "X appears here, defined elsewhere." Points to syntax specs, EBNF, pglib signatures, scenario examples

2. **Full syntax coverage** — Every syntax construct that appears in any doc (prose or code blocks) without explanation must have a `@u:` back-reference to where that construct is defined. No implicit, unlinked syntax appearances.

3. **Change propagation** — When a concept or syntax definition changes, all docs referencing it via `@c:` or `@u:` can be identified and updated systematically.

## Approach

- Both `@c:` and `@u:` are mandatory read-before-write imports for Claude (semantic tag, not behavioral difference)
- References stay as HTML comments (`<!-- @c:... -->`, `<!-- @u:... -->`) — invisible in rendered docs
- Untyped `@` remains valid during migration (backward compat for existing 409 refs)
- Wikilink display prefixes (`c:`, `u:`) defined in protocol but deferred to migrate-on-touch
- Three-plan structure:
  1. Define the protocol (audit rules)
  2. Classify existing refs + identify coverage gaps (user checkpoint)
  3. Migrate existing refs + add missing `@u:` coverage

## Constraints

- Nearly all 595 docs are specs/references — no guides/tutorials exist
- Usage refs (`@u:`) point to existing spec content (EBNF, syntax, scenarios, pglib), not hypothetical future docs
- Batch migration preferred over indefinite dual-convention coexistence

## Open Questions (for planning, not user)

- Exact count of new `@u:` refs needed for full syntax coverage across 595 files
- Whether some files are too dense with syntax to annotate every construct individually

## Source

Discussion date: 2026-04-10
Discussed with: user
