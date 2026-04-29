# Aljam3 — Claude Code Instructions

## Documentation Rules

Before writing or editing any documentation, read `docs/audit/README.md` first.
All documentation must comply with the rules in `docs/audit/`.

## Authority Chain

1. `docs/vision.md` — authoritative product vision (highest priority)
   1a. `docs/philosophy/` — philosophy sub-pages (derived from vision.md)
2. `docs/audit/` — documentation ground truth and writing rules
3. `.paul/` — project management process and state

## Key Rules

- Smart references (`@` imports) are mandatory — read the referenced section before proceeding
- Use dual referencing: `@` for Claude imports + `[[wikilinks]]` for Obsidian
- Use `docs/audit/reference/glossary.md` terms exactly — do not improvise definitions
- Audit rules apply to ALL documentation, not just `docs/` files
