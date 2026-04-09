---
audience: pg-coder
type: specification
updated: 2026-04-05
status: complete
---

# Block System

<!-- @line-structure -->
<!-- @identifiers -->
Two bracket types with distinct roles. Each line within a block follows [[line-structure]] rules. Expressions use [[identifiers]] with prefix sigils. Every `{X}` definition creates a branch on the `%` metadata tree — `{#}` at `%#`, `{-}` at `%-`, `{T}` at `%T`, `{W}` at `%W`, `{Q}` at `%Q`, `{!}` at `%!`, `{_}` at `%_`, `{N}` at `%Native` (see [[data-is-trees]]).

## `{X}` — Definition Elements

Define top-level structures. Open a scope that continues with indentation.

| Marker | Defines |
|--------|---------|
| `{@}` | Package declaration (mandatory, first in file). See [[packages]] |
| `{#}` | Struct definition. See [[syntax/types/structs#Enum Fields vs Value Fields]] |
| `{-}` | Pipeline definition. Supports marker declarations (`{-}[exe]`, subsets). See [[concepts/pipelines/INDEX\|pipelines]] |
| `{T}` | Trigger pipeline definition (subtype of `{-}`). See [[concepts/pipelines/io-triggers#Trigger Definitions]] |
| `{W}` | Wrapper definition (subtype of `{-}`). See [[wrappers]] |
| `{Q}` | Queue — dual-purpose block. `{Q} #Queue:Name` defines a queue instance (subtype of `{#}`, data definition). `{Q} -Q.*` defines a queue pipeline operation (subtype of `{-}`, equivalent to `{-}[Q]`). The identifier prefix (`#` vs `-`) disambiguates. See [[concepts/pipelines/queue#Queue]] |
| `{!}` | Error tree definition (subtype of `{#}`). See [[errors#Defining Custom Errors]] |
| `{_}` | Permission object — named, reusable permission policy with `#Ceiling` or `#Grant` intent. Uses `_`/`__`/`___` tier system mirroring `#`/`##`/`###`. See [[permissions]] |
| `{N}` | Native definition — compiler primitive with no Polyglot body. `[%]` metadata implicitly scopes to `%Native.*`. Non-user-extendable. See [[concepts/pipelines/INDEX#Native vs Derived\|Native vs Derived]] |
| `{Array}` | Array collection definition. See [[concepts/collections/INDEX|collections]] |
| `{ }` | Comment. See [[comments]] |

**Marker declarations on `{-}`:** The `[exe]` marker declares the pipeline as an execution pipeline, invocable via `[-]`, `[=]`, or `[b]`. `{-}` without a marker defaults to `{-}[exe]` — no warning. Subsets like `{-}[b]` (background-only) or `{-}[-=]` (sequential/parallel only) restrict how the pipeline can be invoked. Subtypes (`{T}`, `{W}`, `{Q}`) have fixed implicit markers and cannot take `marker_decl`. See [[concepts/pipelines/INDEX#Marker Declarations|Marker Declarations]] for full details.

**Native definitions `{N}`:** Compiler primitives implemented in the host language (e.g., Rust). `{N}` blocks contain only `[%]` metadata and `(-)` IO declarations — no execution body, no `[T]`, `[Q]`, or `[W]`. The `[%]` metadata under `{N}` implicitly scopes to `%Native.*` with fixed fields: `.Kind` (`#NativeKind`), and per-language bindings (`.Rust`, `.Cpp`, etc.). See [[concepts/pipelines/INDEX#Native vs Derived|Native vs Derived]].

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
| `[_]` | Permission reference — references a named `{_}` permission object by name (e.g., `[_] _MyCeiling`) |

### Data Flow

<!-- @io -->
See [[io]] for IO parameter patterns and [[concepts/collections/INDEX|collections]] for expand/collect semantics.

| Marker | Meaning |
|--------|---------|
| `(-)` | Pipeline IO line — scopes to parent operator via indentation (top-level IO, `[Q]`, `[W]`, `[-]`/`[=]`/`[b]`). See [[io#IO Line Pattern]] |
| `(=)` | Collection-expand IO line. See [[concepts/collections/expand#Expand Operators]] |
| `(*)` | Collection-collect IO line. See [[concepts/collections/collect#Collect-All & Race Collectors]] |
| `(*) <<` | Wait input — wait for variable to be Final (used inside `(*)` blocks). See [[concepts/collections/collect#Collect-All & Race Collectors]] |
| `(*) >>` | Collect output — in race blocks, losing inputs cancelled, output receives winner. See [[concepts/collections/collect#Collect-All & Race Collectors]] |
| `(>)` | Output parameter handling — scoped under `(-)` output line. See [[io#IO Parameter Handling]] |
| `(<)` | Input parameter handling — scoped under `(-)` input line. See [[io#IO Parameter Handling]] |

### Execution

| Marker | Meaning |
|--------|---------|
| `[-]` | Run/execute in series; match header (with `>>` and `[?]` children). Without `[?]` children, `[-] $x >> $y` is a plain assignment. See [[conditionals#Match Syntax]] |
| `[=]` | Run/execute in parallel |
| `[b]` | Run/execute in background (fire and forget) |
| `[#]` | Load serialized data into typed structure |

### Control Flow

<!-- @pipelines -->
See [[concepts/pipelines/INDEX|pipelines]] for trigger/queue/wrapper structure and error scoping rules.

| Marker | Meaning |
|--------|---------|
| `[?]` | Conditional switch flow; match arm (under `[-]` `>>` match). See [[conditionals#Match Syntax]] |
| `[!]` | Error handling — scoped under `[-]` call. See [[concepts/pipelines/error-handling#Error Handling]] |
| `[!] >>` | Error raise — raises a declared error. See [[errors#Raising Errors]] |
| `[T]` | Trigger. See [[concepts/pipelines/io-triggers#Triggers]] |
| `[Q]` | Queue. See [[concepts/pipelines/queue#Queue]] |
| `[W]` | Wrapper. See [[concepts/pipelines/wrappers#Wrappers]] |

### Scope

| Marker | Meaning |
|--------|---------|
| `[\]` | Setup |
| `[/]` | Cleanup |
| `[{]` | From outer scope (in Wrappers) |
| `[}]` | To outer scope (in Wrappers) |

### Data Access

| Marker | Meaning |
|--------|---------|
| `[.]` | Fixed subfield access |
| `[:]` | Flexible subfield access |

### Metadata

| Marker | Meaning |
|--------|---------|
| `[%]` | Definition metadata and aliases |

`[%]` lives inside any `{x}` definition (`{#}`, `{-}`, `{W}`, `{Q}`). One definition = one metadata set (class-level). Two kinds of fields: user-declared (via `<<` assignment) and Polyglot-managed (`live`, read-only). Aliases use `[%] %alias` with `[:]` children — each child is a `#NestedKeyString` alias name. Multiple aliases per definition are allowed; all must be globally unique (PGE12002).

See [[metadata]] for the full metadata tree, field listings, `live` semantics, and access patterns.

### Metadata Accessors

<!-- @metadata -->

`%This`, `%Parent`, and `%name` provide scoped access to definition metadata from within `{x}` blocks.

| Accessor | Returns |
|----------|---------|
| `%This` | Innermost enclosing `{x}` definition block |
| `%Parent` | One level up from `%This` |
| `%name` | Definition name string from the `{x}` block header |
| `%name.Last` | Splits `%name` by `.` and returns the last segment |

`%name` examples:

| Context | `%name` returns |
|---------|----------------|
| `{#} #ThisName` | `"ThisName"` |
| `{-} -Pipeline.Name` | `"Pipeline.Name"` |
| `{W} -W.Polyglot` | `"W.Polyglot"` |

`%name.Last` splits by `.` and returns the final segment — `{-} -Pipeline.Name` yields `%name.Last` = `"Name"`.

`%This` scoping:

| Context | `%This` refers to |
|---------|-------------------|
| Inside `{#} #MyType` | The type definition |
| Inside `{-} -MyPipeline` | The pipeline definition |
| Outside any `{x}` block | Compile error |

`%Parent` refers to one level up from `%This` — useful inside nested definition contexts.

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
| `[~]` | Line continuation — appends to preceding logical line |

The originating line keeps its normal block marker. Only continuation lines get `[~]`. The parser joins all `[~]` lines with the preceding logical line. Strings can span across `[~]` boundaries (multi-line string content preserved). `[~]` is only valid when the preceding expression is incomplete.

```polyglot
[-] .complex_result#string
[~] << "suffix
[~]  more"
```

### Foreign Code

| Marker | Meaning |
|--------|---------|
| `[C]` | Inline foreign code — embed another language's code lines within an `-RT.*` pipeline call |

<!-- @concepts/pipelines/INDEX -->
`[C]` is a block element (not a block type) for embedding foreign code lines passed to `-RT.*` runtime pipelines. Each `[C]` line is one line of foreign code — raw text, not parsed as Polyglot. The language is determined by which `-RT.*` pipeline is called (e.g., `-RT.Python.Script.Inline`, `-RT.JS.Script.Inline`). The block ends when a line without `[C]` appears.

`[C]` lines are passed as the `<code` input to the `-RT.*` pipeline call:

```polyglot
[-] -RT.Python.Script.Inline
   (-) <env << $env
   (-) >output#Code:Python.Output >> $output
   (-) <code <<
      [C] import pandas as pd
      [C] df = pd.read_csv("data.csv")
      [C] result = df.describe()
```

### Comments

See [[comments]] for full comment syntax.

| Marker | Meaning |
|--------|---------|
| `[ ]` | Comment. See [[comments]] |

## Closing: Indentation-Based

Blocks close by returning to the parent indentation level — no explicit closing markers needed for scope termination.
