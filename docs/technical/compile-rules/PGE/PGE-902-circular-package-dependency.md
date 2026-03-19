---
rule: "9.2"
code: PGE-902
name: Circular Package Dependency
severity: error
---

### Rule 9.2 — Circular Package Dependency
`PGE-902`

**Statement:** Package imports must form a directed acyclic graph (DAG). If Package A imports Package B (directly or transitively) and Package B imports Package A, PGE-902 fires. The compiler must report the cycle path.
**Rationale:** Circular dependencies create initialization order ambiguity — neither package can be fully resolved before the other. This is always a design error in Polyglot's deterministic execution model.
**Detection:** The compiler builds a dependency graph from all `[@]` import declarations across files. If a cycle is detected (via topological sort or DFS), PGE-902 fires with the cycle path (e.g., "A → B → C → A").

**See also:** PGE-901 (undefined alias), PGE-103 (one package per file)

**VALID:**
```polyglot
[ ] ✓ Package A imports B, B imports C — no cycle
[ ] File: PackageA.pg
{@} @Local:999.A:v1.0.0
   [@] @b << @Local:999.B:v1.0.0

[ ] File: PackageB.pg
{@} @Local:999.B:v1.0.0
   [@] @c << @Local:999.C:v1.0.0

[ ] File: PackageC.pg
{@} @Local:999.C:v1.0.0
   [ ] no imports — leaf package
```

**INVALID:**
```polyglot
[ ] ✗ PGE-902 — circular dependency: A → B → A
[ ] File: PackageA.pg
{@} @Local:999.A:v1.0.0
   [@] @b << @Local:999.B:v1.0.0      [ ] A depends on B

[ ] File: PackageB.pg
{@} @Local:999.B:v1.0.0
   [@] @a << @Local:999.A:v1.0.0      [ ] ✗ PGE-902 — B depends on A, creating cycle
```

**Open point:** None.
