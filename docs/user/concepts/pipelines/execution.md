---
audience: user
type: specification
updated: 2026-03-30
---

<!-- @concepts/pipelines/INDEX -->

## Execution

The execution section contains `[r]`, `[p]`, `[b]`, `[s]`, `[?]` lines — see [[blocks#Execution]]. For collection operations within execution, see [[concepts/collections/INDEX|collections]].

### Execution Rules

Every line in the execution body must begin with a block element marker — `[r]`, `[p]`, `[b]`, `[?]`, `[s]`, or an expand operator (PGE-116). Use `[r]` for process steps and assignment, not `[=]` — the `[=]` marker is reserved for IO declarations (PGE-117).

## See Also

- [[concepts/pipelines/wrappers|Wrappers]] — setup/cleanup scope that wraps the execution body
- [[concepts/collections/INDEX|Collections]] — expand/collect operators used within execution
- [[concepts/pipelines/chains|Chain Execution]] — chaining multiple pipelines on a single `[r]` line
