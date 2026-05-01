---
audience: automation-builder
type: specification
updated: 2026-04-10
status: draft
---

# Environment Declaration

<!-- @c:environments -->
Every pipeline declares its environment requirement using `(-) ;EnvName`. This is mandatory — the compiler raises PGE01036 if missing. See [[environments]] for `{;}` definition syntax.

```aljam3
(-) <data#serial
(-) >result#string
(-) !File.NotFound
(-) ;MLPythonEnv
```

**Reading order in `(-)`:** inputs (`<`), outputs (`>`), errors (`!`), environments (`;`).

Pure Aljam3 pipelines use `(-) ;Aljam3`. Foreign-environment pipelines reference a user-defined `{;}` environment and wire it through the [[jm3lib/pipelines/W/Env|-W.Env]] wrapper:

```aljam3
{-} =ProcessData
   (-) <data#serial
   (-) >result#string
   (-) !File.NotFound
   (-) ;MLPythonEnv
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Env
      (-) <env#; << ;MLPythonEnv
   [ ]
   [-]
      ...
```

## Multiple Environments

A pipeline can declare multiple `(-) ;` lines when it calls sub-pipelines requiring different environments. Multiple declarations on separate lines are implicitly AND (all required). Use `(+)` on a new line with the same `;` prefix for OR (alternative environments):

```aljam3
(-) ;PythonML
(-) ;RustCore
```

The compiler tracks which environment is active for each call site and validates availability.
