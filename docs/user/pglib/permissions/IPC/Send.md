---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __IPC.Send

<!-- @c:permissions -->

Capability-level generic permission for sending messages. Sugar over [[INDEX|__IPC]] with `.Send` capability baked in.

## Definition

```polyglot
{_} __IPC.Send
   [#] <scope;string

   [.] .intent << #Grant
   [.] .IPC.Send "{$scope}"
```

## Usage

```polyglot
(-) __IPC.Send
   (_) <scope << "queue://jobs"
```

## Related

- [[INDEX|__IPC]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
