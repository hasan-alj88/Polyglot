---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
metadata_definition: "%definition.-:Math.Power"
metadata_instance: "%-:Math.Power:N"
---

# -Math.Power

Raises the base to the power of the exponent. Accepts exactly two inputs in positional order. When any input is `#float`, the output is `#float`.

## Definition

```aljam3
{N} -Math.Power
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "MathPower"
   [%] .description << "Binary exponentiation (base raised to exponent)"
   (-) <<#int (exactly 2 — base, exponent)
   (-) >>#int
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| (positional 1) | `#int` or `#float` | Base |
| (positional 2) | `#int` or `#float` | Exponent |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| (positional) | `#int` or `#float` | Result (base ^ exponent) |

## Errors

None.

## Permissions

None.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Math.Power` | Compile-time pipeline template |
| Instance | `%-:Math.Power:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/Math/INDEX|-Math.* Numeric Pipelines]]
