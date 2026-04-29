---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
metadata_definition: "%!.Env"
---

# `!Env`

No `[@]` import needed — pglib built-in.

```aljam3
{!} !Env
   [.] .NotFound#Error
      (-) .MessageTemplate << "Environment not found: {name}"
      (-) .Info
         [:] :name#string
   [.] .VersionMismatch#Error
      (-) .MessageTemplate << "Environment {name} version mismatch: expected {expected}, got {actual}"
      (-) .Info
         [:] :name#string
         [:] :expected#string
         [:] :actual#string
   [.] .SetupFailed#Error
      (-) .MessageTemplate << "Environment setup failed: {name}"
      (-) .Info
         [:] :name#string
   [.] .TeardownFailed#Error
      (-) .MessageTemplate << "Environment teardown failed: {name}"
      (-) .Info
         [:] :name#string
   [:] :Dependency
      [.] .Missing#Error
         (-) .MessageTemplate << "Missing dependency: {dependency}"
         (-) .Info
            [:] :dependency#string
      [.] .VersionConflict#Error
         (-) .MessageTemplate << "Dependency {dependency} version conflict: {expected} vs {actual}"
         (-) .Info
            [:] :dependency#string
            [:] :expected#string
            [:] :actual#string
      [.] .InstallFailed#Error
         (-) .MessageTemplate << "Failed to install dependency: {dependency}"
         (-) .Info
            [:] :dependency#string
```

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Tree path | `%!.Env` | Fixed error namespace (no instances) |

See [[metadata-tree/FULL-TREE|Full Metadata Tree]] for the authoritative tree.

See [[pglib/errors/builtin/rt#`!Env` vs `!RT` — Phase Distinction]] for the distinction between `!Env.*` (wrapper setup) and `!RT.*` (body execution) errors.
