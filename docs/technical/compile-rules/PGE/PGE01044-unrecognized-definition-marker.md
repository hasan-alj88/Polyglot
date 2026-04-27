---
code: PGE01044
name: Unrecognized Definition Marker
---
# Rule 1.44 — Unrecognized Definition Marker

**Statement:** A definition bracket `{}` was found, but the character inside is not a valid definition marker (e.g., `{Z}`).

**Rationale:** Polyglot has a strict set of valid definition markers (`{-}`, `{#}`, `{T}`, etc.). An unknown character inside curly braces indicates a typo or an unsupported marker.

**Detection:** The Lexer emits `InvalidDefinitionMarker(String)`. The Compiler intercepts this token during EBNF validation and throws PGE01044.
