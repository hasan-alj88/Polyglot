---
audience: automation-builder
type: specification
updated: 2026-04-13
status: draft
metadata_definition: "%definition.-:Run.<Lang>.Bind"
metadata_instance: "%-:Run.<Lang>.Bind:N"
---

# -Run.\<Lang\>.Bind

Foreign code imports the aljam3 lib and controls data flow via `pull()`/`push()` calls.

> **Supersedes:** `-RT.<Lang>.Bind.Inline` and `-RT.<Lang>.Bind.File`. See [[jm3lib/pipelines/RT/Bind.Inline|@d:-RT.\<Lang\>.Bind.Inline]] and [[jm3lib/pipelines/RT/Bind.File|@d:-RT.\<Lang\>.Bind.File]].

## Definition

```aljam3
{N} -Run.<Lang>.Bind
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RunBind"
   [%] .description << "Foreign code imports aljam3 lib for data flow."
   (-) <env#<Lang>Env
   (-) >output#Code:<Lang>.Output
   (-) <code#Code:Source
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<env` | `#<Lang>Env` | Runtime environment from `-W.Env` |
| `<code` | `#Code:Source` | Code with aljam3 lib imports (inline `[C]` or file) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |

## Compiler Validation

No binding validation -- `pull()`/`push()` calls are opaque runtime strings. The compiler cannot verify that the names passed to these functions correspond to any Aljam3 IO ports.

This is the most flexible but least validated mode. Prefer `.Script` when Aljam3-controlled binding is sufficient.

## Code Source

Uses `#Code:Source` with `%##Active` one -- provide **either** inline or file, never both (PGE01038):

```aljam3
[ ] inline via [C] blocks
(-) <code.inline <<
   [C] from aljam3 import pull, push
   [C] data = pull("input_data")
   [C] push("result", data.upper())
```

```aljam3
[ ] file reference
(-) <code.file#path << "/scripts/processor.py"
```

## Example

```aljam3
{;} ;PyProcessor
   [.] .language << "python"
   [.] .version << "3.14"

{_} _ProcessGrant
   [.] .intent << #Grant
   [.] .System.Process "python3"

{-} =ProcessWithNativeLib
   (-) _ProcessGrant
   (-) <inputData#serial
   (-) >processLog#Code:Python.Output
   (-) ;PyProcessor
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Env
      (-) <env#; << ;PyProcessor

   [ ]
   [-] -Run.Python.Bind
      (-) <env#PyEnv << $pyenv
      (-) >output#Code:Python.Output >> >processLog
      (-) <code.inline <<
         [C] from aljam3 import pull, push
         [C] import json
         [C] data = pull("inputData")
         [C] processed = json.loads(data)
         [C] processed["status"] = "complete"
         [C] push("result", json.dumps(processed))
```

## When to Use `.Bind` vs `.Script`

| Concern | `.Script` | `.Bind` |
|---------|-----------|---------|
| Who controls data flow | Aljam3 (`<Bind`/`>Bind`) | Foreign code (`pull()`/`push()`) |
| Compiler validates names | Yes (PGE01033/PGE01034) | No |
| Data flow timing | Before/after execution | Any point during execution |
| Best for | Simple inject-and-read patterns | Complex async or event-driven code |

## Errors

See [[jm3lib/pipelines/Run/INDEX#Binding Compiler Errors]].

## Permissions

Requires `System.Process` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Run.<Lang>.Bind` | Compile-time pipeline template |
| Instance | `%-:Run.<Lang>.Bind:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/Run/INDEX|-Run.* Foreign Code Execution]]
- [[jm3lib/pipelines/W/Env|-W.Env]] -- wrapper that manages runtime environments
- [[jm3lib/types/rt|Runtime types]] -- `#Code`, `#PyEnv`, `#RsEnv`
