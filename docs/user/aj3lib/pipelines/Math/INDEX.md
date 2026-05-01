---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# -Math.* Numeric Pipelines

jm3lib pipelines for arithmetic on `#int` and `#float` values. No `[@]` import needed.

Math pipelines use **positional (unnamed) IO** — inputs and outputs are ordered by position, not named. Use `<<` for inputs and `>>` for outputs without field names.

All pipelines accept both `#int` and `#float` operands. When any input is `#float`, the output is `#float`.

## Usage Example

```aljam3
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
| [[jm3lib/pipelines/Math/Add\|-Math.Add]] | Addition of two or more values |
| [[jm3lib/pipelines/Math/Multiply\|-Math.Multiply]] | Multiplication of two or more values |

### Binary (exactly 2 inputs)

| Pipeline | Description |
|----------|-------------|
| [[jm3lib/pipelines/Math/Subtract\|-Math.Subtract]] | Subtraction (minuend − subtrahend) |
| [[jm3lib/pipelines/Math/Divide\|-Math.Divide]] | Division (dividend ÷ divisor) |
| [[jm3lib/pipelines/Math/Modulo\|-Math.Modulo]] | Modulo (dividend mod divisor) |
| [[jm3lib/pipelines/Math/Power\|-Math.Power]] | Exponentiation (base ^ exponent) |

### Unary (exactly 1 input)

| Pipeline | Description |
|----------|-------------|
| [[jm3lib/pipelines/Math/Abs\|-Math.Abs]] | Absolute value |
| [[jm3lib/pipelines/Math/Negate\|-Math.Negate]] | Arithmetic negation |

## Permissions

None. All operations are pure computation.

## Related

- [[jm3lib/INDEX|jm3lib Pipeline Index]]
- [[jm3lib/pipelines/Math/Add|-Math.Add]]
- [[jm3lib/pipelines/Math/Subtract|-Math.Subtract]]
- [[jm3lib/pipelines/Math/Multiply|-Math.Multiply]]
- [[jm3lib/pipelines/Math/Divide|-Math.Divide]]
- [[jm3lib/pipelines/Math/Modulo|-Math.Modulo]]
- [[jm3lib/pipelines/Math/Power|-Math.Power]]
- [[jm3lib/pipelines/Math/Abs|-Math.Abs]]
- [[jm3lib/pipelines/Math/Negate|-Math.Negate]]
