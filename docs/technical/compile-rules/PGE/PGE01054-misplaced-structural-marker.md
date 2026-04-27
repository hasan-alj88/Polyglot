---
code: PGE01054
name: Misplaced Structural Marker
---
# Rule 1.54 — Misplaced Structural Marker

**Statement:** A structural marker (Action, Definition, or IO) must be placed exactly after the Scope token (i.e., immediately following the line's indentation). It cannot be nested inside or appended to the end of a data expression. The only exception is the `[ ]`, `{ }`, and `( )` comment markers.

**Rationale:** The Polyglot compiler strictly enforces hierarchical semantics by ensuring structural markers define the entire line's behavior. Allowing markers mid-expression creates severe ambiguity for execution engines and destroys the visual layout consistency of the language.

**Detection:** The Lexer intercepts structural markers embedded inside an expression and emits `MisplacedMarker`. The Compiler Validation phase traps this and emits PGE01054, explicitly declaring the marker structurally invalid for its horizontal position.
