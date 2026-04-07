---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =RT.\<Lang\>.Script.File

Run a source file with variable bindings.

## Definition

```polyglot
{N} =RT.<Lang>.Script.File
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RtScriptFile"
   [%] .description << "Run a source file with variable bindings."
   [=] <env#<Lang>Env
   [=] <Bind#serial
   [=] >Bind#serial
   [=] >output#Code:<Lang>.Output
   [=] <file#path
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<env` | `#<Lang>Env` | Runtime environment from `=W.RT` |
| `<Bind` | `#serial` | Variable bindings (optional) |
| `<file` | `#path` | Path to source file |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Bind` | `#serial` | Final state of bound variables (optional) |
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |

## Compiler Validation

The compiler validates that `<Bind` variable names exist as identifiers in the source file.

## Example

```polyglot
[r] =RT.Python.Script.File
   [=] <env#PyEnv << $pyenv
   [=] <Bind#serial << {"target_dir": $targetDir}
   [=] >output#Code:Python.Output >> >log
   [=] >Bind#serial >> >state
   [=] <file#path << =Path"/scripts/cleanup.py"
```

## Errors

None.

## Permissions

Requires `System.Process` capability.

## Related

- [[pglib/pipelines/RT/INDEX|=RT.* Runtime Execution]]
