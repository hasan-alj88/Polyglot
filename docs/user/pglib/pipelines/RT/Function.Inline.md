---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:RT.<Lang>.Function.Inline"
metadata_instance: "%-:RT.<Lang>.Function.Inline:N"
---

# -RT.\<Lang\>.Function.Inline

Call a named function in inline foreign code.

## Definition

```polyglot
{N} -RT.<Lang>.Function.Inline
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RtFunctionInline"
   [%] .description << "Call a named function in inline foreign code."
   (-) <env#<Lang>Env
   (-) <func#string
   (-) <arg#array.string
   (-) <kwarg#map:string:string
   (-) >output#Code:<Lang>.Output
   (-) >return#serial
   (-) <code#string
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<env` | `#<Lang>Env` | Runtime environment from `-W.RT` |
| `<func` | `#string` | Function name (compiler-validated against `[C]` block) |
| `<arg` | `#array.string` | Positional arguments |
| `<kwarg` | `#map:string:string` | Keyword arguments (optional) |
| `<code` | `#string` | Inline code via `[C]` blocks |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |
| `>return` | `#serial` | Function return value |

## Compiler Validation

The compiler validates that the `<func` name exists as a function definition in the `[C]` block.

## Example

```polyglot
{_} _RuntimeCeiling
   [.] .intent << #Ceiling
   [.] .System.Process "*"

{@} @Local:Example.PythonStats
   [_] _RuntimeCeiling

{_} _PythonGrant
   [.] .intent << #Grant
   [.] .System.Process "python3"

{-} =CalculateStats
   [_] _PythonGrant
   (-) <numbers#array.string
   (-) >result#Code:Python.Output
   (-) >stats#serial
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.RT:Python:3:14
      (-) >RTpy#PyEnv >> $pyenv

   [-] -RT.Python.Function.Inline
      (-) <env#PyEnv << $pyenv
      (-) <func#string << "calculate"
      (-) <arg#array.string << $numbers
      (-) >output#Code:Python.Output >> >result
      (-) >return#serial >> >stats
      (-) <code#string <<
         [C] import statistics
         [C] def calculate(args):
         [C]     nums = [float(x) for x in args]
         [C]     return {"mean": statistics.mean(nums), "stdev": statistics.stdev(nums)}
```

## Errors

None.

## Permissions

Requires `System.Process` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:RT.<Lang>.Function.Inline` | Compile-time pipeline template |
| Instance | `%-:RT.<Lang>.Function.Inline:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/RT/INDEX|-RT.* Runtime Execution]]
