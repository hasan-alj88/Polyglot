---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __IPC

<!-- @c:permissions -->

Category-level generic permission for inter-process communication. Takes a capability and scope, yields a `_` permission object with `#Grant` intent.

## Definition

```aljam3
{_} __IPC
   [#] <capability;IPCCapability
   [#] <scope;string

   [.] .intent << #Grant
   [.] .IPC.$capability "{$scope}"
```

## Usage

```aljam3
(-) __IPC
   (_) <capability << .Send
   (_) <scope << "queue://jobs"
```

## Capability-Level Generics

| Generic | Input | Description |
|---------|-------|-------------|
| [[Send\|__IPC.Send]] | `<scope;string` | Send messages |
| [[Receive\|__IPC.Receive]] | `<scope;string` | Receive messages |
| [[Subscribe\|__IPC.Subscribe]] | `<scope;string` | Subscribe to channels |

## Related

- [[aj3lib/permissions/INDEX]] -- all aj3lib generic permissions
- [[concepts/permissions]] -- permission system overview
- [[aj3lib/types/PermissionCategory]] -- category enum
