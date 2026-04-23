---
audience: developer
rule: "1.33"
code: PGE01033
name: Unbound Script Variable
severity: error
---

# Rule 1.33 — Unbound Script Variable
`PGE01033`

<!-- @u:pglib/pipelines/Run/INDEX -->
<!-- @u:pglib/pipelines/Run/Script -->

**Statement:** Every `<Bind#Record` field name in a `-Run.<Lang>.Script` call must exist as an identifier in the foreign code. A field name with no matching identifier is a compile error.
**Rationale:** `<Bind` field names become native local variables — the runtime injects them before execution. If a field name doesn't appear in the code, the injected variable is unused, which strongly suggests a typo or wiring mistake.
**Detection:** The compiler scans the `[C]` block (or source file at runtime) for identifier tokens matching each `<Bind#Record` field name. Applies at compile time for `<code.inline` only; deferred to runtime for `<code.file`.

**VALID:**
```polyglot
[ ] field names match identifiers in [C] block
[-] -Run.Python.Script
   (-) <env#PyEnv << $pyenv
   (-) <Bind#Record
      [.] .target_dir#path << $targetDir
   (-) >output#Code:Python.Output >> $output
   (-) <code.inline <<
      [C] import os
      [C] files = os.listdir(target_dir)
```

**INVALID:**
```polyglot
[ ] PGE01033 — .tgt_dir not found in code (typo for target_dir)
[-] -Run.Python.Script
   (-) <env#PyEnv << $pyenv
   (-) <Bind#Record
      [.] .tgt_dir#path << $targetDir
   (-) >output#Code:Python.Output >> $output
   (-) <code.inline <<
      [C] import os
      [C] files = os.listdir(target_dir)
```

**Diagnostic:** "Unbound script variable — `<Bind` field `.tgt_dir` not found as identifier in code"
