---
rule: "10.2"
code: PGW-1002
name: Missing Inline Format Metadata
severity: warning
---

### Rule 10.2 — Missing Inline Format Metadata
`PGW-1002`

**Statement:** When an inline pipeline call `=Foo"arg"` targets a pipeline that does not declare `.inlineFormat` metadata, the compiler emits a warning. The inline argument cannot be validated at compile time.
**Rationale:** Without `.inlineFormat`, the compiler cannot verify that the inline string argument is well-formed. The call may succeed or fail at runtime depending on whether the target pipeline can parse the argument. This warning encourages pipeline authors to declare format expectations for compile-time safety.
**Detection:** The compiler checks the target pipeline's `[%]` metadata for `.inlineFormat`. If absent and the pipeline is called with an inline argument, the warning is emitted.

**See also:** PGE-1002 (invalid inline pipeline argument — fires when format IS declared and doesn't match)

**WARNING:**
```polyglot
[ ] ⚠ PGW-1002 — no .inlineFormat declared on target pipeline
{=} =CustomPipeline
   [%] .description << "Does something"
   [ ] no .inlineFormat metadata
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >out;string
   [r] >out << $input

{=} =Caller
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] >out;string
   [r] =CustomPipeline"some arg"        [ ] ⚠ PGW-1002 — no format validation available
      [=] >result >> >out
```

**Open point:** None.
