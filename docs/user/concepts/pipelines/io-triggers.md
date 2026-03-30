---
audience: user
type: specification
updated: 2026-03-30
---

<!-- @concepts/pipelines/INDEX -->

## IO as Implicit Triggers

IO inputs act as implicit trigger gates based on their assignment operator:

| Assignment | Behavior |
|------------|----------|
| `<input << "value"` | **Constant** — always satisfied, value is locked |
| `<input <~ "value"` | **Has default** — uses default if all other triggers fire but nothing fills this input |
| `<input` (no assignment) | **Must be filled externally** — via caller or trigger wiring. Pipeline will not fire until this input reaches Final state |

There is no need to validate inputs with `[?]` checks — unfilled required inputs prevent the pipeline from triggering.

## Triggers

Every pipeline must have at least one `[t]` trigger — omitting it is a compile error (PGE-105).

- `=T.Call` — invoked when called from another pipeline
- Standard library triggers live under `=T.*` namespace — no `[@]` import needed (see [[packages#Usage]])
- Triggers with arguments: `=T.Daily"3AM"`, `=T.Webhook"/path"`, `=T.Folder.NewFiles"/dir/"`
If a trigger's boolean expression evaluates to the same value for all combinations of trigger states, it is a tautology or contradiction (PGE-118).

- Triggers that produce outputs wire them to pipeline inputs via indented `[=]` IO lines:

```polyglot
[=] <NewFiles#array:path
[t] =T.Folder.NewFiles"/inbox/"
   [=] >NewFiles >> <NewFiles
```

## See Also

- [[concepts/pipelines/INDEX|Pipeline Structure]] — full pipeline element ordering
- [[concepts/pipelines/queue|Queue]] — queue configuration that follows triggers
- [[concepts/pipelines/inline-calls|Inline Calls]] — trigger string argument syntax
