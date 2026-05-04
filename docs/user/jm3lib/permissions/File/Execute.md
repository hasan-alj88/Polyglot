---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __File.Execute

<!-- @c:permissions -->

Capability-level generic permission for executing files. Sugar over [[INDEX|__File]] with `.Execute` capability baked in.

## Definition

```aljam3
{_} __File.Execute
   [#] <scope;path

   [.] .intent << #Grant
   [.] .File.Execute "{$scope}"
```

## Usage

```aljam3
(-) __File.Execute
   (_) <scope << "/usr/local/bin/ffmpeg"
```

## Related

- [[INDEX|__File]] -- category-level generic
- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
