---
audience: developer
rule: "1.37"
code: PGE01037
name: Bind Schema Mismatch
severity: error
---

# Rule 1.37 — Bind Schema Mismatch
`PGE01037`

<!-- @u:pglib/pipelines/Run/INDEX -->

**Statement:** When assigning an inline Record to `<Bind`, `<arg`, `<kwarg`, or `>Bind`, the assigned value must match the declared Record schema topology. A structural mismatch is a compile error.
**Rationale:** Record bindings drive native variable injection and type marshalling. If the assigned value has fields that don't match the declared schema, the runtime will either inject unexpected variables or fail during serialization — both are preventable at compile time.
**Detection:** The compiler checks that the assigned inline Record has exactly the fields declared in the target Record schema, with compatible types.

**VALID:**
```aljam3
[ ] inline Record matches declared schema
{#} #ResizeInputs
   [.] .input_path#path
   [.] .target_w#int
   [.] .target_h#int

[-] -Run.Python.Script
   (-) <Bind#ResizeInputs
      [.] .input_path#path << $imageFile
      [.] .target_w#int << $width
      [.] .target_h#int << $height
```

**INVALID:**
```aljam3
[ ] PGE01037 — .extra_field not in #ResizeInputs schema
{#} #ResizeInputs
   [.] .input_path#path
   [.] .target_w#int
   [.] .target_h#int

[-] -Run.Python.Script
   (-) <Bind#ResizeInputs
      [.] .input_path#path << $imageFile
      [.] .target_w#int << $width
      [.] .extra_field#string << "oops"
```

**Diagnostic:** "Bind schema mismatch — field `.extra_field` not declared in `#ResizeInputs`"
