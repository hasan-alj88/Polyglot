---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Web.Listen

<!-- @c:permissions -->

Capability-level generic permission for listening on ports. Sugar over [[INDEX|__Web]] with `.Listen` capability baked in.

## Definition

```aljam3
{_} __Web.Listen
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Web.Listen "{$scope}"
```

## Usage

```aljam3
(-) __Web.Listen
   (_) <scope << ":8080"
```

## Related

- [[INDEX|__Web]] -- category-level generic
- [[aj3lib/permissions/INDEX]] -- all aj3lib generic permissions
