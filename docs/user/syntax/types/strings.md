---
audience: user
type: specification
updated: 2026-03-30
---

# String Interpolation and Path Type

<!-- @syntax/types/INDEX -->

## String Interpolation

Strings support variable interpolation using `{$variable}` inside string literals. Any `$`-prefixed identifier inside `{...}` within a double-quoted string is expanded to its value:

```polyglot
[r] $greeting#string << "Hello {$name}, you are {$age} years old"
[r] $path#string << "/users/{$userId}/profile"
```

Interpolation works with any `$`-prefixed variable, including flexible-field paths:

```polyglot
[r] $msg#string << "User {$user:name} logged in from {$user:location}"
```

For literal curly braces inside strings, use `{{` and `}}`.

## Path Type

`path` is a stdlib struct with OS-specific subfields:

```polyglot
{#} #path
   [.] .Unix#string
   [.] .Windows#string
```

### Explicit Subfield Assignment

Assign both subfields so code works cross-platform:

```polyglot
[r] $AppDir#path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"
```

At runtime, the Polyglot runtime resolves `$AppDir` to the correct subfield based on the current OS (see `=Sys.OS` in [[stdlib/INDEX|Standard Library]]).

Assigning only one subfield triggers a portability warning (PGW-408). If the missing subfield is for the current OS, the compiler raises an error (PGE-408).

A plain string cannot be assigned to a `#path` variable — `[r] $dir#path << "/tmp"` is a type mismatch (PGE-401). Use `=Path"..."` instead.

### `=Path"..."` Inline Notation

`=Path"..."` is an inline pipeline call ([[stdlib/pipelines/Path|=Path]], [[concepts/pipelines/inline-calls#Inline Pipeline Calls]]) that creates a `#path` value from a string:

```polyglot
[r] $LogDir#path << =Path"/tmp/MyApp/logs"
[r] $AppDir#path << =Path"{.}/MyApp"
```

Both `/` and `\` are treated as path separators and normalized to the correct separator for the current OS. These two lines produce identical results:

```polyglot
[r] $a#path << =Path"{.}\MyApp\logs"
[r] $b#path << =Path"{.}/MyApp/logs"
[ ] $a and $b resolve to the same path on any OS
```

`{$var}` interpolation works inside `=Path"..."` strings — interpolated variables must be `#path` values with both OS subfields defined (e.g., `{.}`, `{..}`, or a user-defined `#path` variable). `{{` and `}}` produce literal brace characters, same as in regular string interpolation.

### Path Roots and Interpolation

Define a root path, then build on it with interpolation:

```polyglot
[r] $Root#path
   [.] .Unix << "/tmp"
   [.] .Windows << "C:"

[ ] renders as "/tmp/MyApp" on Unix, "C:\MyApp" on Windows
[r] $AppDir#path << =Path"{$Root}/MyApp"
```

Path interpolation with `{$pathVar}` inside `=Path"..."` resolves the path variable to its OS-appropriate subfield.

### File Path Shorthands

- `{.}` — current file's directory (`#path` value, defined for all OS)
- `{..}` — parent directory (`#path` value, defined for all OS)

These are equivalent to a built-in `$cfd` variable and are available in any path context, including `=Path"..."` calls and `[@]` multi-file package references (see [[packages#Multi-File Packages]]).

### Trigger Path Strings

Trigger inline string arguments that contain file paths parse as path strings:

```polyglot
[t] =T.Folder.NewFiles"/inbox/"
```

The `"/inbox/"` argument is parsed as a path string — separators are normalized per OS, same as `=Path"..."`.

### Related

- `=Path` — stdlib pipeline for creating `#path` values from strings. See [[stdlib/pipelines/Path|=Path]]
- `#OS` — stdlib enum with `.Unix` and `.Windows` variants. See [[stdlib/INDEX|Standard Library]]
- `=Sys.OS` — stdlib pipeline that yields `>os#OS`. See [[stdlib/INDEX|Standard Library]]
- PGE-407 — invalid path string (compile error)
- PGE-408 — missing path platform subfield (compile error)
- PGW-408 — single-platform path (warning)
