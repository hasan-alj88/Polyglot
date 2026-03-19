---
audience: user
type: specification
updated: 2026-03-15
status: draft
---

# Type System

<!-- @identifiers -->
<!-- @variable-lifecycle -->

## Type Annotation

Types are annotated on variables using the `;` separator. The `;` is part of the [[identifiers]] system — see [[variable-lifecycle]] for how typed variables move through lifecycle stages:

```polyglot
[r] $IntValue;int <~ 42
[r] $StringValue;string <~ "Hello World"
[r] $ArrayValue;array <~ {1, 2, 3, 4, 5}
```

The `;` is purely structural — it separates the variable name from its type. Each special character has one job:

| Character | Role |
|-----------|------|
| `@#=$!` | Identity (what it is) |
| `.` `:` | Navigation (fixed/flexible fields) |
| `;` | Annotation (what type it holds) |

## Basic Types

Bare names for common types — no namespace prefix required:

- `string`, `int`, `float`, `bool`
- `array` — ordered collection, enumerated flat keys starting at 0. See [[collections]]
- `serial` — dynamic data structure with flexible fields, added on the fly. Unlike `{#}` data definitions which have a predefined schema, `serial` allows arbitrary fields. Easily converts to/from JSON-like formats. See [[collections]]

## Element-Typed Arrays

Arrays can specify their element type using `.` (fixed field) notation:

```polyglot
[r] $files;array.path <~ {}
[r] $names;array.string <~ {}
[r] $scores;array.int <~ {}
```

This constrains the array to hold only elements of the specified type.

## User-Defined Types

<!-- @blocks -->
Reference data schemas declared with `{#}` (see [[blocks]]). In type annotations (after `;`), the `#` prefix is always dropped — the `;` already signals a type context:

```polyglot
[r] $user;UserRecord <~ ...
[r] $users;array.UserRecord <~ {}
[=] <incoming;Alert
```

The `#` prefix is only used when **referencing** a data definition outside of type annotations (e.g., `#Boolean.True`, `@alias#DataName.EnumField`).

## Inline Data Shorthand

Curly braces with comma-separated values create inline data:

```polyglot
[r] $values;array <~ {1, 2, 3, 4, 5}
```

This is shorthand for explicit field assignment: `#data.1 << 1`, `#data.2 << 2`, etc.

## Enum Fields vs Value Fields

In `{#}` data definitions, fields are either **enum fields** or **value fields**:

| Field Type | Has `;type`? | Has assignment? | Example |
|------------|-------------|-----------------|---------|
| Enum | No | No | `[.] .Critical` |
| Value | Yes | Optional | `[.] .code;int <~ 500` |

**Rules:**
- No type annotation (`;type`) implies an **enum field**
- Enum fields always use `[.]` fixed fields
- All siblings at the same level must be the same kind (all enum or all value)
- Enum fields can nest value sub-fields

```polyglot
{#} #Severity
   [.] .Critical
   [.] .Error
   [.] .Info
   [.] .Other
      [ ] Enum field with nested value sub-fields
      [.] .code;int <~ 500
```

Reference enum values cross-package: `@alias#DataName.EnumField` (e.g., `@Alert#Severity.Critical`). See [[packages#Usage]] for import syntax and [[identifiers#Serialized Identifiers]] for `.` fixed-field navigation.

## String Interpolation

Strings support variable interpolation using `{$variable}` inside string literals. Any `$`-prefixed identifier inside `{...}` within a double-quoted string is expanded to its value:

```polyglot
[r] $greeting;string << "Hello {$name}, you are {$age} years old"
[r] $path;string << "/users/{$userId}/profile"
```

Interpolation works with any `$`-prefixed variable, including flexible-field paths:

```polyglot
[r] $msg;string << "User {$user:name} logged in from {$user:location}"
```

For literal curly braces inside strings, use `{{` and `}}`.

## Live Type Modifier

`live` is a type modifier reserved for `[%]` metadata fields managed by the Polyglot runtime. Users can read `live` fields via the `%` accessor but never assign to them. The type uses dot notation: `;live.#PipelineStatus`, `;live.int`, `;live.array.error`.

`live` fields are **implicit** on every `{=}` pipeline, `$` variable, and `{#}` data definition. They do not need to be declared — the runtime populates them automatically and updates them in real-time.

See [[metadata]] for the full metadata tree, all `live` field listings, and access patterns.

## Path Type

`path` is a structured type with OS-specific subfields. Its schema is the stdlib `#path` data definition:

```polyglot
{#} #path
   [.] .Unix;string
   [.] .Windows;string
```

Assign both subfields so code works cross-platform:

```polyglot
[r] $AppDir;path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"
```

At runtime, the Polyglot runtime resolves `$AppDir` to the correct subfield based on the current OS (see `=Sys.OS` in [[STDLIB]]).

### Path Roots and Interpolation

Define a root path, then build on it with interpolation:

```polyglot
[r] $Root;path
   [.] .Unix << "/tmp"
   [.] .Windows << "C:"

[ ] renders as "/tmp\MyApp" on Unix, "C:\MyApp" on Windows
[r] $AppDir;path << "{$Root}\MyApp"
```

Path interpolation with `{$pathVar}` inside a string resolves the path variable to its OS-appropriate subfield. The separator `\` in the string is normalized to `/` on Unix automatically.

### File Path Shorthands

- `"{.}"` — current file's directory
- `"{..}"` — parent directory

These are available in any path context, including `[@]` multi-file package references (see [[packages#Multi-File Packages]]).

### Related

- `#OS` — stdlib enum with `.Unix` and `.Windows` variants. See [[STDLIB]]
- `=Sys.OS` — stdlib pipeline that yields `>os;OS`. See [[STDLIB]]
- PGE-407 — invalid path string (compile error)

## Namespaced Types

Types use dot notation for namespaces — these are fixed schema fields. Namespacing is optional for basic types but available when needed (e.g., referencing enumeration fields from `{#}` definitions).


