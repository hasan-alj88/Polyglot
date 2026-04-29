---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __File.Create

<!-- @c:permissions -->

Capability-level generic permission for creating files. Sugar over [[INDEX|__File]] with `.Create` capability baked in.

## Definition

```aljam3
{_} __File.Create
   [#] <scope;path

   [.] .intent << #Grant
   [.] .File.Create "{$scope}"
```

## Usage

```aljam3
(-) __File.Create
   (_) <scope << "/tmp/output/*"
```

## Related

- [[INDEX|__File]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
