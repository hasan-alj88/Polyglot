---
code: PGE01047
name: Unknown Polyglot Object
---
# Rule 1.47 — Unknown Polyglot Object

**Statement:** An identifier was found, but its prefix or structure does not map to any known Polyglot object type.

**Rationale:** All Polyglot objects have strict identifiable prefixes (e.g., `-` for Pipeline, `#` for Data, `$` for Variable). An identifier without a valid prefix or containing invalid characters cannot be correctly classified by the engine.

**Detection:** The Lexer emits `UnknownPolyglotObject(String)`. The Compiler intercepts this token during EBNF validation and throws PGE01047.
