---
audience: pg-coder
type: specification
updated: 2026-04-12
status: complete
---

# __Memory.Shared

<!-- @c:permissions -->

Capability-level generic permission for shared memory access. Sugar over [[INDEX|__Memory]] with `.Shared` capability baked in.

## Definition

```polyglot
{_} __Memory.Shared
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Memory.Shared "{$scope}"
```

## Usage

```polyglot
[_] __Memory.Shared
   (_) <scope << "shm://buffer"
```

## Related

- [[INDEX|__Memory]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
