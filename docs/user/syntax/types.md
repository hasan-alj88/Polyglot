---
audience: user
type: specification
updated: 2026-03-21
status: complete
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

All Polyglot data is serialized strings. The type system is a schema layer on top of strings that constrains how each string is interpreted.

### RawString — The True Primitive

`RawString` is the only true primitive in Polyglot — a compiler intrinsic. It is a sequence of literal raw characters: no interpolation, no substitutions, no escaping. Every character is literal.

`RawString` literals use inline pipeline syntax: `=RawString"..."` or the alias `=rs"..."`. Users can annotate variables as `;RawString` directly, though this is rare — most code works with `;string` instead.

### #String — The String Struct

<!-- @types -->
What `;string` refers to is `#String` — a struct built on `RawString`:

```polyglot
{#} #String
   [.] .string;RawString
   [.] .re;RawString
```

- `.string` — the raw string value
- `.re` — a regular expression constraint. Defaults to `""` (accept any string). Can be pushed once to Final (see [[variable-lifecycle]])

A string literal (quoted text with `{$var}` interpolation) is always `;string`. When `.re` is set, the string value must match the pattern — violations are caught at compile time for literals (PGE-410) and at runtime for dynamic values (handled with `[!]` error blocks).

### Numeric Types — #String Subtypes

`int`, `float`, and future numeric types are subtypes of `#String` on a flexible level. Each has a pre-set `.re` that constrains its string value:

| Type | `.re` pattern | Example values |
|------|--------------|----------------|
| `int` | `^-?[0-9]+$` | `42`, `-7`, `007` |
| `float` | `^-?[0-9]+\.[0-9]+$` | `3.14`, `-0.5`, `007.00` |
| `sci` | TBD | Scientific notation (deferred) |
| `eng` | TBD | Engineering notation (deferred) |

Users can define custom string subtypes with their own `.re`:

```polyglot
{#} #String.emailAddress
   [.] .string;RawString
   [.] .re;RawString << "^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$"
```

Literal numeric values always match their RE by construction — no error handling needed.

