---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __System.Signal

<!-- @c:permissions -->

Capability-level generic permission for sending and receiving signals. Sugar over [[INDEX|__System]] with `.Signal` capability baked in.

## Definition

```aljam3
{_} __System.Signal
   [#] <scope;string

   [.] .intent << #Grant
   [.] .System.Signal "{$scope}"
```

## Usage

```aljam3
(-) __System.Signal
   (_) <scope << "SIGTERM"
```

## Related

- [[INDEX|__System]] -- category-level generic
- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
