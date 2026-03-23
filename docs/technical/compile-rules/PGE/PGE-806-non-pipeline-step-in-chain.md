---
rule: "8.6"
code: PGE-806
name: Non-Pipeline Step in Chain
severity: error
---

### Rule 8.6 — Non-Pipeline Step in Chain
`PGE-806`

**Statement:** Every step in a chain (`>>`) must be a pipeline reference. Inline pipeline calls (pipeline ref + string literal, e.g., `=Path"/tmp"`) are value expressions, not callable steps, and must not appear in any chain position.
**Rationale:** A chain `=A >> =B >> =C` auto-wires output from one pipeline as input to the next. Each step must be a callable pipeline that can receive input and produce output. An inline call like `=Path"/tmp"` is a value expression — it already has its input (the inline string) and produces a value, just like `$variable`. Writing `=Path"/tmp" >> =Process` is equivalent to writing `$tmp >> =Process`, which is nonsensical — values are not pipeline steps.
**Detection:** The compiler inspects each step in a chain expression. Any step that is not a plain `pipeline_ref` is rejected — this includes `inline_pipeline_call` (pipeline ref + string literal).

**See also:** PGE-804 (ambiguous step reference), PGE-805 (unresolved step reference), PGE-807 (invalid assignment target)

**VALID:**
```polyglot
[ ] ✓ inline call as value in IO wiring — not in a chain
{=} =UsePath
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out;string
   [r] =Process
      [=] <path << =Path"/tmp"
      [=] >result >> >out
```

```polyglot
[ ] ✓ inline call as value in variable assignment
[r] $configPath;path << =Path"/etc/config"
```

```polyglot
[ ] ✓ regular pipeline refs in chain — no inline args
{=} =Pipeline
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data;string
   [=] >out;string
   [r] =Parse >> =Validate >> =Store
      [=] <input << $data
      [=] >result >> >out
```

**INVALID:**
```polyglot
[ ] ✗ PGE-806 — inline call as first step in chain
{=} =BadFirst
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out;string
   [r] =Path"/tmp" >> =Process          [ ] ✗ PGE-806 — value expr in chain
      [=] >result >> >out
```

```polyglot
[ ] ✗ PGE-806 — inline call as middle step
{=} =BadMiddle
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data;string
   [=] >out;string
   [r] =Parse >> =Path"/tmp" >> =Store  [ ] ✗ PGE-806 — value expr in chain
      [=] <input << $data
      [=] >result >> >out
```

```polyglot
[ ] ✗ PGE-806 — inline call as last step
{=} =BadLast
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data;string
   [=] >out;path
   [r] =Parse >> =Path"/tmp"            [ ] ✗ PGE-806 — value expr in chain
      [=] <input << $data
      [=] >result >> >out
```

```polyglot
[ ] ✗ PGE-806 — inline call with interpolation in chain
{=} =BadInterp
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <dir;string
   [=] >out;string
   [r] =Fetch >> =Path"{$dir}/output" >> =Store   [ ] ✗ PGE-806 — value expr in chain
      [=] <url << $dir
      [=] >result >> >out
```

**Diagnostic:** "Chain step at position N is an inline pipeline call `=Name\"...\"` — all chain steps must be pipeline references"

**Open point:** None.
