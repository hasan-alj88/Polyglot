---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Device.Location

<!-- @c:permissions -->

Capability-level generic permission for location access. Sugar over [[INDEX|__Device]] with `.Location` capability baked in.

## Definition

```aljam3
{_} __Device.Location
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Device.Location "{$scope}"
```

## Usage

```aljam3
(-) __Device.Location
   (_) <scope << "coarse"
```

## Related

- [[INDEX|__Device]] -- category-level generic
- [[aj3lib/permissions/INDEX]] -- all aj3lib generic permissions
