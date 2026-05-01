---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Web

<!-- @c:permissions -->

Category-level generic permission for network/web operations. Takes a capability and scope, yields a `_` permission object with `#Grant` intent.

## Definition

```aljam3
{_} __Web
   [#] <capability;WebCapability
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Web.$capability "{$scope}"
```

## Usage

```aljam3
(-) __Web
   (_) <capability << .Request
   (_) <scope << "https://api.example.com/*"
```

## Capability-Level Generics

| Generic | Input | Description |
|---------|-------|-------------|
| [[Request\|__Web.Request]] | `<scope;string` | HTTP requests |
| [[Socket\|__Web.Socket]] | `<scope;string` | WebSocket connections |
| [[Listen\|__Web.Listen]] | `<scope;string` | Listen on ports |

## Related

- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
- [[concepts/permissions]] -- permission system overview
- [[jm3lib/types/PermissionCategory]] -- category enum
