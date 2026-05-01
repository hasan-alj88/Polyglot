---
audience: developer
rule: "1.16"
code: PGE01016
name: Unmarked Execution Line
severity: error
---

# Rule 1.16 — Unmarked Execution Line
`PGE01016`

<!-- @u:syntax/blocks -->
<!-- @u:concepts/conditionals -->
<!-- @u:syntax/operators -->
<!-- @u:syntax/types -->

**Statement:** Every line in a pipeline execution section must begin with a block element marker (`[-]`, `[=]`, `[b]`, `[#]`, `[?]`, or expand operators). Bare expressions, assignments, or pipeline calls without a marker are compile errors.
**Rationale:** Block element markers define execution semantics — `[-]` is synchronous, `[=]` is parallel, `[b]` is fire-and-forget, `[#]` loads data, `[?]` branches conditionally. A line without a marker has no defined execution semantics. The marker is not optional syntax sugar — it is the instruction that tells the runtime *how* to execute the line.
**Detection:** The compiler checks each line in the `execution_section` against the `exec_line` grammar production. Any line that does not match (i.e., does not start with a recognized block element marker) is rejected with a diagnostic suggesting the appropriate marker.

**See also:** PGE01001 (pipeline section misordering), PGE01005 (missing trigger), PGE01006 (missing queue)

**VALID:**
```aljam3
[ ] ✓ assignment under [-] marker
{-} -Example
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >out#string
   [ ]
   [-] >out << $input
```

```aljam3
[ ] ✓ data load under [#] marker
{-} -LoadConfig
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) >out#Config
   [#] $cfg#Config << "config.json"
   [ ]
   [-] >out << $cfg
```

**INVALID:**
```aljam3
[ ] ✗ PGE01016 — bare assignment without marker
{-} -Bad
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >out#string
   $out <~ $input                       [ ] ✗ PGE01016 — missing block element marker
```

```aljam3
[ ] ✗ PGE01016 — bare inline pipeline call without marker
{-} -AlsoBad
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) >out#path
   -Path"/tmp/data"                     [ ] ✗ PGE01016 — missing [-] marker
```

```aljam3
[ ] ✗ PGE01016 — bare pipeline call without marker
{-} -NoPrefixCall
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <data#string
   (-) >out#string
   -Transform                           [ ] ✗ PGE01016 — missing [-], [=], or [b] marker
      (-) <input << $data
      (-) >result >> >out
```

**Open point:** None.

## See Also

- [[concepts/pipelines/execution|Execution]] — documents block element marker requirement, references PGE01016
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01016
