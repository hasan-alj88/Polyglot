---
name: Compound Condition Partition Refinement
type: algorithm
consumes: PGE06005, PGE06008
prerequisite: PGE06013
---

# Compound Condition Partition Refinement Algorithm

Determines whether compound conditional branches (`[&]` AND, `[|]` OR, `[^]` XOR) are collectively exhaustive (PGE06008) and mutually exclusive (PGE06005).

## Inputs

- A `[?]` conditional block containing compound conditions
- Type information for each tested variable

## Domain Classification

The algorithm classifies each tested variable's type as **closed** or **open**:

| Type | Classification | Partition Method |
|------|---------------|-----------------|
| Enum (`.` fixed fields) | Closed | Finite variant set |
| Bool (`#Boolean`) | Closed | {True, False} |
| int | Closed (rangeable) | Split at boundary values into intervals |
| float | Closed (rangeable) | Split at boundary values into intervals |
| string (`#String`) | Open | Cannot partition |
| RawString | Open | Cannot partition |
| Flexible field (`:`) | Open | Cannot partition |

## Algorithm Steps

### Step 0 — PGE06013 Pre-Check

Evaluate each branch's compound expression individually. If any single branch is always-True (tautology) or always-False (contradiction), PGE06013 fires. The partition algorithm does not run — all branches must be individually satisfiable.

### Step 1 — Open-Type Short-Circuit

If **any** variable in the compound condition has an open type → the entire compound is open-ended → `*?` is mandatory. Skip grid analysis. If `*?` is absent, PGE06008 fires.

### Step 2 — Build Domain Partitions

For each variable, collect all boundary values from all branches and partition the domain:

**Enum/Bool:**
The partition is the full variant set. Example: `#Color` with `.Red`, `.Green`, `.Blue` → partition = {Red, Green, Blue}.

**int/float (interval splitting):**
Collect all comparison values from all branches. Sort them. Create intervals between boundaries.

Example: branches test `$age >? 18` and `$age <=? 65`
- Boundary values: {18, 65}
- Partitions: (-∞, 18], (18, 65], (65, +∞)

The boundary inclusivity follows the comparison operators used (`>?` vs `>=?`, `<?` vs `<=?`).

### Step 3 — Cartesian Product Grid

Compute the full space as the Cartesian product of all variable partitions.

Example: `$status ∈ {Active, Inactive}` × `$age ∈ {(-∞,18], (18,65], (65,+∞)}`
→ 2 × 3 = 6 cells:
```
{(Active, ≤18), (Active, 18-65), (Active, >65),
 (Inactive, ≤18), (Inactive, 18-65), (Inactive, >65)}
```

### Step 4 — Map Branches to Cells

For each branch, determine which cells in the grid its compound expression covers:

- **`[&]` AND:** A cell is covered if **all** sub-conditions are satisfied → intersection
- **`[|]` OR:** A cell is covered if **any** sub-condition is satisfied → union
- **`[^]` XOR:** A cell is covered if **exactly one** sub-condition is satisfied → symmetric difference

Example: `$status =? .Active [|] $age >? 65` (OR)
- `$status =? .Active` covers: {(Active, ≤18), (Active, 18-65), (Active, >65)}
- `$age >? 65` covers: {(Active, >65), (Inactive, >65)}
- OR union: {(Active, ≤18), (Active, 18-65), (Active, >65), (Inactive, >65)}

### Step 5 — Check Overlap (PGE06005)

Scan the grid. If any cell is covered by more than one branch → **PGE06005** fires.

The diagnostic must include:
- The overlapping branch numbers
- A concrete input from the shared cell as counterexample

Overlap is always an error, regardless of whether `*?` is present.

### Step 6 — Check Exhaustiveness (PGE06008)

Compute the union of all branch regions. If the union does not equal the full grid → not exhaustive.

- If `*?` is present → exhaustive (wildcard covers remaining cells)
- If `*?` is absent → **PGE06008** fires

The diagnostic must include the uncovered cells as counterexample.

## Complexity

**Time:** O(K^N) where K = max partitions per variable, N = number of variables tested.

**Practical bounds:** Real Polyglot code typically uses 2–3 variables with 2–4 partitions each, yielding grids of 4–64 cells. The exponential bound is theoretical — the algorithm is efficient for practical code patterns.

**Worst case:** A conditional testing 5 enum variables with 10 variants each → 100,000 cells. The compiler may emit a diagnostic suggesting simplification if the grid exceeds a threshold.

## Walkthrough — Test 5 (Numeric Range × Enum)

```
Variables: $status ∈ {Active, Inactive}, $age#int
Boundaries for $age: {18}
Partitions: $status → {Active, Inactive}, $age → {(-∞,18], (18,+∞)}
Grid (4 cells):
  (Active, ≤18), (Active, >18), (Inactive, ≤18), (Inactive, >18)

Branch 1: $status =? .Active [&] $age >? 18   → {(Active, >18)}
Branch 2: $status =? .Active [&] $age <=? 18  → {(Active, ≤18)}
Branch 3: $status =? .Inactive                 → {(Inactive, ≤18), (Inactive, >18)}

Overlap check: no cell covered twice ✓
Union: all 4 cells ✓ exhaustive
Result: PASS
```

## See Also

- [PGE06005 — Compound Condition Overlap](../PGE/PGE06005-compound-condition-overlap.md)
- [PGE06008 — Compound Condition Exhaustiveness](../PGE/PGE06008-compound-condition-exhaustiveness.md)
- [PGE06013 — Tautological or Contradictory Branch Condition](../PGE/PGE06013-tautological-branch-condition.md)
