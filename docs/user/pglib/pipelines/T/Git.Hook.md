---
audience: automation-builder
type: specification
updated: 2026-04-16
status: draft
metadata_definition: "%definition.T:Git.Hook"
metadata_instance: "%T:Git.Hook:N"
---

# -T.Git.Hook

Fires on a local Git hook invocation. Hook name provided via inline call: `-T.Git.Hook"post-commit"`.

This is a **transport-tier** trigger — it receives raw hook events from the local `.git/hooks/` dispatcher. For higher-level semantic triggers that work across local and remote sources, see `-T.Git.Push`, `-T.Git.PR`, and `-T.Git.Tag`.

## Activation

The Aljam3 runtime installs a thin shell dispatcher into `.git/hooks/` (or uses Git's `core.hooksPath`). When Git fires a hook:

1. Git runs `.git/hooks/<hook-name>` (the installed dispatcher)
2. Dispatcher POSTs to `localhost:<port>/hooks/<hook-name>` with hook arguments
3. Aljam3 runtime receives the HTTP request — same code path as `-T.Webhook`
4. Runtime fires matching `-T.Git.Hook` triggers

## Definition

```aljam3
{N} -T.Git.Hook
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TGitHook"
   [%] .description << "Fires on a local Git hook invocation."
   (-) %InlineString << "{hookName}"
   (-) <hookName#string <~ ""
   (-) >sha#string
   (-) >files#array.Git.DiffStat
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `hookName` | `#string` | Git hook name (e.g. `"post-commit"`, `"pre-push"`). Provided inline: `-T.Git.Hook"post-commit"`. Defaults to `""`. |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>sha` | `#string` | Current HEAD commit SHA at time of hook invocation |
| `>files` | `#array.Git.DiffStat` | Files changed in the triggering operation |

## Errors

None.

## Permissions

System.Process — required to install the hook dispatcher script.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.T:Git.Hook` | Compile-time pipeline template |
| Instance | `%T:Git.Hook:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/T/INDEX|-T.* Trigger Pipelines]]
- [[pglib/types/git|#Git Type Tree]] — event payload types
- [[pglib/pipelines/T/Git.Push|-T.Git.Push]] — semantic push trigger (uses Hook as transport)
