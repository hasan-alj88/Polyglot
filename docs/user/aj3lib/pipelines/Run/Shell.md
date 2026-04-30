---
audience: automation-builder
type: specification
updated: 2026-04-16
status: draft
metadata_definition: "%definition.-:Run.Shell"
metadata_instance: "%-:Run.Shell:N"
---

# -Run.Shell

Execute a shell command string through the system shell (`sh -c "..."`). Unlike `-Run.<Lang>.CLI` which invokes a specific compiled binary with structured arguments, `-Run.Shell` runs arbitrary shell expressions including pipes, redirections, and compound commands.

No `[@]` import needed.

**PRIMITIVE** — aj3lib runtime pipeline implemented by the Aljam3 runtime.

> **Language-agnostic:** `-Run.Shell` has no `<Lang>` placeholder. It delegates to the system shell, not a language-specific runtime. No `-W.Env` environment is needed — uses `-W.Aljam3`.

## Definition

```aljam3
{N} -Run.Shell
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "RunShell"
   [%] .description << "Execute a shell command string."
   (-) <command#string
   (-) <workdir#path
   (-) >exitCode#int
   (-) >stdout#string
   (-) >stderr#string
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<command` | `#string` | Shell command string — passed to `sh -c` for interpretation |
| `<workdir` | `#path` | Working directory for the shell process |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>exitCode` | `#int` | Process exit code (0 = success) |
| `>stdout` | `#string` | Captured standard output |
| `>stderr` | `#string` | Captured standard error |

## Shell vs CLI

| Aspect | `-Run.Shell` | `-Run.<Lang>.CLI` |
|--------|-------------|-------------------|
| Invokes | System shell (`sh -c`) | Specific binary by path |
| Arguments | Single command string | Structured `<arg`/`<kwarg` Records |
| Shell features | Pipes, redirections, `&&`, `\|\|`, globbing | None — binary receives literal strings |
| Binding | No `<Bind`/`>Bind` — uses exit code + stdout/stderr | `<arg`/`<kwarg` Record binding + `>Bind` |
| Validation | None — command is opaque string | `<arg`/`<kwarg` fields must be `#string` (PGE01039) |
| Permission | System.Process + System.Shell | System.Process |

**When to use Shell:** Running shell pipelines (`git log --oneline \| head -5`), compound commands (`mkdir -p dir && cd dir && init`), or commands with redirections.

**When to use CLI:** Invoking a known binary with structured, type-checked arguments.

## Compiler Validation

No code or binding validation applies to `-Run.Shell`:

- `<command` is an opaque `#string` — the compiler cannot inspect shell syntax
- No `<Bind`/`<arg`/`<kwarg` Records — PGE01033–PGE01039 do not apply
- The compiler validates only that IO types match (`#string`, `#path`, `#int`)

## Example

```aljam3
{_} _ShellCeiling
   [.] .intent << #Ceiling
   [.] .System.Process "*"
   [.] .System.Shell "*"

{@} @Local:Example.GitStatus
   (-) _ShellCeiling

{_} _ShellGrant
   [.] .intent << #Grant
   [.] .System.Process "sh"
   [.] .System.Shell "*"

{#} #ShellResult
   [.] .exitCode#int
   [.] .stdout#string
   [.] .stderr#string

{-} =GetGitStatus
   (-) _ShellGrant
   (-) <repoPath#path
   (-) >result#ShellResult
   [T] -T.Call
   [Q] -Q.Default
   [W] -W.Aljam3

   [ ]
   [-] -Run.Shell
      (-) <command#string << "git status --porcelain"
      (-) <workdir#path << $repoPath
      (-) >exitCode#int >> >result.exitCode
      (-) >stdout#string >> >result.stdout
      (-) >stderr#string >> >result.stderr
```

## Permissions

Requires **both** `System.Process` and `System.Shell` capabilities. Shell execution is a higher privilege than invoking a known binary — the command string can run arbitrary programs, chain commands, and access shell built-ins.

| Capability | Scope | Why |
|------------|-------|-----|
| `System.Process` | Process identifier (e.g., `"sh"`) | Spawns a shell process |
| `System.Shell` | Command scope (e.g., `"*"`, `"git *"`) | Permits shell interpretation of command strings |

## Errors

Shell-specific runtime errors use the `!Run` namespace:

| Error | Condition |
|-------|-----------|
| `!Run.Shell.NotFound` | System shell (`sh`) not available |
| `!Run.Shell.PermissionDenied` | Shell execution blocked by permission system |
| `!Run.Shell.Timeout` | Command exceeded queue timeout |

No binding compiler errors apply (PGE01033–PGE01039 are irrelevant — no `<Bind`, `<arg`, `<kwarg`).

## Process Lifecycle

When `-Run.Shell` spawns a shell process, the runtime writes process information to the runtime data store (Redis) keyed by job UID. This allows any Queue Handler node to manage the process across the cluster.

**On spawn:**

| Redis Key | Field | Value |
|-----------|-------|-------|
| `job:{UID}:process` | `pid` | OS process ID |
| | `command` | The `<command` string |
| | `workdir` | The `<workdir` path |
| | `startTime` | Epoch timestamp |
| | `status` | `running` |

**On completion:** The runtime updates `status` to `exited` and writes `exitCode`. The key expires after the job's retention window.

**QH process control:** When the Queue Handler receives a management command (`-Q.Kill.*`, `-Q.Pause.*`, `-Q.Resume`), it reads `job:{UID}:process` from Redis to resolve the PID and host, then sends the appropriate OS signal:

| QH Operation | Signal | Effect |
|---|---|---|
| `-Q.Kill.Hard` | `SIGKILL` | Immediate termination |
| `-Q.Kill.Graceful` | `SIGTERM` + timeout | Graceful shutdown, then `SIGKILL` |
| `-Q.Pause.Hard` | `SIGSTOP` | Freeze process |
| `-Q.Resume` | `SIGCONT` | Resume frozen process |

This is the same process registration pattern used by `-Run.<Lang>.CLI` — any native function that spawns a child process writes to `job:{UID}:process` so the QH can act on it.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.-:Run.Shell` | Compile-time pipeline template |
| Instance | `%-:Run.Shell:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[aj3lib/pipelines/Run/INDEX|-Run.* Foreign Code Execution]]
- [[aj3lib/pipelines/Run/CLI|-Run.\<Lang\>.CLI]] -- structured binary invocation (no shell)
- [[aj3lib/pipelines/W/Aljam3|-W.Aljam3]] -- wrapper for non-runtime execution
- [[aj3lib/pipelines/T/Git.Hook|-T.Git.Hook]] -- local Git hook trigger (depends on shell execution)
