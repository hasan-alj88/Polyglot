---
audience: automation-builder
type: specification
updated: 2026-04-22
---

# $MIME Constructor

<!-- @u:syntax/constructors -->
<!-- @c:jm3lib/types/MIME -->

The `$MIME` constructor produces `#MIME` values from media type strings. All overloads are compile-time guaranteed — no error surface, no `[!]` handling needed.

## String-Parsing Overload

### Type/Subtype

```aljam3
{$} $MIME"{type}/{subtype}"
   ($) <type.re << "[a-zA-Z0-9][a-zA-Z0-9!#$&.+^_-]*"
   ($) <subtype.re << "[a-zA-Z0-9][a-zA-Z0-9!#$&.+^_-]*"
   [$] #MIME
   [.] .type << <type
   [.] .subtype << <subtype
```

Matches MIME strings in `type/subtype` format (RFC 6838 token rules). The `/` literal separator between captures is structural — the structural integrity check confirms neither capture regex can match `/`, ensuring unambiguous parsing.

## Overload Resolution

Single overload — no ambiguity.

## Usage

```aljam3
[ ] compile-time guaranteed — no error handling
[-] $json << $MIME"application/json"
[-] $csv << $MIME"text/csv"
[-] $binary << $MIME"application/octet-stream"

[ ] for dynamic strings, use -MIME.Parse with error handling
[-] $contentType#mime << -MIME.Parse
   (<) <raw#string << $headerValue
   [!] !Parse.MIME.InvalidFormat
      [-] $contentType << $MIME"application/octet-stream"
```

## Related

- [[constructors/INDEX|jm3lib Constructors]] -- constructor registry
- [[syntax/constructors]] -- `{$}` block specification
- [[jm3lib/pipelines/MIME.Parse|-MIME.Parse]] -- runtime MIME string parsing
- [[jm3lib/types/MIME|#MIME type]] -- media type definition
