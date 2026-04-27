---
user-story: "US002-EBNF-Compliance-Validation"
github-issue-link: "#"
status: "todo"
assignee: "@developer"
---
# Task: EBNF Validation - Definition & Execution Markers

## Description
Validate that all Definition (`{X}`) and Execution (`[X]`) markers in the token stream are followed by the exact grammatical sequences required by the Polyglot EBNF specification.

## Instructions
1. Iterate over the linear token stream.
2. Whenever a definition marker (e.g., `DEF_PIPELINE`, `DEF_DATA`) is encountered, ensure it is immediately followed by a `TOK_SPACE` and the correct object identifier token (e.g., `PIPELINE`, `DATA`).
3. Whenever an execution marker (e.g., `ACTION_EXEC_SEQ`, `ACTION_EXEC_PAR`) is encountered, ensure it is followed by `TOK_SPACE` and the expected target object.
4. If the sequence is broken, emit a PGE (Polyglot Grammar Error) with the line and column number.

## Acceptance Criteria
- [ ] Correctly accepts valid `Def_Pipeline`, `Def_Data`, `Execution_Seq`, and `Execution_Par` EBNF structures.
- [ ] Throws a PGE error on invalid sequences following a definition or execution marker.
