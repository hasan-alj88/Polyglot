---
audience: developer
rule: "1.56"
code: PGE01056
name: Invalid Data Field Definition
severity: error
---

# Rule 1.56 — Invalid Data Field Definition
`PGE01056`

**Statement:** When defining fields inside a Data `{#}` block, the property must be defined using the structurally correct marker, which for data properties is `[#]`, not an IO marker like `(#)` or parameter directional syntax like `<`.
**Rationale:** Data blocks describe static structure (schema), not active IO parameters. Using parameter IO syntax like `(#) <input` inside a Data block violates the semantic distinction between structured objects and execution parameter passing.
**Detection:** The compiler verifies that field definitions inside `{#}` and `{Q}` contexts do not use `(#)` or `<`. If it detects an IO-style definition where a structural data field should be, it emits an error.

**VALID:**
```aljam3
[ ] ✓ Using [#] to define a data field
{#} #User
   [#] .id#string
   [#] .name#string
```

**INVALID:**
```aljam3
[ ] ✗ PGE01056 — Using (#) and < in a Data block
{#} #User
   (#) <id#string                            [ ] ✗ PGE01056 — should be [#] .id#string
```

**Diagnostic:** "Data field defined using IO marker `(#)` or `<` instead of `[#] .field_name#Type`. Use `[#] .field_name` to define fields inside a `{#}` Data context."
