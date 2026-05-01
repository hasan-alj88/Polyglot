---
audience: automation-builder
type: specification
updated: 2026-03-30
---

# String Interpolation and Path Type

<!-- @syntax/types/INDEX -->

## String Interpolation

Strings support variable interpolation using `{$variable}` inside string literals. Any `$`-prefixed identifier inside `{...}` within a double-quoted string is expanded to its value:

```aljam3
[-] $greeting#string << "Hello {$name}, you are {$age} years old"
[-] $path#string << "/users/{$userId}/profile"
```

Interpolation works with any `$`-prefixed variable, including flexible-field paths:

```aljam3
[-] $msg#string << "User {$user:name} logged in from {$user:location}"
```

For literal curly braces inside strings, use `{{` and `}}`.

## Path Type

`path` is a jm3lib struct with OS-specific subfields:

```aljam3
{#} #path
   [.] .Unix#string
   [.] .Windows#string
```

### Explicit Subfield Assignment

Assign both subfields so code works cross-platform:

```aljam3
[-] $AppDir#path
   [.] .Unix << "/tmp/MyApp"
   [.] .Windows << "C:\MyApp"
```

At runtime, the Aljam3 runtime resolves `$AppDir` to the correct subfield based on the current OS (see `-Sys.OS` in [[jm3lib/INDEX|Standard Library]]).

Assigning only one subfield triggers a portability warning (PGW04001). If the missing subfield is for the current OS, the compiler raises an error (PGE04008).

A plain string cannot be assigned to a `#path` variable — `[-] $dir#path << "/tmp"` is a type mismatch (PGE04001). Use `$Path"..."` constructor instead.

### `$Path"..."` Constructor Notation

`$Path"..."` is a constructor call ([[syntax/constructors]]) that creates a `#path` value from a string literal. On infrastructure lines (`[T]`/`[Q]`/`[W]`), the inline pipeline form `-Path"..."` remains valid — see [[jm3lib/pipelines/Path|-Path]].

```aljam3
[-] $LogDir#path << $Path"/tmp/MyApp/logs"
[-] $AppDir#path << $Path"{.}/MyApp"
```

Both `/` and `\` are treated as path separators and normalized to the correct separator for the current OS. These two lines produce identical results:

```aljam3
[-] $a#path << $Path"{.}\MyApp\logs"
[-] $b#path << $Path"{.}/MyApp/logs"
[ ] $a and $b resolve to the same path on any OS
```

`{$var}` interpolation works inside `$Path"..."` strings — interpolated variables must be constructor-sourced `#path` values (e.g., `{.}`, `{..}`, or a user-defined `#path` variable from another `$Path` call). `{{` and `}}` produce literal brace characters, same as in regular string interpolation. For dynamic/untrusted path strings, use `-Path.Parse` with error handling.

### Path Roots and Interpolation

Define a root path, then build on it with interpolation:

```aljam3
[-] $Root#path
   [.] .Unix << "/tmp"
   [.] .Windows << "C:"

[ ] renders as "/tmp/MyApp" on Unix, "C:\MyApp" on Windows
[-] $AppDir#path << $Path"{$Root}/MyApp"
```

Path interpolation with `{$pathVar}` inside `$Path"..."` resolves the path variable to its OS-appropriate subfield.

### File Path Shorthands

- `{.}` — current file's directory (`#path` value, defined for all OS)
- `{..}` — parent directory (`#path` value, defined for all OS)

These are equivalent to a built-in `$cfd` variable and are available in any path context, including `-Path"..."` calls and `[@]` multi-file package references (see [[packages#Multi-File Packages]]).

### Trigger Path Strings

Trigger inline string arguments that contain file paths parse as path strings:

```aljam3
[T] -T.Folder.NewFiles"/inbox/"
```

The `"/inbox/"` argument is parsed as a path string — separators are normalized per OS, same as `-Path"..."`.

### Related

- `-Path` — jm3lib pipeline for creating `#path` values from strings. See [[jm3lib/pipelines/Path|-Path]]
- `#OS` — jm3lib enum with `.Unix` and `.Windows` variants. See [[jm3lib/INDEX|Standard Library]]
- `-Sys.OS` — jm3lib pipeline that yields `>os#OS`. See [[jm3lib/INDEX|Standard Library]]
- PGE04007 — invalid path string (compile error)
- PGE04008 — missing path platform subfield (compile error)
- PGW04001 — single-platform path (warning)
