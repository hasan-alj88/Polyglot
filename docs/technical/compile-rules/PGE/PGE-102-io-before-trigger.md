---
rule: "1.2"
code: PGE-102
name: IO Before Trigger
---

### Rule 1.2 — IO Before Trigger
`PGE-102`

**Statement:** IO input declarations (`[=] <param`) must appear **before** any `[t]` trigger line that pushes into them. A trigger cannot reference an undeclared IO parameter.
**Rationale:** The IO variable must exist before it can be assigned. `[W]` is macro invocation (setup/cleanup scope from `{M}`), not wiring — this rule applies to `[t]` only.

**VALID:**
```polyglot
[ ] ✓ <filepath declared before [t] pushes into it
{=} =Process
   [=] <filepath#path
   [t] =T.Folder.NewFiles"/inbox/"
      [=] >NewFiles >> <filepath
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =File.Text.Read
      [=] <path << <filepath
      [=] >content >> >content
```

**INVALID:**
```polyglot
[ ] ✗ PGE-102 — [t] pushes into <filepath before it is declared
{=} =Process
   [t] =T.Folder.NewFiles"/inbox/"
      [=] >NewFiles >> <filepath    [ ] ✗ PGE-102
   [=] <filepath#path
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =File.Text.Read
      [=] <path << <filepath
      [=] >content >> >content
```
