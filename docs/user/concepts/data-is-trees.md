---
audience: automation-builder
type: spec
updated: 2026-04-15
---

# Everything is a Tree

<!-- @c:types:RawString -->
<!-- @u:identifiers:Serialized Identifiers -->
<!-- @u:technical/spec/metadata-tree/INDEX -->
<!-- @u:metadata -->
<!-- @u:philosophy/data-trees -->

All Polyglot data is serialized strings. Every object — structs, pipelines, variables, collections, errors, packages — is a branch on one unified tree rooted at `%`. Understanding this tree is the key to understanding how every concept in Polyglot Code connects.

**"Data tree" and "serialized data" are the same thing.** A data tree is the structure; serialized data is the format. In Polyglot, these are inseparable — when code is parsed, every object becomes a tree of `RawString` leaves serialized as JSON. The compiler and runtime operate on this serialized tree directly. The type `#serial` is the untyped form: a data tree with no schema constraints, accepting any topology.

## All Polyglot Objects Are Data Trees

Every `{X}` definition block in Polyglot Code is fundamentally a `{#}` struct — a description of **data tree topology**. The different block types (`{-}`, `{T}`, `{W}`, `{Q}`, `{!}`, `{_}`, `{N}`) are `{#}` with specific schemas designed for designated purposes:

- `{-}` is a struct with IO ports, trigger, queue, wrapper, and execution body fields
- `{T}` is a struct with trigger-specific schema (subtype of `{-}`)
- `{W}` is a struct with setup/cleanup scope schema (subtype of `{-}`)
- `{Q}` is a struct with queue dispatch schema
- `{!}` is a struct with error namespace schema (subtype of `{#}`). Terminals carry `##Error` schema — if a `{!}` definition's terminals do not have `##Error`, it is a compile error
- `{_}` is a struct with permission policy schema. A `_Permission` object is a variable `$` with `##Permission` schema

A **datatype** is a description of data tree topology — the shape, depth, field names, field types, and constraints that define how a tree is structured. When Polyglot Code is parsed, it is serialized as data (JSON) and sent to the Polyglot service. The compiler and runtime operate entirely on these serialized data trees.

### Type Identity and Compatibility

Data types are defined by the topology of the data tree together with the schema metadata (`%##`) and leaf metadata (`%###`) attached to its nodes.

**Same type:** Two data trees are the **same datatype** if they have the same topology AND the same `%##` schema metadata AND the same `%###` leaf metadata. Assignment between variables requires same type — anything less is a compile error. There is no implicit coercion.

**Compatible type:** Two data trees are **compatible** if they have the same topology only — meaning there is a correct mapping between the nodes of the two trees, but their `%##` or `%###` metadata may differ. Compatible-but-not-same types cannot be assigned to each other — the compiler raises an error because the metadata mismatch means the data carries different semantic constraints even if the shape is identical.

### Parameterization vs Instantiation

These are independent operations:

