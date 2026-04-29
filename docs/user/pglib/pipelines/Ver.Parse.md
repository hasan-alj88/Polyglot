---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.-:Ver.Parse"
metadata_instance: "%-:Ver.Parse:N"
---

# -Ver.Parse

Parses a dynamic string into a `#ver` value at runtime. Use this when the input comes from IO or other runtime sources where the compiler cannot prove validity at compile time.

For known literals in execution body, use the `$Ver` constructor instead — it guarantees no error surface.

## Definition

```aljam3
{N} -Ver.Parse
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "VerParse"
   [%] .description << "Parse dynamic string to semantic version"
   (-) <raw#string
   (-) >ver#ver
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<raw` | `#string` | --- | String to parse as a semantic version (expected format: `major.minor.patch[-prerelease][+build]`) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>ver` | `#ver` | Parsed version value with `.major`, `.minor`, `.patch` (and optionally `.prerelease`, `.build`) fields populated |

## Errors

| Error | Description |
|-------|-------------|
| `!Parse.Ver.InvalidFormat` | Input string does not match SemVer 2.0 format (missing components, non-numeric major/minor/patch, invalid prerelease characters) |

## Permissions

None required. Pure computation pipeline.

## Usage

```aljam3
[-] $version#ver << -Ver.Parse
   (<) <raw#string << $versionString
   [!] !Parse.Ver.InvalidFormat
      [-] $version << $Ver"0.0.0"
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Ver.Parse` | Compile-time pipeline template |
| Instance | `%-:Ver.Parse:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/constructors/Ver|$Ver constructor]] -- compile-time version construction
- [[pglib/types/Ver|#Ver type]] -- semantic version type definition
