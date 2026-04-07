---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =RT.\<Lang\>.Bind.File

Native code imports the polyglot lib and calls `pull()`/`push()` to interact with Polyglot IO ports. File variant.

## Definition

```polyglot
{N} =RT.<Lang>.Bind.File
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RtBindFile"
   [%] .description << "Native code imports polyglot lib and calls pull()/push(). File variant."
   [=] <env#<Lang>Env
   [=] >output#Code:<Lang>.Output
   [=] <file#path
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<env` | `#<Lang>Env` | Runtime environment from `=W.RT` |
| `<file` | `#path` | Path to source file |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |

## Example

```polyglot
[r] =RT.Python.Bind.File
   [=] <env#PyEnv << $pyenv
   [=] >output#Code:Python.Output >> >fileResult
   [=] <file#path << =Path"/scripts/transform.py"
```

## Errors

None.

## Permissions

Requires `System.Process` capability.

## Related

- [[pglib/pipelines/RT/INDEX|=RT.* Runtime Execution]]
