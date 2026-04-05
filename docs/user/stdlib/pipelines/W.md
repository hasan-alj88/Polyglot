---
audience: pg-coder
type: specification
updated: 2026-03-31
status: complete
---

# =W — Wrappers

<!-- @pipelines -->
Wrappers are placed on `[W]` lines. Each wrapper is a `{W}` definition that provides setup (`[\]`) and cleanup (`[/]`) around the pipeline execution body. Wrapper IO (`[{]` input, `[}]` output) is wired at the `[W]` line using `[=]` with `$` variables. See [[concepts/pipelines/wrappers#Wrappers]] for wrapper usage rules.

Execution order: `[T],[=]` -> `[Q]` -> `[\]` -> Body -> `[/]`

No `[@]` import needed.

**PRIMITIVE** — Stdlib wrappers are direct OS/runtime integrations. They are implemented by the Polyglot runtime and cannot be reimplemented in user `.pg` files. User-defined wrappers are created as `{W}` definitions and referenced on `[W]` lines — see [[concepts/pipelines/wrappers#Wrappers]].

## Permissions

<!-- @permissions -->
Wrappers that manage external resources require a `{_}` permission object granting the listed capabilities. See [[permissions]] for the permission system and [[errors#Built-in Error Namespaces]] for `!Permission.*` errors.

| Pipeline | Required Capability | Category |
|----------|-------------------|----------|
| `=W.Polyglot` | None | — |
| `=W.DB.Connection` | Database.Connect | Database |
| `=W.DB.Transaction` | Database.Connect | Database |
| `=W.File.Lock` | File.Read | File |
| `=W.File.TempDir` | File.Write | File |
| `=W.HTTP.Session` | Web.Request | Web |
| `=W.SSH.Session` | System.Process | System |
| `=W.Auth.Token` | Crypto.Key | Crypto |
| `=W.Log.Context` | None | — |
| `=W.Queue.Consumer` | IPC.Receive | IPC |
| `=W.Cache.Scope` | Database.Read | Database |
| `=W.RT` | System.Process | System |

```polyglot
=W
   .Polyglot
      [ ] Pure Polyglot Code — no external runtime, no setup/cleanup.
      [ ] Calls =DoNothing for both [\] and [/].
      [ ] Every pipeline requires [W]; this no-op confirms intent.

   .DB
      .Connection
         [{] $connectionString#string
         [}] $dbConn
         [ ] Opens DB connection on setup, closes on cleanup.
      .Transaction
         [{] $connectionString#string
         [}] $tx
         [}] $dbConn
         [ ] Opens connection + begins transaction on setup.
         [ ] Commits transaction + closes connection on cleanup.
         [ ] If execution body errors, transaction rolls back.

   .File
      .Lock
         [{] $lockPath#path
         [}] $lock
         [ ] Acquires file lock on setup, releases on cleanup.
      .TempDir
         [{] $prefix#string
         [}] $tempDir
         [ ] Creates temp directory on setup, deletes on cleanup.

   .HTTP
      .Session
         [{] $baseUrl#string
         [{] $timeout#int
         [}] $httpClient
         [ ] Creates HTTP client on setup, closes on cleanup.

   .SSH
      .Session
         [{] $host#string
         [{] $credentials#string
         [}] $sshSession
         [ ] Connects SSH on setup, disconnects on cleanup.

   .Auth
      .Token
         [{] $clientId#string
         [{] $clientSecret#string
         [}] $authToken
         [ ] Acquires OAuth token on setup, revokes on cleanup.

   .Log
      .Context
         [{] $traceId#string
         [}] $logScope
         [ ] Opens structured log scope on setup, closes on cleanup.

   .Queue
      .Consumer
         [{] $queueUrl#string
         [}] $consumer
         [ ] Connects to message queue on setup, disconnects on cleanup.

   .Cache
      .Scope
         [{] $cacheUrl#string
         [}] $cache
         [ ] Connects cache on setup, flushes + disconnects on cleanup.

   .RT
      :Python
         :3
            :14
               [}] $pyenv#PyEnv
               [ ] Starts Python 3.14 runtime on setup, stops on cleanup.
      :Rust
         :1
            :84
               [}] $rsenv#RsEnv
               [ ] Starts Rust 1.84 runtime on setup, stops on cleanup.
```

NOTE: Retry/timeout/rate-limiting are `[Q]` queue strategies, not wrappers.
