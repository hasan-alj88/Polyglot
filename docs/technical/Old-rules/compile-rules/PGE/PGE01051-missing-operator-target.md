---
code: PGE01051
name: Missing Operator Target
---
# Rule 1.51 — Missing Operator Target

**Statement:** An assignment or data flow operator (`<<`, `>>`, `<~`, `~>`, `!>`, `!<`) must be followed by a target token.

**Rationale:** An assignment operation is a binary relationship. If the sequence ends abruptly after an operator (e.g., end of file or line without continuation), the expression is fundamentally incomplete.

**Detection:** The Compiler Validation phase scans the immediate next token following any operator. If it encounters the end of the stream without a target, it emits PGE01051.
