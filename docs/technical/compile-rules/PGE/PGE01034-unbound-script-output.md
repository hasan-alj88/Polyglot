---
audience: developer
rule: "1.34"
code: PGE01034
name: Unbound Script Output
severity: error
---

### Rule 1.34 — Unbound Script Output
`PGE01034`

<!-- @u:pglib/pipelines/Run/INDEX -->
<!-- @u:pglib/pipelines/Run/Script -->

**Statement:** Every `>Bind#Record` field name in a `-Run.<Lang>.Script` call must exist as an identifier in the foreign code. A field name with no matching identifier is a compile error.
**Rationale:** `>Bind` field names are read back from native scope after execution. If a field name doesn't appear as an assigned identifier in the code, the runtime will either read `undefined`/`None` or raise a name error — both are programming mistakes catchable at compile time.
**Detection:** The compiler scans the `[C]` block (or source file at runtime) for identifier tokens matching each `>Bind#Record` field name. Applies at compile time for `<code.inline` only; deferred to runtime for `<code.file`.

**VALID:**
```polyglot
[ ] output field names match identifiers in [C] block
[-] -Run.Python.Script
   (-) <env#PyEnv << $pyenv
   (-) <Bind#Record
      [.] .input_path#path << $imageFile
   (-) >Bind#Record
      [.] .resized_path#path
   (-) <code.inline <<
      [C] from PIL import Image
      [C] img = Image.open(input_path)
      [C] resized_path = input_path.with_suffix(".resized.png")
      [C] img.save(resized_path)
```

**INVALID:**
```polyglot
[ ] PGE01034 — .output_file not found in code
[-] -Run.Python.Script
   (-) <env#PyEnv << $pyenv
   (-) >Bind#Record
      [.] .output_file#path
   (-) <code.inline <<
      [C] from PIL import Image
      [C] resized_path = "out.png"
```

**Diagnostic:** "Unbound script output — `>Bind` field `.output_file` not found as identifier in code"
