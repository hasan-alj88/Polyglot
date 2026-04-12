---
audience: pg-coder
type: specification
updated: 2026-04-12
status: complete
---

# __File.Read

<!-- @c:permissions -->

Capability-level generic permission for reading files. Sugar over [[INDEX|__File]] with `.Read` capability baked in.

## Definition

```polyglot
{_} __File.Read
   [#] <scope;path

   [.] .intent << #Grant
   [.] .File.Read "{$scope}"
```

## Usage

```polyglot
[_] __File.Read
   (_) <scope << "/var/log/app/*.log"
```

## Related

- [[INDEX|__File]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
