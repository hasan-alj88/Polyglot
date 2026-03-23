---
rule: "8.7"
code: PGE-807
name: Inline Pipeline on Assignment LHS
severity: error
---

### Rule 8.7 — Inline Pipeline on Assignment LHS
`PGE-807`

**Statement:** An inline pipeline call (pipeline reference + string literal, e.g., `=Path"/tmp"`) must not appear on the left-hand side of any assignment operator (`<<`, `<~`, `<!`). Inline calls are value expressions — they produce values, they cannot be assigned to.
**Rationale:** An inline pipeline call resolves to a value, just like a `$variable`. Writing `=Path"/tmp" << $value` is equivalent to writing `"some value" << $value` — you cannot push data into a value expression. Assignment targets must be variables (`$name`), output ports (`>name`), or field paths (`$var.field`).
**Detection:** The compiler checks the left-hand side of every assignment expression. If it is an `inline_pipeline_call` rather than a valid `assign_target`, the expression is rejected.

**See also:** PGE-806 (non-pipeline step in chain — same principle, different misuse), PGE-116 (unmarked execution line)

**VALID:**
```polyglot
[ ] ✓ inline call on RHS — produces value for assignment
{=} =MakePath
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out;path
   [r] $p <~ =Path"/tmp/data"
   [r] >out << $p
```

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

**INVALID:**
```polyglot
[ ] ✗ PGE-807 — inline call as final assignment target
{=} =BadAssign
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <value;string
   [r] =Path"/tmp" << $value            [ ] ✗ PGE-807 — can't assign to value expr
```

```polyglot
[ ] ✗ PGE-807 — inline call as default assignment target
{=} =BadDefault
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <value;string
   [r] =Path"/tmp" <~ $value            [ ] ✗ PGE-807 — can't assign to value expr
```

```polyglot
[ ] ✗ PGE-807 — inline call as fallback target
{=} =BadFallback
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <value;string
   [r] =Path"/tmp" <! $value            [ ] ✗ PGE-807 — can't assign to value expr
```

**Open point:** None.
