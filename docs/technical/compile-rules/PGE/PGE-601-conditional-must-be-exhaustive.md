---
rule: "6.1"
code: PGE-601
name: Conditional Must Be Exhaustive
severity: error
---

### Rule 6.1 — Conditional Must Be Exhaustive
`PGE-601`

**Statement:** Every `[?]` conditional block must be exhaustive — every possible value of the branched type must have a defined path. Exhaustiveness is proven in two ways:

1. **Static proof** — the compiler verifies all values are covered (closed types)
2. **`*?` catch-all** — required for open types where static proof is impossible

If neither static proof nor `*?` is present, PGE-601 fires.

**Rationale:** Every conditional must route every possible input. Missing branches cause undefined behavior at runtime.
**Detection:** The compiler determines the branched type, dispatches to the appropriate type-specific rule, and checks for `*?`. If neither the type-specific rule accepts the coverage nor `*?` is present, PGE-601 fires.

---

#### Exhaustiveness by Type

| Type | Value Set | Rule | `*?` Required? |
|------|-----------|------|----------------|
| `{#}` enum (`.` fields) | Closed (finite variants) | PGE-602 | No — if all variants listed |
| `bool` (`#Boolean`) | Closed (2 variants) | PGE-602 | No — if both listed |
| `int` / `float` | Open but rangeable | PGE-603 (coverage), PGE-604 (overlap) | No — if ranges cover -∞ to +∞ |
| Fixed field (`.`) | Closed | PGE-602 — all siblings present | No — if all siblings listed |
| `string` | Open (infinite) | PGE-606 | Yes — always |
| Flexible field (`:`) | Open | PGE-607 | Yes — always |
| Compound (`[&]`/`[+]`/`[^]`) | Complex | PGE-608, PGE-605 (overlap) | Yes — always |

**Key principle:** `*?` is a fallback for types where the compiler cannot prove exhaustiveness. When the compiler *can* prove exhaustiveness (enums with all variants, numeric ranges covering -∞ to +∞), `*?` is optional.

---

**See also:**
- [PGE-602 — Enum Exhaustiveness](PGE-602-enum-exhaustiveness.md)
- [PGE-603 — Numeric Range Not Exhaustive](PGE-603-numeric-range-not-exhaustive.md)
- [PGE-604 — Numeric Range Overlap](PGE-604-numeric-range-overlap.md)
- [PGE-605 — Compound Condition Overlap](PGE-605-compound-condition-overlap.md)
- [PGE-606 — String Exhaustiveness](PGE-606-string-exhaustiveness.md)
- [PGE-607 — Flexible Field Exhaustiveness](PGE-607-flexible-field-exhaustiveness.md)
- [PGE-608 — Compound Condition Exhaustiveness](PGE-608-compound-condition-exhaustiveness.md)
- [PGE-609 — Conditional Missing Comparison Operator](PGE-609-conditional-missing-comparison-operator.md)
