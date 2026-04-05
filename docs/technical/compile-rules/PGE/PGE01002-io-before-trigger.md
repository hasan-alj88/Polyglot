---
audience: developer
rule: "1.2"
code: PGE01002
name: IO Before Trigger
---

### Rule 1.2 — IO Before Trigger
`PGE01002`

**Statement:** IO input declarations (`[=] <param`) must appear **before** any `[T]` trigger line that pushes into them. A trigger cannot reference an undeclared IO parameter.
**Rationale:** The IO variable must exist before it can be assigned. `[W]` is macro invocation (setup/cleanup scope from `{M}`), not wiring — this rule applies to `[T]` only.

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
