---
code: PGE01042
name: Incorrect Indentation Multiple
---
# Rule 1.42 — Incorrect Indentation Multiple

**Statement:** Indentation must occur strictly in multiples of 3 spaces.

**Rationale:** Strict space-based indentation parsing avoids tabs vs. spaces ambiguity and visually defines nested scopes consistently. Any leading space count that is not a multiple of 3 (e.g., 2, 4, 5) breaks the AST generation phase.

**Detection:** The Lexer calculates leading spaces on each line. If `spaces % 3 != 0`, it emits `IncorrectIndent(String)`. The Compiler intercepts this token during EBNF validation and throws PGE01042.
