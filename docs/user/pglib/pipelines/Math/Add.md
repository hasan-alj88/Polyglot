---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
metadata_definition: "%definition.-:Math.Add"
metadata_instance: "%-:Math.Add:N"
---

# -Math.Add

Adds two or more numeric values. Accepts variadic input (2 or more operands). When any input is `#float`, the output is `#float`.

## Definition

```aljam3
{N} -Math.Add
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "MathAdd"
   [%] .description << "Variadic addition of two or more numeric values"
   (-) <<#int (variadic — 2+)
   (-) >>#int
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| (positional) | `#int` or `#float` | Operands to add (2 or more) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| (positional) | `#int` or `#float` | Sum of all inputs |

## Errors

None.

## Permissions

None.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Math.Add` | Compile-time pipeline template |
| Instance | `%-:Math.Add:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Math/INDEX|-Math.* Numeric Pipelines]]
