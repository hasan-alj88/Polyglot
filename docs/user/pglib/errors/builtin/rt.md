---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.RT"
---

# `!RT`

No `[@]` import needed — pglib built-in.

```aljam3
{!} !RT
   [.] .CompileError#Error
      (-) .MessageTemplate << "Compile error in {language}: {reason}"
      (-) .Info
         [:] :language#string
         [:] :reason#string
   [.] .RuntimeError#Error
      (-) .MessageTemplate << "Runtime error in {language}: {reason}"
      (-) .Info
         [:] :language#string
         [:] :reason#string
   [.] .Timeout#Error
      (-) .MessageTemplate << "Execution timed out after {duration}"
      (-) .Info
         [:] :duration#string
   [.] .EnvironmentError#Error
      (-) .MessageTemplate << "Environment error in {language}: {reason}"
      (-) .Info
         [:] :language#string
         [:] :reason#string
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.RT` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.

## `!Env` vs `!RT` — Phase Distinction

| Namespace | Phase | Scope |
|-----------|-------|-------|
| `!Env.*` | `[W]` wrapper setup | Environment availability + dependency resolution |
| `!RT.*` | `[-]` body execution | Foreign code compile/runtime errors |

`!Env.*` errors fire during [[pglib/pipelines/W/Env|-W.Env]] wrapper setup when the environment cannot be established. `!RT.EnvironmentError` fires during execution when the foreign runtime encounters an environment issue within already-running code. Both coexist — they cover different execution phases. See [[environments]] for the `{;}` environment system.
