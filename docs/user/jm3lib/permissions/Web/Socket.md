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

```aljam3
{_} __Web.Socket
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Web.Socket "{$scope}"
```

## Usage

```aljam3
(-) __Web.Socket
   (_) <scope << "wss://stream.example.com/*"
```

## Related

- [[INDEX|__Web]] -- category-level generic
- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
