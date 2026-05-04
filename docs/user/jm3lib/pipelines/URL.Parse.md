---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.-:URL.Parse"
metadata_instance: "%-:URL.Parse:N"
---

# -URL.Parse

Parses a dynamic string into a `#url` value at runtime. Use this when the input comes from IO or other runtime sources where the compiler cannot prove validity at compile time.

For known literals in execution body, use the `$URL` constructor instead — it guarantees no error surface.

## Definition

```aljam3
{N} -URL.Parse
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "UrlParse"
   [%] .description << "Parse dynamic string to URL"
   (-) <raw#string
   (-) >url#url
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<raw` | `#string` | --- | String to parse as a URL (expected format: `scheme://host[:port][/path][?query][#fragment]`) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>url` | `#url` | Parsed URL value with `.scheme`, `.host`, and optional `.port`, `.path`, `.query`, `.fragment` fields populated |

## Errors

| Error | Description |
|-------|-------------|
| `!Parse.URL.InvalidFormat` | Input string does not conform to RFC 3986 URL syntax (missing scheme, malformed authority, invalid characters) |
| `!Parse.URL.InvalidScheme` | Scheme component is empty or contains invalid characters |

## Permissions

None required. Pure computation pipeline.

## Usage

```aljam3
[-] $endpoint#url << -URL.Parse
   (<) <raw#string << $userInput
   [!] !Parse.URL.InvalidFormat
      [-] $endpoint << $URL"https://fallback.example.com"
   [!] !Parse.URL.InvalidScheme
      [-] $endpoint << $URL"https://fallback.example.com"
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:URL.Parse` | Compile-time pipeline template |
| Instance | `%-:URL.Parse:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/constructors/URL|$URL constructor]] -- compile-time URL construction
- [[jm3lib/types/URL|#URL type]] -- URL type definition
