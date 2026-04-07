---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# =Math.Abs

Returns the absolute value of the input. Accepts exactly one input. When the input is `#float`, the output is `#float`.

## Definition

```polyglot
{N} =Math.Abs
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "MathAbs"
   [%] .description << "Absolute value of a numeric value"
   [=] <<#int (exactly 1)
   [=] >>#int
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

## Related

- [[pglib/pipelines/Math/INDEX|=Math.* Numeric Pipelines]]
- [[pglib/pipelines/Math/Negate|=Math.Negate]]
