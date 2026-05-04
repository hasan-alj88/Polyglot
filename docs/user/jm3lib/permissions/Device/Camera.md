---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Device.Camera

<!-- @c:permissions -->

Capability-level generic permission for camera access. Sugar over [[INDEX|__Device]] with `.Camera` capability baked in.

## Definition

```aljam3
{_} __Device.Camera
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Device.Camera "{$scope}"
```

## Usage

```aljam3
(-) __Device.Camera
   (_) <scope << "front"
```

## Related

- [[INDEX|__Device]] -- category-level generic
- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
