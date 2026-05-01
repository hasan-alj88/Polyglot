---
audience: developer
rule: "2.04"
code: PGE02004
name: Double Default Push
severity: error
---

# Rule 2.04 — Double Default Push
`PGE02004`

**Statement:** A variable cannot receive a default assignment (e.g., `<~` or `~>`) if it is already in the `Default` state. Once a variable has a default value, the next assignment to it must be a Final assignment (`<<` or `>>`).
**Rationale:** Default assignments (`<~`) establish a fallback. Overwriting a default with another default implies logical uncertainty and usually indicates a bug. Once a default is set, the only valid data flow is confirming a final, definitive result.
**Detection:** The compiler tracks variable state. If a variable is in the `Default` state and encounters a `DefaultPush` operator targeting it, `PGE02004` is triggered.

**VALID:**
```aljam3
[ ] ✓ Default followed by Final
{-} -ProcessData
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] $var#int <~ 1
   [-] $var << 2
```

**INVALID:**
```aljam3
[ ] ✗ PGE02004 — Double Default assignment
{-} -ProcessData
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   [ ]
   [-] $var#int <~ 1
   [-] $var <~ 2                            [ ] ✗ PGE02004 — Double default push
```

**Diagnostic:** "Variable `$var` received a second default assignment without a final assignment in between. Default-assigned variables can only receive a final push (`<<` or `>>`) next."
