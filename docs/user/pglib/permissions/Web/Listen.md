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

```polyglot
{_} __Web.Listen
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Web.Listen "{$scope}"
```

## Usage

```polyglot
(-) __Web.Listen
   (_) <scope << ":8080"
```

## Related

- [[INDEX|__Web]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
