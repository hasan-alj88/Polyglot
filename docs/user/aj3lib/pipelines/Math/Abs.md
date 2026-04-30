---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
metadata_definition: "%definition.-:Math.Abs"
metadata_instance: "%-:Math.Abs:N"
---

# -Math.Abs

Returns the absolute value of the input. Accepts exactly one input. When the input is `#float`, the output is `#float`.

## Definition

```aljam3
{N} -Math.Abs
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "MathAbs"
   [%] .description << "Absolute value of a numeric value"
   (-) <<#int (exactly 1)
   (-) >>#int
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| (positional) | `#int` or `#float` | Value to take the absolute value of |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| (positional) | `#int` or `#float` | Absolute value of the input |

## Errors

None.

## Permissions

None.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Math.Abs` | Compile-time pipeline template |
| Instance | `%-:Math.Abs:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/Math/INDEX|-Math.* Numeric Pipelines]]
- [[aj3lib/pipelines/Math/Negate|-Math.Negate]]
