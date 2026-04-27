---
epic: "Polyglot Lexer & AST Generator"
github-issue-link: "https://github.com/hasan-alj88/Polyglot/issues/358"
status: "done"
assignee: "@scrum_master"
dependencies: []
---
# Feature: Lexer (Token Stream Generator)

## Objective
Consume `*.pg` files and emit a predictable, linear stream of primitive syntax tokens that structural parsers can consume. This layer validates lexical constraints like spacing and bracket syntax.

## Requirements
- Scan `*.pg` source code files line by line (ignore entirely blank lines).
- Evaluate scope: Indentation is **strictly 3 spaces per level**. If not a multiple of 3, trigger a compile syntax error.
- Enforce syntactic boundaries: Isolate markers bound by `{X}`, `[X]`, or `(X)`.
- Handle Comments: Completely empty brackets (`{}`, `[]`, or `()`) treat the entire line as a comment.
- Extract the remaining `{1 expression}` safely.
- Output: A linear stream of tokens with accurate source map tracking (line number, column).

## Linked User Stories
- [US001: Lexer Line Parsing & Scope Tracker](./user-stories/US001-Lexer-Line-Parsing.md)
