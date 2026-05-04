---
audience: automation-builder
type: specification
updated: 2026-04-22
---

# $Ver Constructor

<!-- @u:syntax/constructors -->
<!-- @c:jm3lib/types/Ver -->

The `$Ver` constructor produces `#Ver` values from semantic version strings. All overloads are compile-time guaranteed — no error surface, no `[!]` handling needed.

## String-Parsing Overloads

### Major.Minor.Patch-Prerelease+Build

```aljam3
{$} $Ver"{major}.{minor}.{patch}-{pre}+{build}"
   ($) <major.re << "[0-9]+"
   ($) <minor.re << "[0-9]+"
   ($) <patch.re << "[0-9]+"
   ($) <pre.re << "[a-zA-Z0-9.]+"
   ($) <build.re << "[a-zA-Z0-9.]+"
   [$] #Ver
   [.] .major << <major
   [.] .minor << <minor
   [.] .patch << <patch
   [.] .prerelease << <pre
   [.] .build << <build
```

Matches full semver strings like `"1.2.3-alpha.1+20260422"`. The `-` and `+` literal separators between captures are structural — the capture regexes cannot match these characters, ensuring unambiguous parsing.

### Major.Minor.Patch-Prerelease

```aljam3
{$} $Ver"{major}.{minor}.{patch}-{pre}"
   ($) <major.re << "[0-9]+"
   ($) <minor.re << "[0-9]+"
   ($) <patch.re << "[0-9]+"
   ($) <pre.re << "[a-zA-Z0-9.]+"
   [$] #Ver
   [.] .major << <major
   [.] .minor << <minor
   [.] .patch << <patch
   [.] .prerelease << <pre
```

Matches version strings with prerelease label but no build metadata (e.g., `"2.0.0-rc.1"`).

### Major.Minor.Patch+Build

```aljam3
{$} $Ver"{major}.{minor}.{patch}+{build}"
   ($) <major.re << "[0-9]+"
   ($) <minor.re << "[0-9]+"
   ($) <patch.re << "[0-9]+"
   ($) <build.re << "[a-zA-Z0-9.]+"
   [$] #Ver
   [.] .major << <major
   [.] .minor << <minor
   [.] .patch << <patch
   [.] .build << <build
```

Matches version strings with build metadata but no prerelease label (e.g., `"1.0.0+sha.abc123"`).

### Major.Minor.Patch

```aljam3
{$} $Ver"{major}.{minor}.{patch}"
   ($) <major.re << "[0-9]+"
   ($) <minor.re << "[0-9]+"
   ($) <patch.re << "[0-9]+"
   [$] #Ver
   [.] .major << <major
   [.] .minor << <minor
   [.] .patch << <patch
```

Matches basic semver strings (e.g., `"1.2.3"`). The `.` literal separators between captures are structural — `[0-9]+` cannot match `.`, ensuring unambiguous parsing.

## Overload Resolution

The four overloads are distinguished by the presence of `-` and `+` literal separators:

| Overload | Distinguishing Feature |
|---|---|
| Full `"{M}.{m}.{p}-{pre}+{build}"` | Contains both `-` and `+` separators |
| Prerelease `"{M}.{m}.{p}-{pre}"` | Contains `-` but no `+` |
| Build `"{M}.{m}.{p}+{build}"` | Contains `+` but no `-` |
| Basic `"{M}.{m}.{p}"` | No `-` or `+` separators |

Resolution order: longest match first (full before prerelease/build, those before basic).

## Usage

```aljam3
[ ] compile-time guaranteed — no error handling
[-] $appVersion << $Ver"2.1.0"
[-] $preRelease << $Ver"3.0.0-beta.2"
[-] $withBuild << $Ver"1.0.0+20260422"
[-] $full << $Ver"2.0.0-rc.1+sha.abc123"

[ ] for dynamic strings, use -Ver.Parse with error handling
[-] $parsed#ver << -Ver.Parse
   (<) <raw#string << $versionString
   [!] !Parse.Ver.InvalidFormat
      [-] $parsed << $Ver"0.0.0"
```

## Related

- [[constructors/INDEX|jm3lib Constructors]] -- constructor registry
- [[syntax/constructors]] -- `{$}` block specification
- [[jm3lib/pipelines/Ver.Parse|-Ver.Parse]] -- runtime version string parsing
- [[jm3lib/types/Ver|#Ver type]] -- semantic version type definition
