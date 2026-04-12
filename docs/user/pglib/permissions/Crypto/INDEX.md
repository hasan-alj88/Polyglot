---
audience: pg-coder
type: specification
updated: 2026-04-12
status: complete
---

# __Crypto

<!-- @c:permissions -->

Category-level generic permission for cryptographic operations. Takes a capability and scope, yields a `_` permission object with `#Grant` intent.

## Definition

```polyglot
{_} __Crypto
   [#] <capability;CryptoCapability
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Crypto.$capability "{$scope}"
```

## Usage

```polyglot
[_] __Crypto
   (_) <capability << .Key
   (_) <scope << "aes-256-gcm"
```

## Capability-Level Generics

| Generic | Input | Description |
|---------|-------|-------------|
| [[Key\|__Crypto.Key]] | `<scope;string` | Key management |
| [[Sign\|__Crypto.Sign]] | `<scope;string` | Signing operations |
| [[Encrypt\|__Crypto.Encrypt]] | `<scope;string` | Encryption operations |

## Related

- [[pglib/permissions/INDEX]] -- all pglib generic permissions
- [[concepts/permissions]] -- permission system overview
- [[pglib/types/PermissionCategory]] -- category enum
