---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Device.Bluetooth

<!-- @c:permissions -->

Capability-level generic permission for Bluetooth access. Sugar over [[INDEX|__Device]] with `.Bluetooth` capability baked in.

## Definition

```aljam3
{_} __Device.Bluetooth
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Device.Bluetooth "{$scope}"
```

## Usage

```aljam3
(-) __Device.Bluetooth
   (_) <scope << "ble"
```

## Related

- [[INDEX|__Device]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
