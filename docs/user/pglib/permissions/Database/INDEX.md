---
audience: pg-coder
type: specification
updated: 2026-04-12
status: complete
---

# __Database

<!-- @c:permissions -->

Category-level generic permission for database operations. Takes a capability and scope, yields a `_` permission object with `#Grant` intent.

## Definition

```polyglot
{_} __Database
   [#] <capability;DatabaseCapability
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Database.$capability "{$scope}"
```

## Usage

```polyglot
[_] __Database
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

- [[pglib/permissions/INDEX]] -- all pglib generic permissions
- [[concepts/permissions]] -- permission system overview
- [[pglib/types/PermissionCategory]] -- category enum
