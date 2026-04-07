---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# =Math.Divide

Divides the dividend by the divisor. Accepts exactly two inputs in positional order. When any input is `#float`, the output is `#float`. Raises `!Math.DivideByZero` if the divisor is zero.

## Definition

```polyglot
{N} =Math.Divide
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "MathDivide"
   [%] .description << "Binary division (dividend divided by divisor)"
   [=] <<#int (exactly 2 — dividend, divisor)
   [=] >>#int
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| (positional 1) | `#int` or `#float` | Dividend (value to divide) |
| (positional 2) | `#int` or `#float` | Divisor (value to divide by) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| (positional) | `#int` or `#float` | Quotient (dividend ÷ divisor) |

## Errors

`!Math.DivideByZero` — raised when the divisor is zero.

```polyglot
[r] =Math.Divide
   [=] << $numerator
   [=] << $denominator
   [=] >> $result
   [!] !Math.DivideByZero
      [r] >result << 0
```

## Permissions

None.

## Related

- [[pglib/pipelines/Math/INDEX|=Math.* Numeric Pipelines]]
- [[pglib/pipelines/Math/Modulo|=Math.Modulo]]
