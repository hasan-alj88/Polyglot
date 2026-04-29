---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Database.Connect

<!-- @c:permissions -->

Capability-level generic permission for connecting to databases. Sugar over [[INDEX|__Database]] with `.Connect` capability baked in.

## Definition

```aljam3
{_} __Database.Connect
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Database.Connect "{$scope}"
```

## Usage

```aljam3
(-) __Database.Connect
   (_) <scope << "postgres://db.internal/analytics"
```

## Related

- [[INDEX|__Database]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
