---
rule: "9.3"
code: PGE-903
name: Unresolved Pipeline Reference
severity: error
---

### Rule 9.3 — Unresolved Pipeline Reference
`PGE-903`

**Statement:** Every `=Pipeline` reference in an `[r]` or `[p]` call must resolve to either a stdlib pipeline or a `{=}` definition within the same package. If the reference matches neither, it is a compile error. Cross-package pipelines must use `@alias=Pipeline` syntax with a valid `[@]` import (see PGE-901). This rule does not apply to `@alias=Pipeline` references — those are validated by PGE-901 (alias resolution) and then by the imported package's own definitions.
**Rationale:** Calling a pipeline that doesn't exist is always a bug — a typo, a deleted definition, or a missing import. Catching unresolved references at compile time prevents runtime lookup failures.
**Detection:** The compiler collects all `{=}` definitions in the current package and the stdlib pipeline set. When an `[r]` or `[p]` call references `=Name` (without an `@` alias prefix), the compiler checks the name against both sets. If no match is found, PGE-903 fires.

**See also:** PGE-901 (undefined import alias — validates `@alias` prefix), PGE-110 (pipeline IO name mismatch — validated after the pipeline reference resolves)

**VALID:**
```polyglot
{@} @Local:999.MyApp:v1.0.0

{=} =Transform
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >output;string
   [r] >output << $input

[ ] ✓ =Transform is defined in the same package
{=} =Main
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =Transform
      [=] <input;string << "hello"
      [=] >output;string >> $result
```

```polyglot
[ ] ✓ stdlib pipelines are always available
{=} =Main
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =File.Text.Read
      [=] <path;path << $filePath
      [=] >content;string >> $text
```

**INVALID:**
```polyglot
{@} @Local:999.MyApp:v1.0.0

[ ] ✗ PGE-903 — =Process not defined in this package and not stdlib
{=} =Main
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =Process                            [ ] ✗ PGE-903 — no {=} =Process in package, not stdlib
      [=] <data;string << $input
```

```polyglot
{@} @Local:999.MyApp:v1.0.0

[ ] ✗ PGE-903 — typo in pipeline name
{=} =Transform
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input;string
   [=] >output;string
   [r] >output << $input

{=} =Main
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =Transfrom                          [ ] ✗ PGE-903 — typo: =Transfrom not found (did you mean =Transform?)
      [=] <input;string << "hello"
```

```polyglot
{@} @Local:999.MyApp:v1.0.0

[ ] ✗ PGE-903 — cross-package pipeline without @alias
{=} =Main
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =Validate                           [ ] ✗ PGE-903 — =Validate is in another package; use @alias=Validate with [@] import
      [=] <input;string << $data
```

**Open point:** None.
