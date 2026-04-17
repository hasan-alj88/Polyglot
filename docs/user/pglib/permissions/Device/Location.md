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

```polyglot
{_} __Device.Location
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Device.Location "{$scope}"
```

## Usage

```polyglot
(-) __Device.Location
   (_) <scope << "coarse"
```

## Related

- [[INDEX|__Device]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
