---
issue: "023"
title: No compiler rules for [@] import validation
related: PGE-103 (Rule 1.3)
priority: enhancement
status: resolved
created: 2026-03-19
---

# 023 — No compiler rules for [@] import validation

## Problem

The `[@]` import system has no dedicated compiler rules. Currently:

- `[@] @alias =PackageRef` imports a package
- `@alias=PipelineName` calls a pipeline from that package
- `@alias#DataName` references a data type from that package

No rules exist for:

### 1. Undefined alias reference
Using `@alias=Pipeline` when `@alias` was never declared with `[@]`.

### 2. Circular package dependencies
Package A imports Package B which imports Package A. No detection rule exists.

### 3. Cross-package type visibility
Can a pipeline in Package A use `{#}` types defined in Package B without importing? Are all types public or is there encapsulation?

### 4. Import scope
Where can `[@]` appear? Is it file-scoped or pipeline-scoped? PGE-103 says one `{@}` per file, but doesn't address `[@]` import placement rules.

## Affected Rules

- No existing rules — new category needed (possibly PGE-9xx for imports)

## Proposed Resolution

Create a new rule category (9xx — Imports & Packages):

| Code | Rule | Name |
|------|------|------|
| PGE-901 | 9.1 | Undefined Import Alias |
| PGE-902 | 9.2 | Circular Package Dependency |

Defer visibility/encapsulation rules until the package system design is finalized. Start with the two clear compile-time errors.

## See also

- [PGE-103 — One Package Declaration Per File](../compile-rules/PGE/PGE-103-one-package-declaration-per-file.md)
