---
audience: developer
name: Conditional Overlap Detection (Unified Dispatch)
type: algorithm
consumes: PGE06004, PGE06005
---

# Conditional Overlap Detection Algorithm

<!-- @u:syntax/types -->

Detects when two or more `[?]` branches cover the same value, creating ambiguity. The compiler dispatches to a type-specific overlap algorithm based on the branched variable's type.

## Inputs

- A `[?]` conditional block with two or more branches
- Type information for the branched variable

## Dispatch Table

| Variable Type | Overlap Method | Rule | Complexity |
|---|---|---|---|
| int / float | Interval intersection | PGE06004 | O(N²) pairwise |
| Enum (`.` fixed fields) | Bit set AND | PGE06004 (enum) | O(N²) pairwise, O(1) per pair |
| Bool (`#Boolean`) | Bit set AND | PGE06004 (enum) | O(N²) pairwise, O(1) per pair |
| Compound (`[&]`/`[\|]`/`[^]`) | Grid cell coverage | PGE06005 | O(N × K^M) — see [compound algorithm](compound-exhaustiveness.md) |
| String (`#String`) | Exact literal match | PGE06004 (string) | O(N²) pairwise |
| Flexible field (`:`) | Exact literal match | PGE06004 (flexible) | O(N²) pairwise |

N = number of branches, K = max partitions per variable, M = number of variables in compound.

## Algorithm 1 — Numeric Interval Intersection (int/float)

### Step 1: Normalize branches to intervals

Each branch's comparison operator or range notation maps to an interval `[lo, hi]` with boundary inclusivity:

| Syntax | Interval |
|---|---|
| `$x ?> V` | (V, +∞) |
| `$x ?>= V` | [V, +∞) |
| `$x ?< V` | (-∞, V) |
| `$x ?<= V` | (-∞, V] |
| `$x ?= V` | [V, V] |
| `$x ?!= V` | (-∞, V) ∪ (V, +∞) |
| `$x ?[A,B]` | [A, B] |
| `$x ?(A,B]` | (A, B] |
| `$x ?[A,B)` | [A, B) |
| `$x ?(A,B)` | (A, B) |

**Negation (`?!=`):** Produces two disjoint intervals. Each half is tested independently against other branches.

### Step 2: Pairwise intersection

For each pair of branches (i, j), compute the intersection of their intervals:

```gherkin
Given interval A = [a_lo, a_hi] and B = [b_lo, b_hi]:
  intersection_lo = max(a_lo, b_lo)
  intersection_hi = min(a_hi, b_hi)
```

The intersection is non-empty if:
- `intersection_lo < intersection_hi`, OR
- `intersection_lo == intersection_hi` AND both boundaries are inclusive at that point

### Step 3: Boundary inclusivity

The boundary type (open vs closed) determines whether the exact boundary value is shared:

| A boundary | B boundary | Overlap at boundary? |
|---|---|---|
| `]` closed | `[` closed | Yes — value is in both |
| `)` open | `[` closed | No — A excludes, B includes |
| `]` closed | `(` open | No — A includes, B excludes |
| `)` open | `(` open | No — both exclude |

Example:
- `$x ?[0, 10]` and `$x ?(10, 20]` → intersection at 10: A is `]` closed, B is `(` open → **no overlap**
- `$x ?[0, 10]` and `$x ?[10, 20]` → intersection at 10: both `]`/`[` closed → **overlap at {10}**

### Step 4: Report

If any pair has non-empty intersection → **PGE06004** with:
- The two overlapping branch numbers
- The overlapping interval as counterexample

## Algorithm 2 — Enum/Bool Bit Set Intersection

### Step 1: Build bit sets

Each branch maps to a bit set of covered variants:

| Branch condition | Bit set |
|---|---|
| `$color ?= .Red` | {Red} |
| `$color ?!= .Red` | {Green, Blue, ...} (complement of {Red} in full variant set) |
| `$bool ?= #Boolean.True` | {True} |
| `$bool ?!= #Boolean.True` | {False} |

The full variant set is known at compile time from the `{#}` definition.

### Step 2: Pairwise AND

For each pair of branches (i, j):
```text
overlap = bitset_i AND bitset_j
```

If `overlap` is non-empty → branches share at least one variant.

### Step 3: Report

If any pair has non-empty AND → **PGE06004** with:
- The two overlapping branch numbers
- The shared variants as counterexample

**Edge case — negation overlap:**
- Branch 1: `$color ?!= .Red` → {Green, Blue}
- Branch 2: `$color ?!= .Blue` → {Red, Green}
- AND: {Green} → **PGE06004** — overlap at `.Green`

## Algorithm 3 — String/Flexible Exact Literal Match

String and flexible field types have open (infinite) domains. Full overlap analysis is impossible, but the compiler can detect **identical literal branches**:

### Step 1: Collect literal values

For each branch with an `?=` comparison against a string literal, record the literal value.

### Step 2: Duplicate detection

If two branches test the same literal value → overlap.

Negation branches (`?!=`) on strings are not checked for overlap — the complement of a single string is effectively the entire domain minus one value, which always overlaps with any other branch. This is handled by the `?*` requirement (PGE06006/PGE06007).

### Step 3: Report

If duplicate literals found → **PGE06004** with the duplicate value.

## Algorithm 4 — Compound Condition Grid Cell Overlap

Delegated to the [partition refinement algorithm](compound-exhaustiveness.md), Step 5. The grid is built from the Cartesian product of all tested variables, branches are mapped to cells, and any cell covered by more than one branch triggers **PGE06005**.

## Unified Entry Point

```text
function checkOverlap(conditional):
    variable = conditional.branchedVariable
    branches = conditional.branches

    if branches contain compound operators ([&], [+], [^]):
        → Algorithm 4 (grid cell overlap via partition refinement)
    else if variable.type is int or float:
        → Algorithm 1 (interval intersection)
    else if variable.type is enum or bool:
        → Algorithm 2 (bit set intersection)
    else if variable.type is string or flexible:
        → Algorithm 3 (exact literal match)
```

## See Also

- [PGE06004 — Numeric Range Overlap](../PGE/PGE06004-numeric-range-overlap.md)
- [PGE06005 — Compound Condition Overlap](../PGE/PGE06005-compound-condition-overlap.md)
- [Compound Exhaustiveness Algorithm](compound-exhaustiveness.md)
