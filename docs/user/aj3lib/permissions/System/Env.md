---
audience: automation-builder
type: specification
updated: 2026-04-12
status: complete
---

# __System.Env

<!-- @c:permissions -->

Capability-level generic permission for accessing environment variables. Sugar over [[INDEX|__System]] with `.Env` capability baked in.

## Definition

```aljam3
{_} __System.Env
   [#] <scope;string

   [.] .intent << #Grant
   [.] .System.Env "{$scope}"
```

## Usage

```aljam3
(-) __System.Env
   (_) <scope << "DATABASE_URL"
```

## Related

- [[INDEX|__System]] -- category-level generic
- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
