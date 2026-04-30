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

```aljam3
{_} __Device
   [#] <capability;DeviceCapability
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Device.$capability "{$scope}"
```

## Usage

```aljam3
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

- [[aj3lib/permissions/INDEX]] -- all aj3lib generic permissions
- [[concepts/permissions]] -- permission system overview
- [[aj3lib/types/PermissionCategory]] -- category enum
