---
audience: automation-builder
type: specification
updated: 2026-04-22
---

# $Path Constructor

<!-- @u:syntax/constructors -->
<!-- @c:aj3lib/types/path -->

The `$Path` constructor produces `#path` values from string literals and keywords. All overloads are compile-time guaranteed — no error surface, no `[!]` handling needed.

## String-Parsing Overload

### Path String

```aljam3
{$} $Path"{path}"
   ($) <path.re << "[^\x00]+"
   [$] #path
   [.] .Unix << <path
   [.] .Windows << <path
```

Accepts any non-null string as a path. The runtime normalizes separators per OS — `/` for Unix, `\` for Windows. Both `.Unix` and `.Windows` fields are populated from the same capture; the runtime resolves which representation to use.

Interpolation is supported when the interpolated variable was produced by another constructor:

```aljam3
[-] $base << $Path"/reports"
[-] $full << $Path"{$base}/daily"
```

## Keyword Overloads

### Current Directory

```aljam3
{$} $Path"."
   [$] #path
   [.] .Unix << %Runtime.CWD.Unix
   [.] .Windows << %Runtime.CWD.Windows
```

Produces the current working directory. The runtime resolves the actual path at execution time.

### Parent Directory

```aljam3
{$} $Path".."
   [$] #path
   [.] .Unix << %Runtime.CWD.Parent.Unix
   [.] .Windows << %Runtime.CWD.Parent.Windows
```

Produces the parent of the current working directory.

## Overload Resolution

| Overload | Distinguishing Feature |
|---|---|
| Path string | Any string that is not `.` or `..` exactly |
| `.` | Exact match — single dot |
| `..` | Exact match — double dot |

Keywords are tested first (exact match). Any non-matching string falls through to the path string overload.

## Usage

```aljam3
[ ] compile-time guaranteed — no error handling
[-] $logDir << $Path"/var/log/myapp"
[-] $cwd << $Path"."
[-] $parent << $Path".."
[-] $subDir << $Path"{$logDir}/archives"

[ ] for dynamic strings, use -Path.Parse with error handling
[-] $parsed#path << -Path.Parse
   (<) <raw#string << $userInput
   [!] !Parse.Path.InvalidCharacter
      [-] $parsed << $Path"."
```

## Related

- [[constructors/INDEX|aj3lib Constructors]] -- constructor registry
- [[syntax/constructors]] -- `{$}` block specification
- [[aj3lib/pipelines/Path|-Path pipeline]] -- Path creation pipeline
- [[aj3lib/pipelines/Path.Parse|-Path.Parse]] -- runtime path parsing
- [[aj3lib/types/path|#path type]] -- cross-platform path struct
