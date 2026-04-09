---
audience: developer
rule: "1.2"
code: PGE01002
name: IO Before Trigger
---

### Rule 1.2 — IO Before Trigger
`PGE01002`

**Statement:** IO input declarations (`[=] <param`) must appear **positionally before** all `[T]` trigger lines. Within the trigger/IO section, all `[=]` declarations come first, then all `[T]` lines.
**Rationale:** IO declarations and triggers form one section but have a fixed internal order: declarations first, triggers second. The IO variable must exist before it can be assigned, and this is enforced as a textual ordering requirement — not just a semantic dependency on which trigger pushes where. `[W]` is wrapper invocation (setup/cleanup scope from `{W}`), not wiring — this rule applies to `[T]` only.

**VALID:**
```polyglot
[ ] ✓ <filepath declared before [T] pushes into it
{=} =Process
   [=] <filepath#path
   [T] =T.Folder.NewFiles"/inbox/"
      [=] >NewFiles >> <filepath
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =File.Text.Read
      [=] <path << <filepath
      [=] >content >> >content
```

**INVALID:**
```polyglot
[ ] ✗ PGE01002 — [T] pushes into <filepath before it is declared
{=} =Process
   [T] =T.Folder.NewFiles"/inbox/"
      [=] >NewFiles >> <filepath    [ ] ✗ PGE01002
   [=] <filepath#path
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =File.Text.Read
      [=] <path << <filepath
      [=] >content >> >content
```

### See Also

- [[concepts/pipelines/INDEX|Pipelines Overview]] — references PGE01002 in IO declaration ordering
- [[concepts/pipelines/inline-calls|Inline Calls]] — compile rule quick-reference table includes PGE01002
