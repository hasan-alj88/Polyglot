---
audience: developer
type: spec
updated: 2026-03-25
status: complete
---

# Metadata Tree Specification

<!-- @user/concepts/data-is-trees -->
<!-- @user/concepts/metadata -->
<!-- @EBNF -->

This document formally specifies the `%` metadata tree — the unified structure that organizes all Polyglot objects. For the user-friendly introduction, see [[data-is-trees|user/concepts/data-is-trees]]. For field listings and access patterns, see [[metadata|user/concepts/metadata]].

## Path Grammar

The general path patterns:

```ebnf
schema_path     ::= "%" "definition" "." type_prefix ":" ref
instance_path   ::= "%" type_prefix ":" ref ":" instance { "." field }
permission_path ::= "%" "_" { "." field }
error_path      ::= "%" "!" "." namespace { "." leaf }
                   | "%" "!" ".Error" { ":" user_path }
package_path    ::= "%" "@" ":" registry { ":" id_part } "::" name { ":" segment }
```

| Element | Rule |
|---------|------|
| `type_prefix` | One of: `#`, `=`, `~`, `*`, `$`, `M`, `!`, `@`, `_` |
| `ref` | Object name — flexible field (`:`) |
| `instance` | Instance number — flexible field (`:`) |
| `field` | Fixed field path (`.`) within the instance |

**Branch-specific rules:**
- `%_` — all `.` fixed fields, no `:` anywhere. No ref or instance levels.
- `%!` — `.` for Polyglot-defined namespaces and leaves. `.Error` children use `:` (user-extensible).
- `%@` — `::` separates registry+ID from package name. `::` is treated as `:` in the tree.

### Shorthand in User Code

User code uses shorthand accessors that resolve to full instance paths:

| Shorthand | Resolves to |
|-----------|-------------|
| `=MyPipeline%status` | `%=:MyPipeline:<current>.status` |
| `$myVar%state` | `%$:myVar:<current>.state` |
| `#Record%lastModified` | `%#:Record:<current>.lastModified` |

The `:<current>` segment is implicit — the runtime resolves it to the calling context's instance.

## Object Type Branches

The `%` root has fixed branches for each object type prefix:

| Branch | Objects | Name level | Description |
|--------|---------|------------|-------------|
| `%#` | Structs | Flexible (`:type`) | All `{#}` type definitions |
| `%=` | Pipelines | Flexible (`:name`) | All `{=}` pipeline definitions |
| `%~` | Expanders | Flexible (`:name`) | All `~ForEach.*` expand operators |
| `%*` | Collectors | Flexible (`:name`) | All `*Into.*`, `*Agg.*`, `*All`, `*First`, `*Nth` |
| `%$` | Variables | Flexible (`:name`) | All `$`-prefixed variables |
| `%M` | Macros | Flexible (`:name`) | All `{M}` macro definitions |
| `%!` | Errors | Fixed (`.namespace`) | Polyglot-defined namespaces; `.Error` has flexible `:` children |
| `%@` | Packages | Flexible (`:<registry>:<id>::<name>`) | All `@`-prefixed package addresses; `::` separates registry from name |
| `%_` | Permissions | All fixed (`.`) | All `_`-prefixed permission declarations; no instances, no `:` levels |

Plus `%definition` (fixed) for compile-time schema templates.

No `%Data` prefix exists — instance paths go directly to `%{type}:{ref}:{instance}.{fields}`.

## Instance Lifecycle

### Creation

An instance is created when:
- A pipeline is triggered (`%=:Name:N` where N is the next sequential number)
- A variable is declared (`%$:name:N`)
- An expand/collect operator begins execution (`%~:Name:N`, `%*:Name:N`)

### Numbering

Instances use sequential zero-based numbering: `:0`, `:1`, `:2`, etc. Numbers are scoped to the definition — each pipeline, variable, or operator tracks its own counter.

### Release

