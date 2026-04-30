---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Memory

<!-- @c:permissions -->

Category-level generic permission for memory operations. Takes a capability and scope, yields a `_` permission object with `#Grant` intent.

## Definition

```aljam3
{_} __Memory
   [#] <capability;MemoryCapability
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Memory.$capability "{$scope}"
```

## Usage

```aljam3
(-) __Memory
   (_) <capability << .Allocate
   (_) <scope << "512MB"
```

## Capability-Level Generics

| Generic | Input | Description |
|---------|-------|-------------|
| [[Allocate\|__Memory.Allocate]] | `<scope;string` | Memory allocation |
| [[Shared\|__Memory.Shared]] | `<scope;string` | Shared memory access |

## Related

- [[aj3lib/permissions/INDEX]] -- all aj3lib generic permissions
- [[concepts/permissions]] -- permission system overview
- [[aj3lib/types/PermissionCategory]] -- category enum
