---
audience: automation-builder
type: specification
updated: 2026-04-05
---

<!-- @concepts/pipelines/INDEX -->

## Permissions

<!-- @c:permissions -->
Pipelines declare permissions by referencing named `{_}` grant objects via `[_]` lines. `[_]` lines go after the `{-}` header (and `[%]` metadata, if present), before `[T]`, `[Q]`, `[W]`, and IO. See [[permissions]] for the full permission system, `{_}` object syntax, and the Ceiling vs Grant model.

```polyglot
{_} _LogGrant
   [.] .intent << #Grant
   [.] .File.Read "/var/log/app/*.log"

{-} -AnalyzeLogs
   [_] _LogGrant
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   (-) <logPath#path
   (-) >summary#string
   [-] $content << -File.Text.Read >> "{$logPath}"
   [-] >summary << ...
```

- **Grant must be a subset of ceiling** — every `{_}` grant referenced by a pipeline must fall within the package `{@}` ceiling (PGE10001). See [[packages#Permissions]] for ceiling rules.
- **Narrowing allowed, expanding not** — a grant can request fewer capabilities than the ceiling allows, but never capabilities outside the ceiling.
- **No `[_]` = pure computation** — a pipeline with no `[_]` lines cannot perform IO, even if the package has a ceiling. Any IO call is a compile error.
- **Explicit request** — permissions are never inherited from the package. Each pipeline must reference the `{_}` grant objects it needs.

## See Also

- [[permissions]] — full `{_}` permission object system, `_`/`__`/`___` prefixes, per-category enums
- [[concepts/pipelines/INDEX|Pipeline Structure]] — where `[_]` fits in pipeline element order
- [[concepts/pipelines/metadata|Pipeline Metadata]] — `[%]` metadata that precedes permissions
