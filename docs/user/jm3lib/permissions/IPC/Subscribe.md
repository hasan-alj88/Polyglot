---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __IPC.Subscribe

<!-- @c:permissions -->

Capability-level generic permission for subscribing to channels. Sugar over [[INDEX|__IPC]] with `.Subscribe` capability baked in.

## Definition

```aljam3
{_} __IPC.Subscribe
   [#] <scope;string

   [.] .intent << #Grant
   [.] .IPC.Subscribe "{$scope}"
```

## Usage

```aljam3
(-) __IPC.Subscribe
   (_) <scope << "topic://events.*"
```

## Related

- [[INDEX|__IPC]] -- category-level generic
- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
