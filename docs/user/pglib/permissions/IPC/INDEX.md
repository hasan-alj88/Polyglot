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

```polyglot
{_} __IPC
   [#] <capability;IPCCapability
   [#] <scope;string

   [.] .intent << #Grant
   [.] .IPC.$capability "{$scope}"
```

## Usage

```polyglot
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

- [[pglib/permissions/INDEX]] -- all pglib generic permissions
- [[concepts/permissions]] -- permission system overview
- [[pglib/types/PermissionCategory]] -- category enum
