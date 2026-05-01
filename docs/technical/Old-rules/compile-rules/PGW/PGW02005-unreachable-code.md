> [!WARNING]
> **DEPRECATED:** This general rule has been deprecated and split into specific edge-case rules for stricter compiler enforcement.

---
audience: developer
rule: "2.9"
code: PGW02005
name: Unreachable Code
severity: warning
---

# Rule 2.9 — Unreachable Code
`PGW02005`

<!-- @u:syntax/blocks -->
<!-- @u:concepts/conditionals -->
<!-- @u:syntax/operators -->

**Statement:** Executable statements that appear after a terminal operation — where all output ports have been pushed to Final in every code path, or where every branch of a conditional terminates — produce a warning. The code is syntactically valid but can never execute.
**Rationale:** Unreachable code is dead weight that misleads developers about what the pipeline actually does. This is a warning rather than an error because the pipeline is structurally complete and the unreachable statements do not affect correctness.
**Detection:** The compiler performs basic control flow analysis after each statement. When all output ports reach Final state (directly or through exhaustive conditional branches), any subsequent statements are flagged as unreachable.

**See also:** PGE02009 (Unreachable Code error — same detection, error severity), PGW01001 (Empty Execution Body — covers empty bodies; this rule covers non-empty bodies with dead code)

**VALID:**
```aljam3
[ ] ✓ code after non-terminal conditional — reachable
{-} -Process
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >output#string
   [ ]
   [?] $input =? "fast"
      [-] -ProcessFast
         (-) <data << $input
         (-) >result >> $fast
   [?] *?
      [-] -ProcessSlow
         (-) <data << $input
         (-) >result >> $slow
   [ ] ✓ reachable — conditional didn't push >output Final
   [-] -Log
      (-) <msg << "processing complete"
   [-] >output << $fast
```

```aljam3
[ ] ✓ multiple outputs — only one is Final, code still reachable
{-} -MultiOut
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >main#string
   (-) >log#string
   [ ]
   [-] >main << $input                          [ ] >main is Final
   [ ] ✓ reachable — >log is still open
   [-] -Format
      (-) <msg << "processed: {$input}"
      (-) >result >> >log                      [ ] >log is now Final
```

**WARNING:**
```aljam3
[ ] ⚠ PGW02005 — statement after output is Final
{-} -DeadAfterFinal
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >result#string
   [ ]
   [-] >result << $input                        [ ] >result is now Final
   [-] -Log                                    [ ] ⚠ PGW02005 — unreachable
      (-) <msg << "this never runs"
```

```aljam3
[ ] ⚠ PGW02005 — code after every branch terminates
{-} -DeadAfterExhaustive
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <data#string
   (-) >output#string
   [ ]
   [?] $data =? "ok"
      [-] >output << $data
   [?] *?
      [-] >output << "error"
   [ ] >output is Final in all paths
   [-] -Log                                    [ ] ⚠ PGW02005 — unreachable
      (-) <msg << "dead code"
```

```aljam3
[ ] ⚠ PGW02005 — all outputs Final across multiple pushes
{-} -MultiOutDead
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >main#string
   (-) >log#string
   [ ]
   [-] >main << $input                          [ ] >main is Final
   [-] >log << "done"                           [ ] >log is Final
   [ ] all output ports are now Final
   [-] -Cleanup                                [ ] ⚠ PGW02005 — unreachable
      (-) <data << $input
```

**Fix:** If post-finalization work is needed (logging, cleanup, resource release), move it to the `[/]` cleanup section. Cleanup executes after all output ports are finalized (or after the execution scope ends if the pipeline has no outputs).

**VALID (fix):**
```aljam3
[ ] ✓ post-finalization work in [/] cleanup
{-} -CorrectCleanup
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >result#string
   [ ]
   [-] >result << $input                       [ ] >result is Final
   [ ] ✓ no dead code — cleanup handles post-finalization work
   [/] cleanup
      [-] -Log
         (-) <msg << "processing complete"
```

**Diagnostic:** "Unreachable code after line N — all output ports are Final. Move post-finalization work to `[/]` cleanup"

**Open point:** None.
