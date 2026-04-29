---
audience: automation-builder
type: specification
updated: 2026-04-16
status: draft
metadata_definition: "%definition.T:Git.Push"
metadata_instance: "%T:Git.Push:N"
---

# -T.Git.Push

Fires on a push to a branch. Supports branch and path filters. This is a **semantic trigger** — it works over both local hooks (`-T.Git.Hook`) and remote webhooks (`-T.Webhook`), with the runtime resolving the source based on `-Env.*` configuration.

## Definition

```aljam3
{N} -T.Git.Push
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TGitPush"
   [%] .description << "Fires on a push to a branch with optional filters."
   (-) <branch#array.string <~ {}
   (-) <paths#array.string <~ {}
   (-) >branch#string
   (-) >sha#string
   (-) >commits#array.Git.Commit
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<branch` | `#array.string` | Branch name patterns to match (OR logic). E.g. `"main"`, `"release/*"`. Empty = all branches. |
| `<paths` | `#array.string` | Path glob patterns to match changed files. E.g. `"src/**"`. Empty = all paths. |

### Filter Syntax

Filters are provided via `(-) >>` lines under the `[T]` block:

```aljam3
[T] -T.Git.Push
   (-) >> branch: "main", "develop"
   (-) >> paths: "src/**", "tests/**"
   (-) >branch >> <branch
   (-) >sha >> <sha
   (-) >commits >> <commits
```

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>branch` | `#string` | Branch name that was pushed to |
| `>sha` | `#string` | HEAD commit SHA after push |
| `>commits` | `#array.Git.Commit` | Commits included in the push |

## Errors

None.

## Permissions

System.Process (local mode) or Web.Socket (remote webhook mode).

## Queue Composability

- `-Q.Debounce` — batch rapid pushes into a single pipeline run
- `-Q.Allow.One` — prevent concurrent runs for the same branch

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.T:Git.Push` | Compile-time pipeline template |
| Instance | `%T:Git.Push:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/T/INDEX|-T.* Trigger Pipelines]]
- [[pglib/types/git|#Git Type Tree]] — `#Git.Commit` payload type
- [[pglib/pipelines/T/Git.Hook|-T.Git.Hook]] — transport layer for local pushes
