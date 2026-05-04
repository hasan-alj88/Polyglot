---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.-:MIME.Parse"
metadata_instance: "%-:MIME.Parse:N"
---

# -MIME.Parse

Parses a dynamic string into a `#mime` value at runtime. Use this when the input comes from IO or other runtime sources where the compiler cannot prove validity at compile time.

For known literals in execution body, use the `$MIME` constructor instead — it guarantees no error surface.

## Definition

```aljam3
{N} -MIME.Parse
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "MimeParse"
   [%] .description << "Parse dynamic string to MIME type"
   (-) <raw#string
   (-) >mime#mime
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<raw` | `#string` | --- | String to parse as a MIME type (expected format: `type/subtype`) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>mime` | `#mime` | Parsed MIME type value with `.type` and `.subtype` fields populated |

## Errors

| Error | Description |
|-------|-------------|
| `!Parse.MIME.InvalidFormat` | Input string does not match `type/subtype` pattern (missing `/`, empty components, invalid characters) |

## Permissions

None required. Pure computation pipeline.

## Usage

```aljam3
[-] $contentType#mime << -MIME.Parse
   (<) <raw#string << $headerValue
   [!] !Parse.MIME.InvalidFormat
      [-] $contentType << $MIME"application/octet-stream"
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:MIME.Parse` | Compile-time pipeline template |
| Instance | `%-:MIME.Parse:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[jm3lib/constructors/MIME|$MIME constructor]] -- compile-time MIME construction
- [[jm3lib/types/MIME|#MIME type]] -- media type definition
