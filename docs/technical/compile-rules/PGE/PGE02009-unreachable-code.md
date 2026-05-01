---
audience: developer
rule: "2.9"
code: PGE02009
name: Unreachable Code
severity: error
---

# Rule 2.9 — Unreachable Code
`PGE02009`

<!-- @u:syntax/blocks -->
<!-- @u:concepts/conditionals -->
<!-- @u:syntax/operators -->

**Statement:** Code that appears after all output ports have reached Final state in every code path is unreachable and produces a compile error. The pipeline has terminated — no further statements can have any effect.
**Rationale:** Unlike dead code that merely wastes space (PGW02005), unreachable code after full pipeline termination indicates a logic error. The developer believes work is being done, but the pipeline has already completed. This is an error rather than a warning because it reveals a misunderstanding of control flow that should be fixed before deployment. Aljam3's exhaustive coverage model means every code path must be intentional — unreachable code indicates the developer's mental model diverges from the pipeline's actual behaviour.
**Detection:** The compiler tracks the lifecycle state of all output ports through control flow analysis. When every output port has been pushed to Final — either directly or through exhaustive conditional branches where every branch pushes all remaining ports Final — the pipeline is considered terminated. Any subsequent executable statement triggers PGE02009.

**See also:** PGE02003 (Final Is Push-Once — individual port violation), PGE02008 (Access After Release — scope-based unreachability), PGW02005 (Unreachable Code warning — softer diagnostic for the same pattern)

**VALID:**
```aljam3
[ ] ✓ code between partial Final pushes — still reachable
{-} -MultiStage
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >main#string
   (-) >log#string
   [ ]
   [-] >main << $input                         [ ] >main is Final
   [ ] ✓ reachable — >log is still open
   [-] -Format
      (-) <msg << "processed: {$input}"
      (-) >result >> $logMsg
   [-] >log << $logMsg                         [ ] >log is now Final
```

**INVALID:**
```aljam3
[ ] ✗ PGE02009 — code after single output pushed Final
{-} -DeadAfterFinal
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >result#string
   [ ]
   [-] >result << $input                       [ ] >result is now Final
   [-] -Log                                    [ ] ✗ PGE02009 — unreachable
      (-) <msg << "this never runs"
```

```aljam3
[ ] ✗ PGE02009 — code after exhaustive conditional terminates all outputs
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
   [-] -Log                                    [ ] ✗ PGE02009 — unreachable
      (-) <msg << "dead code"
```

```aljam3
[ ] ✗ PGE02009 — multiple outputs, all Final
{-} -MultiOutDead
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <input#string
   (-) >main#string
   (-) >log#string
   [ ]
   [-] >main << $input                         [ ] >main is Final
   [-] >log << "done"                          [ ] >log is Final
   [ ] all output ports are now Final
   [-] -Cleanup                                [ ] ✗ PGE02009 — unreachable
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

**Diagnostic:** "Unreachable code at line N — all output ports reached Final; pipeline has terminated. Move post-finalization work to `[/]` cleanup"

**Open point:** None.

## See Also

- [[concepts/variable-lifecycle|Variable Lifecycle]] — references PGE02009 for code after variable release
