---
audience: developer
rule: "6.1"
code: PGE06001
name: Conditional Must Be Exhaustive
severity: error
---

# Rule 6.1 — Conditional Must Be Exhaustive
`PGE06001`

<!-- @u:syntax/types -->

**Statement:** Every `[?]` conditional block must be exhaustive — every possible value of the branched type must have a defined path. Exhaustiveness is proven in two ways:

1. **Static proof** — the compiler verifies all values are covered (closed types)
2. **`*?` catch-all** — required for open types where static proof is impossible

If neither static proof nor `*?` is present, PGE06001 fires.

**Rationale:** Every conditional must route every possible input. Missing branches cause undefined behavior at runtime. This enforces Polyglot's exhaustive coverage principle — if something can go wrong, the compiler catches it before production rather than discovering gaps at runtime.
**Detection:** The compiler determines the branched type, dispatches to the appropriate type-specific rule, and checks for `*?`. If neither the type-specific rule accepts the coverage nor `*?` is present, PGE06001 fires.

---

## Exhaustiveness by Type

| Type | Value Set | Rule | `*?` Required? |
|------|-----------|------|----------------|
| `{#}` enum (`.` fields) | Closed (finite variants) | PGE06002 | No — if all variants listed |
| `bool` (`#Boolean`) | Closed (2 variants) | PGE06002 | No — if both listed |
| `int` / `float` | Open but rangeable | PGE06003 (coverage), PGE06004 (overlap) | No — if ranges cover -∞ to +∞ |
| Fixed field (`.`) | Closed | PGE06002 — all siblings present | No — if all siblings listed |
| `string` | Open (infinite) | PGE06006 | Yes — always |
| Flexible field (`:`) | Open | PGE06007 | Yes — always |
| Compound (`[&]`/`[\|]`/`[^]`) | Complex | PGE06008, PGE06005 (overlap), PGE06013 (pre-check) | No — if all variables closed and partition proof succeeds; Yes — if any variable open |

**Key principle:** `*?` is a fallback for types where the compiler cannot prove exhaustiveness. When the compiler *can* prove exhaustiveness (enums with all variants, numeric ranges covering -∞ to +∞), `*?` is optional.

---

**See also:**
- [PGE06002 — Enum Exhaustiveness](PGE06002-enum-exhaustiveness.md)
- [PGE06003 — Numeric Range Not Exhaustive](PGE06003-numeric-range-not-exhaustive.md)
- [PGE06004 — Numeric Range Overlap](PGE06004-numeric-range-overlap.md)
- [PGE06005 — Compound Condition Overlap](PGE06005-compound-condition-overlap.md)
- [PGE06006 — String Exhaustiveness](PGE06006-string-exhaustiveness.md)
- [PGE06007 — Flexible Field Exhaustiveness](PGE06007-flexible-field-exhaustiveness.md)
- [PGE06008 — Compound Condition Exhaustiveness](PGE06008-compound-condition-exhaustiveness.md)
- [PGE06009 — Conditional Missing Comparison Operator](PGE06009-conditional-missing-comparison-operator.md)

## See Also

- [[user/concepts/conditionals|Conditionals]] — exhaustiveness rules and PGE06001 error table
- [[user/syntax/operators|Operators]] — exhaustiveness rule for `[?]` chains references PGE06001
