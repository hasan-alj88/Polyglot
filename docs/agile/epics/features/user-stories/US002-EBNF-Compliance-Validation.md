---
feature: "F003-Compiler"
github-issue-link: "#"
status: "todo"
assignee: "@product_owner"
---
# User Story: EBNF Compliance Validation

**As a** compiler consumer,
**I want** the compiler to linearly scan the token stream and enforce strict EBNF grammar compliance,
**So that** invalid grammatical sequences, invalid markers, and unknown Polyglot objects are caught and reported as PGE (Polyglot Error) codes before any hierarchical AST is built.

## Acceptance Criteria
- [ ] Validates that tokens on each line form a permitted EBNF sequence.
- [ ] Catches invalid token combinations and throws a structured PGE error with line/col metadata.
- [ ] Throws a PGE error if an `invalid marker` is encountered in the token stream.
- [ ] Throws a PGE error if an `unknown Polyglot object` (unrecognized token) is encountered in the token stream.
- [ ] Processing happens purely linearly, checking line-by-line grammar.

## Tasks
- [TASK005: EBNF Validation - Definition & Execution Markers](./tasks/TASK005-EBNF-Def-Exec.md)
- [TASK006: EBNF Validation - IO Rules](./tasks/TASK006-EBNF-IO.md)
- [TASK007: EBNF Validation - Assignments & Data Loading](./tasks/TASK007-EBNF-Assignments.md)
- [TASK008: EBNF Validation - Macros & Terminals](./tasks/TASK008-EBNF-Macros.md)
- [TASK009: EBNF Validation - Invalid Markers & Unknown Objects](./tasks/TASK009-EBNF-Invalid-Tokens.md)
- [TASK010: Error Data Schema & Aggregation](./tasks/TASK010-Error-Data-Schema.md)
- [TASK011: Error Formatting Display](./tasks/TASK011-Error-Formatting-Display.md)
- [TASK012: Validation CLI Integration](./tasks/TASK012-Validation-CLI-Integration.md)
