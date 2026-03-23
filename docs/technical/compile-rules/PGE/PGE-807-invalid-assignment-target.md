---
rule: "8.7"
code: PGE-807
name: Invalid Assignment Target
severity: error
---

### Rule 8.7 — Invalid Assignment Target
`PGE-807`

**Statement:** The left-hand side of any assignment operator (`<<`, `<~`, `>>`, `~>`) must be a valid assignment target: a variable (`$name`), an output port (`>name`), or a field path (`$var.field`). Value expressions — literals, inline pipeline calls, or any other non-variable construct — cannot be assigned to.
**Rationale:** Assignments push data into a destination. Only variables and output ports in Declared or Default state can receive data (see PGE-203, PGE-205, PGE-208 for state-based constraints). A value expression like `=Path"/tmp"` or `"hello"` produces a value — it has no storage location to push into. This rule catches structural misuse; lifecycle state rules (PGE-202, PGE-203, PGE-205, PGE-208) catch state-based misuse.
**Detection:** The compiler checks the left-hand side of every assignment expression. If it is not a valid `assign_target` (variable, output port, or field path), the expression is rejected.

**See also:**
- PGE-202 (declared state unreadable — can't pull from Declared)
- PGE-203 (final is push-once — can't push into Final)
- PGE-205 (failed is terminal — can't push into Failed)
- PGE-208 (access after release — can't access Released)
- PGE-806 (non-pipeline step in chain — same principle, different context)

**VALID:**
```polyglot
[ ] ✓ variable on LHS — valid assignment target
{=} =Store
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >out;string
   [r] >out << $input
```

```polyglot
[ ] ✓ field path on LHS — valid assignment target
[r] $user.name << "Alice"
[r] $user.age << 30
```

```polyglot
[ ] ✓ inline call on RHS — value expressions are valid as sources
{=} =MakePath
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out;path
   [r] $p <~ =Path"/tmp/data"
   [r] >out << $p
```

**INVALID:**
```polyglot
[ ] ✗ PGE-807 — inline pipeline call as assignment target
{=} =BadInline
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <value;string
   [r] =Path"/tmp" << $value            [ ] ✗ PGE-807 — value expr, not a variable
```

```polyglot
[ ] ✗ PGE-807 — inline call as default assignment target
{=} =BadDefault
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <value;string
   [r] =Path"/tmp" <~ $value            [ ] ✗ PGE-807 — value expr, not a variable
```

```polyglot
[ ] ✗ PGE-807 — literal as assignment target
{=} =BadLiteral
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <value;string
   [r] "hello" << $value                [ ] ✗ PGE-807 — literal is not a variable
```

**Diagnostic:** "Invalid assignment target at line N — left-hand side must be a variable (`$`), output port (`>`), or field path, not a value expression"

**Open point:** None.
