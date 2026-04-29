---
user-story: "US002-EBNF-Compliance-Validation"
github-issue-link: "#"
status: "todo"
assignee: "@developer"
---
# Task: EBNF Validation - Invalid Markers & Unknown Objects

## Description
Ensure that any invalid syntax caught by the Lexer and passed into the stream as an unknown or invalid token directly triggers a formal Aljam3 Grammar Error (PGE).

## Instructions
1. The lexer emits `TOK_UNRECOGNIZED` for garbage characters or invalid marker structures.
2. During the EBNF validation pass, intercept any `TOK_UNRECOGNIZED` tokens.
3. If an invalid marker is detected, throw a PGE error formatted with its coordinates.
4. If an unknown Aljam3 object (e.g. a prefix that wasn't successfully resolved into a valid identifier token) is detected, throw a PGE error.

## Acceptance Criteria
- [ ] `invalid marker -> PGE error` is strictly enforced.
- [ ] `unknown Aljam3 object -> PGE error` is strictly enforced.
- [ ] The compiler provides actionable error messages based on the raw unrecognized string.
