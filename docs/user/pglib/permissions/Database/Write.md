---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Database.Write

<!-- @c:permissions -->

Capability-level generic permission for writing to databases. Sugar over [[INDEX|__Database]] with `.Write` capability baked in.

## Definition

```polyglot
{_} __Database.Write
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Database.Write "{$scope}"
```

## Usage

```polyglot
[_] __Database.Write
   (_) <scope << "postgres://db.internal/analytics"
```

## Related

- [[INDEX|__Database]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
