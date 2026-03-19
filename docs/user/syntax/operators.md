---
audience: user
type: specification
updated: 2026-03-15
status: draft
---

# Operators

<!-- @variable-lifecycle -->
<!-- @io -->
<!-- @collections -->

## Assignment Operators

Directional — the arrow indicates data flow. See [[variable-lifecycle]] for lifecycle semantics, [[io]] for IO parameter context, [[pipelines#IO as Implicit Triggers]] for how assignment mode affects trigger behavior.

| Operator | Name | Direction | Usage |
|----------|------|-----------|-------|
| `<<` | Push (Final) | Right → left | `$x << "value"`, `@alias << @pkg`. See [[variable-lifecycle#Final]] |
| `>>` | Pull (Final) | Left → right | `>array >> $arr`. See [[variable-lifecycle#Final]] |
| `<~` | Default | Right → left | `.field <~ "value"`. See [[variable-lifecycle#Default]] |
| `~>` | Default | Left → right | `>output ~> ""`. See [[variable-lifecycle#Default]] |

## Comparison Operators

Used in `[?]` conditional blocks — see [[blocks#Control Flow]].

| Operator | Meaning |
|----------|---------|
| `=?` | Equal |
| `>?` | Greater than |
| `<?` | Less than |
| `>=?` | Greater or equal |
| `<=?` | Less or equal |
| `=!?` | Not equal |
| `<!?` | Not less than (negated less-than) |
| `>!?` | Not greater than (negated greater-than) |
| `<=!?` | Not less-or-equal (negated) |
| `>=!?` | Not greater-or-equal (negated) |
| `*?` | Wildcard (else/catch-all) |

**Negation pattern:** Any comparison operator can be negated by inserting `!` before `?`. This replaces the need for a standalone NOT logical operator.

**Exhaustiveness rule:** All `[?]` conditional chains must be exhaustive. If the conditions do not cover every possible case, a `[?] *?` catch-all branch is mandatory.

## Range Operators

Range checks use mathematical interval notation. The `?` prefix starts the range, then `[` (inclusive) or `(` (exclusive) for each bound:

| Syntax | Left bound | Right bound | Example |
|--------|-----------|-------------|---------|
| `?[lo,hi]` | Inclusive | Inclusive | `$val ?[1,10]` — 1 ≤ val ≤ 10 |
| `?(lo,hi)` | Exclusive | Exclusive | `$val ?(0,100)` — 0 < val < 100 |
| `?[lo,hi)` | Inclusive | Exclusive | `$val ?[1,10)` — 1 ≤ val < 10 |
| `?(lo,hi]` | Exclusive | Inclusive | `$val ?(0,10]` — 0 < val ≤ 10 |

## Collection Operators

Prefixes, not identifiers. See [[collections]] for full semantics.

| Prefix | Operation | Usage |
|--------|-----------|-------|
| `~` | Expand (iterate) | `~ForEach.Array`. See [[collections#Expand Operators]] |
| `*` | Collect (aggregate) | `*Into.Array`, `*Agg.Sum`. See [[collections#Collect Operators]] |
