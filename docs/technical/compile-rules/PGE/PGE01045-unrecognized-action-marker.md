---
code: PGE01045
name: Unrecognized Action Marker
---
# Rule 1.45 — Unrecognized Action Marker

**Statement:** An action bracket `[]` was found, but the character inside is not a valid action marker (e.g., `[Z]`).

**Rationale:** Polyglot has a strict set of valid action markers (`[-]`, `[=]`, `[#]`, etc.). An unknown character inside square brackets indicates a typo or an unsupported marker.

**Detection:** The Lexer emits `InvalidActionMarker(String)`. The Compiler intercepts this token during EBNF validation and throws PGE01045.
