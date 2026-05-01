---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Database.Read

<!-- @c:permissions -->

Capability-level generic permission for reading from databases. Sugar over [[INDEX|__Database]] with `.Read` capability baked in.

## Definition

```aljam3
{_} __Database.Read
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Database.Read "{$scope}"
```

## Usage

```aljam3
(-) __Database.Read
   (_) <scope << "postgres://db.internal/analytics"
```

## Related

- [[INDEX|__Database]] -- category-level generic
- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
