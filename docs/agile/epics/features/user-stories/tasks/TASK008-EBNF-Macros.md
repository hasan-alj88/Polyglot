---
user-story: "US002-EBNF-Compliance-Validation"
github-issue-link: "#"
status: "todo"
assignee: "@developer"
---
# Task: EBNF Validation - Macros & Terminals

## Description
Validate EBNF rules for isolated terminals, standalone variables, and error-raising macros.

## Instructions
1. Check the token stream for standard macro/terminal patterns.
2. Validate `Raise_Error_Macro`: e.g., `OP_PUSH_RIGHT` -> `TOK_SPACE` -> `ERROR`.
3. Validate `Isolated_Terminal` and `Standalone_Variable` rules, ensuring they don't contain dangling operators on the same line.
4. Throw a PGE error if these specific sequences are violated.

## Acceptance Criteria
- [ ] Validates `Raise_Error_Macro`, `Isolated_Terminal`, and `Standalone_Variable`.
- [ ] Throws a PGE error on syntax failure.
