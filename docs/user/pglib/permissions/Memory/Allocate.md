---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Memory.Allocate

<!-- @c:permissions -->

Capability-level generic permission for memory allocation. Sugar over [[INDEX|__Memory]] with `.Allocate` capability baked in.

## Definition

```polyglot
{_} __Memory.Allocate
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Memory.Allocate "{$scope}"
```

## Usage

```polyglot
[_] __Memory.Allocate
   (_) <scope << "512MB"
```

## Related

- [[INDEX|__Memory]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
