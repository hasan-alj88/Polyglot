---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __IPC.Send

<!-- @c:permissions -->

Capability-level generic permission for sending messages. Sugar over [[INDEX|__IPC]] with `.Send` capability baked in.

## Definition

```aljam3
{_} __IPC.Send
   [#] <scope;string

   [.] .intent << #Grant
   [.] .IPC.Send "{$scope}"
```

## Usage

```aljam3
(-) __IPC.Send
   (_) <scope << "queue://jobs"
```

## Related

- [[INDEX|__IPC]] -- category-level generic
- [[aj3lib/permissions/INDEX]] -- all aj3lib generic permissions
