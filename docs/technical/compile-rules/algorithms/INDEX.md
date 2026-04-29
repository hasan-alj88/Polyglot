---
audience: developer
type: spec-index
updated: 2026-04-23
status: stable
---

<!-- @compile-rules/algorithms/INDEX -->

# Compile-Rule Algorithms

Detection algorithms shared across compile rules. Each algorithm declares the rules it supports via its `consumes:` frontmatter; this index expands those lists into clickable cross-references so the dependency between an algorithm and the rules it enforces is visible from a single page.

## Contents

| Algorithm | Consumes (rules that rely on this algorithm) |
|-----------|----------------------------------------------|
| [[compound-exhaustiveness\|Compound Condition Partition Refinement]] | [[../PGE/PGE06005-compound-condition-overlap\|PGE06005]], [[../PGE/PGE06008-compound-condition-exhaustiveness\|PGE06008]] |
| [[cycle-detection\|Pipeline Call Cycle Detection]] | [[../PGE/PGE09013-circular-pipeline-call\|PGE09013]] |
| [[overlap-detection\|Conditional Overlap Detection (Unified Dispatch)]] | [[../PGE/PGE06004-numeric-range-overlap\|PGE06004]], [[../PGE/PGE06005-compound-condition-overlap\|PGE06005]] |
| [[queue-semantics-validation\|Queue Semantics Validation (Contextual Binding)]] | [[../PGE/PGE01064-queue-strategy-override\|PGE01064]], [[../PGE/PGE01065-queue-host-override\|PGE01065]], [[../PGE/PGE01066-queue-max-instances-exceeded\|PGE01066]], [[../PGE/PGE01067-queue-max-concurrent-exceeded\|PGE01067]], [[../PGE/PGE01068-queue-kill-propagation-conflict\|PGE01068]], [[../PGE/PGE01069-queue-wait-time-violation\|PGE01069]], [[../PGE/PGE01070-queue-resource-tag-mismatch\|PGE01070]] |

See [[../INDEX|Compile Rules Index]] for the full directory map, and [[../PGE/INDEX|PGE Index]] for severity and grouping of every rule code referenced above.
