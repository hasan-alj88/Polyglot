---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __System

<!-- @c:permissions -->

Category-level generic permission for system-level operations. Takes a capability and scope, yields a `_` permission object with `#Grant` intent.

## Definition

```aljam3
{_} __System
   [#] <capability;SystemCapability
   [#] <scope;string

   [.] .intent << #Grant
   [.] .System.$capability "{$scope}"
```

## Usage

```aljam3
(-) __System
   (_) <capability << .Env
   (_) <scope << "DATABASE_URL"
```

## Capability-Level Generics

| Generic | Input | Description |
|---------|-------|-------------|
| [[Env\|__System.Env]] | `<scope;string` | Access environment variables |
| [[Process\|__System.Process]] | `<scope;string` | Spawn/manage processes |
| [[Signal\|__System.Signal]] | `<scope;string` | Send/receive signals |
| [[Shell\|__System.Shell]] | `<scope;string` | Execute shell commands |

## Related

- [[aj3lib/permissions/INDEX]] -- all aj3lib generic permissions
- [[concepts/permissions]] -- permission system overview
- [[aj3lib/types/PermissionCategory]] -- category enum
