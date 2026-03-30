---
audience: user
type: specification
updated: 2026-03-24
status: complete
---

# Operators

<!-- @variable-lifecycle -->
<!-- @io -->
<!-- @collections -->

## Assignment Operators

Directional — the arrow indicates data flow. Operators push and pull data across branches of the `%` metadata tree (see [[data-is-trees]]). See [[variable-lifecycle]] for lifecycle semantics, [[io]] for IO parameter context, [[concepts/pipelines/io-triggers#IO as Implicit Triggers]] for how assignment mode affects trigger behavior.

| Operator | Name | Direction | Usage |
|----------|------|-----------|-------|
| `<<` | Push (Final) | Right → left | `$x << "value"`, `@alias << @pkg`. See [[variable-lifecycle#Final]] |
| `>>` | Pull (Final) | Left → right | `>array >> $arr`. See [[variable-lifecycle#Final]] |
| `<~` | Default | Right → left | `.field <~ "value"`. See [[variable-lifecycle#Default]] |
| `~>` | Default | Left → right | `>output ~> ""`. See [[variable-lifecycle#Default]] |
| `<!` | Fallback (Error) | Right → left | `<! "fallback"`. See [[errors#Error Fallback Operators]] |
| `!>` | Fallback (Error) | Left → right | `"fallback" !> >output`. See [[errors#Error Fallback Operators]] |
| `=>` | Chain | Left → right | `=A=>=B=>=C`. See [[concepts/pipelines/chains#Chain Execution]] |

## Comparison Operators

Used in `[?]` conditional blocks — see [[blocks#Control Flow]] and [[conditionals]] for exhaustiveness and logical operators.

Every `[?]` arm must include a comparison operator — bare subject lines like `[?] $variable` are invalid ([[PGE-609|PGE-609]]). The operator must be compatible with the subject variable's type ([[PGE-415|PGE-415]]).

| Operator | Meaning |
|----------|---------|
| `=?` | Equal |
| `>?` | Greater than |
| `<?` | Less than |
| `>=?` | Greater or equal |
| `<=?` | Less or equal |
| `=!?` | Not equal |
| `*?` | Wildcard (else/catch-all) |

### Negation Operators

Any comparison operator can be negated by inserting `!` before `?`. This replaces the need for a standalone NOT logical operator. Negation inverts the comparison result:

| Operator | Meaning | Equivalent |
|----------|---------|------------|
| `=!?` | Not equal | opposite of `=?` |
| `<!?` | Not less than | equivalent to `>=` |
| `>!?` | Not greater than | equivalent to `<=` |
| `<=!?` | Not less-or-equal | equivalent to `>` |
| `>=!?` | Not greater-or-equal | equivalent to `<` |

```polyglot
[ ] Not less than — age is at least 18
[?] $age <!? 18
   [r] $eligible#bool << #Boolean.True
[?] *?
   [r] $eligible#bool << #Boolean.False

[ ] Not greater than — score capped at 100
[?] $score >!? 100
   [r] $capped#bool << #Boolean.True
[?] *?
   [r] $capped#bool << #Boolean.False
```

### Type-Operator Compatibility

The comparison operator must match the subject variable's type ([[PGE-415|PGE-415]]):

- **Numeric** (`#int`, `#float`): all comparison and range operators. Int and float interoperate freely.
- **String** (`#string`): equality only (`=?`, `=!?`). Ordering and ranges are invalid.
- **Bool** (`#bool`): equality only (`=?`, `=!?`).
- **Enum**: equality with enum variants of the same type. Ordering, ranges, and cross-type matches are invalid.
- **Wildcard** (`*?`): always valid on any type.

**Exhaustiveness rule:** All `[?]` conditional chains must be exhaustive. If the conditions do not cover every possible case, a `[?] *?` catch-all branch is mandatory. See [[conditionals]] for full exhaustiveness rules ([[PGE-601|PGE-601]]).

## Range Operators

Range checks use mathematical interval notation. The `?` prefix starts the range, then `[` (inclusive) or `(` (exclusive) for each bound. Ranges apply only to numeric types — `#int` and `#float` ([[PGE-415|PGE-415]]).

| Syntax | Left bound | Right bound | Example |
|--------|-----------|-------------|---------|
| `?[lo,hi]` | Inclusive | Inclusive | `$val ?[1,10]` — 1 ≤ val ≤ 10 |
| `?(lo,hi)` | Exclusive | Exclusive | `$val ?(0,100)` — 0 < val < 100 |
| `?[lo,hi)` | Inclusive | Exclusive | `$val ?[1,10)` — 1 ≤ val < 10 |
| `?(lo,hi]` | Exclusive | Inclusive | `$val ?(0,10]` — 0 < val ≤ 10 |

The lower bound must not exceed the upper bound ([[PGE-412|PGE-412]]). For inclusive ranges `?[lo,hi]`, `lo > hi` is invalid. For exclusive ranges `?(lo,hi)`, `lo >= hi` is invalid (the range would be empty). Single-point inclusive `?[5,5]` is valid (matches exactly 5).

## Arithmetic

Polyglot does not have raw arithmetic operators. Arithmetic is performed through `=Math.*` stdlib pipelines — raw tokens `+`, `-`, `*`, `/` in expression context are a compile error ([[PGE-410|PGE-410]]). This design keeps all operations inside the pipeline execution model (trigger → queue → wrapper → body) and avoids conflicts with existing operator meanings (`*` is a collector prefix).

| Operation | Stdlib Pipeline | Arity |
|-----------|----------------|-------|
| Addition | `=Math.Add` | variadic (2+) |
| Subtraction | `=Math.Subtract` | exactly 2 |
| Multiplication | `=Math.Multiply` | variadic (2+) |
| Division | `=Math.Divide` | exactly 2 |
| Modulo | `=Math.Modulo` | exactly 2 |
| Power | `=Math.Power` | exactly 2 |
| Absolute value | `=Math.Abs` | exactly 1 |
| Negate | `=Math.Negate` | exactly 1 |

All accept `#int` and `#float` operands. When any input is `#float`, the output is `#float`. Division or modulo with a literal `0` divisor is a compile error ([[PGE-411|PGE-411]]).

```polyglot
[ ] Addition
[r] =Math.Add
   [=] << $price
   [=] << $tax
   [=] >> $total

[ ] Division with error handling
[r] =Math.Divide
   [=] << $numerator
   [=] << $denominator
   [=] >> $result
   [!] !Math.DivideByZero
      [r] >result << 0
```

For string building, use `{$var}` interpolation — not concatenation. See [[syntax/types/strings#String Interpolation]] and [[PGE-405|PGE-405]] (undefined interpolation variable).

## Collection Operators

Prefixes, not identifiers. See [[concepts/collections/INDEX|collections]] for full semantics.

| Prefix | Operation | Usage |
|--------|-----------|-------|
| `~` | Expand (iterate) | `~ForEach.Array`. See [[concepts/collections/expand#Expand Operators]] |
| `*` | Collect (aggregate) | `*Into.Array`, `*Agg.Sum`. See [[concepts/collections/collect#Collect Operators]] |
