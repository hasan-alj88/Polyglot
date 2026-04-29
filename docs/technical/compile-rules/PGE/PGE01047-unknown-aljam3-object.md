---
code: PGE01047
name: Unknown Aljam3 Object
---
# Rule 1.47 — Unknown Aljam3 Object

**Statement:** An identifier was found, but its prefix or structure does not map to any known Aljam3 object type.

**Rationale:** All Aljam3 objects have strict identifiable prefixes (e.g., `-` for Pipeline, `#` for Data, `$` for Variable). An identifier without a valid prefix or containing invalid characters cannot be correctly classified by the engine.

**Detection:** The Lexer emits `UnknownAljam3Object(String)`. The Compiler intercepts this token during EBNF validation and throws PGE01047.
