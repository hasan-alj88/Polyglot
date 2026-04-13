---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Crypto.Key

<!-- @c:permissions -->

Capability-level generic permission for key management. Sugar over [[INDEX|__Crypto]] with `.Key` capability baked in.

## Definition

```polyglot
{_} __Crypto.Key
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Crypto.Key "{$scope}"
```

## Usage

```polyglot
[_] __Crypto.Key
   (_) <scope << "aes-256-gcm"
```

## Related

- [[INDEX|__Crypto]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
