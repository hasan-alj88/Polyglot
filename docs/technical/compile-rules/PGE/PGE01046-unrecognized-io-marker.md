---
code: PGE01046
name: Unrecognized IO Marker
---
# Rule 1.46 — Unrecognized IO Marker

**Statement:** An IO bracket `()` was found, but the character inside is not a valid IO marker (e.g., `(Z)`).

**Rationale:** Polyglot has a strict set of valid IO markers (`(-)`, `(>)`, `(<)`, etc.). An unknown character inside parentheses indicates a typo or an unsupported marker.

**Detection:** The Lexer emits `InvalidIOMarker(String)`. The Compiler intercepts this token during EBNF validation and throws PGE01046.
