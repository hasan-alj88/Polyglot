---
rule: "2.3"
code: PGW02003
name: Unpushed Output Port
severity: warning
---

### Rule 2.3 — Unpushed Output Port
`PGW02003`

**Statement:** An output `>name` declared in a pipeline's `[=]` IO section that is never pushed to by any code path is flagged with a warning. The output will remain in Declared state (never reach Final), which may block downstream pipelines waiting for it.
**Rationale:** A declared but unpushed output port is likely a wiring oversight. Callers expecting this output will see a variable stuck in Declared state, preventing downstream triggers from firing. This is a warning rather than an error because the output might be intentionally optional in some designs.
**Detection:** The compiler analyzes all code paths within the pipeline body and checks whether each declared `>name` output has at least one push (`<<` or `<~`) on every code path. If any output has zero pushes on any path, the warning is emitted.

**See also:** PGE02003 (final is push-once — covers push-once semantics), PGE01010 (pipeline IO name mismatch — covers call-site wiring), PGW02002 (unused variable — analogous for `$` variables), PGE08009 (uncaptured required output at call site — the caller-side counterpart), PGW08003 (uncaptured output with default/fallback — caller-side warning counterpart)

**VALID:**
```polyglot
[ ] ✓ all declared outputs are pushed
{=} =Transform
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >result#string
   [=] >count#int
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
   [=] <status#int
   [=] >message#string
   [?] $status
      [?] ?[200,299]
         [r] >message << "ok"           [ ] ✓ pushed in this branch
      [?] *?
         [r] >message << "error"        [ ] ✓ pushed in this branch
```

**WARNING:**
```polyglot
[ ] ⚠ PGW02003 — >forgotten declared but never pushed
{=} =Incomplete
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >result#string
   [=] >forgotten#string               [ ] ⚠ PGW02003 — never pushed to
   [r] >result << $input
```

```polyglot
[ ] ⚠ PGW02003 — >count only pushed in one branch
{=} =PartialPush
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <flag#bool
   [=] >result#string
   [=] >count#int                       [ ] ⚠ PGW02003 — not pushed in *? branch
   [?] $flag
      [?] =? .True
         [r] >result << "yes"
         [r] >count << 1
      [?] *?
         [r] >result << "no"
         [ ] >count never pushed here
```

```polyglot
[ ] ⚠ PGW02003 — >debug declared but no code references it
{=} =Leftover
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] >out#string
   [=] >debug#string                    [ ] ⚠ PGW02003 — >debug never pushed to
   [r] =Process
      [=] <input << $data
      [=] >result >> >out
```

**Open point:** None.
