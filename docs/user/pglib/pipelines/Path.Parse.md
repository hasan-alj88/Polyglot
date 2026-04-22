---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.-:Path.Parse"
metadata_instance: "%-:Path.Parse:N"
---

# -Path.Parse

Parses a dynamic string into a `#path` value at runtime. Use this when the input comes from IO or other runtime sources where the compiler cannot prove validity at compile time.

For known literals in execution body, use the `$Path` constructor instead — it guarantees no error surface.

## Definition

```polyglot
{N} -Path.Parse
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "PathParse"
   [%] .description << "Parse dynamic string to path"
   (-) <raw#string
   (-) >path#path
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<raw` | `#string` | --- | String to parse as a file system path |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>path` | `#path` | Parsed path value with `.Unix` and `.Windows` fields |

## Errors

| Error | Description |
|-------|-------------|
| `!Parse.Path.InvalidCharacter` | Input contains characters not valid in file paths (e.g., null bytes) |
| `!Parse.Path.Empty` | Input is an empty string |

## Permissions

None required. Pure computation pipeline.

## Usage

```polyglot
[-] $parsed#path << -Path.Parse
   (<) <raw#string << $userInput
   [!] !Parse.Path.InvalidCharacter
      [-] $parsed << $Path"."
   [!] !Parse.Path.Empty
      [-] $parsed << $Path"."
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Path.Parse` | Compile-time pipeline template |
| Instance | `%-:Path.Parse:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/Path|-Path pipeline]] -- Path creation pipeline
- [[pglib/constructors/Path|$Path constructor]] -- compile-time path construction
- [[pglib/types/path|#path type]] -- cross-platform path struct
