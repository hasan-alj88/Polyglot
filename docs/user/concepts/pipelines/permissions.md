---
audience: automation-builder
type: specification
updated: 2026-04-17
---

<!-- @concepts/pipelines/INDEX -->
<!-- @u:philosophy/cybersecurity -->
<!-- @u:philosophy/accountability -->

## Permissions

<!-- @c:permissions -->
Pipelines declare permissions by referencing named `{_}` grant objects via `(-)` IO lines. Permission IO goes with other `(-)` lines, before `[T]`, `[Q]`, `[W]`, and data IO. See [[permissions]] for the full permission system, `{_}` object syntax, and the Ceiling vs Grant model.

```aljam3
{_} _LogGrant
   [.] .intent << #Grant
   [.] .category #File
   [.] .capability #Read
   [.] .scope "/var/log/app/*.log"
   [.] .path "/var/log/app/current.log"

{-} -AnalyzeLogs
   (-) _LogGrant
   [T] -T.CLI
   [Q] -Q.Default
   [W] -W.Aljam3
   (-) <logPath#path
   (-) >summary#string

   [ ]
   [-] -File.Text.Read
      (-) <path << _LogGrant
      (-) >content >> $content
   [-] >summary << ...
```

The pipeline receives the whole `_` permission object — `-File.Text.Read` extracts `.path` from `_LogGrant`. The permission object carries both the grant (what you're allowed to do) and the resource locator (where).

- **Grant must be a subset of ceiling** — every `{_}` grant referenced by a pipeline must fall within the package `{@}` ceiling (PGE10001). See [[packages#Permissions]] for ceiling rules.
- **Narrowing allowed, expanding not** — a grant can request fewer capabilities than the ceiling allows, but never capabilities outside the ceiling.
- **No `_` IO = pure computation** — a pipeline with no `_` IO declarations cannot perform IO, even if the package has a ceiling. Any IO call is a compile error (PGE10004).
- **Explicit request** — permissions are never inherited from the package. Each pipeline must declare the `{_}` grant objects it needs via `(-)` IO.

## See Also

- [[permissions]] — full `{_}` permission object system, `_`/`__`/`___` prefixes, per-category enums
- [[concepts/pipelines/INDEX|Pipeline Structure]] — where permission IO fits in pipeline element order
- [[concepts/pipelines/metadata|Pipeline Metadata]] — `[%]` metadata that precedes permissions
