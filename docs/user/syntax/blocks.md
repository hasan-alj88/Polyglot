---
audience: user
type: specification
updated: 2026-03-24
status: complete
---

# Block System

<!-- @line-structure -->
<!-- @identifiers -->
Two bracket types with distinct roles. Each line within a block follows [[line-structure]] rules. Expressions use [[identifiers]] with prefix sigils. Every `{X}` definition creates a branch on the `%` metadata tree — `{#}` at `%#`, `{=}` at `%=`, `{M}` at `%M`, `{Q}` at `%Q`, `{!}` at `%!` (see [[data-is-trees]]).

## `{X}` — Definition Elements

Define top-level structures. Open a scope that continues with indentation.

| Marker | Defines |
|--------|---------|
| `{@}` | Package declaration (mandatory, first in file). See [[packages]] |
| `{#}` | Struct definition. See [[types#Enum Fields vs Value Fields]] |
| `{=}` | Pipeline definition. See [[pipelines]] |
| `{M}` | Macro definition |
| `{Q}` | Queue definition. See [[pipelines#Queue]] |
| `{!}` | Error tree definition. See [[errors#Defining Custom Errors]] |
| `{Array}` | Array collection definition. See [[collections]] |
| `{ }` | Comment. See [[comments]] |

## `[X]` — Block Elements

Mark individual lines within blocks.

### Registry

| Marker | Meaning |
|--------|---------|
| `[@]` | Import/register package |

### Permissions

<!-- @permissions -->
See [[permissions]] for inline/IO forms, permission categories, and hierarchical scoping rules.

| Marker | Meaning |
|--------|---------|
| `[_]` | Permission declaration — declares IO capabilities (inline or IO form) |

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
| `[r]` | Run/execute in series; match header (with `>>` and `[?]` children). See [[conditionals#Match Syntax]] |
| `[p]` | Run/execute in parallel |
| `[b]` | Run/execute in background (fire and forget) |
| `[#]` | Load serialized data into typed structure |

### Control Flow

<!-- @pipelines -->
See [[pipelines]] for trigger/queue/wrapper structure and error scoping rules.

| Marker | Meaning |
|--------|---------|
| `[?]` | Conditional switch flow; match arm (under `[r]` `>>` match). See [[conditionals#Match Syntax]] |
| `[!]` | Error handling — scoped under `[r]` call. See [[pipelines#Error Handling]] |
| `[!] >>` | Error raise — raises a declared error. See [[errors#Raising Errors]] |
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
| `[\|]` | OR |
| `[-]` | NOT |
| `[^]` | XOR |

### Line Continuation

| Marker | Meaning |
|--------|---------|
| `[+]` | Line continuation — appends to preceding logical line |

The originating line keeps its normal block marker. Only continuation lines get `[+]`. The parser joins all `[+]` lines with the preceding logical line. Strings can span across `[+]` boundaries (multi-line string content preserved). `[+]` is only valid when the preceding expression is incomplete.

```polyglot
[r] .complex_result;string
[+] << "suffix
[+]  more"
```

### Foreign Code

| Marker | Meaning |
|--------|---------|
| `[c]` | Foreign code injection — embed another language via `#Code:<Language>:<Version>` |

The first `[c]` line declares the language. All body lines also get `[c]` prefix. Body content is raw text — not parsed as Polyglot. The block ends when a line without `[c]` appears.

```polyglot
[c] #Code:Python:3:14
[c] import pandas as pd
[c] df = pd.read_csv("data.csv")
[c] result = df.describe()
```

### Comments

See [[comments]] for full comment syntax.

| Marker | Meaning |
|--------|---------|
| `[ ]` | Comment. See [[comments]] |

## Closing: Indentation-Based

Blocks close by returning to the parent indentation level — no explicit closing markers needed for scope termination.
