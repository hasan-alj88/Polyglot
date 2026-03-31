---
rule: "2.9"
code: PGE02009
name: Unreachable Code
severity: error
---

### Rule 2.9 — Unreachable Code
`PGE02009`

**Statement:** Code that appears after all output ports have reached Final state in every code path is unreachable and produces a compile error. The pipeline has terminated — no further statements can have any effect.
**Rationale:** Unlike dead code that merely wastes space (PGW02005), unreachable code after full pipeline termination indicates a logic error. The developer believes work is being done, but the pipeline has already completed. This is an error rather than a warning because it reveals a misunderstanding of control flow that should be fixed before deployment.
**Detection:** The compiler tracks the lifecycle state of all output ports through control flow analysis. When every output port has been pushed to Final — either directly or through exhaustive conditional branches where every branch pushes all remaining ports Final — the pipeline is considered terminated. Any subsequent executable statement triggers PGE02009.

**See also:** PGE02003 (Final Is Push-Once — individual port violation), PGE02008 (Access After Release — scope-based unreachability), PGW02005 (Unreachable Code warning — softer diagnostic for the same pattern)

**VALID:**
```polyglot
[ ] ✓ code between partial Final pushes — still reachable
{=} =MultiStage
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >main#string
   [=] >log#string
   [r] >main << $input                         [ ] >main is Final
   [ ] ✓ reachable — >log is still open
   [r] =Format
      [=] <msg << "processed: {$input}"
      [=] >result >> $logMsg
   [r] >log << $logMsg                         [ ] >log is now Final
```

**INVALID:**
```polyglot
[ ] ✗ PGE02009 — code after single output pushed Final
{=} =DeadAfterFinal
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >result#string
   [r] >result << $input                       [ ] >result is now Final
   [r] =Log                                    [ ] ✗ PGE02009 — unreachable
      [=] <msg << "this never runs"
```

```polyglot
[ ] ✗ PGE02009 — code after exhaustive conditional terminates all outputs
{=} =DeadAfterExhaustive
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] >output#string
   [?] $data =? "ok"
      [r] >output << $data
   [?] *?
      [r] >output << "error"
   [ ] >output is Final in all paths
   [r] =Log                                    [ ] ✗ PGE02009 — unreachable
      [=] <msg << "dead code"
```

```polyglot
[ ] ✗ PGE02009 — multiple outputs, all Final
{=} =MultiOutDead
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >main#string
   [=] >log#string
   [r] >main << $input                         [ ] >main is Final
   [r] >log << "done"                          [ ] >log is Final
   [ ] all output ports are now Final
   [r] =Cleanup                                [ ] ✗ PGE02009 — unreachable
      [=] <data << $input
```

**Fix:** If post-finalization work is needed (logging, cleanup, resource release), move it to the `[/]` cleanup section. Cleanup executes after all output ports are finalized (or after the execution scope ends if the pipeline has no outputs).

**VALID (fix):**
```polyglot
[ ] ✓ post-finalization work in [/] cleanup
{=} =CorrectCleanup
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >result#string
   [r] >result << $input                       [ ] >result is Final
   [ ] ✓ no dead code — cleanup handles post-finalization work
   [/] cleanup
      [r] =Log
         [=] <msg << "processing complete"
```

**Diagnostic:** "Unreachable code at line N — all output ports reached Final; pipeline has terminated. Move post-finalization work to `[/]` cleanup"

**Open point:** None.

### See Also

- [[concepts/variable-lifecycle|Variable Lifecycle]] — references PGE02009 for code after variable release
