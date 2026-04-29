---
audience: automation-builder
type: specification
updated: 2026-04-17
status: draft
metadata_definition: "%definition.W:Env"
metadata_instance: "%W:Env:N"
---

# -W.Env

Sets up a language environment on setup, tears it down on cleanup. Replaces the deprecated [[pglib/pipelines/W/RT|-W.Env]] wrapper.

The wrapper receives a `{;}` environment definition as input and handles all platform-level setup: runtime installation, dependency resolution, virtual environment creation, and environment variable injection. See [[environments]] for the `{;}` definition syntax.

## Definition

```polyglot
{N} -W.Env
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WEnv"
   [%] .description << "Sets up language environment on setup, tears it down on cleanup."
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<env` | `#;` | Environment definition reference — a `{;}` block |

## Outputs

None. The wrapper's job is to set up the environment for the duration of the pipeline call, not to produce a handle. The compiler tracks active environments through the call graph.

## Usage

Wire the `{;}` definition to the wrapper on the `[W]` line using inline `;EnvName` syntax:

```polyglot
[W] -W.Env;MLPythonEnv
```

This is the preferred shorthand. The environment name is appended directly to the wrapper reference with the `;` prefix. The compiler resolves `;MLPythonEnv` to the matching `{;}` definition.

### Verbose Form

The equivalent verbose form wires the environment via `(-)` IO:

```polyglot
[W] -W.Env
   (-) <env#; << ;MLPythonEnv
```

The `#;` type represents a reference to a `{;}` environment definition. See [[environments#The #; Type]].

### Full Pipeline Example

```polyglot
{-} =ProcessData
   (-) <data#serial
   (-) >result#string
   (-) ;MLPythonEnv
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Env;MLPythonEnv
   [ ]
   [-] ...
```

## Errors

```polyglot
-W.Env
   [=] !Env.NotFound
   [=] !Env.VersionMismatch
   [=] !Env.SetupFailed
   [=] !Env.TeardownFailed
   [=] !Env.Dependency.Missing
   [=] !Env.Dependency.VersionConflict
   [=] !Env.Dependency.InstallFailed
```

See [[pglib/errors/errors#Built-in Error Namespaces]] for the full `!Env` error tree.

### Error Handling at Call Site

Callers handle environment errors using `[!]` blocks or `(>) !>` fallbacks:

```polyglot
[-]
   [-] -SomePipeline
      (-) <file << $File
      (-) >result >> $Result
      [!] !Env.NotFound
         [-] >result << "env unavailable"
      [!] !Env.Dependency.Missing
         [-] >result << "missing deps"
```

## Permissions

System.Process

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:Env` | Compile-time pipeline template |
| Instance | `%W:Env:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[environments]] — `{;}` environment definition syntax
- [[pglib/pipelines/W/INDEX|-W.* Wrappers]]
- [[pglib/pipelines/W/RT|-W.Env]] (deprecated — use `-W.Env`)
- [[pglib/errors/errors#Built-in Error Namespaces]] — `!Env.*` errors
