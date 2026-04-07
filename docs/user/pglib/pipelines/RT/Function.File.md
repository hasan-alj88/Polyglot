---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.=:RT.<Lang>.Function.File"
metadata_instance: "%=:RT.<Lang>.Function.File:N"
---

# =RT.\<Lang\>.Function.File

Call a named function in a source file.

## Definition

```polyglot
{N} =RT.<Lang>.Function.File
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RtFunctionFile"
   [%] .description << "Call a named function in a source file."
   [=] <env#<Lang>Env
   [=] <func#string
   [=] <arg#array.string
   [=] <kwarg#map:string:string
   [=] >output#Code:<Lang>.Output
   [=] >return#serial
   [=] <file#path
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<env` | `#<Lang>Env` | Runtime environment from `=W.RT` |
| `<func` | `#string` | Function name (compiler-validated against source file) |
| `<arg` | `#array.string` | Positional arguments |
| `<kwarg` | `#map:string:string` | Keyword arguments (optional) |
| `<file` | `#path` | Path to source file |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |
| `>return` | `#serial` | Function return value |

## Compiler Validation

The compiler validates that the `<func` name exists as a function definition in the source file.

## Example

```polyglot
[r] =RT.Python.Function.File
   [=] <env#PyEnv << $pyenv
   [=] <func#string << "calculate"
   [=] <arg#array.string << $numbers
   [=] <kwarg#map:string:string << {"precision": "4"}
   [=] >output#Code:Python.Output >> >result
   [=] >return#serial >> >stats
   [=] <file#path << =Path"/scripts/stats.py"
```

## Errors

None.

## Permissions

Requires `System.Process` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:RT.<Lang>.Function.File` | Compile-time pipeline template |
| Instance | `%=:RT.<Lang>.Function.File:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/RT/INDEX|=RT.* Runtime Execution]]
