---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __IPC.Receive

<!-- @c:permissions -->

Capability-level generic permission for receiving messages. Sugar over [[INDEX|__IPC]] with `.Receive` capability baked in.

## Definition

```aljam3
{_} __IPC.Receive
   [#] <scope;string

   [.] .intent << #Grant
   [.] .IPC.Receive "{$scope}"
```

## Usage

```aljam3
(-) __IPC.Receive
   (_) <scope << "queue://results"
```

## Related

- [[INDEX|__IPC]] -- category-level generic
- [[aj3lib/permissions/INDEX]] -- all aj3lib generic permissions
