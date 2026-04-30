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

```aljam3
{_} __File.Delete
   [#] <scope;path

   [.] .intent << #Grant
   [.] .File.Delete "{$scope}"
```

## Usage

```aljam3
(-) __File.Delete
   (_) <scope << "/tmp/cache/*"
```

## Related

- [[INDEX|__File]] -- category-level generic
- [[aj3lib/permissions/INDEX]] -- all aj3lib generic permissions
