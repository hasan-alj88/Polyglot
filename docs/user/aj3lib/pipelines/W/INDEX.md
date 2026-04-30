---
audience: automation-builder
type: specification
updated: 2026-04-10
status: complete
---

# -W — Wrappers

<!-- @c:pipelines -->
Wrappers are placed on `[W]` lines. Each wrapper is a `{W}` definition that provides setup (`[\]`) and cleanup (`[/]`) around the pipeline execution body. Wrapper IO (`(-)` with `<` input and `>` output) is wired at the `[W]` line using `(-)` with `$` variables. See [[concepts/pipelines/wrappers#Wrappers]] for wrapper usage rules.

Execution order: `(-),[T]` -> `[Q]` -> `[\]` -> Body -> `[/]`

No `[@]` import needed.

**PRIMITIVE** — aj3lib wrappers are direct OS/runtime integrations. They are implemented by the Aljam3 runtime and cannot be reimplemented in user `.aj3` files. User-defined wrappers are created as `{W}` definitions and referenced on `[W]` lines — see [[concepts/pipelines/wrappers#Wrappers]].

## Permissions

<!-- @c:permissions -->
Wrappers that manage external resources require a `{_}` permission object granting the listed capabilities. See [[permissions]] for the permission system and [[errors#Built-in Error Namespaces]] for `!Permission.*` errors.

| Pipeline | Required Capability | Category |
|----------|-------------------|----------|
| `-W.Aljam3` | None | — |
| `-W.DB.Connection` | Database.Connect | Database |
| `-W.DB.Transaction` | Database.Connect | Database |
| `-W.File.Lock` | File.Read | File |
| `-W.File.TempDir` | File.Write | File |
| `-W.HTTP.Session` | Web.Request | Web |
| `-W.SSH.Session` | System.Process | System |
| `-W.Auth.Token` | Crypto.Key | Crypto |
| `-W.Log.Context` | None | — |
| `-W.Queue.Consumer` | IPC.Receive | IPC |
| `-W.Cache.Scope` | Database.Read | Database |
| `-W.Env` | System.Process | System |
| `-W.RT` *(deprecated)* | System.Process | System |

## Pipeline Listing

### No-Op

| Pipeline | Description |
|----------|-------------|
| [[aj3lib/pipelines/W/Aljam3\|-W.Aljam3]] | Pure Aljam3 Code — no external runtime, no setup/cleanup |

### Database

| Pipeline | Description |
|----------|-------------|
| [[aj3lib/pipelines/W/DB.Connection\|-W.DB.Connection]] | Opens DB connection on setup, closes on cleanup |
| [[aj3lib/pipelines/W/DB.Transaction\|-W.DB.Transaction]] | Opens connection + begins transaction on setup, commits + closes on cleanup |

### File System

| Pipeline | Description |
|----------|-------------|
| [[aj3lib/pipelines/W/File.Lock\|-W.File.Lock]] | Acquires file lock on setup, releases on cleanup |
| [[aj3lib/pipelines/W/File.TempDir\|-W.File.TempDir]] | Creates temp directory on setup, deletes on cleanup |

### Network

| Pipeline | Description |
|----------|-------------|
| [[aj3lib/pipelines/W/HTTP.Session\|-W.HTTP.Session]] | Creates HTTP client on setup, closes on cleanup |
| [[aj3lib/pipelines/W/SSH.Session\|-W.SSH.Session]] | Connects SSH on setup, disconnects on cleanup |

### Security

| Pipeline | Description |
|----------|-------------|
| [[aj3lib/pipelines/W/Auth.Token\|-W.Auth.Token]] | Acquires OAuth token on setup, revokes on cleanup |

### Observability

| Pipeline | Description |
|----------|-------------|
| [[aj3lib/pipelines/W/Log.Context\|-W.Log.Context]] | Opens structured log scope on setup, closes on cleanup |

### Messaging

| Pipeline | Description |
|----------|-------------|
| [[aj3lib/pipelines/W/Queue.Consumer\|-W.Queue.Consumer]] | Connects to message queue on setup, disconnects on cleanup |

### Caching

| Pipeline | Description |
|----------|-------------|
| [[aj3lib/pipelines/W/Cache.Scope\|-W.Cache.Scope]] | Connects cache on setup, flushes + disconnects on cleanup |

### Environment

| Pipeline | Description |
|----------|-------------|
| [[aj3lib/pipelines/W/Env\|-W.Env]] | Sets up language environment on setup, tears down on cleanup |

### Runtime (Deprecated)

| Pipeline | Description |
|----------|-------------|
| [[aj3lib/pipelines/W/RT\|-W.RT]] | **Deprecated** — use [[aj3lib/pipelines/W/Env\|-W.Env]]. Starts language runtime on setup, stops on cleanup |

NOTE: Retry/timeout/rate-limiting are `[Q]` queue strategies, not wrappers.

## Related

- [[concepts/pipelines/wrappers]]
- [[aj3lib/INDEX]]
