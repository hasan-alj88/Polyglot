---
audience: user
type: specification
updated: 2026-03-30
---

<!-- @concepts/pipelines/INDEX -->

## Trigger Definitions

`{T}` defines a trigger pipeline — a specialized subtype of `{=}` that contains only IO declarations. Triggers define event sources: they detect conditions and signal when a pipeline should fire. `{T}` is syntactic sugar for `{=}[T]`.

Every trigger must output `>IsTriggered#bool`. Triggers can produce additional outputs that wire to the consuming pipeline's inputs via indented `[=]` IO lines under `[T]`.

**Base trigger** — simplest form (stdlib):

```polyglot
{T} =T.Call
   [=] >IsTriggered#bool
```

**Composed trigger** — with inputs and additional outputs:

```polyglot
{T} =T.Folder.NewFiles
   [%] .description << "Fires when new files appear in watched directory"
   [=] <path#path
   [=] >IsTriggered#bool
   [=] >NewFiles#array:path
```

Trigger definitions have no execution body, no `[Q]`, and no `[W]` — they are IO-only. Stdlib triggers (`=T.*`) are base pipelines backed by native code — see [[concepts/pipelines/INDEX|Pipeline Structure]] for the base vs derived distinction.

## IO as Implicit Triggers

IO inputs act as implicit trigger gates based on their assignment operator:

| Assignment | Behavior |
|------------|----------|
| `<input << "value"` | **Constant** — always satisfied, value is locked |
| `<input <~ "value"` | **Has default** — uses default if all other triggers fire but nothing fills this input |
| `<input` (no assignment) | **Must be filled externally** — via caller or trigger wiring. Pipeline will not fire until this input reaches Final state |

There is no need to validate inputs with `[?]` checks — unfilled required inputs prevent the pipeline from triggering.

## Triggers

Every pipeline must have at least one `[T]` trigger — omitting it is a compile error (PGE01005).

- `=T.Call` — invoked when called from another pipeline
- Standard library triggers live under `=T.*` namespace — no `[@]` import needed (see [[packages#Usage]])
- Triggers with arguments: `=T.Daily"3AM"`, `=T.Webhook"/path"`, `=T.Folder.NewFiles"/dir/"`
If a trigger's boolean expression evaluates to the same value for all combinations of trigger states, it is a tautology or contradiction (PGE01018).

- Triggers that produce outputs wire them to pipeline inputs via indented `[=]` IO lines:

```polyglot
[=] <NewFiles#array:path
[T] =T.Folder.NewFiles"/inbox/"
   [=] >NewFiles >> <NewFiles
```

## See Also

- [[concepts/pipelines/INDEX|Pipeline Structure]] — full pipeline element ordering
- [[concepts/pipelines/queue|Queue]] — queue configuration that follows triggers
- [[concepts/pipelines/inline-calls|Inline Calls]] — trigger string argument syntax
- [[technical/ebnf/09-definition-blocks#9.4a|EBNF §9.4a]] — formal trigger definition grammar
