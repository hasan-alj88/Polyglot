---
audience: developer
rule: "2.3"
code: PGW02003
name: Unpushed Output Port
severity: warning
---

# Rule 2.3 — Unpushed Output Port
`PGW02003`

<!-- @u:syntax/blocks -->
<!-- @u:concepts/conditionals -->
<!-- @u:syntax/operators -->

**Statement:** An output `>name` declared in a pipeline's `(-)` IO section that is never pushed to by any code path is flagged with a warning. The output will remain in Declared state (never reach Final), which may block downstream pipelines waiting for it.
**Rationale:** A declared but unpushed output port is likely a wiring oversight. Callers expecting this output will see a variable stuck in Declared state, preventing downstream triggers from firing. This is a warning rather than an error because the output might be intentionally optional in some designs.
**Detection:** The compiler analyzes all code paths within the pipeline body and checks whether each declared `>name` output has at least one push (`<<` or `<~`) on every code path. If any output has zero pushes on any path, the warning is emitted.

**See also:** PGE02003 (final is push-once — covers push-once semantics), PGE01010 (pipeline IO name mismatch — covers call-site wiring), PGW02002 (unused variable — analogous for `$` variables), PGE08009 (uncaptured required output at call site — the caller-side counterpart), PGW08003 (uncaptured output with default/fallback — caller-side warning counterpart)

**VALID:**
```aljam3
[ ] ✓ all declared outputs are pushed
{-} -Transform
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >result#string
   (-) >count#int
   [ ]
   [-] -Parse
      (-) <data << $input
      (-) >parsed >> $clean
   [-] >result << $clean
   [-] >count << 1
```

```aljam3
[ ] ✓ output pushed in all conditional branches
{-} -Route
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <status#int
   (-) >message#string
   [ ]
   [?] $status
      [?] ?[200,299]
         [-] >message << "ok"           [ ] ✓ pushed in this branch
      [?] ?*?
         [-] >message << "error"        [ ] ✓ pushed in this branch
```

**WARNING:**
```aljam3
[ ] ⚠ PGW02003 — >forgotten declared but never pushed
{-} -Incomplete
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >result#string
   (-) >forgotten#string               [ ] ⚠ PGW02003 — never pushed to
   [ ]
   [-] >result << $input
```

```aljam3
[ ] ⚠ PGW02003 — >count only pushed in one branch
{-} -PartialPush
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <flag#bool
   (-) >result#string
   (-) >count#int                       [ ] ⚠ PGW02003 — not pushed in ?* branch
   [ ]
   [?] $flag
      [?] ?= .True
         [-] >result << "yes"
         [-] >count << 1
      [?] ?*?
         [-] >result << "no"
         [ ] >count never pushed here
```

```aljam3
[ ] ⚠ PGW02003 — >debug declared but no code references it
{-} -Leftover
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <data#string
   (-) >out#string
   (-) >debug#string                    [ ] ⚠ PGW02003 — >debug never pushed to
   [ ]
   [-] -Process
      (-) <input << $data
      (-) >result >> >out
```

**Open point:** None.
