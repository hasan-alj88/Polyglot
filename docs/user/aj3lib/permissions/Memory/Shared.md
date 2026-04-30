---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Memory.Shared

<!-- @c:permissions -->

Capability-level generic permission for shared memory access. Sugar over [[INDEX|__Memory]] with `.Shared` capability baked in.

## Definition

```aljam3
{_} __Memory.Shared
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Memory.Shared "{$scope}"
```

## Usage

```aljam3
(-) __Memory.Shared
   (_) <scope << "shm://buffer"
```

## Related

- [[INDEX|__Memory]] -- category-level generic
- [[aj3lib/permissions/INDEX]] -- all aj3lib generic permissions
