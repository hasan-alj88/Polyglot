---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
metadata_definition: "%definition.-:Math.Subtract"
metadata_instance: "%-:Math.Subtract:N"
---

# -Math.Subtract

Subtracts the subtrahend from the minuend. Accepts exactly two inputs in positional order. When any input is `#float`, the output is `#float`.

## Definition

```polyglot
{N} -Math.Subtract
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "MathSubtract"
   [%] .description << "Binary subtraction (minuend minus subtrahend)"
   (-) <<#int (exactly 2 — minuend, subtrahend)
   (-) >>#int
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| (positional 1) | `#int` or `#float` | Minuend (value to subtract from) |
| (positional 2) | `#int` or `#float` | Subtrahend (value to subtract) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| (positional) | `#int` or `#float` | Difference (minuend − subtrahend) |

## Errors

None.

## Permissions

None.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Math.Subtract` | Compile-time pipeline template |
| Instance | `%-:Math.Subtract:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Math/INDEX|-Math.* Numeric Pipelines]]
