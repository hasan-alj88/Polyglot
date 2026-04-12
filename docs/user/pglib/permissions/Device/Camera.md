---
audience: pg-coder
type: specification
updated: 2026-04-12
status: complete
---

# __Device.Camera

<!-- @c:permissions -->

Capability-level generic permission for camera access. Sugar over [[INDEX|__Device]] with `.Camera` capability baked in.

## Definition

```polyglot
{_} __Device.Camera
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Device.Camera "{$scope}"
```

## Usage

```polyglot
[_] __Device.Camera
   (_) <scope << "front"
```

## Related

- [[INDEX|__Device]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
