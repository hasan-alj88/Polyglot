---
user-story: "US002-EBNF-Compliance-Validation"
github-issue-link: "#"
status: "todo"
assignee: "@developer"
---
# Task: EBNF Validation - IO Rules

## Description
Validate that all IO brackets (`(X)`) in the token stream adhere strictly to their EBNF grammar combinations.

## Instructions
1. Scan the token stream for IO marker tokens (e.g., `IO_PIPELINE`, `IO_PARAM_OUT_FALLBACK`).
2. Verify the sequence matches the EBNF definitions. For example, `IO_PIPELINE` -> `TOK_SPACE` -> `INPUT_PARAMETER` -> `TOK_SPACE` -> `OP_PUSH_LEFT` -> `TOK_SPACE` -> `VARIABLE`.
3. Account for all valid IO combinations, including outputs and fallbacks.
4. If an IO sequence is malformed, emit a standard PGE error.

## Acceptance Criteria
- [ ] Accurately validates `IO_Pipeline_Input`, `IO_Pipeline_Out`, `IO_Fallback_Out`, and `IO_Fallback_In` EBNF patterns.
- [ ] Throws a PGE error on missing or incorrectly ordered IO operators/variables.
