---
rule: "2.3"
code: PGW-203
name: Unpushed Output Port
severity: warning
---

### Rule 2.3 — Unpushed Output Port
`PGW-203`

**Statement:** An output `>name` declared in a pipeline's `[=]` IO section that is never pushed to by any code path is flagged with a warning. The output will remain in Declared state (never reach Final), which may block downstream pipelines waiting for it.
**Rationale:** A declared but unpushed output port is likely a wiring oversight. Callers expecting this output will see a variable stuck in Declared state, preventing downstream triggers from firing. This is a warning rather than an error because the output might be intentionally optional in some designs.
**Detection:** The compiler analyzes all code paths within the pipeline body and checks whether each declared `>name` output has at least one push (`<<` or `<~`) on every code path. If any output has zero pushes on any path, the warning is emitted.

**See also:** PGE-203 (final is push-once — covers push-once semantics), PGE-110 (pipeline IO name mismatch — covers call-site wiring), PGW-202 (unused variable — analogous for `$` variables)

**VALID:**
```polyglot
[ ] ✓ all declared outputs are pushed
{=} =Transform
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >result;string
   [=] >count;int
   [r] =Parse
      [=] <data << $input
      [=] >parsed >> $clean
   [r] >result << $clean
   [r] >count << 1
```

```polyglot
[ ] ✓ output pushed in all conditional branches
{=} =Route
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <status;int
   [=] >message;string
   [?] $status
      [?] ?[200,299]
         [r] >message << "ok"           [ ] ✓ pushed in this branch
      [?] *?
         [r] >message << "error"        [ ] ✓ pushed in this branch
```

**WARNING:**
```polyglot
[ ] ⚠ PGW-203 — >forgotten declared but never pushed
{=} =Incomplete
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >result;string
   [=] >forgotten;string               [ ] ⚠ PGW-203 — never pushed to
   [r] >result << $input
```

```polyglot
[ ] ⚠ PGW-203 — >count only pushed in one branch
{=} =PartialPush
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <flag;bool
   [=] >result;string
   [=] >count;int                       [ ] ⚠ PGW-203 — not pushed in *? branch
   [?] $flag
      [?] =? .True
         [r] >result << "yes"
         [r] >count << 1
      [?] *?
         [r] >result << "no"
         [ ] >count never pushed here
```

```polyglot
[ ] ⚠ PGW-203 — >debug declared but no code references it
{=} =Leftover
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data;string
   [=] >out;string
   [=] >debug;string                    [ ] ⚠ PGW-203 — >debug never pushed to
   [r] =Process
      [=] <input << $data
      [=] >result >> >out
```

**Open point:** None.
