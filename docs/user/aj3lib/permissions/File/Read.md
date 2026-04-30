---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __File.Read

<!-- @c:permissions -->

Capability-level generic permission for reading files. Sugar over [[INDEX|__File]] with `.Read` capability baked in.

## Definition

```aljam3
{_} __File.Read
   [#] <scope;path

   [.] .intent << #Grant
   [.] .File.Read "{$scope}"
```

## Usage

```aljam3
(-) __File.Read
   (_) <scope << "/var/log/app/*.log"
```

## Related

- [[INDEX|__File]] -- category-level generic
- [[aj3lib/permissions/INDEX]] -- all aj3lib generic permissions
