---
audience: automation-builder
type: specification
updated: 2026-04-22
---

# $Re Constructor

<!-- @u:syntax/constructors -->
<!-- @c:jm3lib/types/Re -->

The `$Re` constructor produces `#Re` values from regex pattern strings. The compiler validates that the pattern is syntactically valid regex at compile time — no error surface, no `[!]` handling needed.

## String-Parsing Overload

### Regex Pattern

```aljam3
{$} $Re"{pattern}"
   ($) <pattern.re << ".+"
   [$] #Re
   [.] .pattern << <pattern
```

Captures the entire argument string as a regex pattern. The `($)` regex `.+` accepts any non-empty string — the actual validation is performed by the `PgRegex` native class at compile time.

**Native validation pattern:** Unlike most constructors where `($) .re` constraints are sufficient to prove validity, regex patterns cannot be validated by regex. The compiler invokes an actual regex parser (the `PgRegex` native class) to confirm the captured string is syntactically valid. This is compile-time validation, not runtime — an invalid pattern is a compile error, not a runtime error.

## Overload Resolution

Single overload — no ambiguity.

## Usage

```aljam3
[ ] compile-time guaranteed — no error handling
[-] $emailPattern << $Re"^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$"
[-] $digits << $Re"^[0-9]+$"
[-] $isoDate << $Re"^[0-9]{4}-[0-1][0-9]-[0-3][0-9]$"

[ ] for dynamic strings, use -Re.Parse with error handling
[-] $userPattern#re << -Re.Parse
   (<) <raw#string << $userInput
   [!] !Parse.Re.InvalidSyntax
      [-] $userPattern << $Re".*"
```

## Related

- [[constructors/INDEX|jm3lib Constructors]] -- constructor registry
- [[syntax/constructors]] -- `{$}` block specification
- [[jm3lib/pipelines/Re.Parse|-Re.Parse]] -- runtime regex string parsing
- [[jm3lib/types/Re|#Re type]] -- regex pattern type definition
