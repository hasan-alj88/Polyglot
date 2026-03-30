---
rule: "1.27"
code: PGE01027
name: Empty Foreign Code Block
severity: error
---

### Rule 1.27 — Empty Foreign Code Block
`PGE01027`

**Statement:** A `[c]` foreign code block must contain at least one line of code after the header. A `[c]` with only the header and no body is a compile error.
**Rationale:** Foreign code blocks exist to embed non-Polyglot code. An empty block serves no purpose — it declares a foreign code context but provides nothing to execute.
**Detection:** The compiler checks that each `[c]` header is followed by at least one `foreign_code_line`.

**VALID:**
```polyglot
[ ] ✓ foreign code with body
[c] #Code:Python:3
   import pandas as pd
   df = pd.DataFrame(data)
```

**INVALID:**
```polyglot
[ ] ✗ PGE01027 — empty foreign code block
[c] #Code:Python:3
```

**Diagnostic:** "Empty foreign code block — `[c]` requires at least one code line"
