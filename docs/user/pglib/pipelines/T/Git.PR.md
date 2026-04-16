---
audience: automation-builder
type: specification
updated: 2026-04-16
status: draft
metadata_definition: "%definition.T:Git.PR"
metadata_instance: "%T:Git.PR:N"
---

# -T.Git.PR

Fires on pull request events. This is a **semantic trigger** — remote only (no local equivalent). Receives events via `-T.Webhook` from GitHub, GitLab, or other Git hosting platforms.

## Definition

```polyglot
{N} -T.Git.PR
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TGitPR"
   [%] .description << "Fires on pull request events with optional filters."
   (-) <action#array.Git.PRAction <~ {}
   (-) <target#array.string <~ {}
   (-) >number#int
   (-) >title#string
   (-) >source#string
   (-) >target#string
   (-) >sha#string
   (-) >commits#array.Git.Commit
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<action` | `#array.Git.PRAction` | PR actions to match (OR logic). E.g. `#Git.PRAction.Opened`, `#Git.PRAction.Synchronize`. Empty = all actions. |
| `<target` | `#array.string` | Target branch patterns to match. E.g. `"main"`. Empty = all targets. |

### Filter Syntax

Filters are provided via `(-) >>` lines under the `[T]` block:

```polyglot
[T] -T.Git.PR
   (-) >> action: #Git.PRAction.Opened, #Git.PRAction.Synchronize
   (-) >> target: "main"
   (-) >number >> <prNumber
   (-) >title >> <title
   (-) >source >> <source
   (-) >target >> <target
   (-) >sha >> <sha
```

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>number` | `#int` | Pull request number |
| `>title` | `#string` | Pull request title |
| `>source` | `#string` | Source branch name |
| `>target` | `#string` | Target branch name |
| `>sha` | `#string` | HEAD commit SHA of the PR |
| `>commits` | `#array.Git.Commit` | Commits in the pull request |

## Errors

None.

## Permissions

Web.Socket — required for receiving remote webhook events.

## Queue Composability

- `-Q.Allow.One` — only one PR gate running at a time per PR
- `-Q.Default` with `#NoDuplicate` retrigger — skip if same SHA already queued

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.T:Git.PR` | Compile-time pipeline template |
| Instance | `%T:Git.PR:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/T/INDEX|-T.* Trigger Pipelines]]
- [[pglib/types/git|#Git Type Tree]] — `#Git.PR`, `#Git.PRAction` payload types
- [[pglib/pipelines/T/Webhook|-T.Webhook]] — transport layer for remote PR events
