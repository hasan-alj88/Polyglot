---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.-:Dur.Parse"
metadata_instance: "%-:Dur.Parse:N"
---

# -Dur.Parse

Parses a dynamic string into a `#duration` value at runtime. Use this when the input comes from IO or other runtime sources where the compiler cannot prove validity at compile time.

For known literals in execution body, use the `$Dur` constructor instead — it guarantees no error surface.

## Definition

```aljam3
{N} -Dur.Parse
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DurParse"
   [%] .description << "Parse dynamic string to Duration"
   (-) <raw#string
   (-) >dur#duration
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<raw` | `#string` | --- | String to parse as a duration (e.g., `"1h30m"`, `"45s"`, `"2h15m30s"`) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>dur` | `#duration` | Parsed Duration value with `.seconds` and `.nanos` fields populated |

## Errors

| Error | Description |
|-------|-------------|
| `!Parse.Duration.InvalidFormat` | Input string does not match any recognized duration format (`NhNmNs`, `NhNm`, `NmNs`, `Nh`, `Nm`, `Ns`) |
| `!Parse.Duration.OutOfRange` | Parsed numeric values exceed valid range for duration fields |

## Permissions

None required. Pure computation pipeline.

## Usage

```aljam3
[-] $timeout#duration << -Dur.Parse
   (<) <raw#string << $userInput
   [!] !Parse.Duration.InvalidFormat
      [-] $timeout << $Dur"30s"
   [!] !Parse.Duration.OutOfRange
      [-] $timeout << $Dur"1h"
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Dur.Parse` | Compile-time pipeline template |
| Instance | `%-:Dur.Parse:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/constructors/Dur|$Dur constructor]] -- compile-time duration construction
- [[pglib/types/datetime/core-components|#Duration type]] -- duration type definition
- [[pglib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]] -- related DateTime pipelines
- [[pglib/constructors/DT|$DT constructor]] -- companion DateTime constructor
