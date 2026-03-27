---
rule: "1.16"
code: PGE-116
name: Unmarked Execution Line
severity: error
---

### Rule 1.16 — Unmarked Execution Line
`PGE-116`

**Statement:** Every line in a pipeline execution section must begin with a block element marker (`[r]`, `[p]`, `[b]`, `[#]`, `[?]`, or expand operators). Bare expressions, assignments, or pipeline calls without a marker are compile errors.
**Rationale:** Block element markers define execution semantics — `[r]` is synchronous, `[p]` is parallel, `[b]` is fire-and-forget, `[#]` loads data, `[?]` branches conditionally. A line without a marker has no defined execution semantics. The marker is not optional syntax sugar — it is the instruction that tells the runtime *how* to execute the line.
**Detection:** The compiler checks each line in the `execution_section` against the `exec_line` grammar production. Any line that does not match (i.e., does not start with a recognized block element marker) is rejected with a diagnostic suggesting the appropriate marker.

**See also:** PGE-101 (pipeline section misordering), PGE-105 (missing trigger), PGE-106 (missing queue)

**VALID:**
```polyglot
[ ] ✓ assignment under [r] marker
{=} =Example
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >out#string
   [r] >out << $input
```

```polyglot
[ ] ✓ data load under [#] marker
{=} =LoadConfig
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#Config
   [#] $cfg#Config << "config.json"
   [r] >out << $cfg
```

**INVALID:**
```polyglot
[ ] ✗ PGE-116 — bare assignment without marker
{=} =Bad
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >out#string
   $out <~ $input                       [ ] ✗ PGE-116 — missing block element marker
```

```polyglot
[ ] ✗ PGE-116 — bare inline pipeline call without marker
{=} =AlsoBad
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out#path
   =Path"/tmp/data"                     [ ] ✗ PGE-116 — missing [r] marker
```

```polyglot
[ ] ✗ PGE-116 — bare pipeline call without marker
{=} =NoPrefixCall
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] >out#string
   =Transform                           [ ] ✗ PGE-116 — missing [r], [p], or [b] marker
      [=] <input << $data
      [=] >result >> >out
```

**Open point:** None.
