---
audience: developer
rule: "1.38"
code: PGE01038
name: Code Source Conflict
severity: error
---

# Rule 1.38 — Code Source Conflict
`PGE01038`

<!-- @u:jm3lib/pipelines/Run/INDEX -->
<!-- @u:syntax/blocks -->

**Statement:** A `-Run.*` pipeline call must provide either `<code.inline` (with `[C]` blocks) or `<code.file` (with a path), never both. Providing both is a compile error.
**Rationale:** `#Code:Source` has `%##Active` = `#ActiveKind.One` — exactly one field must be active. Providing both creates ambiguity about which code the runtime should execute.
**Detection:** The compiler checks that exactly one of `.inline` or `.file` is assigned within the `<code#Code:Source` input.

**VALID:**
```aljam3
[ ] inline code only
[-] -Run.Python.Script
   (-) <env#PyEnv << $pyenv
   (-) <code.inline <<
      [C] result = 42

[ ] file reference only
[-] -Run.Python.Script
   (-) <env#PyEnv << $pyenv
   (-) <code.file#path << "/scripts/process.py"
```

**INVALID:**
```aljam3
[ ] PGE01038 — both .inline and .file provided
[-] -Run.Python.Script
   (-) <env#PyEnv << $pyenv
   (-) <code.inline <<
      [C] result = 42
   (-) <code.file#path << "/scripts/process.py"
```

**Diagnostic:** "Code source conflict — `<code` provides both `.inline` and `.file`; `#Code:Source` requires exactly one"
