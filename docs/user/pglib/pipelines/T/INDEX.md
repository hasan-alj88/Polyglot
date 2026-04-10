---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# -T.* Trigger Pipelines

<!-- @c:pipelines -->
Triggers are placed on `[T]` lines inside `{-}` pipeline definitions. No `[@]` import needed. See [[concepts/pipelines/io-triggers#Triggers]] for trigger usage rules.

**PRIMITIVE** — Trigger pipelines are direct OS/runtime integrations. They are implemented by the Polyglot runtime and cannot be reimplemented in user `.pg` files.

## Permissions

<!-- @c:permissions -->
Most triggers require no permissions. IO-touching triggers require a `{_}` permission object granting the listed capabilities. See [[permissions]] for the permission system and [[errors#Built-in Error Namespaces]] for `!Permission.*` errors.

| Pipeline | Required Capability | Category |
|----------|-------------------|----------|
| `-T.Call` | None | — |
| `-T.Manual` | None | — |
| `-T.Daily` | None | — |
| `-T.Folder.NewFiles` | File.Read | File |
| `-T.Webhook` | Web.Socket | Web |

## Pipeline Listing

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/T/Call\|-T.Call]] | Pipeline invoked when called from another pipeline |
| [[pglib/pipelines/T/Manual\|-T.Manual]] | Pipeline invoked manually (CLI or test harness) |
| [[pglib/pipelines/T/Daily\|-T.Daily]] | Fires once per day at specified time |
| [[pglib/pipelines/T/Folder.NewFiles\|-T.Folder.NewFiles]] | Fires when new files appear in folder |
| [[pglib/pipelines/T/Webhook\|-T.Webhook]] | Fires on incoming HTTP request |

## Related

- [[concepts/pipelines/io-triggers]]
- [[pglib/INDEX]]
