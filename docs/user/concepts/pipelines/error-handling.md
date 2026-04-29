---
audience: automation-builder
type: specification
updated: 2026-03-30
---

<!-- @concepts/pipelines/INDEX -->

## Error Handling

`[!]` error blocks are scoped to the specific `[-]` call that can produce them, indented under the call (after its `(-)` IO lines):

```aljam3
[-] @FS-File.Text.Read
   (-) <path << <filepath
   (-) >content >> >content
   [!] !File.NotFound
      [-] >content << "Error: file not found"
   [!] !File.ReadError
      [-] >content << "Error: could not read file"
```

| Pattern | Pipeline continues? | Variable state |
|---------|-------------------|---------------|
| `[!]` pushes replacement (`<<`/`>>`) | Yes | Always Final |
| `(>) !>` fallback on IO line | Yes | Always Final — fallback value used |
| `(>) !ErrorName>` specific error fallback | Yes | Always Final — targeted fallback |

For simple "on error, use this value" cases, use `(>) !>` fallback under the `(-)` output line:

```aljam3
[-] -File.Text.Read
   (-) <path << $file
   (-) >content >> $out
      (>) !> "fallback value"
      (>) !File.NotFound> "file not found"
```

`[!]` blocks run first; `!<` catches what `[!]` didn't handle. The compiler enforces exhaustive error handling (PGE07007) — every failable call must have either an `[!]` block or `!<`/`!>` fallback. No variable may compile if it can reach Failed state without explicit handling. For the full error model — chain error addressing, fallback operators, standard error trees, and the Failed state — see [[errors]]. Errors live at the `%!` branch of the metadata tree (see [[data-is-trees#How Concepts Connect]]).

## See Also

- [[concepts/pipelines/chains|Chain Execution]] — error addressing with step indices in chains
- [[concepts/pipelines/metadata|Pipeline Metadata]] — error tree declarations with `(-) !ErrorName`
- [[concepts/pipelines/execution|Execution]] — execution body where `[!]` blocks are scoped
