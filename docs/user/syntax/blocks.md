---
audience: user
type: specification
updated: 2026-03-22
status: draft
---

# Block System

<!-- @line-structure -->
<!-- @identifiers -->
Two bracket types with distinct roles. Each line within a block follows [[line-structure]] rules. Expressions use [[identifiers]] with prefix sigils. Every `{X}` definition creates a branch on the `%` metadata tree — `{#}` at `%#`, `{=}` at `%=`, `{M}` at `%M`, `{Q}` at `%Q` (see [[data-is-trees]]).

## `{X}` — Definition Elements

Define top-level structures. Open a scope that continues with indentation.

| Marker | Defines |
|--------|---------|
| `{@}` | Package declaration (mandatory, first in file). See [[packages]] |
| `{#}` | Struct definition. See [[types#Enum Fields vs Value Fields]] |
| `{=}` | Pipeline definition. See [[pipelines]] |
| `{M}` | Macro definition |
| `{Q}` | Queue definition. See [[pipelines#Queue]] |
| `{Array}` | Array collection definition. See [[collections]] |
| `{ }` | Comment. See [[comments]] |

## `[X]` — Block Elements

Mark individual lines within blocks.

### Registry

| Marker | Meaning |
|--------|---------|
| `[@]` | Import/register package |

### Data Flow

<!-- @io -->
See [[io]] for IO parameter patterns and [[collections]] for expand/collect semantics.

| Marker | Meaning |
|--------|---------|
| `[=]` | Pipeline IO line. See [[io#IO Line Pattern]] |
| `[~]` | Collection-expand IO line. See [[collections#Expand Operators]] |
| `[*]` | Collection-collect IO line. See [[collections#Collect Operators]] |
| `[*] <<` | Wait input — wait for variable to be Final (used inside `[*]` blocks). See [[collections#Sync & Race Collectors]] |
| `[*] >>` | Collect output — in race blocks, losing inputs cancelled, output receives winner. See [[collections#Sync & Race Collectors]] |
| `[>]` | Output fallback — scoped under `[=]` output line. See [[errors#Error Fallback Operators]] |
| `[<]` | Input fallback — scoped under `[=]` input line. See [[errors#Error Fallback Operators]] |

### Execution

| Marker | Meaning |
|--------|---------|
| `[r]` | Run/execute in series |
| `[p]` | Run/execute in parallel |
| `[b]` | Run/execute in background (fire and forget) |
| `[#]` | Load serialized data into typed structure |

### Control Flow

<!-- @pipelines -->
See [[pipelines]] for trigger/queue/wrapper structure and error scoping rules.

| Marker | Meaning |
|--------|---------|
| `[?]` | Conditional switch flow |
| `[!]` | Error handling — scoped under `[r]` call. See [[pipelines#Error Handling]] |
| `[t]` | Trigger. See [[pipelines#Triggers]] |
| `[Q]` | Queue. See [[pipelines#Queue]] |
| `[W]` | Wrapper. See [[pipelines#Wrappers]] |

### Scope

| Marker | Meaning |
|--------|---------|
| `[\]` | Setup |
| `[/]` | Cleanup |
| `[{]` | From outer scope (in Macros) |
| `[}]` | To outer scope (in Macros) |

### Data Access

| Marker | Meaning |
|--------|---------|
| `[.]` | Fixed subfield access |
| `[:]` | Flexible subfield access |

### Metadata

| Marker | Meaning |
|--------|---------|
| `[%]` | Definition metadata and aliases |

`[%]` lives inside any `{x}` definition (`{#}`, `{=}`, `{M}`, `{Q}`). One definition = one metadata set (class-level). Two kinds of fields: user-declared (via `<<` assignment) and Polyglot-managed (`live`, read-only). Alias under a `[.]` field: `[%] .alias << #AliasName` resolves to the fully qualified path. Aliases preserve type prefix (`#` for data, `=` for pipelines).

See [[metadata]] for the full metadata tree, field listings, `live` semantics, and access patterns.

### Logical

| Marker | Meaning |
|--------|---------|
| `[&]` | AND |
| `[+]` | OR |
| `[-]` | NOT |
| `[^]` | XOR |

### Comments

See [[comments]] for full comment syntax.

| Marker | Meaning |
|--------|---------|
| `[ ]` | Comment. See [[comments]] |

## Closing: Indentation-Based

Blocks close by returning to the parent indentation level — no explicit closing markers needed for scope termination.
