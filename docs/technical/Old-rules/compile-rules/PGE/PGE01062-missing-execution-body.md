---
audience: developer
rule: "1.62"
code: PGE01062
name: Missing Execution Body
severity: error
---

# Rule 1.62 — Missing Execution Body
`PGE01062`

**Statement:** Every pipeline `{-}` must contain at least one execution step (e.g., `[-]`, `[=]`, `[?]`) after its setup markers (`[T]`, `[Q]`, `[W]`). A pipeline with only setup blocks and IO markers is structurally incomplete.
**Rationale:** Pipelines exist to perform work. A pipeline without execution steps is a dead node in the execution graph. The compiler treats empty pipelines as errors rather than ignoring them to ensure developers do not accidentally leave dead code or incomplete logic.
**Detection:** The compiler verifies that at least one token classifying as an execution action is present inside the pipeline scope.

**VALID:**
```aljam3
[ ] ✓ Pipeline has an execution step
{-} -ProcessData
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   [-] >run
```

**INVALID:**
```aljam3
[ ] ✗ PGE01062 — Missing execution step
{-} -ProcessData
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
                                             [ ] ✗ PGE01062 — Pipeline lacks execution body
```

**Diagnostic:** "Pipeline lacks an execution body. All pipelines must contain at least one execution block (e.g. `[-]`, `[=]`, `[?]`) to perform work."
