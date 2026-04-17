---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Device

<!-- @c:permissions -->

Category-level generic permission for device access operations. Takes a capability and scope, yields a `_` permission object with `#Grant` intent.

## Definition

```polyglot
{_} __Device
   [#] <capability;DeviceCapability
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Device.$capability "{$scope}"
```

## Usage

```polyglot
(-) __Device
   (_) <capability << .Camera
   (_) <scope << "front"
```

## Capability-Level Generics

| Generic | Input | Description |
|---------|-------|-------------|
| [[Camera\|__Device.Camera]] | `<scope;string` | Camera access |
| [[Microphone\|__Device.Microphone]] | `<scope;string` | Microphone access |
| [[Location\|__Device.Location]] | `<scope;string` | Location access |
| [[Bluetooth\|__Device.Bluetooth]] | `<scope;string` | Bluetooth access |

## Related

- [[pglib/permissions/INDEX]] -- all pglib generic permissions
- [[concepts/permissions]] -- permission system overview
- [[pglib/types/PermissionCategory]] -- category enum
