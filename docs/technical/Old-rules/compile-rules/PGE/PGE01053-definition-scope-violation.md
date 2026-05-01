---
code: PGE01053
name: Definition Scope Violation
---
# Rule 1.53 — Definition Scope Violation

**Statement:** A definition marker (`{-}`, `{@}`, `{#}`, etc.) must strictly reside at the root level of the file, equivalent to `Scope(0)` (no indentation). The only exception is the definition comment marker (`{//}`).

**Rationale:** Aljam3 enforces strict, unambiguous hierarchical structures. Definitions cannot be nested inside other definitions or within execution blocks. They must exclusively represent top-level global entities.

**Detection:** The Compiler Validation phase tracks the indentation scope of each line. If it encounters a definition marker while the active scope is `> 0` (indented), it emits PGE01053.
