---
audience: ai-finder
type: audit-reference
scope: decision-record
category: syntax
date: 2026-04-24
updated: 2026-04-24
---

# Remove Multiline Comment Syntax

## Summary

Multiline comments (`[ ]<` ... `[ ]>`) are removed from the Aljam3 grammar. Single-line comments via `[ ]` and `{ }` remain the only comment mechanisms. This simplifies the lexer implementation and eliminates an under-specified grammar production.

## Before

```ebnf
multiline_comment   ::= "[ ]<" NEWLINE
                         { any_text NEWLINE }
                         "[ ]>" ;
```

Multiline comments were defined in EBNF §13 as a block opened by `[ ]<` and closed by `[ ]>`, allowing arbitrary text lines between.

## After

Multiline comments are removed entirely. Developers use consecutive single-line `[ ]` comments for multi-line commentary. The EBNF §13 production is retired.

## Impact

- EBNF file modified: `docs/technical/ebnf/13-comments.md`
- No PGE/PGW codes affected (multiline comments had no associated compile rules)
- Lexer token list simplified: no `TOK_MULTILINE_COMMENT_OPEN` / `TOK_MULTILINE_COMMENT_CLOSE` tokens needed

## Rationale

- **Lexer simplicity:** Multiline comments introduce a stateful lexer mode (tracking open/close across lines) that adds complexity disproportionate to the feature's value.
- **Consistency:** Every other Aljam3 construct is single-line based with `[~]` continuation for overflow. Multiline comments break this pattern.
- **Low usage:** No existing code samples or documentation examples use multiline comments.
- **Easy workaround:** Consecutive `[ ]` lines achieve the same result without special syntax.

## Related

- EBNF §13 Comments
- TASK003-Lexer-Identify-Tokens
