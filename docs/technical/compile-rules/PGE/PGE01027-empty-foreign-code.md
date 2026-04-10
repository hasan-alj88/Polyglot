---
audience: developer
rule: "1.27"
code: PGE01027
name: Empty Foreign Code Block
severity: error
---

### Rule 1.27 — Empty Foreign Code Block
`PGE01027`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** A `[C]` foreign code block must contain at least one line of code. A `<script` input with `[C]` but no code lines is a compile error.
**Rationale:** Foreign code blocks exist to embed non-Polyglot code passed to `-RT.*` pipelines. An empty block serves no purpose — it declares a foreign code context but provides nothing to execute.
**Detection:** The compiler checks that each `[C]` sequence contains at least one `foreign_code_line`.

**VALID:**
```polyglot
[ ] ✓ foreign code with body
[-] -RT.Python.Script
   (-) <env << $env
   (-) <script <<
      [C] import pandas as pd
      [C] df = pd.DataFrame(data)
   (-) >stdout >> $output
```

**INVALID:**
```polyglot
[ ] ✗ PGE01027 — empty foreign code block
[-] -RT.Python.Script
   (-) <env << $env
   (-) <script <<
   (-) >stdout >> $output
```

**Diagnostic:** "Empty foreign code block — `[C]` requires at least one code line"
