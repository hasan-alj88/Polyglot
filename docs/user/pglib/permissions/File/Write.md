---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __File.Write

<!-- @c:permissions -->

Capability-level generic permission for writing files. Sugar over [[INDEX|__File]] with `.Write` capability baked in.

## Definition

```polyglot
{_} __File.Write
   [#] <scope;path

   [.] .intent << #Grant
   [.] .File.Write "{$scope}"
```

## Usage

```polyglot
(-) __File.Write
   (_) <scope << "/tmp/reports/*"
```

## Related

- [[INDEX|__File]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
