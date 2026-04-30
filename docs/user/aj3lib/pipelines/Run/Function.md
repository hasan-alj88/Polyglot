---
audience: automation-builder
type: specification
updated: 2026-04-13
status: draft
metadata_definition: "%definition.-:Run.<Lang>.Function"
metadata_instance: "%-:Run.<Lang>.Function:N"
---

# -Run.\<Lang\>.Function

Call a named function in foreign code with structured arguments.

> **Supersedes:** `-RT.<Lang>.Function.Inline` and `-RT.<Lang>.Function.File`. See [[aj3lib/pipelines/RT/Function.Inline|@d:-RT.\<Lang\>.Function.Inline]] and [[aj3lib/pipelines/RT/Function.File|@d:-RT.\<Lang\>.Function.File]].

## Definition

```aljam3
{N} -Run.<Lang>.Function
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RunFunction"
   [%] .description << "Call a named function in foreign code."
   (-) <env#<Lang>Env
   (-) <func#string
   (-) <arg#Record
   (-) <kwarg#Record
   (-) >Bind#Record
   (-) >output#Code:<Lang>.Output
   (-) <code#Code:Source
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<env` | `#<Lang>Env` | Runtime environment from `-W.Env` |
| `<func` | `#string` | Function name (compiler-validated against code) |
| `<arg` | `#Record` | Positional arguments -- field order = argument order |
| `<kwarg` | `#Record` | Keyword arguments -- field names = parameter names (optional) |
| `<code` | `#Code:Source` | Function definition (inline `[C]` or file) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Bind` | `#Record` | Return value fields captured by name |
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |

## Compiler Validation

The compiler validates that:
- `<func` name exists as a function definition in the code (PGE01035)
- `<arg` Record field names match positional parameters of `<func` (PGE01035)
- `<kwarg` Record field names match keyword parameters of `<func` (PGE01036)

**Note:** Validation applies at compile time for `<code.inline` only. When `<code.file` is used, validation is deferred to runtime.

## Code Source

Uses `#Code:Source` with `%##Active` one -- provide **either** inline or file, never both (PGE01038):

```aljam3
[ ] inline via [C] blocks
(-) <code.inline <<
   [C] def calculate(nums):
   [C]     return {"mean": sum(nums) / len(nums)}
```

```aljam3
[ ] file reference
(-) <code.file#path << "/scripts/stats.py"
```

## Example

```aljam3
{_} _RuntimeCeiling
   [.] .intent << #Ceiling
   [.] .System.Process "*"

{@} @Local:Example.PythonStats
   (-) _RuntimeCeiling

{;} ;PyStats
   [.] .language << "python"
   [.] .version << "3.14"
   [.] .packages << ["statistics"]

{_} _PythonGrant
   [.] .intent << #Grant
   [.] .System.Process "python3"

{#} #StatsResult
   [.] .mean#float
   [.] .stdev#float

{-} =CalculateStats
   (-) _PythonGrant
   (-) <numbers#array.float
   (-) >result#Code:Python.Output
   (-) >stats#StatsResult
   (-) ;PyStats
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Env
      (-) <env#; << ;PyStats

   [ ]
   [-] -Run.Python.Function
      (-) <env#PyEnv << $pyenv
      (-) <func#string << "calculate"
      (-) <arg#Record
         [.] .numbers#array.float << $numbers
      (-) >Bind#StatsResult >> >stats
      (-) >output#Code:Python.Output >> >result
      (-) <code.inline <<
         [C] import statistics
         [C] def calculate(numbers):
         [C]     return {"mean": statistics.mean(numbers), "stdev": statistics.stdev(numbers)}
```

## Errors

See [[aj3lib/pipelines/Run/INDEX#Binding Compiler Errors]].

## Permissions

Requires `System.Process` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Run.<Lang>.Function` | Compile-time pipeline template |
| Instance | `%-:Run.<Lang>.Function:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/Run/INDEX|-Run.* Foreign Code Execution]]
- [[aj3lib/pipelines/W/Env|-W.Env]] -- wrapper that manages runtime environments
- [[aj3lib/types/rt|Runtime types]] -- `#Code`, `#PyEnv`, `#RsEnv`