> **Note:** The full metadata path for `int` is `%#:String:int` — String subtypes are nested under `:String` at a flexible level. `;int` is an alias for `;String.int`. See [[data-is-trees#String Subtypes — Nested Under `:String`]] for how subtypes connect to the unified tree, and [[metadata#String Subtypes in the Tree]] for the complete type registry structure.

### Other Types

- `bool` — alias for the `#Boolean` struct. An enum with `.True` and `.False` fields. `#Boolean` is a separate struct (not a `#String` subtype) — it uses enum fields, not RE-based value fields. See [[STDLIB#Boolean]].
- `array` — a struct with enumerated keys (`.0`, `.1`, `.2`, ...). Element count is dynamic but each element conforms to the declared element type. See [[collections]].
- `serial` — schema-free. Always uses flexible fields (`:`), even if dot notation is used in access. Any keys, any types, any depth. No compile-time validation of shape. Easily converts to/from JSON-like formats. See [[collections]].
- struct (`{#}`) — defined schema. Compile-time enforced field names and types. See [[#Struct Types]].

## Element-Typed Arrays

Arrays can specify their element type using `.` (fixed field) notation:

```polyglot
[r] $files;array.path <~ {}
[r] $names;array.string <~ {}
[r] $scores;array.int <~ {}
```

This constrains the array to hold only elements of the specified type.

## Struct Types

<!-- @blocks -->
<!-- @identifiers -->
A **struct** is any type with a defined schema, declared with `{#}` (see [[blocks]]). The term "struct" refers to the type category — `{#}` is the declaration syntax. Every struct definition creates a schema template at `%definition.#:{StructName}` and instances at `%#:{StructName}:{n}` — see [[data-is-trees#Schema vs Instance]].

Stdlib structs include `path`, `#Boolean`, `#OS`, `#PipelineStatus`, `#VarState`, and `#DateTime`. User-defined structs follow the same rules.

In type annotations (after `;`), the `#` prefix is always dropped — the `;` already signals a type context:

```polyglot
[r] $user;UserRecord <~ ...
[r] $users;array.UserRecord <~ {}
[=] <incoming;Alert
```

The `#` prefix is only used when **referencing** a struct outside of type annotations (e.g., `#Boolean.True`, `@alias#DataName.EnumField`).

### Struct Level Rules

Each level in a struct must be homogeneous in two ways (see [[identifiers#Serialization Rules]]):

1. **Separator homogeneity** — all siblings at a level must be all fixed (`.`) or all flexible (`:`) — never mixed (PGE-501)
2. **Kind homogeneity** — all siblings at a level must be all enum fields or all value fields — never mixed (PGE-502)

A field whose type is itself a struct defines the later levels. It is invalid to declare sub-levels after a field typed as a struct — the struct's definition already specifies those levels.

```polyglot
{#} #Config
   [.] .timeout;int
   [.] .server;ServerInfo      [ ] later levels defined by #ServerInfo

{#} #ServerInfo
   [.] .host;string
   [.] .port;int
```

## Inline Data Shorthand

Curly braces with comma-separated values create inline data:

```polyglot
[r] $values;array <~ {1, 2, 3, 4, 5}
```

This is shorthand for explicit field assignment: `#data.1 << 1`, `#data.2 << 2`, etc.

## Enum Fields vs Value Fields

In `{#}` struct definitions, fields are either **enum fields** or **value fields**:

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

## Typed Flexible Fields

A struct can have levels that use flexible (`:`) fields. At such a level, the `[:] :*;Type` syntax declares that ALL `:` siblings share the same type — a typed dictionary. The `:*` wildcard means "collectively, every key at this level has this type."

```polyglot
{#} #Registry
   [.] .builtins
      [.] .http;Handler
      [.] .grpc;Handler
   [.] .plugins
      [:] :*;Handler
```

Here `.plugins` has flexible children. Every `:key` under `.plugins` must be `;Handler`. Users can push `:myPlugin`, `:anotherPlugin`, etc. — all constrained to `;Handler`.

### Schema Enforcement on New Keys

When a typed flexible level references a struct type, all new keys inherit that struct's schema:

```polyglot
{#} #SubStruct
   [.] .level4;string

{#} #Example
   [.] .level1
      [.] .level2
         [:] :*;SubStruct
```

To create a new key `:new` alongside existing `:level3`: push to `#Example.level1.level2:new.level4` — the compiler knows `:new` is `;SubStruct`, so `.level4;string` is enforced.

### Constraints

- **No extra levels** — a flexible field's children are fully defined by its type annotation. You cannot insert additional levels between the flexible field and its typed children.
- **Multi-level flexibility** — a struct can have multiple flexible levels (each level independently homogeneous):

```polyglot
{#} #DeepFlex
   [.] .config
      [:] :*;Section

{#} #Section
   [:] :*;Setting

{#} #Setting
   [.] .value;string
   [.] .default;string
```

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

`live` fields are **implicit** on every `{=}` pipeline, `$` variable, and `{#}` struct. They do not need to be declared — the runtime populates them automatically and updates them in real-time.

See [[metadata]] for the full metadata tree, all `live` field listings, and access patterns.

## Path Type

`path` is a stdlib struct with OS-specific subfields:

```polyglot
{#} #path
   [.] .Unix;string
   [.] .Windows;string
```

### Explicit Subfield Assignment

Assign both subfields so code works cross-platform:

```polyglot
[r] $AppDir;path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"
```

At runtime, the Polyglot runtime resolves `$AppDir` to the correct subfield based on the current OS (see `=Sys.OS` in [[STDLIB]]).

Assigning only one subfield triggers a portability warning (PGW-408). If the missing subfield is for the current OS, the compiler raises an error (PGE-408).

A plain string cannot be assigned to a `;path` variable — `[r] $dir;path << "/tmp"` is a type mismatch (PGE-401). Use `=Path"..."` instead.

### `=Path"..."` Inline Notation

`=Path"..."` is an inline pipeline call ([[STDLIB#=Path]], [[pipelines#Inline Pipeline Calls]]) that creates a `;path` value from a string:

```polyglot
[r] $LogDir;path << =Path"/tmp/MyApp/logs"
[r] $AppDir;path << =Path"{.}/MyApp"
```

Both `/` and `\` are treated as path separators and normalized to the correct separator for the current OS. These two lines produce identical results:

```polyglot
[r] $a;path << =Path"{.}\MyApp\logs"
[r] $b;path << =Path"{.}/MyApp/logs"
[ ] $a and $b resolve to the same path on any OS
```

`{$var}` interpolation works inside `=Path"..."` strings — interpolated variables must be `;path` values with both OS subfields defined (e.g., `{.}`, `{..}`, or a user-defined `;path` variable). `{{` and `}}` produce literal brace characters, same as in regular string interpolation.

### Path Roots and Interpolation

Define a root path, then build on it with interpolation:

```polyglot
[r] $Root;path
   [.] .Unix << "/tmp"
   [.] .Windows << "C:"

[ ] renders as "/tmp/MyApp" on Unix, "C:\MyApp" on Windows
[r] $AppDir;path << =Path"{$Root}/MyApp"
```

Path interpolation with `{$pathVar}` inside `=Path"..."` resolves the path variable to its OS-appropriate subfield.

### File Path Shorthands

- `{.}` — current file's directory (`;path` value, defined for all OS)
- `{..}` — parent directory (`;path` value, defined for all OS)

These are equivalent to a built-in `$cfd` variable and are available in any path context, including `=Path"..."` calls and `[@]` multi-file package references (see [[packages#Multi-File Packages]]).

### Trigger Path Strings

Trigger inline string arguments that contain file paths parse as path strings:

```polyglot
[t] =T.Folder.NewFiles"/inbox/"
```

The `"/inbox/"` argument is parsed as a path string — separators are normalized per OS, same as `=Path"..."`.

### Related

- `=Path` — stdlib pipeline for creating `;path` values from strings. See [[STDLIB#=Path]]
- `#OS` — stdlib enum with `.Unix` and `.Windows` variants. See [[STDLIB]]
- `=Sys.OS` — stdlib pipeline that yields `>os;OS`. See [[STDLIB]]
- PGE-407 — invalid path string (compile error)
- PGE-408 — missing path platform subfield (compile error)
- PGW-408 — single-platform path (warning)

## Type Conversions

### Struct → Serial

Always allowed. A struct's fixed (`.`) fields are converted to flexible (`:`) fields in the serial. The struct is always a valid subset of serial's openness.

```polyglot
{#} #UserRecord
   [.] .name;string
   [.] .age;int

[r] $user;UserRecord
   [r] $user.name << "Alice"
   [r] $user.age << 30

[ ] ✓ struct → serial is always safe
[r] $data;serial << $user
[ ] $data now has :name and :age as flexible fields
```

### Serial → Struct

Allowed only if the serial's fields satisfy the struct's schema. Extra fields in the serial are ignored; missing fields are an error.

The compiler performs best-effort static analysis:
- **Provably matches** — no handling needed
- **Provably mismatches** — PGE-402 (schema mismatch)
- **Cannot prove match** — user must handle with `[!]` + `*Continue >FallBack`. If absent → PGE-409

```polyglot
{#} #UserRecord
   [.] .name;string
   [.] .age;int

[ ] ✓ handled — *Continue provides fallback if serial doesn't match
[r] $defaultUser;UserRecord
   [r] $defaultUser.name << "Unknown"
   [r] $defaultUser.age << 0

[r] $user;UserRecord << $dynamicSerial
   [!] !SchemaMismatch
      [*] *Continue >FallBack << $defaultUser
```

See [TYPE-IDENTITY](../../technical/compile-rules/TYPE-IDENTITY.md) rules 5 and 6, [PGE-409](../../technical/compile-rules/PGE/PGE-409-unhandled-serial-struct-conversion.md).

## Namespaced Types

Types use dot notation for namespaces — these are fixed schema fields. Namespacing is optional for basic types but available when needed (e.g., referencing enumeration fields from `{#}` definitions).

```polyglot
[ ] Direct type annotation — most common
[r] $score;int <~ 0

[ ] Fully qualified — equivalent to the above
[r] $score;String.int <~ 0

[ ] Struct enum field — must use # outside type annotations
[r] $severity << #Severity.Critical

[ ] Cross-package reference — @alias#DataName.Field
[r] $status << @alerts#Severity.Error
```

In type annotations (after `;`), the `#` prefix is dropped — the compiler knows `;` starts a type context. Outside annotations, struct references keep the `#` prefix. See [[identifiers#Serialized Identifiers]] for the full prefix rules.


