---
type: enhancement
status: resolved
related: PGE-601 (Rule 6.1), PGE-605 (Compound Condition Overlap)
resolved: 2026-03-19
---

# Issue 006 — Compound Condition Exhaustiveness

## Resolution

**Set-theoretic partition refinement algorithm.** Full algorithm documented in:
`docs/technical/plan/TODO/006-compound-exhaustiveness-algorithm.md`

### Summary

- Each branch maps to a **set** over the variable's domain (Cartesian product space)
- **Exhaustive** = union of all branch sets = universal set
- **Open domain short-circuit**: if ANY dimension uses a non-exhaustive type (string, flexible field), `*?` is mandatory — no analysis attempted
- For exhaustive types (boolean, enum, numeric range), the compiler discretizes each dimension into partitions, builds a grid, and checks cell coverage
- **Overlap detection**: if two branch sets intersect, fires **PGE-605** (new error code, mirrors PGE-604 for numeric ranges)
- **Counterexamples**: uncovered cells are reported as concrete input combinations

### Complexity

Grid size = product of partition counts. Typical: 2–3 variables × 2–4 partitions = 4–64 cells. Polynomial and tractable.

### Industry context

No mainstream compiler does this (Rust, Swift, Kotlin all require wildcard for guarded conditions). Haskell's "Lower Your Guards" (2020, Peyton Jones) is closest. Polyglot's approach is novel.

## Original Problem

Compound conditions using `[&]` (AND), `[+]` (OR), and `[^]` (XOR) combine multiple comparisons. Previously the compiler could not determine whether compound branches were mutually exclusive and collectively exhaustive, so `*?` was always required.

## Example

```polyglot
[ ] Proven exhaustive by partition refinement — *? NOT required
[?] $age >? 18
   [&] $hasLicense =? #Boolean.True
      [r] =Allow.Drive
[?] $age >? 18
   [&] $hasLicense =? #Boolean.False
      [r] =Deny.NoLicense
[?] $age <=? 18
   [r] =Deny.Underage
[ ] Grid: 4 cells (≤18,T), (≤18,F), (>18,T), (>18,F) — all covered
```
