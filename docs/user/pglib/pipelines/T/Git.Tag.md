---
audience: automation-builder
type: specification
updated: 2026-04-16
status: draft
metadata_definition: "%definition.T:Git.Tag"
metadata_instance: "%T:Git.Tag:N"
---

# -T.Git.Tag

Fires on tag creation. Supports tag name pattern filters. This is a **semantic trigger** — it works over both local hooks (`-T.Git.Hook`) and remote webhooks (`-T.Webhook`), with the runtime resolving the source based on `-Env.*` configuration.

## Definition

```aljam3
{N} -T.Git.Tag
   [%] .Kind << #NativeKind.Trigger
   [%] .Rust << "TGitTag"
   [%] .description << "Fires on tag creation with optional name pattern filter."
   (-) <pattern#array.string <~ {}
   (-) >name#string
   (-) >sha#string
   (-) >tagger#Git.Author
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<pattern` | `#array.string` | Tag name glob patterns to match. E.g. `"v*"`, `"release-*"`. Empty = all tags. |

### Filter Syntax

Filters are provided via `(-) >>` lines under the `[T]` block:

```aljam3
[T] -T.Git.Tag
   (-) >> pattern: "v*"
   (-) >name >> <tagName
   (-) >sha >> <sha
   (-) >tagger >> <tagger
```

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>name` | `#string` | Tag name (e.g. `"v1.2.3"`) |
| `>sha` | `#string` | Commit SHA the tag points to |
| `>tagger` | `#Git.Author` | Author of the tag (for annotated tags) |

## Errors

None.

## Permissions

System.Process (local mode) or Web.Socket (remote webhook mode).

## Queue Composability

- `-Q.Allow.One` — prevent duplicate release builds for the same tag

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.T:Git.Tag` | Compile-time pipeline template |
| Instance | `%T:Git.Tag:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/T/INDEX|-T.* Trigger Pipelines]]
- [[pglib/types/git|#Git Type Tree]] — `#Git.Tag`, `#Git.Author` payload types
- [[pglib/pipelines/T/Git.Hook|-T.Git.Hook]] — transport layer for local tag creation
