---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __File.Delete

<!-- @c:permissions -->

Capability-level generic permission for deleting files. Sugar over [[INDEX|__File]] with `.Delete` capability baked in.

## Definition

```polyglot
{_} __File.Delete
   [#] <scope;path

   [.] .intent << #Grant
   [.] .File.Delete "{$scope}"
```

## Usage

```polyglot
[_] __File.Delete
   (_) <scope << "/tmp/cache/*"
```

## Related

- [[INDEX|__File]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
