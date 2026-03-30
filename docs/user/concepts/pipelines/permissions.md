---
audience: user
type: specification
updated: 2026-03-30
---

<!-- @concepts/pipelines/INDEX -->

## Permissions

<!-- @permissions -->
Pipelines can declare `[_]` permission lines after the `{=}` header (and `[%]` metadata, if present), before `[t]`, `[Q]`, `[W]`, and IO. The same applies to all `{x}` definitions (`{M}` macros, etc.). See [[permissions]] for the full permission system.

```polyglot
{=} =AnalyzeLogs
   [_] _File.read"/var/log/*.log"
   [t] =T.Manual
   [Q] =Q.Default
   [W] =W.Polyglot
   [=] <logPath#path
   [=] >summary#string
   [r] $content << =File.Text.Read >> "{$logPath}"
   [r] >summary << ...
```

- **Subset of ceiling** — every `[_]` in a pipeline must fall within the package `{@}` ceiling (PGE10001). See [[packages#Permissions]] for ceiling rules.
- **No `[_]` = pure computation** — a pipeline with no `[_]` lines cannot perform IO, even if the package has a ceiling. Any IO call is a compile error.
- **Explicit request** — permissions are never inherited from the package. Each pipeline must declare what it needs.

## See Also

- [[concepts/pipelines/INDEX|Pipeline Structure]] — where `[_]` fits in pipeline element order
- [[concepts/pipelines/metadata|Pipeline Metadata]] — `[%]` metadata that precedes permissions
