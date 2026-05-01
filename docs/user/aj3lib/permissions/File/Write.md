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

```aljam3
{_} __File.Write
   [#] <scope;path

   [.] .intent << #Grant
   [.] .File.Write "{$scope}"
```

## Usage

```aljam3
(-) __File.Write
   (_) <scope << "/tmp/reports/*"
```

## Related

- [[INDEX|__File]] -- category-level generic
- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
