---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:RT.<Lang>.Bind.File"
metadata_instance: "%-:RT.<Lang>.Bind.File:N"
---

# -RT.\<Lang\>.Bind.File

Native code imports the aljam3 lib and calls `pull()`/`push()` to interact with Aljam3 IO ports. File variant.

## Definition

```aljam3
{N} -RT.<Lang>.Bind.File
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RtBindFile"
   [%] .description << "Native code imports aljam3 lib and calls pull()/push(). File variant."
   (-) <env#<Lang>Env
   (-) >output#Code:<Lang>.Output
   (-) <file#path
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<env` | `#<Lang>Env` | Runtime environment from `-W.RT` |
| `<file` | `#path` | Path to source file |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |

## Example

```aljam3
[-] -RT.Python.Bind.File
   (-) <env#PyEnv << $pyenv
   (-) >output#Code:Python.Output >> >fileResult
   (-) <file#path << -Path"/scripts/transform.py"
```

## Errors

None.

## Permissions

Requires `System.Process` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:RT.<Lang>.Bind.File` | Compile-time pipeline template |
| Instance | `%-:RT.<Lang>.Bind.File:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/RT/INDEX|-RT.* Runtime Execution]]
