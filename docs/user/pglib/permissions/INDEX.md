---
audience: pg-coder
type: specification
updated: 2026-04-12
status: complete
---

# pglib Generic Permissions

<!-- @c:permissions -->

pglib provides `__` generic permission templates — syntax sugar that produces concrete `_` permission objects at compile time. Two levels:

- **Category-level** (`__File`, `__Web`, ...) — takes capability + scope as inputs
- **Capability-level** (`__File.Read`, `__Web.Request`, ...) — takes only scope, sugar over category-level

All generics produce `_` objects with `#Grant` intent. For `#Ceiling` objects, write a full `{_}` block instead.

## Category-Level Generics

| Generic | Inputs | Category |
|---------|--------|----------|
| [[File/INDEX\|__File]] | `<capability;FileCapability`, `<scope;path` | File system operations |
| [[Web/INDEX\|__Web]] | `<capability;WebCapability`, `<scope;string` | Network/web operations |
| [[Database/INDEX\|__Database]] | `<capability;DatabaseCapability`, `<scope;string` | Database operations |
| [[System/INDEX\|__System]] | `<capability;SystemCapability`, `<scope;string` | System-level operations |
| [[Crypto/INDEX\|__Crypto]] | `<capability;CryptoCapability`, `<scope;string` | Cryptographic operations |
| [[IPC/INDEX\|__IPC]] | `<capability;IPCCapability`, `<scope;string` | Inter-process communication |
| [[Device/INDEX\|__Device]] | `<capability;DeviceCapability`, `<scope;string` | Device access operations |
| [[Memory/INDEX\|__Memory]] | `<capability;MemoryCapability`, `<scope;string` | Memory operations |

## Capability-Level Generics

| Generic | Input | Description |
|---------|-------|-------------|
| [[File/Read\|__File.Read]] | `<scope;path` | Read files |
| [[File/Write\|__File.Write]] | `<scope;path` | Write files |
| [[File/Execute\|__File.Execute]] | `<scope;path` | Execute files |
| [[File/Delete\|__File.Delete]] | `<scope;path` | Delete files |
| [[File/Create\|__File.Create]] | `<scope;path` | Create files |
| [[Web/Request\|__Web.Request]] | `<scope;string` | HTTP requests |
| [[Web/Socket\|__Web.Socket]] | `<scope;string` | WebSocket connections |
| [[Web/Listen\|__Web.Listen]] | `<scope;string` | Listen on ports |
| [[Database/Connect\|__Database.Connect]] | `<scope;string` | Connect to databases |
| [[Database/Read\|__Database.Read]] | `<scope;string` | Read from databases |
| [[Database/Write\|__Database.Write]] | `<scope;string` | Write to databases |
| [[System/Env\|__System.Env]] | `<scope;string` | Access environment variables |
| [[System/Process\|__System.Process]] | `<scope;string` | Spawn/manage processes |
| [[System/Signal\|__System.Signal]] | `<scope;string` | Send/receive signals |
| [[Crypto/Key\|__Crypto.Key]] | `<scope;string` | Key management |
| [[Crypto/Sign\|__Crypto.Sign]] | `<scope;string` | Signing operations |
| [[Crypto/Encrypt\|__Crypto.Encrypt]] | `<scope;string` | Encryption operations |
| [[IPC/Send\|__IPC.Send]] | `<scope;string` | Send messages |
| [[IPC/Receive\|__IPC.Receive]] | `<scope;string` | Receive messages |
| [[IPC/Subscribe\|__IPC.Subscribe]] | `<scope;string` | Subscribe to channels |
| [[Device/Camera\|__Device.Camera]] | `<scope;string` | Camera access |
| [[Device/Microphone\|__Device.Microphone]] | `<scope;string` | Microphone access |
| [[Device/Location\|__Device.Location]] | `<scope;string` | Location access |
| [[Device/Bluetooth\|__Device.Bluetooth]] | `<scope;string` | Bluetooth access |
| [[Memory/Allocate\|__Memory.Allocate]] | `<scope;string` | Memory allocation |
| [[Memory/Shared\|__Memory.Shared]] | `<scope;string` | Shared memory access |

## Related

- [[concepts/permissions]] -- permission system overview
- [[pglib/types/PermissionCategory]] -- 8-category enum
- [[pglib/types/PermissionIntent]] -- Ceiling vs Grant intent
