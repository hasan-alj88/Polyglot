---
audience: developer
rule: "9.3"
code: PGE09003
name: Unresolved Pipeline Reference
severity: error
---

### Rule 9.3 — Unresolved Pipeline Reference
`PGE09003`

**Statement:** Every `=Pipeline` reference in an `[r]` or `[p]` call must resolve to either a stdlib pipeline or a `{=}` definition within the same package. If the reference matches neither, it is a compile error. Cross-package pipelines must use `@alias=Pipeline` syntax with a valid `[@]` import (see PGE09001). This rule does not apply to `@alias=Pipeline` references — those are validated by PGE09001 (alias resolution) and then by the imported package's own definitions.
**Rationale:** Calling a pipeline that doesn't exist is always a bug — a typo, a deleted definition, or a missing import. Catching unresolved references at compile time prevents runtime lookup failures.
**Detection:** The compiler collects all `{=}` definitions in the current package and the stdlib pipeline set. When an `[r]` or `[p]` call references `=Name` (without an `@` alias prefix), the compiler checks the name against both sets. If no match is found, PGE09003 fires.

**See also:** PGE09001 (undefined import alias — validates `@alias` prefix), PGE01010 (pipeline IO name mismatch — validated after the pipeline reference resolves)

**VALID:**
```polyglot
{@} @Local:999.MyApp:v1.0.0

{=} =Transform
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >output#string
   [r] >output << $input

[ ] ✓ =Transform is defined in the same package
{=} =Main
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =Transform
      [=] <input#string << "hello"
      [=] >output#string >> $result
```

```polyglot
[ ] ✓ stdlib pipelines are always available
{=} =Main
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =File.Text.Read
      [=] <path#path << $filePath
      [=] >content#string >> $text
```

**INVALID:**
```polyglot
{@} @Local:999.MyApp:v1.0.0

[ ] ✗ PGE09003 — =Process not defined in this package and not stdlib
{=} =Main
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =Process                            [ ] ✗ PGE09003 — no {=} =Process in package, not stdlib
      [=] <data#string << $input
```

```polyglot
{@} @Local:999.MyApp:v1.0.0

[ ] ✗ PGE09003 — typo in pipeline name
{=} =Transform
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <input#string
   [=] >output#string
   [r] >output << $input

{=} =Main
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =Transfrom                          [ ] ✗ PGE09003 — typo: =Transfrom not found (did you mean =Transform?)
      [=] <input#string << "hello"
```

```polyglot
{@} @Local:999.MyApp:v1.0.0

[ ] ✗ PGE09003 — cross-package pipeline without @alias
{=} =Main
   [T] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [r] =Validate                           [ ] ✗ PGE09003 — =Validate is in another package; use @alias=Validate with [@] import
      [=] <input#string << $data
```

### See Also

- [[user/syntax/packages|Packages]] — references PGE09003 in dependency rules

**Open point:** None.
