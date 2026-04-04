---
audience: pg-coder
type: specification
updated: 2026-03-30
---

<!-- @concepts/pipelines/INDEX -->

## Error Handling

`[!]` error blocks are scoped to the specific `[r]` call that can produce them, indented under the call (after its `[=]` IO lines):

```polyglot
[r] @FS=File.Text.Read
   [=] <path << <filepath
   [=] >content >> >content
   [!] !File.NotFound
      [r] >content << "Error: file not found"
   [!] !File.ReadError
      [r] >content << "Error: could not read file"
```

| Pattern | Pipeline continues? | Variable state |
|---------|-------------------|---------------|
| `[!]` pushes replacement (`<<`/`>>`) | Yes | Always Final |
| `[!]` without replacement (default) | No — ends on error | Never Failed |
| `[!]` with `[*] *Continue >IsFailed >> $var` | Yes | May be Failed — handle via `$var` boolean |
| `[>] <!` fallback on IO line | Yes | Always Final — fallback value used |

For simple "on error, use this value" cases, use `[>] <!` fallback under the `[=]` output line:

```polyglot
[r] =File.Text.Read
   [=] <path << $file
   [=] >content >> $out
      [>] <! "fallback value"
      [>] <!File.NotFound "file not found"
```

`[!]` blocks run first; `<!` catches what `[!]` didn't handle. For the full error model — chain error addressing, `*Continue` recovery patterns, fallback operators, standard error trees, and the Failed state — see [[errors]]. Errors live at the `%!` branch of the metadata tree (see [[data-is-trees#How Concepts Connect]]).

## See Also

- [[concepts/pipelines/chains|Chain Execution]] — error addressing with step indices in chains
- [[concepts/pipelines/metadata|Pipeline Metadata]] — error tree declarations with `[=] !ErrorName`
- [[concepts/pipelines/execution|Execution]] — execution body where `[!]` blocks are scoped
