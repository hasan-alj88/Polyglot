---
audience: automation-builder
type: specification
updated: 2026-04-22
status: complete
metadata_definition: "%definition.-:Re.Parse"
metadata_instance: "%-:Re.Parse:N"
---

# -Re.Parse

Parses a dynamic string into a `#re` value at runtime. Use this when the input comes from IO or other runtime sources where the compiler cannot prove validity at compile time.

For known literals in execution body, use the `$Re` constructor instead — it guarantees no error surface.

## Definition

```polyglot
{N} -Re.Parse
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "ReParse"
   [%] .description << "Parse dynamic string to regex pattern"
   (-) <raw#string
   (-) >re#re
```

## Inputs

| Name | Type | Default | Description |
|------|------|---------|-------------|
| `<raw` | `#string` | --- | String to parse as a regex pattern |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>re` | `#re` | Parsed regex pattern value |

## Errors

| Error | Description |
|-------|-------------|
| `!Parse.Re.InvalidSyntax` | Input string is not syntactically valid regex (unmatched brackets, invalid quantifiers, etc.) |

## Permissions

None required. Pure computation pipeline.

## Usage

```polyglot
[-] $pattern#re << -Re.Parse
   (<) <raw#string << $userInput
   [!] !Parse.Re.InvalidSyntax
      [-] $pattern << $Re".*"
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Re.Parse` | Compile-time pipeline template |
| Instance | `%-:Re.Parse:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/constructors/Re|$Re constructor]] -- compile-time regex construction
- [[pglib/types/Re|#Re type]] -- regex pattern type definition
