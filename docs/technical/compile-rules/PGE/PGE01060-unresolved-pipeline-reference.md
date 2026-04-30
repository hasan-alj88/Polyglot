---
audience: developer
rule: "1.60"
code: PGE01060
name: Unresolved Pipeline Reference
severity: error
---

# Rule 1.60 — Unresolved Pipeline Reference
`PGE01060`

**Statement:** Any pipeline executed via an execution action (`[-]`, `[=]`, `[?]`) must be either defined within the current package or explicitly prefixed with a standard library namespace (e.g., `T.`, `Q.`, `W.`, `Status.`, `File.`, `Do.`). Calling an unknown pipeline is a compile error.
**Rationale:** Aljam3 requires all dependencies to be statically resolvable at compile time. This prevents runtime "command not found" errors and ensures that the dependency graph is fully known before execution begins.
**Detection:** The compiler first scans the package for all `{-}` defined pipelines. When validating execution steps, it checks the target pipeline name against this set of defined pipelines and standard library prefixes. If no match is found, compilation fails.

**VALID:**
```aljam3
[ ] ✓ Calling a defined pipeline
{-} -ProcessData
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Aljam3
   [-] -CleanData

{-} -CleanData
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Aljam3
   [-] >run
```

**INVALID:**
```aljam3
[ ] ✗ PGE01060 — Calling an undefined pipeline
{-} -ProcessData
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Aljam3
   [-] -CleanData                            [ ] ✗ PGE01060 — -CleanData is not defined
```

**Diagnostic:** "The pipeline `-CleanData` is not defined in the current package. Ensure the pipeline is defined. External imports must be explicitly prefixed if not in aj3lib."
