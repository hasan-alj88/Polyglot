---
audience: automation-builder
type: specification
updated: 2026-04-16
status: complete
---

# __System.Shell

<!-- @c:permissions -->

Capability-level generic permission for shell command execution. Sugar over [[INDEX|__System]] with `.Shell` capability baked in. Shell execution is a higher privilege than `System.Process` — it permits arbitrary command interpretation through the system shell.

## Definition

```aljam3
{_} __System.Shell
   [#] <scope;string

   [.] .intent << #Grant
   [.] .System.Shell "{$scope}"
```

## Usage

```aljam3
(-) __System.Shell
   (_) <scope << "*"
```

## Related

- [[INDEX|__System]] -- category-level generic
- [[Process|__System.Process]] -- process spawning (lower privilege)
- [[jm3lib/permissions/INDEX]] -- all jm3lib generic permissions
- [[jm3lib/pipelines/Run/Shell|-Run.Shell]] -- pipeline requiring this capability
