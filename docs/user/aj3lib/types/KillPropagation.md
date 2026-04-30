---
audience: automation-builder
type: specification
updated: 2026-04-09
status: complete
metadata_definition: "%definition.#:KillPropagation"
metadata_instance: "%#:KillPropagation:N"
---

# #KillPropagation Enum

<!-- @c:types -->

Runtime `##Enum` type available in every `.aj3` file. Uses `[#] ##Enum` (enum classification), `[#] ##Scalar` (depth 1), and `[#] ###ScalarEnum` (leaf content is variant selection).

---

## Definition

```aljam3
{#} #KillPropagation
   [%] .description << "How kill signals propagate from parent job to sub-jobs"
   [%] .version << "1.0.0"
   [#] ##Enum
   [#] ##Scalar
   [#] ###ScalarEnum
   [#] %##Alias << "killpropagation"
   [.] .Cascade
   [.] .Downgrade
```

| Variant | Behavior |
|---------|----------|
| `#Cascade` | Sub-jobs receive the same kill type as parent (hard->hard, graceful->graceful) |
| `#Downgrade` | Hard kill on parent -> graceful kill on sub-jobs (allows `[/]` cleanup) |

Default: `#Cascade`. Orphan jobs are never permitted — every sub-job must be terminated when its parent is killed.

---

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:KillPropagation` | Compile-time type template |
| Instance | `%#:KillPropagation:0` | Runtime instance (enum — one active field) |

---

## Related

- [[enums]] — other aj3lib enum types
- [[Queue]] — queue configuration
- [[Job]] — job lifecycle
- [[syntax/types/INDEX|types]] — full type system specification
