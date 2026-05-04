---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __File

<!-- @c:permissions -->

Category-level generic permission for file system operations. Takes a capability and scope, yields a `_` permission object with `#Grant` intent.

## Definition

```aljam3
{_} __File
   [#] <capability;FileCapability
   [#] <scope;path

   [.] .intent << #Grant
   [.] .File.$capability "{$scope}"
```

## Usage

```aljam3
(-) __File
   (_) <capability << .Read
   (_) <scope << "/var/log/*"
```

## Capability-Level Generics

| Generic | Input | Description |
|---------|-------|-------------|
| [[Read\|__File.Read]] | `<scope;path` | Read files |
| [[Write\|__File.Write]] | `<scope;path` | Write files |
| [[Execute\|__File.Execute]] | `<scope;path` | Execute files |
| [[Delete\|__File.Delete]] | `<scope;path` | Delete files |
| [[Create\|__File.Create]] | `<scope;path` | Create files |

## Related

- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
- [[concepts/permissions]] -- permission system overview
- [[jm3lib/types/PermissionCategory]] -- category enum
