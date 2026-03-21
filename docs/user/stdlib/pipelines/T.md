---
audience: user
type: specification
updated: 2026-03-20
status: draft
---

# =T — Triggers

Triggers are placed on `[t]` lines inside `{=}` pipeline definitions. No `[@]` import needed.

```
=T
   .Call
      [ ] Pipeline is invoked when called from another pipeline.
      [ ] No additional parameters.

   .Daily
      <InlineStringLiteral;string <~ ""
      [ ] Fires once per day at the specified time.
      [ ] Time string provided via inline call: =T.Daily"3AM"

   .Folder
      .NewFiles
         <Folder;path
         >NewFiles;array.path

   .Schedule (?)
      .Cron (?)
         <cron;string (?)
         [ ] Fires on a cron expression.

   .HTTP (?)
      .Webhook (?)
         <endpoint;string (?)
         <method;string (?)
         [ ] Fires on an incoming HTTP request.

   .File (?)
      .Created (?)
         <path;path (?)
         <pattern;string (?)
         [ ] Fires when a matching file is created.
```
