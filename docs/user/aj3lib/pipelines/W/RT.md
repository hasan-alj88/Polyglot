---
audience: automation-builder
type: specification
updated: 2026-04-10
status: deprecated
metadata_definition: "%definition.W:RT"
metadata_instance: "%W:RT:N"
---

# -W.RT (Deprecated)

> **Deprecated:** Use [[aj3lib/pipelines/W/Env|-W.Env]] with `{;}` environment definitions instead. `-W.RT` and its version tree syntax are retired. See [[environments]] for the replacement system.

Starts language runtime on setup, stops on cleanup. Uses version-specific paths (e.g., `-W.RT:Python:3:14`).

## Definition

```aljam3
{N} -W.RT
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WRt"
   [%] .description << "Starts language runtime on setup, stops on cleanup."
```

## Version Tree

Runtime versions are selected via colon-separated paths:

```aljam3
-W.RT
   :Python
      :3
         :14
            (-) >pyenv;PyEnv
            [ ] Starts Python 3.14 runtime on setup, stops on cleanup.
   :Rust
      :1
         :84
            (-) >rsenv;RsEnv
            [ ] Starts Rust 1.84 runtime on setup, stops on cleanup.
```

## Inputs

None.

## Outputs

Outputs depend on the selected runtime version:

| Version Path | Name | Type | Description |
|-------------|------|------|-------------|
| `:Python:3:14` | `$pyenv` | `#PyEnv` | Python 3.14 environment handle |
| `:Rust:1:84` | `$rsenv` | `#RsEnv` | Rust 1.84 environment handle |

## Errors

None.

## Permissions

System.Process

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:RT` | Compile-time pipeline template |
| Instance | `%W:RT:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/W/INDEX|-W.* Wrappers]]
