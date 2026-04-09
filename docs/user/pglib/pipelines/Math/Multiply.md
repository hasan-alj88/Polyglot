---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
metadata_definition: "%definition.-:Math.Multiply"
metadata_instance: "%-:Math.Multiply:N"
---

# -Math.Multiply

Multiplies two or more numeric values. Accepts variadic input (2 or more operands). When any input is `#float`, the output is `#float`.

## Definition

```polyglot
{N} -Math.Multiply
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "MathMultiply"
   [%] .description << "Variadic multiplication of two or more numeric values"
   (-) <<#int (variadic — 2+)
   (-) >>#int
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| (positional) | `#int` or `#float` | Operands to multiply (2 or more) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| (positional) | `#int` or `#float` | Product of all inputs |

## Errors

None.

## Permissions

None.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Math.Multiply` | Compile-time pipeline template |
| Instance | `%-:Math.Multiply:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Math/INDEX|-Math.* Numeric Pipelines]]
