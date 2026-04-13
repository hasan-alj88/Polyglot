---
audience: automation-builder
type: specification
updated: 2026-03-30
---

<!-- @concepts/pipelines/INDEX -->

## Execution

The execution section contains `[-]`, `[=]`, `[b]`, `[s]`, `[?]` lines — see [[blocks#Execution]]. For collection operations within execution, see [[concepts/collections/INDEX|collections]].

### Execution Rules

Every line in the execution body must begin with a block element marker — `[-]`, `[=]`, `[b]`, `[?]`, `[s]`, or an expand operator (PGE01016). Use `[-]` for process steps and assignment, not `(-)` — the `(-)` marker is reserved for IO declarations (PGE01017).

Each `[-]`, `[=]`, and `[b]` line in the execution body creates a **job** — a task queued for execution. Jobs have implicit triggers based on their position: sequential `[-]` jobs chain on predecessor completion, parallel `[=]` jobs fork from the previous sequential job. See [[concepts/pipelines/INDEX#Implicit Triggers in the Pipeline Body]] for the full trigger rules and collector requirements.

## See Also

- [[concepts/pipelines/INDEX#Pipeline, Instance, and Job]] — the execution hierarchy (pipeline → instance → job)
- [[concepts/pipelines/wrappers|Wrappers]] — setup/cleanup scope that wraps the execution body
- [[concepts/collections/INDEX|Collections]] — expand/collect operators used within execution
- [[concepts/pipelines/chains|Chain Execution]] — chaining multiple pipelines on a single `[-]` line
