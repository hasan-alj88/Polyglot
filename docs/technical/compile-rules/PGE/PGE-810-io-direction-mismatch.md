---
rule: "8.10"
code: PGE-810
name: IO Direction Mismatch
severity: error
---

### Rule 8.10 — IO Direction Mismatch
`PGE-810`

**Statement:** At a call site, input ports (`<name`) must use `<<` (push value in) and output ports (`>name`) must use `>>` (pull value out). Using the wrong assignment direction — pushing into an output or pulling from an input — is a compile error. Inside the pipeline body, the pipeline may push to its own output ports (`>name << value`) because it is writing its own results.
**Rationale:** Inputs are destinations (the caller sends data in) and outputs are sources (the caller receives data out). Reversing the direction is always a bug — it means the developer confused which port they are wiring. This is distinct from PGE-110 (name mismatch) and PGE-807 (invalid assignment target).
**Detection:** The compiler checks each IO line under a `[r]` call site. If a `<name` port uses `>>` or a `>name` port uses `<<`, PGE-810 fires.

**See also:** PGE-110 (pipeline IO name mismatch), PGE-807 (invalid assignment target), PGE-808 (missing required input), PGE-809 (uncaptured required output)

**VALID:**
```polyglot
[ ] ✓ correct IO direction at call site
[r] =File.Text.Read
   [=] <path << $filepath                     [ ] ✓ input << push value in
   [=] >content >> $text                       [ ] ✓ output >> pull value out
```

```polyglot
[ ] ✓ pipeline body writes its own output port — allowed
{=} =Compute
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <x#int
   [=] >result#int
   [r] >result << $x
```

**INVALID:**
```polyglot
[ ] ✗ PGE-810 — reversed IO direction at call site
[r] =File.Text.Read
   [=] >content << $filepath                  [ ] ✗ PGE-810 — output is a source, not a destination
   [=] <path >> $text                          [ ] ✗ PGE-810 — input is a destination, not a source
```

```polyglot
[ ] ✗ PGE-810 — pushing into output at call site
[r] =Format
   [=] <text << $input                         [ ] ✓ correct
   [=] >formatted << $overwrite                [ ] ✗ PGE-810 — cannot push into output at call site
```

```polyglot
[ ] ✗ PGE-810 — pulling from input at call site
[r] =Process
   [=] <data >> $leak                          [ ] ✗ PGE-810 — cannot pull from input at call site
   [=] >result >> $out                         [ ] ✓ correct
```

**Diagnostic:** `"IO direction mismatch at line {N} — {<input|>output} <{name}> on call to ={PipelineName} uses {<<|>>} but {inputs accept << only|outputs provide >> only} at call sites"`

### See Also

- [[user/concepts/pipelines/inline-calls|Inline Calls]] — references PGE-810 in call site rules

**Open point:** None.
