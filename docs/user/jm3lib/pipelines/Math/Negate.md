---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
metadata_definition: "%definition.-:Math.Negate"
metadata_instance: "%-:Math.Negate:N"
---

# -Math.Negate

Returns the arithmetic negation of the input (multiplies by −1). Accepts exactly one input. When the input is `#float`, the output is `#float`.

## Definition

```aljam3
{N} -Math.Negate
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "MathNegate"
   [%] .description << "Arithmetic negation of a numeric value"
   (-) <<#int (exactly 1)
   (-) >>#int
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| (positional) | `#int` or `#float` | Value to negate |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| (positional) | `#int` or `#float` | Negated value (input × −1) |

## Errors

None.

## Permissions

None.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Math.Negate` | Compile-time pipeline template |
| Instance | `%-:Math.Negate:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/pipelines/Math/INDEX|-Math.* Numeric Pipelines]]
- [[jm3lib/pipelines/Math/Abs|-Math.Abs]]
