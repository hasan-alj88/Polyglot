---
audience: user
type: specification
updated: 2026-03-20
status: draft
---

# =W — Wrappers

Wrappers are placed on `[W]` lines. Each wrapper references a Macro (`{M}`) that provides setup (`[\]`) and cleanup (`[/]`) around the pipeline execution body. Macro IO (`[{]` input, `[}]` output) is wired at the `[W]` line using `[=]` with `$` variables.

Execution order: `[t],[=]` -> `[Q]` -> `[\]` -> Body -> `[/]`

No `[@]` import needed.

```
=W
   .Polyglot
      [ ] Pure Polyglot Code — no external runtime, no setup/cleanup.
      [ ] Calls =DoNothing for both [\] and [/].
      [ ] Every pipeline requires [W]; this no-op confirms intent.

   .DB
      .Connection
         [{] $connectionString;string
         [}] $dbConn
         [ ] Opens DB connection on setup, closes on cleanup.
      .Transaction
         [{] $connectionString;string
         [}] $tx
         [}] $dbConn
         [ ] Opens connection + begins transaction on setup.
         [ ] Commits transaction + closes connection on cleanup.
         [ ] If execution body errors, transaction rolls back.

   .File
      .Lock
         [{] $lockPath;path
         [}] $lock
         [ ] Acquires file lock on setup, releases on cleanup.
      .TempDir
         [{] $prefix;string
         [}] $tempDir
         [ ] Creates temp directory on setup, deletes on cleanup.

   .HTTP
      .Session
         [{] $baseUrl;string
         [{] $timeout;int
         [}] $httpClient
         [ ] Creates HTTP client on setup, closes on cleanup.

   .SSH
      .Session
         [{] $host;string
         [{] $credentials;string
         [}] $sshSession
         [ ] Connects SSH on setup, disconnects on cleanup.

   .Auth
      .Token
         [{] $clientId;string
         [{] $clientSecret;string
         [}] $authToken
         [ ] Acquires OAuth token on setup, revokes on cleanup.

   .Log
      .Context
         [{] $traceId;string
         [}] $logScope
         [ ] Opens structured log scope on setup, closes on cleanup.

   .Queue
      .Consumer
         [{] $queueUrl;string
         [}] $consumer
         [ ] Connects to message queue on setup, disconnects on cleanup.

   .Cache
      .Scope
         [{] $cacheUrl;string
         [}] $cache
         [ ] Connects cache on setup, flushes + disconnects on cleanup.

   .Python
      [}] $pyRuntime;PyRT
      [ ] Starts Python runtime on setup, stops on cleanup.
   .Rust (?)
      [ ] Starts Rust runtime on setup, stops on cleanup. (?)
   .Node (?)
      [ ] Starts Node.js runtime on setup, stops on cleanup. (?)
```

NOTE: Retry/timeout/rate-limiting are `[Q]` queue strategies, not wrappers.
