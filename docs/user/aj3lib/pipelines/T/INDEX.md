---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
---

# -T.* Trigger Pipelines

<!-- @c:pipelines -->
Triggers are placed on `[T]` lines inside `{-}` pipeline definitions. No `[@]` import needed. See [[concepts/pipelines/io-triggers#Triggers]] for trigger usage rules.

**PRIMITIVE** — Trigger pipelines are direct OS/runtime integrations. They are implemented by the Aljam3 runtime and cannot be reimplemented in user `.jm3` files.

## Permissions

<!-- @c:permissions -->
Most triggers require no permissions. IO-touching triggers require a `{_}` permission object granting the listed capabilities. See [[permissions]] for the permission system and [[errors#Built-in Error Namespaces]] for `!Permission.*` errors.

| Pipeline | Required Capability | Category |
|----------|-------------------|----------|
| `-T.Call` | None | — |
| `-T.CLI` | None | — |
| `-T.Daily` | None | — |
| `-T.Folder.NewFiles` | File.Read | File |
| `-T.File.Rolled` | File.Read | File |
| `-T.Webhook` | Web.Socket | Web |
| `-T.Git.Hook` | System.Process | System |
| `-T.Git.Push` | System.Process or Web.Socket | System / Web |
| `-T.Git.PR` | Web.Socket | Web |
| `-T.Git.Tag` | System.Process or Web.Socket | System / Web |

## Pipeline Listing

| Pipeline | Description |
|----------|-------------|
| [[jm3lib/pipelines/T/Call\|-T.Call]] | Pipeline invoked when called from another pipeline |
| [[jm3lib/pipelines/T/CLI\|-T.CLI]] | Pipeline invoked manually (CLI or test harness) |
| [[jm3lib/pipelines/T/Daily\|-T.Daily]] | Fires once per day at specified time |
| [[jm3lib/pipelines/T/Folder.NewFiles\|-T.Folder.NewFiles]] | Fires when new files appear in folder |
| [[jm3lib/pipelines/T/File.Rolled\|-T.File.Rolled]] | Fires when a file rotates (log rotation) |
| [[jm3lib/pipelines/T/Webhook\|-T.Webhook]] | Fires on incoming HTTP request |
| [[jm3lib/pipelines/T/Git.Hook\|-T.Git.Hook]] | Fires on local git hook invocation |
| [[jm3lib/pipelines/T/Git.Push\|-T.Git.Push]] | Fires on push to branch (with filters) |
| [[jm3lib/pipelines/T/Git.PR\|-T.Git.PR]] | Fires on pull request events |
| [[jm3lib/pipelines/T/Git.Tag\|-T.Git.Tag]] | Fires on tag creation |

## Three-Tier Trigger Model

Git triggers use a two-layer architecture:

| Tier | Triggers | Source | Description |
|------|----------|--------|-------------|
| **Transport** | `-T.Git.Hook`, `-T.Webhook` | Local / Remote | Raw event delivery — hook dispatcher or HTTP POST |
| **Semantic** | `-T.Git.Push`, `-T.Git.PR`, `-T.Git.Tag` | Either | Unified events — runtime resolves source via `-Env.*` config |

**Transport triggers** deliver raw events. `-T.Git.Hook` installs a shell dispatcher into `.git/hooks/` that POSTs to the local Aljam3 runtime on `localhost` — the same HTTP code path as `-T.Webhook`.

**Semantic triggers** abstract over transport. `-T.Git.Push` fires whether the push event arrives from a local hook or a remote GitHub/GitLab webhook. The runtime resolves the source based on environment configuration (`-Env.*`).

Use transport triggers when you need hook-specific behavior. Use semantic triggers for CI/CD workflows that should work across local and hosted setups.

See [[jm3lib/types/git|#Git Type Tree]] for the typed event payloads produced by these triggers.

## Related

- [[concepts/pipelines/io-triggers]]
- [[jm3lib/INDEX]]
