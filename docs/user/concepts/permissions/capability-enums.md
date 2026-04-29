---
audience: automation-builder
type: specification
updated: 2026-04-17
status: complete
---

# Per-Category Capability Enums

Permission categories use typed enums to prevent nonsensical combinations. Each category has its own capability enum:

| Category | Enum | Capabilities |
|----------|------|-------------|
| `File` | `#FileCapability` | `Read`, `Write`, `Execute`, `Delete`, `Create` |
| `Web` | `#WebCapability` | `Request`, `Socket`, `Listen` |
| `Database` | `#DatabaseCapability` | `Connect`, `Read`, `Write` |
| `System` | `#SystemCapability` | `Env`, `Process`, `Signal`, `Shell` |
| `Crypto` | `#CryptoCapability` | `Key`, `Sign`, `Encrypt` |
| `IPC` | `#IPCCapability` | `Send`, `Receive`, `Subscribe` |
| `Device` | `#DeviceCapability` | `Camera`, `Microphone`, `Location`, `Bluetooth` |
| `Memory` | `#MemoryCapability` | `Allocate`, `Shared` |
| `RAM` | `#RAMCapability` | `Limit` |
| `CPU` | `#CPUCapability` | `Limit`, `Weight` |
| `GPU` | `#GPUCapability` | `Limit`, `Device` |
| `IO` | `#IOCapability` | `Limit`, `Iops` |
| `Processes` | `#ProcessCapability` | `Limit` |
| `Duration` | `#DurationCapability` | `Limit` |

Fourteen predefined categories cover all IO capabilities. Categories and capabilities are Aljam3-defined — not user-extensible. The enum type prevents invalid combinations like `.Device.Read` (devices don't have a `Read` capability).

## Per-Category Resource Fields

Each category defines which resource locator fields are valid in `{_}` objects. The compiler validates that only category-relevant fields are present. See [[permission-schema#Resource Locator Fields]] for the full `__ResourceLocator` schema.

| Category | Required Fields | Optional Fields |
|----------|----------------|-----------------|
| `File` | `.path` | `.format` (#YAML, #JSON, #TOML) |
| `Web` | `.host` | `.port`, `.endpoint` |
| `Database` | `.host`, `.database` | `.port`, `.credentials`, `.table` |
| `System` | `.command` | `.args` |
| `Crypto` | — | — |
| `IPC` | — | — |
| `Device` | — | — |
| `Memory` | — | — |
| `RAM` | `.max` | — |
| `CPU` | `.max` | `.weight` |
| `GPU` | `.max` | `.device` |
| `IO` | — | `.maxBps`, `.maxIops` |
| `Processes` | `.max` | — |
| `Duration` | `.max` | — |

Resource fields carry the **locator** — the specific external resource the permission targets. Pipelines receive the whole `_` permission object and extract the fields they need: `-Yaml.LoadFile` reads `.path` and `.format`, `-DB.Query` reads `.host`, `.port`, `.database`, and `.credentials`.

For file-category permissions, the compiler reads the file at `.path` and content-hashes it at compile time. See [[enforcement#Compile-Time File Binding]].
