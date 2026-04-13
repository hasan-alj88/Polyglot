---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# -Math.* Numeric Pipelines

pglib pipelines for arithmetic on `#int` and `#float` values. No `[@]` import needed.

Math pipelines use **positional (unnamed) IO** — inputs and outputs are ordered by position, not named. Use `<<` for inputs and `>>` for outputs without field names.

All pipelines accept both `#int` and `#float` operands. When any input is `#float`, the output is `#float`.

## Usage Example

```polyglot
[ ] Add three values (positional IO — no names)
[-] -Math.Add
   (-) << $a
   (-) << $b
   (-) << $c
   (-) >> $sum
```

## Pipeline Listing

### Variadic (2+ inputs)

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Math/Add\|-Math.Add]] | Addition of two or more values |
| [[pglib/pipelines/Math/Multiply\|-Math.Multiply]] | Multiplication of two or more values |

### Binary (exactly 2 inputs)

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Math/Subtract\|-Math.Subtract]] | Subtraction (minuend − subtrahend) |
| [[pglib/pipelines/Math/Divide\|-Math.Divide]] | Division (dividend ÷ divisor) |
| [[pglib/pipelines/Math/Modulo\|-Math.Modulo]] | Modulo (dividend mod divisor) |
| [[pglib/pipelines/Math/Power\|-Math.Power]] | Exponentiation (base ^ exponent) |

### Unary (exactly 1 input)

| Pipeline | Description |
|----------|-------------|
| [[pglib/pipelines/Math/Abs\|-Math.Abs]] | Absolute value |
| [[pglib/pipelines/Math/Negate\|-Math.Negate]] | Arithmetic negation |

## Permissions

None. All operations are pure computation.

## Related

- [[pglib/INDEX|pglib Pipeline Index]]
- [[pglib/pipelines/Math/Add|-Math.Add]]
- [[pglib/pipelines/Math/Subtract|-Math.Subtract]]
- [[pglib/pipelines/Math/Multiply|-Math.Multiply]]
- [[pglib/pipelines/Math/Divide|-Math.Divide]]
- [[pglib/pipelines/Math/Modulo|-Math.Modulo]]
- [[pglib/pipelines/Math/Power|-Math.Power]]
- [[pglib/pipelines/Math/Abs|-Math.Abs]]
- [[pglib/pipelines/Math/Negate|-Math.Negate]]
