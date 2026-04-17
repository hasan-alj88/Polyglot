---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Crypto.Sign

<!-- @c:permissions -->

Capability-level generic permission for signing operations. Sugar over [[INDEX|__Crypto]] with `.Sign` capability baked in.

## Definition

```polyglot
{_} __Crypto.Sign
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Crypto.Sign "{$scope}"
```

## Usage

```polyglot
(-) __Crypto.Sign
   (_) <scope << "ed25519"
```

## Related

- [[INDEX|__Crypto]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
