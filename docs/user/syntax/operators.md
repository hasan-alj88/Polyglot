---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
---

# Operators

<!-- @c:variable-lifecycle -->
<!-- @u:io -->
<!-- @u:collections -->
<!-- @u:technical/ebnf/06-operators -->
<!-- @u:technical/edge-cases/06-operators -->
<!-- @u:conditionals#Comparison Operators -->
<!-- @u:errors#Error Fallback Operators -->

## Assignment Operators

Directional — the arrow indicates data flow. Operators push and pull data across branches of the `%` metadata tree (see [[data-is-trees]]). See [[variable-lifecycle]] for lifecycle semantics, [[io]] for IO parameter context, [[concepts/pipelines/io-triggers#IO as Implicit Triggers]] for how assignment mode affects trigger behavior.

| Operator | Name | Direction | Usage |
|----------|------|-----------|-------|
| `<<` | PushLeft (Final) | Right → left | `$x << "value"`, `@alias << @pkg`. See [[variable-lifecycle#Final]] |
| `>>` | PushRight (Final) | Left → right | `>array >> $arr`. See [[variable-lifecycle#Final]] |
| `<~` | DefaultPushLeft | Right → left | `.field <~ "value"`. See [[variable-lifecycle#Default]] |
| `~>` | DefaultPushRight | Left → right | `>output ~> ""`. See [[variable-lifecycle#Default]] |
| `!<` | FallbackPushLeft (Error) | Right → left | `!< "fallback"`. See [[errors#Error Fallback Operators]] |
| `!>` | FallbackPushRight (Error) | Left → right | `"fallback" !> >output`. See [[errors#Error Fallback Operators]] |

**Fallback validity:** `!<` and `!>` require a failable source — the right-hand side must be a pipeline call that can error at runtime. Using fallback with a literal or variable is a compile error ([[PGE07008\|PGE07008]]). A fallback chain (`!< -Pipeline.A !< -Pipeline.B !< "terminal"`) must terminate at a non-failable value; ending at a pipeline call is a compile error ([[PGE07009\|PGE07009]]).
| `->` | Chain | Left → right | `-A->-B->-C`. See [[concepts/pipelines/chains#Chain Execution]] |

## Comparison Operators

Used in `[?]` conditional blocks — see [[blocks#Control Flow]] and [[conditionals]] for exhaustiveness and logical operators.

Every `[?]` arm must include a comparison operator — bare subject lines like `[?] $variable` are invalid ([[PGE06009|PGE06009]]). The operator must be compatible with the subject variable's type ([[PGE04015|PGE04015]]).

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

```aljam3
[ ] Not less than — age is at least 18
[?] $age <!? 18
   [-] $eligible#bool << #Boolean.True
[?] *?
   [-] $eligible#bool << #Boolean.False

[ ] Not greater than — score capped at 100
[?] $score >!? 100
   [-] $capped#bool << #Boolean.True
[?] *?
   [-] $capped#bool << #Boolean.False
```

### Type-Operator Compatibility

The comparison operator must match the subject variable's type ([[PGE04015|PGE04015]]):

- **Numeric** (`#int`, `#float`): all comparison and range operators. Int and float share the same numeric domain — comparable without conversion.
- **String** (`#string`): equality only (`=?`, `=!?`). Ordering and ranges are invalid.
- **Bool** (`#bool`): equality only (`=?`, `=!?`).
- **Enum**: equality with enum variants of the same type. Ordering, ranges, and cross-type matches are invalid.
- **Wildcard** (`*?`): always valid on any type.

**Exhaustiveness rule:** All `[?]` conditional chains must be exhaustive. If the conditions do not cover every possible case, a `[?] *?` catch-all branch is mandatory. See [[conditionals]] for full exhaustiveness rules ([[PGE06001|PGE06001]]).

## Range Operators

Range checks use mathematical interval notation. The `?` prefix starts the range, then `[` (inclusive) or `(` (exclusive) for each bound. Ranges apply only to numeric types — `#int` and `#float` ([[PGE04015|PGE04015]]).

| Syntax | Left bound | Right bound | Example |
|--------|-----------|-------------|---------|
| `?[lo,hi]` | Inclusive | Inclusive | `$val ?[1,10]` — 1 ≤ val ≤ 10 |
| `?(lo,hi)` | Exclusive | Exclusive | `$val ?(0,100)` — 0 < val < 100 |
| `?[lo,hi)` | Inclusive | Exclusive | `$val ?[1,10)` — 1 ≤ val < 10 |
| `?(lo,hi]` | Exclusive | Inclusive | `$val ?(0,10]` — 0 < val ≤ 10 |

The lower bound must not exceed the upper bound ([[PGE04013|PGE04013]]). For inclusive ranges `?[lo,hi]`, `lo > hi` is invalid. For exclusive ranges `?(lo,hi)`, `lo >= hi` is invalid (the range would be empty). Single-point inclusive `?[5,5]` is valid (matches exactly 5).

## Arithmetic

Aljam3 does not have raw arithmetic operators. Arithmetic is performed through `-Math.*` jm3lib pipelines — raw tokens `+`, `-`, `*`, `/` in expression context are a compile error ([[PGE04010|PGE04010]]). This design keeps all operations inside the pipeline execution model (trigger → queue → wrapper → body) and avoids conflicts with existing operator meanings (`*` is a collector prefix).

| Operation | jm3lib Pipeline | Arity |
|-----------|----------------|-------|
| Addition | `-Math.Add` | variadic (2+) |
| Subtraction | `-Math.Subtract` | exactly 2 |
| Multiplication | `-Math.Multiply` | variadic (2+) |
| Division | `-Math.Divide` | exactly 2 |
| Modulo | `-Math.Modulo` | exactly 2 |
| Power | `-Math.Power` | exactly 2 |
| Absolute value | `-Math.Abs` | exactly 1 |
| Negate | `-Math.Negate` | exactly 1 |

All accept `#int` and `#float` operands. When any input is `#float`, the output is `#float`. Division or modulo with a literal `0` divisor is a compile error ([[PGE04011|PGE04011]]).

```aljam3
[ ] Addition
[-] -Math.Add
   (-) << $price
   (-) << $tax
   (-) >> $total

[ ] Division with error handling
[-] -Math.Divide
   (-) << $numerator
   (-) << $denominator
   (-) >> $result
   [!] !Math.DivideByZero
      [-] >result << 0
```

For string building, use `{$var}` interpolation — not concatenation. See [[syntax/types/strings#String Interpolation]] and [[PGE04005|PGE04005]] (undefined interpolation variable).

## Collection Operators

Prefixes, not identifiers. See [[concepts/collections/INDEX|collections]] for full semantics.

| Prefix | Operation | Usage |
|--------|-----------|-------|
| `=ForEach` | Expand (iterate) | `=ForEach.Array`. See [[concepts/collections/expand#Expand Operators]] |
| `*` | Collect (aggregate) | `*Into.Array`, `*Agg.Sum`. See [[concepts/collections/collect#Collect Operators]] |
| `=*` | Reassemble (atomic expand + collect) | `=*Agg.Sum`, `=*Into.Array`. See [[concepts/collections/reassemble#Reassemble Operators]] |
