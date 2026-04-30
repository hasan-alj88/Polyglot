---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.#:Ver"
metadata_instance: "%#:Ver:N"
---

# #Ver — Semantic Version

<!-- @c:types -->
<!-- @c:aj3lib/types/string -->

`#Ver` is a aj3lib type representing a semantic version (SemVer 2.0). It decomposes a version string into its major, minor, patch, prerelease, and build metadata components, enabling type-safe version comparison and manipulation.

See [[scalars]] for scalar subtypes and [[syntax/constructors]] for the `$Ver` constructor.

## Definition

```aljam3
{#} #Ver
   [%] .description << "Semantic version (SemVer 2.0)"
   [#] ##Scalar
   [#] ###ScalarValue
   [#] %##Alias << "ver"
   [.] .major#int
   [.] .minor#int
   [.] .patch#int
   [.] .prerelease#string
      <~ ""
   [.] .build#string
      <~ ""
```

## Fields

| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| `.major` | `#int` | (none) | Major version number |
| `.minor` | `#int` | (none) | Minor version number |
| `.patch` | `#int` | (none) | Patch version number |
| `.prerelease` | `#string` | `""` | Prerelease label (e.g., `"alpha.1"`, `"rc.2"`) |
| `.build` | `#string` | `""` | Build metadata (e.g., `"20260422"`, `"sha.abc123"`) |

## Schema Properties

- `[#] ##Scalar` -- sets `%##Depth.Max << 1`, marking `#Ver` as a scalar type
- `[#] ###ScalarValue` -- single-value scalar (no enum variants)
- `%##Alias << "ver"` -- lets users write `#ver` (lowercase) as shorthand

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.#:Ver` | Compile-time type template |
| Instance | `%#:Ver:N` | Runtime instance (N = instance number) |

## Related

- [[aj3lib/constructors/Ver\|$Ver constructor]] -- compile-time version construction
- [[aj3lib/pipelines/Ver.Parse\|-Ver.Parse]] -- runtime version string parsing
- [[scalars]] -- scalar subtypes overview
- [[string]] -- `#String` foundation type
- [[syntax/types/INDEX\|types]] -- full type system specification
