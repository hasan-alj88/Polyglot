---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Web.Socket

<!-- @c:permissions -->

Capability-level generic permission for WebSocket connections. Sugar over [[INDEX|__Web]] with `.Socket` capability baked in.

## Definition

```polyglot
{_} __Web.Socket
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Web.Socket "{$scope}"
```

## Usage

```polyglot
(-) __Web.Socket
   (_) <scope << "wss://stream.example.com/*"
```

## Related

- [[INDEX|__Web]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
