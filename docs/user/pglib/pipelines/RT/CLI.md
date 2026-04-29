---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:RT.<Lang>.CLI"
metadata_instance: "%-:RT.<Lang>.CLI:N"
---

# -RT.\<Lang\>.CLI

Invoke a compiled binary. No language runtime needed — uses `-W.Aljam3`, not `-W.Env`. No `<env` parameter.

## Definition

```aljam3
{N} -RT.<Lang>.CLI
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RtCli"
   [%] .description << "Invoke compiled binary."
   (-) <binary#path
   (-) <arg#array.string
   (-) <kwarg#map:string:string
   (-) >output#Code:<Lang>.Output
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<binary` | `#path` | Path to executable |
| `<arg` | `#array.string` | Positional arguments (optional) |
| `<kwarg` | `#map:string:string` | CLI flags (optional) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>output` | `#Code:<Lang>.Output` | Console capture (`.stdout`, `.stderr`) |

## Notes

Uses `-W.Aljam3`, not `-W.Env` — no language runtime needed for compiled binaries.

## Example

```aljam3
{_} _BinaryCeiling
   [.] .intent << #Ceiling
   [.] .System.Process "*"

{@} @Local:Example.RustBinary
   (-) _BinaryCeiling

{_} _ToolGrant
   [.] .intent << #Grant
   [.] .System.Process "mytool"

{-} =RunRustTool
   (-) _ToolGrant
   (-) <inputPath#path
   (-) >toolOutput#Code:Rust.Output
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3

   [ ]
   [-] -RT.Rust.CLI
      (-) <binary#path << -Path"/usr/local/bin/mytool"
      (-) <arg#array.string << ["{$inputPath}"]
      (-) <kwarg#map:string:string << {"--format": "json", "--verbose": "true"}
      (-) >output#Code:Rust.Output >> >toolOutput
```

## Errors

None.

## Permissions

Requires `System.Process` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:RT.<Lang>.CLI` | Compile-time pipeline template |
| Instance | `%-:RT.<Lang>.CLI:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/RT/INDEX|-RT.* Runtime Execution]]
