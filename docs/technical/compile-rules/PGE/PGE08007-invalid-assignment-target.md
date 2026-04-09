---
audience: developer
rule: "8.7"
code: PGE08007
name: Invalid Assignment Target
severity: error
---

### Rule 8.7 — Invalid Assignment Target
`PGE08007`

**Statement:** The left-hand side of any assignment operator (`<<`, `<~`, `>>`, `~>`) must be a valid assignment target: a variable (`$name`), an output port (`>name`), or a field path (`$var.field`). Value expressions — literals, inline pipeline calls, or any other non-variable construct — cannot be assigned to.
**Rationale:** Assignments push data into a destination. Only variables and output ports in Declared or Default state can receive data (see PGE02003, PGE02005, PGE02008 for state-based constraints). A value expression like `-Path"/tmp"` or `"hello"` produces a value — it has no storage location to push into. This rule catches structural misuse; lifecycle state rules (PGE02002, PGE02003, PGE02005, PGE02008) catch state-based misuse.
**Detection:** The compiler checks the left-hand side of every assignment expression. If it is not a valid `assign_target` (variable, output port, or field path), the expression is rejected.

**See also:**
- PGE02002 (declared state unreadable — can't pull from Declared)
- PGE02003 (final is push-once — can't push into Final)
- PGE02005 (failed is terminal — can't push into Failed)
- PGE02008 (access after release — can't access Released)
- PGE08006 (non-pipeline step in chain — same principle, different context)

**VALID:**
```polyglot
[ ] ✓ variable on LHS — valid assignment target
{-} -Store
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <input#string
   (-) >out#string
   [-] >out << $input
```

```polyglot
[ ] ✓ field path on LHS — valid assignment target
[-] $user.name << "Alice"
[-] $user.age << 30
```

```polyglot
[ ] ✓ inline call on RHS — value expressions are valid as sources
{-} -MakePath
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >out#path
   [-] $p <~ -Path"/tmp/data"
   [-] >out << $p
```

**INVALID:**
```polyglot
[ ] ✗ PGE08007 — inline pipeline call as assignment target
{-} -BadInline
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <value#string
   [-] -Path"/tmp" << $value            [ ] ✗ PGE08007 — value expr, not a variable
```

```polyglot
[ ] ✗ PGE08007 — inline call as default assignment target
{-} -BadDefault
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <value#string
   [-] -Path"/tmp" <~ $value            [ ] ✗ PGE08007 — value expr, not a variable
```

```polyglot
[ ] ✗ PGE08007 — literal as assignment target
{-} -BadLiteral
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <value#string
   [-] "hello" << $value                [ ] ✗ PGE08007 — literal is not a variable
```

**Diagnostic:** "Invalid assignment target at line N — left-hand side must be a variable (`$`), output port (`>`), or field path, not a value expression"

### See Also

- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE08007 in call site rules

**Open point:** None.
