---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:RT.<Lang>.Bind.Inline"
metadata_instance: "%-:RT.<Lang>.Bind.Inline:N"
---

# -RT.\<Lang\>.Bind.Inline

Native code imports the polyglot lib and calls `pull()`/`push()` to interact with Polyglot IO ports. Inline variant.

## Definition

```polyglot
{N} -RT.<Lang>.Bind.Inline
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RtBindInline"
   [%] .description << "Native code imports polyglot lib and calls pull()/push(). Inline variant."
   (-) <env#<Lang>Env
   (-) >output#Code:<Lang>.Output
   (-) <code#string
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<env` | `#<Lang>Env` | Runtime environment from `-W.RT` |
| `<code` | `#string` | Inline code via `[C]` blocks |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |

## Notes

Foreign-code-controlled binding — native code uses `pull()`/`push()`. The compiler cannot validate these — they are opaque runtime strings.

## Example

```polyglot
[-] -RT.Python.Bind.Inline
   (-) <env#PyEnv << $pyenv
   (-) >output#Code:Python.Output >> >inlineResult
   (-) <code#string <<
      [C] from polyglot import pull, push
      [C] data = pull("input_data")
      [C] push("result", data.upper())
```

## Errors

None.

## Permissions

Requires `System.Process` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:RT.<Lang>.Bind.Inline` | Compile-time pipeline template |
| Instance | `%-:RT.<Lang>.Bind.Inline:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/RT/INDEX|-RT.* Runtime Execution]]
