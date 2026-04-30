---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.#:Color"
metadata_instance: "%#:Color:N"
---

# #Color — Color Value

<!-- @c:types -->
<!-- @c:aj3lib/types/string -->

`#Color` is a aj3lib type representing a color as RGBA components. It stores red, green, blue, and alpha channels as integers (0-255 each). Color validation requires range checking and named color lookup, so `#Color` is backed by a native `PgColor` class.

See [[scalars]] for scalar subtypes and [[syntax/constructors]] for the `$Color` constructor.

## Definition

```aljam3
{#} #Color
   [%] .description << "Color value (RGBA 0-255)"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "color"
   [.] .r#int
   [.] .g#int
   [.] .b#int
   [.] .a#int
      <~ 255
   [%] %Native.Class
      [.] .Rust << "PgColor"
      [.] .Validate << #True
```

## Fields

| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| `.r` | `#int` | (none) | Red channel (0-255) |
| `.g` | `#int` | (none) | Green channel (0-255) |
| `.b` | `#int` | (none) | Blue channel (0-255) |
| `.a` | `#int` | `255` | Alpha channel (0-255, default fully opaque) |

## Schema Properties

- `[#] ##Scalar` -- sets `%##Depth.Max << 1`, marking `#Color` as a scalar type
- `[#] ###ScalarValue` -- single-value scalar (no enum variants)
- `%##Alias << "color"` -- lets users write `#color` (lowercase) as shorthand

## Native Class

`#Color` is backed by a native Rust class (`PgColor`) declared via `[%] %Native.Class`. The class validates that channel values are in range 0-255 and resolves named colors (e.g., `"red"` → r=255, g=0, b=0) from a built-in lookup table. Validation occurs at compile time for constructor literals.

| Field | Value | Meaning |
|-------|-------|---------|
| `.Rust` | `"PgColor"` | Name of the Rust class backing `#Color` |
| `.Validate` | `#True` | The class exposes a validation function |

## Well-Known Named Colors

| Name | R | G | B | Hex |
|------|---|---|---|-----|
| `red` | 255 | 0 | 0 | `#FF0000` |
| `green` | 0 | 128 | 0 | `#008000` |
| `blue` | 0 | 0 | 255 | `#0000FF` |
| `white` | 255 | 255 | 255 | `#FFFFFF` |
| `black` | 0 | 0 | 0 | `#000000` |
| `yellow` | 255 | 255 | 0 | `#FFFF00` |
| `cyan` | 0 | 255 | 255 | `#00FFFF` |
| `magenta` | 255 | 0 | 255 | `#FF00FF` |
| `orange` | 255 | 165 | 0 | `#FFA500` |
| `purple` | 128 | 0 | 128 | `#800080` |

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Color` | Compile-time type template |
| Instance | `%#:Color:N` | Runtime instance (N = instance number) |
| Native class | `%definition.#:Color.%Native.Class` | Rust class backing (`PgColor`) |

## Related

- [[aj3lib/constructors/Color\|$Color constructor]] -- compile-time color construction
- [[aj3lib/pipelines/Color.Parse\|-Color.Parse]] -- runtime color string parsing
- [[scalars]] -- scalar subtypes overview
- [[string]] -- `#String` foundation type
- [[syntax/types/INDEX\|types]] -- full type system specification
