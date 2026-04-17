---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Crypto.Encrypt

<!-- @c:permissions -->

Capability-level generic permission for encryption operations. Sugar over [[INDEX|__Crypto]] with `.Encrypt` capability baked in.

## Definition

```polyglot
{_} __Crypto.Encrypt
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Crypto.Encrypt "{$scope}"
```

## Usage

```polyglot
(-) __Crypto.Encrypt
   (_) <scope << "aes-256-gcm"
```

## Related

- [[INDEX|__Crypto]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
