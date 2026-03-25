---
audience: user
type: specification
updated: 2026-03-25
status: draft
---

# =Math — Numeric Operations

Stdlib pipelines for arithmetic on `;int` and `;float` values. No `[@]` import needed.

Math pipelines use positional (unnamed) IO — inputs and outputs are ordered, not named:

```polyglot
[r] =Math.Add
   [=] << $price
   [=] << $tax
   [=] >> $total
```

```
=Math
   .Add
      <<;int (variadic — 2+)
      >>;int
   .Subtract
      <<;int (exactly 2 — minuend, subtrahend)
      >>;int
   .Multiply
      <<;int (variadic — 2+)
      >>;int
   .Divide
      <<;int (exactly 2 — dividend, divisor)
      >>;int
   .Modulo
      <<;int (exactly 2 — dividend, divisor)
      >>;int
   .Power
      <<;int (exactly 2 — base, exponent)
      >>;int
   .Abs
      <<;int (exactly 1)
      >>;int
   .Negate
      <<;int (exactly 1)
      >>;int
```

All pipelines also accept `;float` operands. When any input is `;float`, the output is `;float`.

## Permissions

No permissions required. All operations are pure computation. See [[permissions]].

## Usage Examples

### Addition (variadic)
```polyglot
[r] =Math.Add
   [=] << $a
   [=] << $b
   [=] << $c
   [=] >> $sum
```

### Division with error handling
```polyglot
[r] =Math.Divide
   [=] << $numerator
   [=] << $denominator
   [=] >> $result
   [!] !Math.DivideByZero
      [r] >result << 0
```

### Chained operations
```polyglot
[ ] (a + b) * c
[r] =Math.Add
   [=] << $a
   [=] << $b
   [=] >> $sum

[r] =Math.Multiply
   [=] << $sum
   [=] << $c
   [=] >> $product
```

## Errors

```
=Math.Divide
   !Math.DivideByZero

=Math.Modulo
   !Math.DivideByZero
```
