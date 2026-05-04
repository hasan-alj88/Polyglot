---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Device.Microphone

<!-- @c:permissions -->

Capability-level generic permission for microphone access. Sugar over [[INDEX|__Device]] with `.Microphone` capability baked in.

## Definition

```aljam3
{_} __Device.Microphone
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Device.Microphone "{$scope}"
```

## Usage

```aljam3
(-) __Device.Microphone
   (_) <scope << "default"
```

## Related

- [[INDEX|__Device]] -- category-level generic
- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
