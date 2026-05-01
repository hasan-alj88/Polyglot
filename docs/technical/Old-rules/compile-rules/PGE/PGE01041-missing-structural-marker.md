---
code: PGE01041
name: Missing Structural Marker
---
# Rule 1.41 — Missing Structural Marker

**Statement:** Every non-empty line of Aljam3 code must begin with a structural marker (e.g., `{-}`, `[#]`, `(-)`). A line lacking any bracketed marker triggers a grammatical failure.

**Rationale:** The Lexer relies on structural markers to determine scope and operational context. Without a marker, the line's intent is ambiguous.

**Detection:** The Lexer emits `MissingMarker` when encountering a line lacking a valid starting bracket. The Compiler intercepts this token during EBNF validation and throws PGE01041.

**See also:** PGE01016 (Unmarked Execution Line)
