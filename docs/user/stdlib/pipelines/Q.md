---
audience: user
type: specification
updated: 2026-03-20
status: draft
---

# =Q — Queue Configurations

Queue modifiers are placed on `[Q]` lines. No `[@]` import needed.

```
=Q
   .Default
      [ ] Standard queue — no special dispatch or kill rules.

   .Priority (?)
      <level;int (?)
      [ ] Sets pipeline execution priority.

   .DispatchIf (?)
      .CPU (?)
         .Available (?)
            .MoreThan (?)
               <threshold;float (?)
      .RAM (?)
         .Available (?)
            .MB (?)
               .MoreThan (?)
                  <threshold;float (?)

   .KillIf (?)
      .ExecutionTime (?)
         .MoreThan (?)
            <timeout;string (?)
      .CPU (?)
         .Usage (?)
            .MoreThan (?)
               <threshold;float (?)

   .ReplaceReTriggeredIf (?)
      .QueueTime (?)
         .MoreThan (?)
            <timeout;string (?)
```
