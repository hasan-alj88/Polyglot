---
audience: design
type: reference
updated: 2026-03-30
---

<!-- @edge-cases/INDEX -->

## 16. Trigger IO Wiring (S16)

### EC-16.1: Trigger that produces outputs — IO declared before trigger, wired inside

<!-- @u:pipelines:Triggers -->
**EBNF:** `trigger_io_section ::= { indent ( io_decl_line | error_decl_line | comment_line ) NEWLINE } { indent ( trigger_line | comment_line ) NEWLINE }`

**What it tests:** IO must be declared **before** the trigger that pushes into it. Trigger outputs wired via indented `(-)` lines. See [[concepts/pipelines/io-triggers#Triggers]].

```polyglot
{-} -Inbox.Monitor
   (-) <NewFiles#array:path
   [T] -T.Folder.NewFiles"/inbox/"
      (-) >NewFiles >> <NewFiles
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ]
   [-] $count#int << 0
```

### EC-16.2: Multiple trigger outputs wired to multiple inputs

**What it tests:** A trigger with two outputs, each wired to a declared input. Order of `(-)` declarations before `[T]` matters.

```polyglot
{-} -Webhook.Receiver
   (-) <payload#serial
   (-) <headers#serial
   [T] -T.Webhook"/api/v2/events"
      (-) >payload >> <payload
      (-) >headers >> <headers
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ]
   [-] $type#string << $payload:eventType
```

### EC-16.3: Mixed trigger modes — some inputs from trigger, some from caller

**What it tests:** Inputs can be filled by trigger wiring **or** left unfilled (must be provided by caller). No mixing of assignment modes on the same param.

```polyglot
{-} -File.Processor
   (-) <file#path
   (-) <options#serial <~ {}
   [T] -T.Folder.NewFiles"/watch/"
      (-) >NewFiles >> <file
   [Q] -Q.Default
   [W] -W.Polyglot
   [ ] $options uses default {}; $file comes from trigger
   [ ]
   [-] $name#string << "{$file}"
```
