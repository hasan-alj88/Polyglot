---
title: Compound Condition Exhaustiveness — Set-Theoretic Algorithm
related: Issue 006, PGE-601 (Rule 6.1), PGE-605 (Compound Condition Overlap)
status: TODO
created: 2026-03-19
---

# Compound Condition Exhaustiveness — Set-Theoretic Algorithm

## Problem

Compound conditions using `[&]` (AND), `[+]` (OR), and `[^]` (XOR) combine multiple comparisons. The compiler must determine whether compound branches are mutually exclusive and collectively exhaustive, so `*?` can be made optional when provably complete.

No mainstream compiler solves this — Rust, Swift, and Kotlin all require a wildcard/default for guarded conditions. Haskell's "Lower Your Guards" (2020, Peyton Jones et al.) is the closest academic work. Polyglot's approach is novel: set-theoretic partition refinement over typed domains.

## Domain Model

Every variable in a conditional has a **domain** — the set of all values it can take:

| Type | Domain | Finite? | Representation |
|------|--------|---------|----------------|
| Boolean | {True, False} | Yes (2) | Bit set |
| Enum | {v₁, v₂, ..., vₙ} | Yes (N) | Bit set |
| Int | ℤ | No, but partitionable | Intervals |
| Float | ℝ | No, but partitionable | Intervals |
| String | Σ* | No | **Non-exhaustive** |
| Flexible | open | No | **Non-exhaustive** |

## Product Space

Compound conditionals test **multiple variables**. The full space is the **Cartesian product** of all tested variable domains:

```
Space = Domain($v₁) × Domain($v₂) × ... × Domain($vₙ)
```

Each branch carves out a **region** (subset) of this space. Exhaustiveness means:

```
Branch₁ ∪ Branch₂ ∪ ... ∪ Branchₖ = Space
```

## Corollary: Open Domain Short-Circuit

If **ANY** dimension has an open domain (string, flexible field), the product space is infinite and uncountable. No finite set of branches can cover it. Therefore `*?` is **mandatory**. The compiler short-circuits before building the grid.

## Algorithm: Partition Refinement

```
FUNCTION IsExhaustive(branches):
   1. Collect all variables V = {$v₁, ..., $vₙ} tested across all branches

   2. For each $vᵢ:
      - If type is String or Flexible → RETURN "require *?"
      - Extract all comparison boundaries → compute partitions Pᵢ

   3. Grid = P₁ × P₂ × ... × Pₙ   (all cells)

   4. For each cell in Grid:
      - For each branch:
        - If branch covers this cell, mark cell as covered; break
      - If no branch covers cell → NOT exhaustive
        - Report uncovered cell as counterexample

   5. All cells covered → EXHAUSTIVE, *? is optional
```

## Partition Rules By Type

### Boolean

Always 2 partitions: `{True, False}`

### Enum

One partition per variant: `{v₁, v₂, ..., vₙ}` — pulled from the `#DataType` definition.

### Numeric (int/float)

Boundaries from conditions create intervals. Example:

- Conditions `>? 18`, `<=? 10` on `$age`
- Partitions: `{≤10, 11..18, >18}`
- Open-ended ranges require the type's full range (int min/max or -∞/+∞)

## Overlap Detection (PGE-605)

Same grid, different check:

```
FUNCTION DetectOverlap(branches):
   For each cell in Grid:
      covering = [branches that cover this cell]
      If |covering| > 1 → OVERLAP error (PGE-605)
        - Report which branches overlap on which cell
```

## Complexity

Grid size = product of partition counts. For typical Polyglot code:

- 2–3 variables, 2–4 partitions each → 4–64 cells
- Perfectly tractable at compile time

Worst case with N variables each having K partitions = O(Kᴺ), but this is bounded by real code patterns — nobody writes 10-dimensional compound conditions.

## Worked Example

```polyglot
[?] $age >? 18
   [&] $hasLicense =? #Boolean.True
      [r] =Allow.Drive
[?] $age >? 18
   [&] $hasLicense =? #Boolean.False
      [r] =Deny.NoLicense
[?] $age <=? 18
   [r] =Deny.Underage
```

**Step 1 — Identify dimensions and split points:**
- `$age`: conditions `>? 18` and `<=? 18` → partitions: {≤18, >18}
- `$hasLicense`: conditions `=? True` and `=? False` → partitions: {True, False}

**Step 2 — Build the grid** (product of all partitions):

| Cell | $age | $hasLicense |
|------|------|-------------|
| C₁ | ≤18 | True |
| C₂ | ≤18 | False |
| C₃ | >18 | True |
| C₄ | >18 | False |

**Step 3 — Map branches to cells:**
- Branch 1: `$age >18 AND $hasLicense=True` → covers **C₃**
- Branch 2: `$age >18 AND $hasLicense=False` → covers **C₄**
- Branch 3: `$age ≤18` (no constraint on $hasLicense) → covers **C₁, C₂**

**Step 4 — Check coverage:**
- C₁ ✓, C₂ ✓, C₃ ✓, C₄ ✓ → **all cells covered → EXHAUSTIVE**

## Non-Exhaustive Example

```polyglot
[?] $age >? 18
   [&] $hasLicense =? #Boolean.True
      [r] =Allow.Drive
[?] $age <=? 18
   [r] =Deny.Underage
```

Grid: same 4 cells.
- Branch 1 covers C₃
- Branch 2 covers C₁, C₂
- **C₄ uncovered** → NOT exhaustive
- Counterexample: `$age >18, $hasLicense=False`
- Compiler fires PGE-601 with: "Uncovered case: $age > 18 and $hasLicense = #Boolean.False"

## Overlap Example

```polyglot
[?] $age >? 18
   [r] =Adult
[?] $age >? 15
   [r] =Teen
[?] $age <=? 15
   [r] =Child
```

Partitions for `$age`: {≤15, 16..18, >18}
- Branch 1 covers {>18}
- Branch 2 covers {16..18, >18}
- Branch 3 covers {≤15}
- Cell {>18} covered by BOTH Branch 1 and Branch 2 → **PGE-605: overlap**