- **Parameterization** — `{#}` definitions with `(#)` inputs define a type template. The inputs (`(#) <InputParameter`, `(#) <#InputDatatype`) resolve the template into a concrete type definition. Parameterization does not produce a variable — it produces a type.
- **Instantiation** — when a `{#}` definition has all its terminal values in **Final** state (see [[variable-lifecycle#Final]]), it becomes a concrete variable `$`. The definition describes the tree shape; the variable is a tree instance with all terminals populated.

## All Data is Serialized Strings

Polyglot has one true primitive: `RawString` — a sequence of literal raw characters (see [[syntax/types/basic-types#RawString — The True Primitive]]). Everything else — `#String`, `int`, `float`, `#Boolean`, arrays, serials, user structs — is built on top of `RawString` through schemas that constrain how the string is interpreted.

This means every Polyglot object is ultimately a tree of strings with typed structure layered on top.

## Leaf-Only Values

> **Clarification:** "Leaf-only" does not contradict "everything is a tree." A single leaf is a valid tree (depth 1). The invariant below is about **node roles** within a tree — each node is either a branch (structure) or a leaf (data), never both. Even a standalone `RawString` value is a tree with one leaf node.

A universal invariant governs every tree in Polyglot: a node is either a **branch** or a **leaf**, never both.

- **Branch nodes** have children but no value — they exist purely for structure and navigation (namespace or enum grouping)
- **Leaf nodes** hold a `RawString` value but have no children — they are the terminal data
- A node CANNOT have both a value and children

This is not a per-type property — it is a universal invariant that applies to every data tree. No `%` metadata flag controls it; the compiler enforces it unconditionally.

## Tree Shape and Leaf Content

Types describe their tree structure through two additional prefix tiers beyond `#`:

- `##` **schema bundles** describe tree shape — depth, key types, ordering, uniformity (e.g., `##Scalar`, `##Flat`, `##Contiguous`). A `##` is syntactic sugar for a reusable group of `%##` metadata assignments
- `###` **leaf bundles** describe leaf content nature — `###Value` for typed data leaves, `###Enum` for variant selector leaves. A `###` is syntactic sugar for a reusable group of `%###` metadata assignments

Child nodes in a tree are accessed with the `<` operator: `$myMap<name`, `$matrix<0<1`. Fixed fields use `.` as before.

See [[syntax/types/prefix-system#Three-Tier Prefix System]] for the full prefix table, [[syntax/types/schema-properties#Approved ## Schema Types]] for all schema definitions, and [[syntax/types/prefix-system#The < Operator]] for accessor details.

## The Structured Tree

The `%` root has fixed branches for every object type in Polyglot:

```polyglot
%
├── #   Structs          — type definitions ({#} blocks)
├── -   Pipelines        — async workflows ({-} blocks)
├── =   Expanders         — expand operators (=ForEach.*)
├── *   Collectors        — collect operators (*Into.*, *Agg.*, *All, *First)
├── $   Variables         — runtime data ($name, $result)
├── !   Errors            — error trees ({!} blocks, pglib !File.*, !No.*, etc.)
├── @   Packages          — package addresses (@Local:999::MyPkg)
├── _   Permissions       — IO capability declarations (permission IO markers)
└── definition            — compile-time schema templates
```

Most branches use flexible (`:`) fields for their instances — `%#:Boolean`, `%-:MyPipeline`, `%$:myVar`. Exceptions: `%_` uses only `.` fixed fields (Polyglot-defined permissions), `%!` uses `.` for Polyglot-defined namespaces (with `:` under `.Error` for user extensions), and `%definition` stores compile-time structural templates.

## How Concepts Connect

Each concept you have learned maps to a branch in the tree:

| You learned | In | Tree branch | Instance example |
|-------------|----|-------------|------------------|
| Struct types | [[syntax/types/structs#Struct Types]] | `%#` | `%#:UserRecord:0` |
| Pipelines | [[concepts/pipelines/INDEX|pipelines]] | `%-` | `%-:ProcessData:0` |
| Variables | [[variable-lifecycle]] | `%$` | `%$:myVar:0` |
| Expand operators | [[concepts/collections/expand#Expand Operators]] | `%=` | `%=:ForEach.Array:0` |
| Collect operators | [[concepts/collections/collect#Collect Operators]] | `%*` | `%*:Into.Array:0` |
| Error trees | [[errors]], `{!}` blocks | `%!` | `%!.File.NotFound` |
| Packages | [[packages]] | `%@` | `%@:Local:999::MyPkg` |
| Permissions | [[permissions]] | `%_` | `%_.File.read` |

## Schema vs Instance

The tree has two layers:

- **Schema** (compile-time) — `%definition.{type}:{ref}` defines the structural template
- **Instance** (runtime) — `%{type}:{ref}:{instance}.{fields}` holds actual values

```polyglot
%definition.#:Boolean       ← schema: lists .True and .False as valid fields
%#:Boolean:0                ← instance 0: has ONE active field (.True or .False)
%#:Boolean:1                ← instance 1: independent, its own active field
```

One definition can have many instances. A pipeline that runs three times concurrently has instances `:0`, `:1`, `:2` — each with its own metadata values.

Each instance runs independently and contains its own **jobs** — the units of work created at IO boundaries within the pipeline. Jobs are identified by UID (not sequential numbers) and tracked at `%-:Pipeline:N.jobs:UID`. See [[glossary]] for the formal distinction between Instance and Job.

The following diagram shows how schema definitions produce runtime instances across three key branches:

```mermaid
graph TD
    root["%"]

    subgraph schema ["Schema Layer (compile-time)"]
        def["%definition"]
        defBool[".#:Boolean"]
        defPipe[".-:ProcessData"]
        defVar[".$:myVar"]
    end

    subgraph instances ["Instance Layer (runtime)"]
        bool0["%#:Boolean:0"]
        bool1["%#:Boolean:1"]
        pipe0["%-:ProcessData:0"]
        var0["%$:myVar:0"]
    end

    root --> def
    def --> defBool
    def --> defPipe
    def --> defVar

    defBool -- ":0" --> bool0
    defBool -- ":1" --> bool1
    defPipe -- ":0" --> pipe0
    defVar -- ":0" --> var0
```

### Worked Examples

| Path | Reads |
|------|-------|
| `%definition.#:UserRecord` | The schema for `#UserRecord` — field names, types, structure |
| `%-:ProcessData:0.status` | Instance 0 of `-ProcessData` — its current `live` status |
| `%$:myVar:0.state` | Instance 0 of `$myVar` — its lifecycle state (Declared, Default, Final, Failed, Released) |

## Key Tree Rules

### Enum Instances — Active-Field-Only

An enum instance collapses to ONE active field. The definition lists all valid branches, but a specific instance has only the active one:

```polyglot
%definition.#:Boolean       ← schema: .True, .False (both listed)
%#:Boolean:0.True           ← instance 0: .True is active, .False does NOT exist
```

Push atomically clears the previous field and sets the new one. Reading a non-active field returns no path.

### String Subtypes — Nested Under `:String`

`int` lives at `%#:String:int` — nested under `:String` at a flexible level. The alias `#int` in user code resolves to `#String.int`. Each subtype uses the `#String` schema with `.regex` pre-filled:

```polyglot
%#:String:int               ← .string#RawString + .regex#RawString (regex = "^-?[0-9]+$")
%#:String:float             ← .string#RawString + .regex#RawString (regex = "^-?[0-9]+\.[0-9]+$")
%#:String:email             ← pglib: .regex = "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$"
```

See [[syntax/types/basic-types#Numeric Types — #String Subtypes]] for details.

### IO Ports — Nested Typed Sections

Pipeline instances carry their IO as nested fixed sections:

```polyglot
%-:ProcessData:0
├── .<                      ← input ports (fixed typed section)
│   └── .filepath#path
└── .>                      ← output ports (fixed typed section)
    └── .content#string
```

## Reading the Tree

The general path notation is:

```text
%{type}:{ref}:{instance}.{fields}
```

| Segment | Meaning |
|---------|---------|
| `%` | Tree root — the metadata accessor |
| `{type}` | Object type prefix (`#`, `=`, `$`, `*`, `!`, `@`, `_`) |
| `:{ref}` | Object name (flexible field) |
| `:{instance}` | Instance number (flexible field) |
| `.{fields}` | Fixed field path within the instance |

**Shorthand in code:** `-MyPipeline%status` reads `%-:MyPipeline:<current>.status` — the current instance is implicit.

For the full field listings (which metadata each branch carries, `live` vs user-declared), see [[metadata]]. For the formal path grammar and instance rules, see [[metadata-tree/INDEX|technical/spec/metadata-tree]].
