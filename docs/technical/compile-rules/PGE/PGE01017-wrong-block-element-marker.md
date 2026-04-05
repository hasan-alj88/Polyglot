---
audience: developer
rule: "1.17"
code: PGE01017
name: Wrong Block Element Marker
severity: error
---

### Rule 1.17 — Wrong Block Element Marker
`PGE01017`

**Statement:** A block element marker that is syntactically valid but used in the wrong context is a compile error. Specifically, `[=]` may only appear in the IO declaration section (for declaring inputs/outputs with optional inline assignment). Using `[=]` in the execution body for a standalone assignment is an error — `[r]` is the correct marker for process steps.
**Rationale:** `[=]` and `[r]` have distinct semantics: `[=]` declares an IO parameter and optionally binds it, while `[r]` executes a process step (pipeline call, assignment, output push). Confusing the two masks whether a line is a declaration or an operation, which breaks both compiler analysis and developer understanding.
**Detection:** The compiler checks each block element marker against its syntactic context. If `[=]` appears outside the IO declaration section (i.e., in the execution body, conditional branch, or expand scope), and the line is not wiring IO on a called pipeline, the compiler rejects it and suggests `[r]` instead.

**See also:** PGE01016 (Unmarked Execution Line — missing marker entirely), PGE01002 (IO Before Trigger — IO placement constraints)

**VALID:**
```polyglot
[ ] ✓ [=] for IO declaration, [r] for execution
{=} =Process
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >output#string
   [r] =Transform
      [=] <data << $input
      [=] >result >> $cleaned
   [r] >output << $cleaned
```

```polyglot
[ ] ✓ [=] inside [r] call — wiring IO on called pipeline
{=} =Caller
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >out#string
   [r] =Worker
      [=] <data << $input                     [ ] ✓ [=] wires IO on =Worker
      [=] >result >> >out                     [ ] ✓ [=] wires IO on =Worker
```

**INVALID:**
```polyglot
[ ] ✗ PGE01017 — [=] used for standalone assignment in execution body
{=} =Bad
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >result#string
   [=] >result << $input                      [ ] ✗ PGE01017 — should be [r]
```

```polyglot
[ ] ✗ PGE01017 — [=] used for output push inside conditional
{=} =AlsoBad
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <data#string
   [=] >output#string
   [?] $data =? "ok"
      [=] >output << $data                    [ ] ✗ PGE01017 — should be [r]
   [?] *?
      [=] >output << "error"                  [ ] ✗ PGE01017 — should be [r]
```

**Diagnostic:** "Wrong marker `[=]` at line N — use `[r]` for process steps; `[=]` is only valid for IO declaration"

**Open point:** None.

### See Also

- [[concepts/pipelines/execution|Execution]] — documents [=] vs [r] marker distinction, references PGE01017
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01017
