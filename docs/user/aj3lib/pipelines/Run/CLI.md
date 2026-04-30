---
audience: automation-builder
type: specification
updated: 2026-04-13
status: draft
metadata_definition: "%definition.-:Run.<Lang>.CLI"
metadata_instance: "%-:Run.<Lang>.CLI:N"
---

# -Run.\<Lang\>.CLI

Invoke a compiled binary with string arguments. No language runtime needed -- uses `-W.Aljam3`, not `-W.Env`.

> **Supersedes:** `-RT.<Lang>.CLI`. See [[aj3lib/pipelines/RT/CLI|@d:-RT.\<Lang\>.CLI]].

## Definition

```aljam3
{N} -Run.<Lang>.CLI
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RunCli"
   [%] .description << "Invoke compiled binary with string arguments."
   (-) <binary#path
   (-) <arg#Record
   (-) <kwarg#Record
   (-) >Bind#Record
   (-) >output#Code:<Lang>.Output
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<binary` | `#path` | Path to the executable |
| `<arg` | `#Record` | Positional arguments -- all fields `#string`, order = argument order |
| `<kwarg` | `#Record` | Named flags -- all fields `#string`, field names = `--flag` names |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Bind` | `#Record` | Output capture (e.g., exit code) |
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |

## Compiler Validation

- `<arg` and `<kwarg` Record fields must all be `#string` (PGE01039)
- No code validation -- the binary is opaque

## Notes

Uses `-W.Aljam3`, not `-W.Env` -- no language runtime needed for compiled binaries. No `<env` parameter. No `<code` parameter.

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
   [-] -Run.Rust.CLI
      (-) <binary#path << -Path"/usr/local/bin/mytool"
      (-) <arg#Record
         [.] .input#string << "{$inputPath}"
      (-) <kwarg#Record
         [.] .format#string << "json"
         [.] .verbose#string << "true"
      (-) >output#Code:Rust.Output >> >toolOutput
```

## Errors

See [[aj3lib/pipelines/Run/INDEX#Binding Compiler Errors]].

## Permissions

Requires `System.Process` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Run.<Lang>.CLI` | Compile-time pipeline template |
| Instance | `%-:Run.<Lang>.CLI:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/Run/INDEX|-Run.* Foreign Code Execution]]
- [[aj3lib/pipelines/W/Aljam3|-W.Aljam3]] -- wrapper for non-runtime execution
