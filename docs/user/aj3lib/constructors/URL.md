---
audience: automation-builder
type: specification
updated: 2026-04-22
---

# $URL Constructor

<!-- @u:syntax/constructors -->
<!-- @c:aj3lib/types/URL -->

The `$URL` constructor produces `#URL` values from URL strings. URL parsing requires an RFC 3986-compliant parser, so this constructor uses the native pipeline conversion pattern (like `$Dur`) — the captured string is passed to `-URL.Decompose` which parses and decomposes it into `#URL` fields. All overloads are compile-time guaranteed — no error surface, no `[!]` handling needed.

## String-Parsing Overload

### Full URL

```aljam3
{$} $URL"{url}"
   ($) <url.re << ".+"
   [-] -URL.Decompose
      (<) <raw << <url
      (-) >scheme >> $s
      (-) >host >> $h
      (-) >port >> $p
      (-) >path >> $pa
      (-) >query >> $q
      (-) >fragment >> $f
   [$] #URL
   [.] .scheme << $s
   [.] .host << $h
   [.] .port << $p
   [.] .path << $pa
   [.] .query << $q
   [.] .fragment << $f
```

Captures the entire argument string as a URL. The `($)` regex `.+` accepts any non-empty string — the actual validation and decomposition is performed by the native `-URL.Decompose` pipeline at compile time.

**Native pipeline conversion pattern:** Like `$Dur`, the `$URL` constructor delegates to a native pipeline (`-URL.Decompose`) that performs both validation and field extraction. This pipeline is a aj3lib-internal compile-time primitive — it is invoked during constructor evaluation, not at runtime.

## Overload Resolution

Single overload — no ambiguity.

## Usage

```aljam3
[ ] compile-time guaranteed — no error handling
[-] $api << $URL"https://api.example.com/v2/users"
[-] $local << $URL"http://localhost:8080/health"
[-] $withQuery << $URL"https://search.example.com/q?term=aljam3&lang=en"
[-] $ssh << $URL"ssh://git@github.com:22/user/repo.git"

[ ] for dynamic strings, use -URL.Parse with error handling
[-] $endpoint#url << -URL.Parse
   (<) <raw#string << $userInput
   [!] !Parse.URL.InvalidFormat
      [-] $endpoint << $URL"https://fallback.example.com"
```

## Related

- [[constructors/INDEX|aj3lib Constructors]] -- constructor registry
- [[syntax/constructors]] -- `{$}` block specification
- [[aj3lib/pipelines/URL.Parse|-URL.Parse]] -- runtime URL string parsing
- [[aj3lib/types/URL|#URL type]] -- URL type definition
