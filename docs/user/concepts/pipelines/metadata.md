---
audience: user
type: specification
updated: 2026-03-30
---

<!-- @concepts/pipelines/INDEX -->

## Pipeline Metadata

Every pipeline carries implicit `live` metadata fields populated by the Polyglot runtime. Pipeline metadata lives at `%=:{name}:{instance}` in the unified tree — see [[data-is-trees#How Concepts Connect]]. Query built-in metadata via the `%` accessor instead of creating custom booleans. See [[metadata]] for the full metadata tree, field listings, and access patterns.

### Error Trees

Every failable pipeline **must** declare its errors with `[=] !ErrorName` in the IO section. This is the pipeline's error tree — a structured list of every error it can raise:

```polyglot
{=} =ValidateUser
   [=] <name#string
   [=] >validated#string
   [=] !Validation.Empty
   [=] !Validation.TooLong
   [t] =T.Call
   [Q] =Q.Default
   [W] =W.Polyglot
   ...
```

Error declarations are mandatory for failable pipelines. A pipeline without `[=] !...` is non-failable — the compiler warns (PGW-701) if a caller adds `[!]` handlers on it. Errors are raised in the execution body with `[!] >> !ErrorName` (see [[errors#Raising Errors]]). Custom error types are defined with `{!}` blocks (see [[errors#Defining Custom Errors]]). For stdlib pipeline error trees, see [[stdlib/errors/errors#Pipeline Error Associations]].

## See Also

- [[concepts/pipelines/error-handling|Error Handling]] — `[!]` block scoping and fallback patterns
- [[concepts/pipelines/permissions|Permissions]] — `[_]` permission lines that precede IO
- [[concepts/pipelines/INDEX|Pipeline Structure]] — full pipeline element ordering
