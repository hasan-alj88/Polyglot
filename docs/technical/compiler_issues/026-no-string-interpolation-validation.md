---
issue: "026"
title: No validation for string interpolation variables
related: PGE-401 (Rule 4.1)
priority: enhancement
status: resolved
created: 2026-03-19
---

# 026 — No validation for string interpolation variables

## Problem

Polyglot supports string interpolation with `{$var}` syntax inside string literals. No compiler rule validates:

### 1. Undefined variable references
```polyglot
[r] >output << "Hello, {$userName}!"      [ ] ✗ $userName may not be declared
```

### 2. Type compatibility
Should non-string types be interpolated? If `$count` is an integer, is `"Total: {$count}"` valid? All Polyglot data is serialized strings, so this may be inherently valid — but the rule should state this explicitly.

### 3. Nested interpolation
```polyglot
[r] >output << "Value: {$items{$index}}"   [ ] ✗ nested interpolation — valid?
```

## Affected Rules

- No existing rules — potentially PGE-4xx category (type system)

## Proposed Resolution

Create `PGE-405 — Undefined Interpolation Variable`:
- Statement: every `{$var}` in a string literal must reference a variable in scope
- Since all Polyglot data is serialized strings, type compatibility is not an issue — any variable can be interpolated
- Nested interpolation should be explicitly disallowed or defined

## See also

- [PGE-401 — Type Mismatch](../compile-rules/PGE/PGE-401-type-mismatch.md)
