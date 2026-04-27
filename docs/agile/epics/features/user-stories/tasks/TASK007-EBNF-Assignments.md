---
user-story: "US002-EBNF-Compliance-Validation"
github-issue-link: "#"
status: "todo"
assignee: "@developer"
---
# Task: EBNF Validation - Assignments & Data Loading

## Description
Validate the grammatical structure of standalone assignments, type declarations, and data load operations.

## Instructions
1. Scan for tokens indicating assignment or data operations (e.g., `VARIABLE` at the start of an expression, or `ACTION_DATA_LOAD`).
2. Check `Assignment` sequences: `VARIABLE` -> `TOK_SPACE` -> `OP_PUSH_LEFT` -> `TOK_SPACE` -> `DATA` (or another valid rhs).
3. Check `Typed_Variable` sequences: `VARIABLE` -> `DATATYPE` (no spaces allowed).
4. Check `Data_Load` sequences ensuring the action marker is followed by a valid assignment block.
5. Halt and throw a PGE error if an assignment lacks an operator or target.

## Acceptance Criteria
- [ ] Accurately validates `Assignment`, `Typed_Variable`, and `Data_Load` sequences.
- [ ] Throws PGE errors for malformed assignment logic.
