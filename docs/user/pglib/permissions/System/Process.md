---
audience: pg-coder
type: specification
updated: 2026-04-12
status: complete
---

# __System.Process

<!-- @c:permissions -->

Capability-level generic permission for spawning and managing processes. Sugar over [[INDEX|__System]] with `.Process` capability baked in.

## Definition

```polyglot
{_} __System.Process
   [#] <scope;string

   [.] .intent << #Grant
   [.] .System.Process "{$scope}"
```

## Usage

```polyglot
[_] __System.Process
   (_) <scope << "ffmpeg"
```

## Related

- [[INDEX|__System]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
