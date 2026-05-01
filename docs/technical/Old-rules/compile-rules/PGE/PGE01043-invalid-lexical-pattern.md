---
code: PGE01043
name: Invalid Lexical Pattern
---
# Rule 1.43 — Invalid Lexical Pattern

**Statement:** A sequence of characters on a line violates the basic lexical grammar of Aljam3.

**Rationale:** The Lexer applies strict regex constraints to find known EBNF patterns. If garbage characters or malformed strings are found that don't conform to any known sequence, the lexer rejects it to prevent the compiler from making arbitrary guesses.

**Detection:** The Lexer emits `InvalidPattern(String)` when an unparseable sequence is found. The Compiler intercepts this token during EBNF validation and throws PGE01043, returning the invalid string to the user.
