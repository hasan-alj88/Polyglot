---
audience: pg-coder
type: specification
updated: 2026-04-12
status: complete
---

# __Web.Request

<!-- @c:permissions -->

Capability-level generic permission for HTTP requests. Sugar over [[INDEX|__Web]] with `.Request` capability baked in.

## Definition

```polyglot
{_} __Web.Request
   [#] <scope;string

   [.] .intent << #Grant
   [.] .Web.Request "{$scope}"
```

## Usage

```polyglot
[_] __Web.Request
   (_) <scope << "https://api.example.com/*"
```

## Related

- [[INDEX|__Web]] -- category-level generic
- [[pglib/permissions/INDEX]] -- all pglib generic permissions
