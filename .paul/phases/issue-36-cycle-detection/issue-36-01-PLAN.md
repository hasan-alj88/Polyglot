---
phase: issue-36-cycle-detection
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - docs/technical/compile-rules/algorithms/cycle-detection.md
autonomous: true
---

<objective>
## Goal
Write the algorithm specification document for pipeline call cycle detection (topological sort / DFS), covering intra-package `{=}` call graphs as required by PGE-914.

## Purpose
PGE-914 defines the compile rule but not the detection algorithm. Issue #36 requests a formal algorithm spec — inputs, steps, complexity, edge cases — matching the style of existing algorithm docs (overlap-detection.md, compound-exhaustiveness.md).

## Output
- `docs/technical/compile-rules/algorithms/cycle-detection.md` — complete algorithm specification
</objective>

<context>
## Project Context
@.paul/PROJECT.md
@.paul/STATE.md

## Source Files
@docs/technical/compile-rules/PGE/PGE-914-circular-pipeline-call.md
@docs/technical/compile-rules/PGE/PGE-902-circular-package-dependency.md
@docs/technical/compile-rules/PGE/PGE-414-recursive-data-definition.md
@docs/technical/compile-rules/algorithms/overlap-detection.md (style reference)
</context>

<skills>
No specialized flows configured.
</skills>

<acceptance_criteria>

## AC-1: Algorithm document exists with correct frontmatter
```gherkin
Given the algorithms directory exists
When the plan completes
Then docs/technical/compile-rules/algorithms/cycle-detection.md exists with YAML frontmatter containing name, type: algorithm, and consumes: PGE-914
```

## AC-2: Algorithm covers all edge cases from issue #36
```gherkin
Given the algorithm specification
When reviewed against the issue description
Then it addresses: self-call, mutual recursion, transitive cycles, diamond (no cycle), and multiple cycles in same package
```

## AC-3: Algorithm specifies inputs, steps, complexity, and diagnostic output
```gherkin
Given the algorithm specification
When read by a compiler implementer
Then it contains: Inputs section, step-by-step algorithm (graph construction + cycle detection), complexity analysis, and diagnostic message format matching PGE-914
```

## AC-4: Cross-references to related rules
```gherkin
Given the algorithm specification
When checked for cross-references
Then it links to PGE-914, PGE-902, and PGE-414 with relative paths
```

</acceptance_criteria>

<tasks>

<task type="auto">
  <name>Task 1: Write cycle detection algorithm specification</name>
  <files>docs/technical/compile-rules/algorithms/cycle-detection.md</files>
  <action>
    Create the algorithm specification following the style of overlap-detection.md:

    **Frontmatter:**
    - name: Pipeline Call Cycle Detection
    - type: algorithm
    - consumes: PGE-914

    **Sections to include:**
    1. **Introduction** — What this algorithm does, why cycles are errors (no recursion in Polyglot)
    2. **Inputs** — Package's {=} pipeline definitions and their [r]/[p]/[b] call sites
    3. **Algorithm — Graph Construction** — Build directed adjacency list: nodes = {=} pipelines, edges = [r]/[p]/[b] references to same-package pipelines. Exclude cross-package calls (handled by PGE-902).
    4. **Algorithm — Cycle Detection (DFS three-color)** — White/gray/black marking. When gray node revisited = back edge = cycle. Recover cycle path from DFS stack.
    5. **Algorithm — Alternative (Kahn's topological sort)** — Brief description of Kahn's as alternative approach.
    6. **Edge Cases** — Table covering: self-call (single node self-edge), mutual recursion (2-node cycle), transitive cycle (3+ nodes), diamond DAG (valid, no cycle), multiple independent cycles (report all).
    7. **Complexity** — O(V + E) for DFS where V = pipeline count, E = call edge count.
    8. **Diagnostic Output** — Format: "Circular pipeline call detected: =A -> =B -> =C -> =A — Polyglot does not support recursion" (matching PGE-914 diagnostic)
    9. **See Also** — Links to PGE-914, PGE-902, PGE-414

    **Style rules:**
    - Follow docs/audit/ conventions (read audit README if uncertain)
    - Use pseudocode (not language-specific code) for algorithm steps
    - Include worked examples for self-call and transitive cycle
  </action>
  <verify>
    - File exists at docs/technical/compile-rules/algorithms/cycle-detection.md
    - Frontmatter has type: algorithm and consumes: PGE-914
    - Contains sections: Inputs, Algorithm, Edge Cases, Complexity, See Also
    - All 5 edge cases from issue #36 addressed
    - Cross-references use relative paths to PGE-914, PGE-902, PGE-414
  </verify>
  <done>AC-1, AC-2, AC-3, AC-4 satisfied</done>
</task>

</tasks>

<boundaries>

## DO NOT CHANGE
- docs/technical/compile-rules/PGE/PGE-914-circular-pipeline-call.md (rule doc is stable)
- docs/technical/compile-rules/PGE/PGE-902-circular-package-dependency.md
- docs/technical/compile-rules/PGE/PGE-414-recursive-data-definition.md
- docs/technical/compile-rules/algorithms/overlap-detection.md
- docs/technical/compile-rules/algorithms/compound-exhaustiveness.md

## SCOPE LIMITS
- Algorithm specification only — no compiler implementation code
- Intra-package cycles only — cross-package is PGE-902's domain
- Do not modify existing compile rule documents
- Do not add entries to COMPILE-RULES.md index (separate task if needed)

</boundaries>

<verification>
Before declaring plan complete:
- [ ] cycle-detection.md exists with valid YAML frontmatter
- [ ] Algorithm is complete: graph construction + DFS cycle detection
- [ ] All 5 edge cases from issue #36 are addressed
- [ ] Diagnostic output format matches PGE-914
- [ ] Cross-references to PGE-914, PGE-902, PGE-414 use correct relative paths
- [ ] Document follows existing algorithm doc style (overlap-detection.md)
- [ ] All acceptance criteria met
</verification>

<success_criteria>
- Algorithm specification document created
- All edge cases covered
- Cross-references correct
- Style consistent with existing algorithm docs
</success_criteria>

<output>
After completion, create `.paul/phases/issue-36-cycle-detection/issue-36-01-SUMMARY.md`
</output>