Instances are released when:
- A pipeline completes or fails (all stages resolved)
- A variable leaves scope ([[variable-lifecycle#Released]])
- An operator finishes collection

Released instances are no longer addressable. Their `live` metadata is discarded.

## String Subtype Nesting

String subtypes live under `%#:String:*` at a flexible level:

```
%#:String
├── :int          ← .string;RawString + .re = "^-?[0-9]+$"
├── :float        ← .string;RawString + .re = "^-?[0-9]+\.[0-9]+$"
├── :emailAddress ← user-defined: .re = custom pattern
└── :(any)        ← extensible — users define new subtypes
```

### Alias Resolution

User code `;int` is an alias for `;String.int`. The compiler resolves:

| User writes | Compiler resolves to | Tree path |
|-------------|---------------------|-----------|
| `;int` | `;String.int` | `%#:String:int` |
| `;float` | `;String.float` | `%#:String:float` |
| `;string` | `;String` | `%#:String` |
| `;emailAddress` | `;String.emailAddress` | `%#:String:emailAddress` |

All subtypes share the `#String` schema (`.string;RawString` + `.re;RawString`) with `.re` pre-filled per subtype.

## Enum Instance Rules

### Active-Field-Only Invariant

An enum instance collapses to exactly ONE active field. The definition (`%definition.#:{EnumName}`) lists all valid branches, but a specific instance has only the active one:

```
%definition.#:Boolean       ← schema: .True, .False (both listed)
%#:Boolean:0.True           ← instance 0: .True is active
                              .False does NOT exist for this instance
```

### Architecture Safeguards

The runtime enforces exactly one active enum field per instance:

| Operation | Behavior |
|-----------|----------|
| Push new field | Atomically clears previous field, sets new one |
| Read active field | Returns the field's value/path |
| Read non-active field | Returns no path (field does not exist on this instance) |
| Multi-set same instance in same scope | Compiler rejects (PGE prevents) |

## IO Port Nesting

`.<` (inputs) and `.>` (outputs) are fixed typed data sections within each pipeline, expander, and collector instance:

```
%=:ProcessData:0
├── .<                      ← input ports
│   ├── .filepath;path
│   └── .options;serial
└── .>                      ← output ports
    └── .content;string
```

Parameter names within `.<` and `.>` are flexible — they follow the pipeline's `[=]` IO declarations.

## Permission Branch

`%_` stores permission declarations. Unlike other branches, `%_` has **no `:{instance}` level** and **no `:` flexible fields** — permissions are compile-time declarations with an entirely fixed schema. All categories and capabilities are Polyglot-defined, not user-extensible. See [[permissions]] for the full permission system.

### Structure

```
%_
├── .File
│   ├── .read               ;string  (glob pattern)
│   ├── .write              ;string
│   ├── .execute            ;string
│   └── .delete             ;string
├── .Web
│   ├── .request
│   │   └── .<              (IO inputs)
│   └── .socket
│       └── .<
├── .Database
│   ├── .connect
│   │   └── .<
│   ├── .read               ;string
│   └── .write              ;string
├── .System
│   ├── .env                ;string
│   ├── .process
│   │   └── .<
│   └── .signal             ;string
├── .Crypto
│   ├── .key, .sign, .encrypt   ;string
├── .IPC
│   ├── .send, .receive
│   │   └── .<
│   └── .subscribe          ;string
├── .Device
│   ├── .camera, .microphone, .location, .bluetooth   ;bool
└── .Memory
    ├── .allocate, .shared   ;string
```

### Key Properties

- **All fixed (`.`)** — every level uses `.` fixed-field navigation. No `:` flexible fields anywhere in `%_`. Permission categories and capabilities are predefined by Polyglot.
- **No instances** — permissions are per-definition, resolved at compile time. No runtime metadata exists.
- **No `live` fields** — all permission data is static. The compiler resolves permissions entirely during compilation.
- **Nested under `%@` and `%=`** — permissions also appear as `._` subsections under package (`%@:<address>._`) and pipeline (`%=:<name>:<instance>._`) branches, representing the package ceiling and pipeline-level declarations respectively.
- **IO-form capabilities** — capabilities like `.request`, `.connect`, `.send` use `.<` for their IO input parameters, mirroring the IO form syntax in `[_]` declarations.

## Definition Templates

`%definition.{type}:{ref}` stores the compile-time structural template for each object. All instances `%{type}:{ref}:{n}` must conform to their definition's structure.

| Definition path | Ensures |
|-----------------|---------|
| `%definition.#:UserRecord` | All `%#:UserRecord:N` instances have `.name;string`, `.age;int` |
| `%definition.=:ProcessData` | All `%=:ProcessData:N` instances have the same IO ports and `live` fields |

Definitions are immutable at runtime — they are resolved entirely at compile time.

## Field Expansion

Any field typed `;string` expands to the full `#String` struct in the tree:

```
.description;string
  → .description
      .string;RawString     ← the raw value
      .re;RawString          ← the regex constraint (default: "" = accept any)
```

This expansion applies recursively — `;array.string` expands each element's `.string` and `.re` subfields.

## Related

- [[EBNF]] — formal grammar productions for `[%]` blocks
- [[COMPILE-RULES]] — error/warning code reference
- [[type-identity|spec/type-identity]] — structural type matching rules
- [[data-is-trees|user/concepts/data-is-trees]] — user-facing tree overview
- [[metadata|user/concepts/metadata]] — field listings and access patterns
- [Metadata Data Tree Decision](../plan/decisions/metadata-data-tree.md) — original design decision (2026-03-21)
