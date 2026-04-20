---
audience: automation-builder
type: specification
updated: 2026-04-10
status: draft
---

# Pipeline Call IO

<!-- @u:pipelines:Error Handling -->
<!-- @u:technical/ebnf/10-execution#Pipeline Call -->
Pipeline calls use `[-]` execution with `(-)` IO lines. Error blocks `[!]` scope under the call — see [[concepts/pipelines/error-handling#Error Handling]]. For pglib pipelines that need no import, see [[packages#Usage]].

```polyglot
[-] -Pipeline.Name
   (-) <InputParameter1 << ...
   (-) >OutputParameter1 >> ...
```
