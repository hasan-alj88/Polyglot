---
feature: "F001-Lexer"
github-issue-link: "https://github.com/hasan-alj88/Polyglot/issues/362"
status: "done"
assignee: "@product_owner"
---
# User Story: Lexer Line Parsing & Scope Tracking

**As a** compiler consumer,
**I want** the lexer to accurately scan raw polyglot source code and compute scoping through indentation,
**So that** my syntax is structurally verified and prepped for Token Generation without hidden spacing errors.

## Acceptance Criteria
- [x] Blank lines are completely ignored.
- [x] Correctly identifies comments `{}`, `[]`, `()` and ignores the line.
- [x] Validates indentation in exact increments of 3 spaces.
- [x] Throws a hard syntax error message capturing the exact line/column for bad indentation.
- [x] Emits `Scope Up` / `Scope Down` tokens.

## Tasks
- [TASK001: Implement 3-Space Indentation Scope Tracker](./tasks/TASK001-Lexer-3Space-Indentation.md)
- [TASK002: Implement Bracket Marker & Expression Isolation](./tasks/TASK002-Lexer-Tokenizer.md)
- [TASK003: Identify Tokens](./tasks/TASK003-Lexer-Identify-Tokens.md)
