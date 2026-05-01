---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __Web.Request

<!-- @c:permissions -->

Capability-level generic permission for HTTP requests. Sugar over [[INDEX|__Web]] with `.Request` capability baked in.

## Definition

```aljam3
{_} __Web.Request
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Web.Request "{$scope}"
```

## Usage

```aljam3
(-) __Web.Request
   (_) <scope << "https://api.example.com/*"
```

## Related

- [[INDEX|__Web]] -- category-level generic
- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
