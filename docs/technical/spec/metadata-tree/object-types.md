---
audience: [architect, designer]
type: spec
updated: 2026-04-05
---

# Object Type Branches

<!-- @source:metadata-tree/INDEX -->

The `%` root has fixed branches for each object type prefix:

| Branch | Objects | Name level | Description |
|--------|---------|------------|-------------|
| `%#` | Structs | Flexible (`:type`) | All `{#}` type definitions |
| `%=` | Pipelines | Flexible (`:name`) | All `{=}` pipeline definitions |
| `%T` | Triggers | Flexible (`:name`) | All `{T}` trigger pipeline definitions |
| `%~` | Expanders | Flexible (`:name`) | All `~ForEach.*` expand operators |
| `%*` | Collectors | Flexible (`:name`) | **Data:** `*Into.*`, `*Agg.*` · **Collect-all:** `*All` · **Race:** `*First`, `*Nth` |
| `%$` | Variables | Flexible (`:name`) | All `$`-prefixed variables |
| `%W` | Wrappers | Flexible (`:name`) | All `{W}` wrapper definitions |
| `%Q` | Queues | Flexible (`:name`) | All `{Q}` queue definitions |
| `%M` | Macros | Flexible (`:name`) | All `{M}` macro definitions |
| `%!` | Errors | Fixed (`.namespace`) | Polyglot-defined namespaces; `.Error` has flexible `:` children (uses `error_path`) |
| `%@` | Packages | Flexible (`:<registry>:<id>::<name>`) | All `@`-prefixed package addresses; `::` separates registry from name (uses `package_path`) |
| `%_` | Permissions | Flexible (`:name`), then fixed (`.`) | Named `{_}` permission objects (`_`/`__`/`___` tiers); no instances, object names via `:`, subfields via `.` (uses `permission_path`) |

Plus `%definition` (fixed) for compile-time schema templates — including `%definition.#:{TypeName}` for type definitions, `%definition.=:{PipelineName}` for pipeline definitions, `%definition.T:{TriggerName}` for trigger definitions, `%definition.W:{WrapperName}` for wrapper definitions, `%definition.Q:{QueueName}` for queue definitions, `%definition.##:{SchemaName}` for `##` schema definitions, and `%definition.###:{FieldTypeName}` for `###` field type definitions.

No `%Data` prefix exists — instance paths go directly to `%{type}:{ref}:{instance}.{fields}`.

See also: [[path-grammar|Path Grammar]], [[instance-lifecycle|Instance Lifecycle]], [[definition-templates|Definition Templates]]
