---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.-:Color.Parse"
metadata_instance: "%-:Color.Parse:N"
---

# -Color.Parse

Parses a dynamic string into a `#color` value at runtime. Accepts hex strings (`#RRGGBB`, `#RRGGBBAA`) and named colors (`red`, `blue`). Use this when the input comes from IO or other runtime sources where the compiler cannot prove validity at compile time.

For known literals in execution body, use the `$Color` constructor instead — it guarantees no error surface.

## Definition

```aljam3
{N} -Color.Parse
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "ColorParse"
   [%] .description << "Parse dynamic string to color value"
   (-) <raw#string
   (-) >color#color
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<raw` | `#string` | --- | String to parse as a color (hex `#RRGGBB`/`#RRGGBBAA` or named color) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>color` | `#color` | Parsed color value with `.r`, `.g`, `.b`, `.a` fields populated |

## Errors

| Error | Description |
|-------|-------------|
| `!Parse.Color.InvalidFormat` | Input string does not match hex color format (wrong length, invalid hex digits, missing `#` prefix for hex) |
| `!Parse.Color.UnknownName` | Input string looks like a named color (no `#` prefix) but is not in the known color table |

## Permissions

None required. Pure computation pipeline.

## Usage

```aljam3
[-] $theme#color << -Color.Parse
   (<) <raw#string << $userColor
   [!] !Parse.Color.InvalidFormat
      [-] $theme << $Color"black"
   [!] !Parse.Color.UnknownName
      [-] $theme << $Color"black"
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Color.Parse` | Compile-time pipeline template |
| Instance | `%-:Color.Parse:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/constructors/Color|$Color constructor]] -- compile-time color construction
- [[aj3lib/types/Color|#Color type]] -- color value type definition
