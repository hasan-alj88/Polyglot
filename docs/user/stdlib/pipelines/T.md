---
audience: pg-coder
type: specification
updated: 2026-03-25
status: complete
---

# =T — Triggers

<!-- @pipelines -->
Triggers are placed on `[T]` lines inside `{=}` pipeline definitions. No `[@]` import needed. See [[concepts/pipelines/io-triggers#Triggers]] for trigger usage rules.

**PRIMITIVE** — Trigger pipelines are direct OS/runtime integrations. They are implemented by the Polyglot runtime and cannot be reimplemented in user `.pg` files.

## Permissions

<!-- @permissions -->
Most triggers require no permissions. IO-touching triggers must declare `[_]` permissions. See [[permissions]] for the permission system and [[errors#Built-in Error Namespaces]] for `!Permission.*` errors.

| Pipeline | Permission | Type |
|----------|-----------|------|
| `=T.Call` | None | — |
| `=T.Manual` | None | — |
| `=T.Daily` | None | — |
| `=T.Folder.NewFiles` | `_File.read` | Inline |
| `=T.Webhook` | `_Web.socket` | IO |

```
=T
   .Call
      [ ] Pipeline is invoked when called from another pipeline.
      [ ] No additional parameters.

   .Manual
      [ ] Pipeline is invoked manually (e.g., from CLI or test harness).
      [ ] No additional parameters.

   .Daily
      <InlineStringLiteral#string <~ ""
      [ ] Fires once per day at the specified time.
      [ ] Time string provided via inline call: =T.Daily"3AM"

   .Folder
      .NewFiles
         <Folder#path
         >NewFiles#array:path
         [ ] Fires when new files appear in the specified folder.
         [ ] Inline path: =T.Folder.NewFiles"/inbox/"

   .Webhook
      <InlineStringLiteral#string <~ ""
      [ ] Fires on an incoming HTTP request to the specified endpoint.
      [ ] Inline path: =T.Webhook"/api/onboarding"
```
