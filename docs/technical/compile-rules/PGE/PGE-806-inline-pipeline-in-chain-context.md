---
rule: "8.6"
code: PGE-806
name: Inline Pipeline in Chain Context
severity: error
---

### Rule 8.6 — Inline Pipeline in Chain Context
`PGE-806`

**Statement:** An inline pipeline call (pipeline reference + string literal, e.g., `=Path"/tmp"`) must not appear in any position of a chain (`>>`). Chain steps must be callable pipeline references — not value expressions. Inline pipeline calls resolve to values, not callable steps.
**Rationale:** A chain `=A >> =B >> =C` auto-wires output from one pipeline as input to the next. An inline call like `=Path"/tmp"` is a value expression — it already has its input (the inline string) and produces a value, just like `$variable`. Writing `=Path"/tmp" >> =Process` is equivalent to writing `$tmp >> =Process`, which is nonsensical — variables and values are not pipeline steps.
**Detection:** The compiler inspects each step in a chain expression. Any step that is an `inline_pipeline_call` (pipeline ref + string literal) rather than a plain `pipeline_ref` is rejected.

**See also:** PGE-804 (ambiguous step reference), PGE-805 (unresolved step reference), PGE-807 (inline pipeline on assignment LHS)

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

**Open point:** None.
