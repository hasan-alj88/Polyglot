---
code: PGE01050
name: Invalid IO Target
---
# Rule 1.50 — Invalid IO Target

**Statement:** An IO marker (`(-)`, `(#)`, `(!>)`, etc.) must be followed by a token that can validly participate in an IO operation, such as an Input/Output Parameter, a Pull/Push operator, or a Variable.

**Rationale:** The Compiler prevents nonsensical grammar sequences early in the compilation process. If a Pipeline IO marker `(-)` is followed immediately by another Pipeline identifier (`-SomePipeline`), the AST cannot be constructed. IO properties apply to parameters and variables, not to definition identifiers.

**Detection:** The Compiler Validation phase scans the immediate next token following any IO marker. If the target is fundamentally invalid for an IO sequence (e.g., `Pipeline`, `Data`, `Package`, `Trigger`), it emits PGE01050.

**Valid Targets (Examples):**
- `(-)` -> `InputParameter`, `OutputParameter`, `Variable`
- `(#)` -> `PullFrom`, `InputParameter`
- `(>)` -> `FallBackPullFrom`, `PullFrom`
