---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
metadata_definition: "%definition.=:Math.Modulo"
metadata_instance: "%=:Math.Modulo:N"
---

# =Math.Modulo

Returns the remainder of dividing the dividend by the divisor. Accepts exactly two inputs in positional order. When any input is `#float`, the output is `#float`. Raises `!Math.DivideByZero` if the divisor is zero.

## Definition

```polyglot
{N} =Math.Modulo
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "MathModulo"
   [%] .description << "Binary modulo (remainder of dividend divided by divisor)"
   [=] <<#int (exactly 2 — dividend, divisor)
   [=] >>#int
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| (positional 1) | `#int` or `#float` | Dividend |
| (positional 2) | `#int` or `#float` | Divisor |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| (positional) | `#int` or `#float` | Remainder (dividend mod divisor) |

## Errors

`!Math.DivideByZero` — raised when the divisor is zero.

## Permissions

None.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.=:Math.Modulo` | Compile-time pipeline template |
| Instance | `%=:Math.Modulo:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Math/INDEX|=Math.* Numeric Pipelines]]
- [[pglib/pipelines/Math/Divide|=Math.Divide]]
