# PGE06015: Boolean Exhaustiveness

code: PGE06015
description: Branching on a boolean must explicitly cover both true and false branches, or include a catch-all.
category: Syntax and Structure

## Description

**Statement:** Branching on a `#bool` must explicitly cover both `true` and `false` branches, or include a `*?` catch-all. If a conditional `[?]` evaluates a boolean subject and fails to provide branches for both states without a wildcard fallback, `PGE06015` is raised.

## Rationale
Ensures that all boolean paths are handled explicitly, preventing unhandled values at runtime.

## Examples
```polyglot
[?] $isValid =? true >> $status << "Valid"
[ ] ✗ PGE06015 — missing false branch or *? catch-all
```
