---
audience: developer
rule: "1.61"
code: PGE01061
name: IO Parameter Scope Mismatch
severity: error
---

# Rule 1.61 — IO Parameter Scope Mismatch
`PGE01061`

**Statement:** Pipeline input parameters `<` must strictly use the input IO marker `(<)`. Pipeline output parameters `>` must strictly use the output IO marker `(>)`. Using the generic Pipeline IO marker `(-)` for explicitly directional parameters is invalid.
**Rationale:** Explicit IO markers guarantee that the directionality of data flow is immediately visible to both the developer and the compiler. Using `(-)` for an input `<` hides whether the parameter is strictly inbound or outbound at the marker level.
**Detection:** When the compiler encounters an input parameter property `<name` or output parameter property `>name`, it validates the preceding IO marker. If `(<)` is missing for `<` or `(>)` is missing for `>`, an error is emitted.

**VALID:**
```aljam3
[ ] ✓ Using directional markers
{-} -ProcessData
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (<) <input#string
   (>) >output#string
```

**INVALID:**
```aljam3
[ ] ✗ PGE01061 — Using generic marker for specific parameter directions
{-} -ProcessData
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string                         [ ] ✗ PGE01061 — should be (<)
   (-) >output#string                        [ ] ✗ PGE01061 — should be (>)
```

**Diagnostic:** "Input parameter property `<` must be prefixed with `(<)` IO marker, not generic `(-)` or others. (or Output parameter property `>` must be prefixed with `(>)`...)"
