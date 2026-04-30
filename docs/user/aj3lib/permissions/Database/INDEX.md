---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Database

<!-- @c:permissions -->

Category-level generic permission for database operations. Takes a capability and scope, yields a `_` permission object with `#Grant` intent.

## Definition

```aljam3
{_} __Database
   [#] <capability;DatabaseCapability
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Database.$capability "{$scope}"
```

## Usage

```aljam3
(-) __Database
   (_) <capability << .Connect
   (_) <scope << "postgres://db.internal/analytics"
```

## Capability-Level Generics

| Generic | Input | Description |
|---------|-------|-------------|
| [[Connect\|__Database.Connect]] | `<scope;string` | Connect to databases |
| [[Read\|__Database.Read]] | `<scope;string` | Read from databases |
| [[Write\|__Database.Write]] | `<scope;string` | Write to databases |

## Related

- [[aj3lib/permissions/INDEX]] -- all aj3lib generic permissions
- [[concepts/permissions]] -- permission system overview
- [[aj3lib/types/PermissionCategory]] -- category enum
