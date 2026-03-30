---
audience: developer
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 14. Variable Lifecycle Constraints

These are semantic rules enforced by the grammar's type system, not syntactic productions:

1. **Declared** — variable appears without assignment. Holds no value.
2. **Default** — assigned via `<~` or `~>`. Allows one further reassignment.
3. **Final** — assigned via `<<` or `>>`. No further assignment permitted.
4. **Failed** — the pipeline responsible for producing the variable's value terminated with an error. The variable will never resolve; downstream pipelines waiting on it will not fire. **Fallback override:** if a `<!` fallback is declared on the IO line, the variable bypasses Failed and becomes Final with the fallback value. When fallback activates, `$var%sourceError` is set to the triggering error.
5. **Released** — scope ends (indentation returns to parent) or collected via `*`.

### Serialization Constraints

1. **Sibling separator homogeneity** — all siblings at the same level must use the same separator (all `.` or all `:`).
2. **Sibling kind homogeneity** — all siblings at the same level must be the same kind (all enum fields or all value fields). Assignment within value fields is individually optional — unassigned value fields remain in Declared state.
3. **Leaf-only assignment** — only leaf fields (those with no children) can have values assigned.

---
