---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.-:DT.Parse"
metadata_instance: "%-:DT.Parse:N"
---

# -DT.Parse

Parses a dynamic string into a `#dt` value at runtime. Use this when the input comes from IO or other runtime sources where the compiler cannot prove validity at compile time.

For known literals in execution body, use the `$DT` constructor instead — it guarantees no error surface.

## Definition

```aljam3
{N} -DT.Parse
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "DtParse"
   [%] .description << "Parse dynamic string to DateTime"
   (-) <raw#string
   (-) >dt#dt
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<raw` | `#string` | --- | String to parse as a datetime value |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>dt` | `#dt` | Parsed DateTime value |

## Errors

| Error | Description |
|-------|-------------|
| `!Parse.DateTime.InvalidFormat` | Input string does not match any recognized datetime format |
| `!Parse.DateTime.OutOfRange` | Parsed values are out of valid range (e.g., month 13, day 32) |

## Permissions

None required. Pure computation pipeline.

## Usage

```aljam3
[-] $parsed#dt << -DT.Parse
   (<) <raw#string << $userInput
   [!] !Parse.DateTime.InvalidFormat
      [-] $parsed << $DT"Today"
   [!] !Parse.DateTime.OutOfRange
      [-] $parsed << $DT"Today"
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:DT.Parse` | Compile-time pipeline template |
| Instance | `%-:DT.Parse:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/DT/INDEX|-DT.* DateTime Pipelines]]
- [[jm3lib/constructors/DT|$DT constructor]] -- compile-time DateTime construction
- [[jm3lib/types/datetime|DateTime types]]
