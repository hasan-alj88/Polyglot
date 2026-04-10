---
audience: developer
rule: "8.6"
code: PGE08006
name: Non-Pipeline Step in Chain
severity: error
---

### Rule 8.6 — Non-Pipeline Step in Chain
`PGE08006`

<!-- @u:syntax/blocks -->
<!-- @u:syntax/io -->
<!-- @u:syntax/operators -->

**Statement:** Every step in a chain (`->`) must be a pipeline reference. Inline pipeline calls (pipeline ref + string literal, e.g., `-Path"/tmp"`) are value expressions, not callable steps, and must not appear in any chain position.
**Rationale:** A chain `-A->-B->-C` auto-wires output from one pipeline as input to the next. Each step must be a callable pipeline that can receive input and produce output. An inline call like `-Path"/tmp"` is a value expression — it already has its input (the inline string) and produces a value, just like `$variable`. Writing `-Path"/tmp"->-Process` is equivalent to writing `$tmp->-Process`, which is nonsensical — values are not pipeline steps.
**Detection:** The compiler inspects each step in a chain expression. Any step that is not a plain `pipeline_ref` is rejected — this includes `inline_pipeline_call` (pipeline ref + string literal).

**See also:** PGE08004 (ambiguous step reference), PGE08005 (unresolved step reference), PGE08007 (invalid assignment target)

**VALID:**
```polyglot
[ ] ✓ inline call as value in IO wiring — not in a chain
{-} -UsePath
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >out#string
   [-] -Process
      (-) <path << -Path"/tmp"
      (-) >result >> >out
```

```polyglot
[ ] ✓ inline call as value in variable assignment
[-] $configPath#path << -Path"/etc/config"
```

```polyglot
[ ] ✓ regular pipeline refs in chain — no inline args
{-} -Pipeline
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <data#string
   (-) >out#string
   [-] -Parse->-Validate->-Store
      (-) <input << $data
      (-) >result >> >out
```

**INVALID:**
```polyglot
[ ] ✗ PGE08006 — inline call as first step in chain
{-} -BadFirst
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) >out#string
   [-] -Path"/tmp"->-Process            [ ] ✗ PGE08006 — value expr in chain
      (-) >result >> >out
```

```polyglot
[ ] ✗ PGE08006 — inline call as middle step
{-} -BadMiddle
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <data#string
   (-) >out#string
   [-] -Parse->-Path"/tmp"->-Store      [ ] ✗ PGE08006 — value expr in chain
      (-) <input << $data
      (-) >result >> >out
```

```polyglot
[ ] ✗ PGE08006 — inline call as last step
{-} -BadLast
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <data#string
   (-) >out#path
   [-] -Parse->-Path"/tmp"              [ ] ✗ PGE08006 — value expr in chain
      (-) <input << $data
      (-) >result >> >out
```

```polyglot
[ ] ✗ PGE08006 — inline call with interpolation in chain
{-} -BadInterp
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <dir#string
   (-) >out#string
   [-] -Fetch->-Path"{$dir}/output"->-Store        [ ] ✗ PGE08006 — value expr in chain
      (-) <url << $dir
      (-) >result >> >out
```

**Diagnostic:** "Chain step at position N is an inline pipeline call `-Name\"...\"` — all chain steps must be pipeline references"

### See Also

- [[user/concepts/pipelines/chains|Chains]] — references PGE08006 in chain execution rules
- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE08006 in chain execution table

**Open point:** None.
