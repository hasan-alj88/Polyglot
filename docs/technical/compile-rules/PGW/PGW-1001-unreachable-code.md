---
rule: "2.9"
code: PGW-1001
name: Unreachable Code
severity: warning
---

### Rule 2.9 — Unreachable Code
`PGW-1001`

**Statement:** Executable statements that appear after a terminal operation — where all output ports have been pushed to Final in every code path, or where every branch of a conditional terminates — produce a warning. The code is syntactically valid but can never execute.
**Rationale:** Unreachable code is dead weight that misleads developers about what the pipeline actually does. This is a warning rather than an error because the pipeline is structurally complete and the unreachable statements do not affect correctness.
**Detection:** The compiler performs basic control flow analysis after each statement. When all output ports reach Final state (directly or through exhaustive conditional branches), any subsequent statements are flagged as unreachable.

**See also:** PGE-209 (Unreachable Code error — same detection, error severity), PGW-101 (Empty Execution Body — covers empty bodies; this rule covers non-empty bodies with dead code)

**VALID:**
```polyglot
[ ] ✓ code after non-terminal conditional — reachable
{=} =Process
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >output;string
   [?] $input =? "fast"
      [r] =ProcessFast
         [=] <data << $input
         [=] >result >> $fast
   [?] *?
      [r] =ProcessSlow
         [=] <data << $input
         [=] >result >> $slow
   [ ] ✓ reachable — conditional didn't push >output Final
   [r] =Log
      [=] <msg << "processing complete"
   [r] >output << $fast
```

```polyglot
[ ] ✓ multiple outputs — only one is Final, code still reachable
{=} =MultiOut
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >main;string
   [=] >log;string
   [r] >main << $input                          [ ] >main is Final
   [ ] ✓ reachable — >log is still open
   [r] =Format
      [=] <msg << "processed: {$input}"
      [=] >result >> >log                      [ ] >log is now Final
```

**WARNING:**
```polyglot
[ ] ⚠ PGW-1001 — statement after output is Final
{=} =DeadAfterFinal
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >result;string
   [r] >result << $input                        [ ] >result is now Final
   [r] =Log                                    [ ] ⚠ PGW-1001 — unreachable
      [=] <msg << "this never runs"
```

```polyglot
[ ] ⚠ PGW-1001 — code after every branch terminates
{=} =DeadAfterExhaustive
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data;string
   [=] >output;string
   [?] $data =? "ok"
      [r] >output << $data
   [?] *?
      [r] >output << "error"
   [ ] >output is Final in all paths
   [r] =Log                                    [ ] ⚠ PGW-1001 — unreachable
      [=] <msg << "dead code"
```

```polyglot
[ ] ⚠ PGW-1001 — all outputs Final across multiple pushes
{=} =MultiOutDead
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >main;string
   [=] >log;string
   [r] >main << $input                          [ ] >main is Final
   [r] >log << "done"                           [ ] >log is Final
   [ ] all output ports are now Final
   [r] =Cleanup                                [ ] ⚠ PGW-1001 — unreachable
      [=] <data << $input
```

**Fix:** If post-finalization work is needed (logging, cleanup, resource release), move it to the `[/]` cleanup section. Cleanup executes after all output ports are finalized (or after the execution scope ends if the pipeline has no outputs).

**VALID (fix):**
```polyglot
[ ] ✓ post-finalization work in [/] cleanup
{=} =CorrectCleanup
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >result;string
   [r] >result << $input                       [ ] >result is Final
   [ ] ✓ no dead code — cleanup handles post-finalization work
   [/] cleanup
      [r] =Log
         [=] <msg << "processing complete"
```

**Diagnostic:** "Unreachable code after line N — all output ports are Final. Move post-finalization work to `[/]` cleanup"

**Open point:** None.
