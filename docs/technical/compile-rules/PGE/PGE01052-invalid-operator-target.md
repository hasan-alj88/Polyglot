---
code: PGE01052
name: Invalid Operator Target
---
# Rule 1.52 — Invalid Operator Target

**Statement:** An assignment or data flow operator (`<<`, `>>`, `<~`, `~>`, `!>`, `!<`) must be followed by a valid data-carrying or handle token, such as a Variable, Data literal, Package, String literal, or Error identifier.

**Rationale:** You cannot assign a value to a grammatical structural marker or pull a value from one. For instance, `<< {-}` or `>> [=]` are fundamentally invalid sequences because markers dictate structure, not data logic.

**Detection:** The Compiler Validation phase scans the immediate next token following any operator. If the target is a structural or definition/action marker (e.g., `DefPackage`, `ActionExecSeq`, `Scope`), it emits PGE01052.
