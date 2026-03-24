---
audience: user
type: specification
updated: 2026-03-24
status: complete
---

# =T — Triggers

<!-- @pipelines -->
Triggers are placed on `[t]` lines inside `{=}` pipeline definitions. No `[@]` import needed. See [[pipelines#Triggers]] for trigger usage rules.

**PRIMITIVE** — Trigger pipelines are direct OS/runtime integrations. They are implemented by the Polyglot runtime and cannot be reimplemented in user `.pg` files.

```
=T
   .Call
      [ ] Pipeline is invoked when called from another pipeline.
      [ ] No additional parameters.

   .Manual
      [ ] Pipeline is invoked manually (e.g., from CLI or test harness).
      [ ] No additional parameters.

   .Daily
      <InlineStringLiteral;string <~ ""
      [ ] Fires once per day at the specified time.
      [ ] Time string provided via inline call: =T.Daily"3AM"

   .Folder
      .NewFiles
         <Folder;path
         >NewFiles;array.path
         [ ] Fires when new files appear in the specified folder.
         [ ] Inline path: =T.Folder.NewFiles"/inbox/"

   .Webhook
      <InlineStringLiteral;string <~ ""
      [ ] Fires on an incoming HTTP request to the specified endpoint.
      [ ] Inline path: =T.Webhook"/api/onboarding"
```
