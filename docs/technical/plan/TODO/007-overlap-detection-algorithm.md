---
title: Conditional Overlap Detection — Unified Algorithm
related: PGE-604 (Rule 6.4), PGE-605 (Rule 6.5), Issue 007
status: TODO
created: 2026-03-19
---

# Conditional Overlap Detection — Unified Algorithm

## Problem

Overlap detection and exhaustiveness checking are **independent properties**:

- **Exhaustiveness** (PGE-601): Does the union of all branches cover the full domain?
- **Mutual exclusiveness** (PGE-604/605): Is the intersection of any two branches empty?

A conditional can be exhaustive but overlapping (ambiguous path), or mutually exclusive but non-exhaustive (missing cases). Overlap is always a compile error because the compiler cannot determine which branch to execute when a value matches multiple branches.

Currently PGE-604 says "check all pairs for intersection" without defining how. This document formalizes the detection algorithm per type and for compound conditions.

## Core Principle

Two branches **overlap** iff their condition sets have a non-empty intersection:

```
Overlap(Bᵢ, Bⱼ) ⟺ Set(Bᵢ) ∩ Set(Bⱼ) ≠ ∅
```

The compiler must check all pairs `(Bᵢ, Bⱼ)` where `i < j`.

## Algorithm By Type

### 1. Numeric (int / float) — Interval Intersection

Each numeric branch defines an interval on the number line.

**Interval representation:**

| Condition | Interval |
|-----------|----------|
| `>? N` | (N, +∞) |
| `>=? N` | [N, +∞) |
| `<? N` | (-∞, N) |
| `<=? N` | (-∞, N] |
| `?[a,b]` | [a, b] |
| `?(a,b)` | (a, b) |
| `?[a,b)` | [a, b) |
| `?(a,b]` | (a, b] |

**Intersection test:**

```
FUNCTION IntervalsOverlap(I₁ = [a, b], I₂ = [c, d]):
   // Adjust for open/closed boundaries:
   //   open boundary = exclusive, closed = inclusive

   IF I₁.upper < I₂.lower → no overlap
   IF I₂.upper < I₁.lower → no overlap
   IF I₁.upper = I₂.lower:
      overlap only if BOTH boundaries are closed (] meets [)
   IF I₂.upper = I₁.lower:
      overlap only if BOTH boundaries are closed
   OTHERWISE → overlap exists

   RETURN overlapping interval for error message
```

**Complexity:** O(N²) pairwise checks where N = number of branches. Typically N < 10 — trivial.

**Overlapping interval for diagnostics:**

```
Intersection([a,b], [c,d]) = [max(a,c), min(b,d)]
```

Boundary openness: each endpoint takes the more restrictive (open) of the two.

### 2. Enum / Boolean — Bit Set Intersection

Each enum/boolean branch matches a set of variants, representable as a bit set.

**Representation:**

| Condition | Bit set |
|-----------|---------|
| `=? #Enum.A` | {A} |
| `=? #Boolean.True` | {True} |
| Multiple branches on same variant | same bit |

**Intersection test:**

```
FUNCTION EnumBranchesOverlap(B₁, B₂):
   S₁ = bit set of variants matched by B₁
   S₂ = bit set of variants matched by B₂
   RETURN S₁ AND S₂ ≠ 0
```

**Complexity:** O(N²) pairwise, each check is O(1) bit operation.

**Note:** For enum/boolean, overlap means two branches test for the same variant. This is straightforward — if `=? #Status.Active` appears in two branches, they overlap.

### 3. Compound Conditions — Grid Cell Intersection

Compound conditions test multiple variables simultaneously. Uses the same grid from the exhaustiveness algorithm (see `006-compound-exhaustiveness-algorithm.md`).

**Algorithm:**

```
FUNCTION DetectCompoundOverlap(branches):
   1. Build the partition grid (same as exhaustiveness check)
      Grid = P₁ × P₂ × ... × Pₙ

   2. For each cell in Grid:
      covering = []
      For each branch Bᵢ:
         If Bᵢ covers this cell → append Bᵢ to covering
      If |covering| > 1:
         REPORT PGE-605: branches covering[0] and covering[1]
            overlap on cell (concrete values)

   3. No cell has |covering| > 1 → mutually exclusive
```

**Complexity:** O(G × N) where G = grid size, N = branches. Same tractability as exhaustiveness.

## Unified Dispatch

The compiler dispatches to the appropriate overlap check based on condition type:

```
FUNCTION CheckOverlap(conditional):
   MATCH type_of(conditional):
      Numeric     → PairwiseIntervalCheck    → fires PGE-604
      Enum        → PairwiseBitSetCheck      → fires PGE-604
      Boolean     → PairwiseBitSetCheck      → fires PGE-604
      Compound    → GridCellIntersection     → fires PGE-605
      String      → no overlap check (non-exhaustive type)
      Flexible    → no overlap check (non-exhaustive type)
```

**Why String/Flexible skip overlap check:** These types require `*?` (wildcard). The wildcard intentionally overlaps with everything — it's the catch-all. Overlap detection only applies to branches that claim specific coverage.

## Error Codes

| Code | Scope | Fires when |
|------|-------|------------|
| PGE-604 | Single-variable overlap | Two branches on the same variable have intersecting condition sets |
| PGE-605 | Compound overlap | Two compound branches cover the same cell in the partition grid |

## Relationship to Exhaustiveness

Both checks use the same underlying domain model and partition logic. In implementation, a single pass can check both:

```
FUNCTION AnalyzeConditional(branches):
   1. Build domain model / grid
   2. Check exhaustiveness (union = full domain?)     → PGE-601/602/603
   3. Check mutual exclusiveness (pairwise ∩ = ∅?)    → PGE-604/605
   // Independent results — both, neither, or one can fail
```

## Worked Examples

### Numeric Overlap

```polyglot
[?] $score >=? 80
   [r] =High
[?] $score >=? 70
   [r] =Mid
[?] $score <? 70
   [r] =Low
```

- Branch 1: [80, +∞)
- Branch 2: [70, +∞)
- Intersection: [80, +∞) — non-empty → **PGE-604**
- Diagnostic: "Branches 1 and 2 overlap on interval [80, +∞)"

### Numeric Non-Overlap

```polyglot
[?] $val <? 0
   [r] =Negative
[?] $val ?[0,50)
   [r] =Low
[?] $val ?[50,100]
   [r] =High
[?] $val >? 100
   [r] =VeryHigh
```

- (-∞,0) ∩ [0,50) = ∅ (open meets closed at 0 — no overlap)
- [0,50) ∩ [50,100] = ∅ (open meets closed at 50 — no overlap)
- [50,100] ∩ (100,+∞) = ∅ (closed meets open at 100 — no overlap)
- All pairs empty → **mutually exclusive**

### Compound Overlap

```polyglot
[?] $age >? 18
   [r] =Adult
[?] $age >? 15
   [r] =Teen
[?] $age <=? 15
   [r] =Child
```

- Partitions for `$age`: {≤15, 16..18, >18}
- Branch 1 covers {>18}
- Branch 2 covers {16..18, >18}
- Cell {>18} covered by Branch 1 AND Branch 2 → **PGE-605**
- Diagnostic: "Branches 1 and 2 overlap when $age > 18"
