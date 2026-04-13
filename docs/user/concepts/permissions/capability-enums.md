---
audience: automation-builder
type: specification
updated: 2026-04-05
status: complete
---

# Per-Category Capability Enums

Permission categories use typed enums to prevent nonsensical combinations. Each category has its own capability enum:

| Category | Enum | Capabilities |
|----------|------|-------------|
| `File` | `#FileCapability` | `Read`, `Write`, `Execute`, `Delete`, `Create` |
| `Web` | `#WebCapability` | `Request`, `Socket`, `Listen` |
| `Database` | `#DatabaseCapability` | `Connect`, `Read`, `Write` |
| `System` | `#SystemCapability` | `Env`, `Process`, `Signal` |
| `Crypto` | `#CryptoCapability` | `Key`, `Sign`, `Encrypt` |
| `IPC` | `#IPCCapability` | `Send`, `Receive`, `Subscribe` |
| `Device` | `#DeviceCapability` | `Camera`, `Microphone`, `Location`, `Bluetooth` |
| `Memory` | `#MemoryCapability` | `Allocate`, `Shared` |

Eight predefined categories cover all IO capabilities. Categories and capabilities are Polyglot-defined — not user-extensible. The enum type prevents invalid combinations like `.Device.Read` (devices don't have a `Read` capability).
