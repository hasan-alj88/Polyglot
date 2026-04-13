---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.-:RT.<Lang>.Script.Inline"
metadata_instance: "%-:RT.<Lang>.Script.Inline:N"
---

# -RT.\<Lang\>.Script.Inline

Run inline code with variable bindings.

## Definition

```polyglot
{N} -RT.<Lang>.Script.Inline
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RtScriptInline"
   [%] .description << "Run inline code with variable bindings."
   (-) <env#<Lang>Env
   (-) <Bind#serial
   (-) >Bind#serial
   (-) >output#Code:<Lang>.Output
   (-) <code#string
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<env` | `#<Lang>Env` | Runtime environment from `-W.RT` |
| `<Bind` | `#serial` | Variable bindings injected as code variables (optional) |
| `<code` | `#string` | Inline code via `[C]` blocks |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Bind` | `#serial` | Final state of bound variables (optional) |
| `>output` | `#Code:<Lang>.Output` | `.stdout`, `.stderr` capture |

## Compiler Validation

The compiler validates that `<Bind` variable names exist as identifiers in the code.

## Script vs Bind

Polyglot-controlled binding — `<Bind` injects vars, `>Bind` captures final state. Compare with `.Bind` modes where foreign code controls data flow via `pull()`/`push()`.

## Example

```polyglot
[-] -RT.Python.Script.Inline
   (-) <env#PyEnv << $pyenv
   (-) <Bind#serial << {"target_dir": $targetDir, "deleted_count": 0}
   (-) >output#Code:Python.Output >> >log
   (-) >Bind#serial >> >state
   (-) <code#string <<
      [C] import os, glob
      [C] files = glob.glob(target_dir + "/stale_*.log")
      [C] for f in files:
      [C]     os.remove(f)
      [C] deleted_count = len(files)
```

## Errors

None.

## Permissions

Requires `System.Process` capability.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:RT.<Lang>.Script.Inline` | Compile-time pipeline template |
| Instance | `%-:RT.<Lang>.Script.Inline:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/RT/INDEX|-RT.* Runtime Execution]]
