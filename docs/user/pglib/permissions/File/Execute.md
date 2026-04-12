---
audience: pg-coder
type: specification
updated: 2026-04-12
status: complete
---

# __File.Execute

<!-- @c:permissions -->

Capability-level generic permission for executing files. Sugar over [[INDEX|__File]] with `.Execute` capability baked in.

## Definition

```polyglot
{_} __File.Execute
   [#] <scope;path

   [.] .intent << #Grant
   [.] .File.Execute "{$scope}"
```

## Usage

```polyglot
[_] __File.Execute
   (_) <scope << "/usr/local/bin/ffmpeg"
```

## Related

- [[INDEX|__File]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
