---
audience: developer
rule: "W1.4"
code: PGW01004
name: Orphaned Foreign Code
severity: warning
---

# Rule W1.4 — Orphaned Foreign Code
`PGW01004`

<!-- @u:syntax/blocks -->
<!-- @u:ebnf/11-control-flow -->

**Statement:** `[C]` foreign code lines should only appear as children of `-RT.*` pipeline calls (e.g., `-RT.Python.Script`, `-RT.JS.Function`). `[C]` lines outside this scope are syntactically valid but have no effect — the compiler cannot pass the code to any runtime.
**Rationale:** The grammar accepts `[C]` anywhere in execution context because `foreign_code_elem` is a general block element (§5). Restricting `[C]` in the EBNF would require the grammar to understand pipeline semantics (which `-` prefixed names are `-RT.*`), which is beyond the grammar's role. A semantic warning catches misplaced `[C]` lines without grammar complexity.
**Detection:** The compiler checks each `[C]` sequence. If the nearest parent `[-]` call is not a `-RT.*` pipeline, PGW01004 fires.

**VALID:**
```aljam3
[ ] ✓ [C] under -RT.Python.Script — compiler knows the runtime target
[-] -RT.Python.Script
   (-) <env << $env
   (-) <script <<
      [C] import pandas as pd
      [C] df = pd.DataFrame(data)
   (-) >stdout >> $output
```

**INVALID:**
```aljam3
[ ] ⚠ PGW01004 — [C] after a non-RT pipeline call
[-] -File.Text.Read
   (-) <path << "/tmp/data.txt"
   (-) >content >> $data
[C] print("orphaned code")
```

```aljam3
[ ] ⚠ PGW01004 — [C] inside conditional without -RT.* parent
[?] $mode =? "debug"
   [C] console.log("debug info")
[?] *?
   [-] -DoNothing
```

**Diagnostic:** "Foreign code `[C]` line is not scoped under a `-RT.*` pipeline call — code has no runtime target"

**See also:**
- [PGE01027 — Empty Foreign Code Block](../PGE/PGE01027-empty-foreign-code.md) — empty `[C]` block (no code lines)
- [[ebnf/11-control-flow#11.6 Foreign Code Injection]] — EBNF production
- [[edge-cases/22-control-flow-gaps#EC-22.5]] — edge case documentation
