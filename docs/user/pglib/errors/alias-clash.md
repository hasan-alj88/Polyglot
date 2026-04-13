---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# `!Alias.Clash`

<!-- @c:errors -->

`!Alias.Clash` is a compile error raised when an alias collides with an existing name in the target namespace. Aliases place definitions at multiple locations in the `%` metadata tree; when a target location is already occupied, this error fires.

## `[<] !Alias.Clash` Fallback Chain

In `{#}` generic type definitions, the `(#) <Alias` parameter can provide a fallback chain of alternative alias values using `[<] !Alias.Clash`. The compiler tries each value in order until one succeeds:

```polyglot
(#) <Alias << "int"
   [<] !Alias.Clash << "integer"
   [<] !Alias.Clash << "Integer"
```

- First, the compiler tries `"int"` as the alias
- If `"int"` clashes with an existing name in the target namespace, `!Alias.Clash` fires and the compiler tries `"integer"`
- If `"integer"` also clashes, the compiler tries `"Integer"`
- If all alternatives are exhausted, the compile error propagates (unrecoverable)

This pattern provides robust alias resolution for scalar type definitions like `##Int`, `##Float`, etc.
