---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# =Math.Negate

Returns the arithmetic negation of the input (multiplies by −1). Accepts exactly one input. When the input is `#float`, the output is `#float`.

## Definition

```polyglot
{N} =Math.Negate
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "MathNegate"
   [%] .description << "Arithmetic negation of a numeric value"
   [=] <<#int (exactly 1)
   [=] >>#int
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

## Related

- [[pglib/pipelines/Math/INDEX|=Math.* Numeric Pipelines]]
- [[pglib/pipelines/Math/Abs|=Math.Abs]]
