---
audience: automation-builder
type: specification
updated: 2026-04-22
---

# $Color Constructor

<!-- @u:syntax/constructors -->
<!-- @c:pglib/types/Color -->

The `$Color` constructor produces `#Color` values from hex strings or named colors. Hex overloads use native pipeline conversion (like `$Dur`) to decompose hex digits into RGBA channels; the named overload uses native validation (like `$Re`) to look up color names. All overloads are compile-time guaranteed — no error surface, no `[!]` handling needed.

## String-Parsing Overloads

### Hex RGB (6-digit)

```aljam3
{$} $Color"#{hex}"
   ($) <hex.re << "[0-9a-fA-F]{6}"
   [-] -Color.HexToRGB
      (<) <hex << <hex
      (-) >r >> $r
      (-) >g >> $g
      (-) >b >> $b
   [$] #Color
   [.] .r << $r
   [.] .g << $g
   [.] .b << $b
```

Matches 6-digit hex color strings like `"#FF8800"`. The `#` prefix is a literal separator. The `-Color.HexToRGB` native pipeline decomposes the hex string into red, green, and blue channel values (0-255 each). Alpha defaults to 255 (fully opaque).

### Hex RGBA (8-digit)

```aljam3
{$} $Color"#{hex}"
   ($) <hex.re << "[0-9a-fA-F]{8}"
   [-] -Color.HexToRGBA
      (<) <hex << <hex
      (-) >r >> $r
      (-) >g >> $g
      (-) >b >> $b
      (-) >a >> $a
   [$] #Color
   [.] .r << $r
   [.] .g << $g
   [.] .b << $b
   [.] .a << $a
```

Matches 8-digit hex color strings like `"#FF880080"`. The last two hex digits encode the alpha channel. `-Color.HexToRGBA` decomposes all four channels.

### Named Color

```aljam3
{$} $Color"{name}"
   ($) <name.re << "[a-z]+"
   [$] #Color
   [.] << -Color.NameLookup
      (<) <name << <name
```

Matches named color strings like `"red"`, `"blue"`, `"orange"`. The `PgColor` native class validates the name against its built-in lookup table and resolves it to RGBA values at compile time. The `[.] <<` notation assigns all fields from the pipeline output at once.

## Overload Resolution

The three overloads are distinguished by the `#` prefix and string length:

| Overload | Distinguishing Feature |
|---|---|
| Hex RGB `"#{hex}"` (6-digit) | Starts with `#`, exactly 6 hex digits |
| Hex RGBA `"#{hex}"` (8-digit) | Starts with `#`, exactly 8 hex digits |
| Named `"{name}"` | No `#` prefix, lowercase letters only |

Resolution order: hex overloads first (anchored by `#` literal prefix), then named. Hex 8-digit vs 6-digit distinguished by `{6}` vs `{8}` regex quantifier.

## Usage

```aljam3
[ ] compile-time guaranteed — no error handling
[-] $primary << $Color"#3366CC"
[-] $transparent << $Color"#FF000080"
[-] $alert << $Color"red"
[-] $bg << $Color"white"

[ ] for dynamic strings, use -Color.Parse with error handling
[-] $theme#color << -Color.Parse
   (<) <raw#string << $userColor
   [!] !Parse.Color.InvalidFormat
      [-] $theme << $Color"black"
   [!] !Parse.Color.UnknownName
      [-] $theme << $Color"black"
```

## Related

- [[constructors/INDEX|pglib Constructors]] -- constructor registry
- [[syntax/constructors]] -- `{$}` block specification
- [[pglib/pipelines/Color.Parse|-Color.Parse]] -- runtime color string parsing
- [[pglib/types/Color|#Color type]] -- color value type definition
