---
audience: developer
rule: "1.4"
code: PGE01004
name: Wrapper Structural Constraints
severity: error
---

### Rule 1.4 — Wrapper Structural Constraints
`PGE01004`

**Statement:** A `{W}` wrapper definition is restricted to setup/cleanup lifecycle elements only. The complete element whitelist:

| Element | Allowed | Scope |
|---------|---------|-------|
| `(-)` IO | Yes | Wrapper IO — inputs (`<`) and outputs (`>`) |
| `[\]` / `[/]` | Yes | Setup and cleanup scopes |
| `[-]` | Yes | Pipeline calls within `[\]` or `[/]` |
| `[W]` | Yes | Composite wrapper calls within `[\]` or `[/]` |
| `[?]` | Yes | Conditionals within `[\]` or `[/]` — for branching setup logic |
| `[!]` | Yes | Error handlers scoped under `[-]` calls within `[\]` or `[/]` |
| `[=]` | Yes | Parallel fork within `[\]` — runs concurrently with execution body |
| `[b]` | Yes | Fire-and-forget within `[\]` — no collection possible |
| `(*)` | Yes | Collectors within `[/]` — collect `[=]` forks started in `[\]` |
| `[T]` | **No** | Triggers are pipeline-only |
| `[Q]` | **No** | Queues are pipeline-only |
| `(-)` pipeline IO | **No** | Pipeline-level IO (with `[T]`/`[Q]`) is pipeline-only |

**Rationale:** Wrappers exist to encapsulate reusable setup/cleanup logic around pipeline execution. Allowing triggers or queues inside a wrapper would make it behave like a pipeline, breaking the structural separation between lifecycle wrappers and executable pipelines. Parallel forking (`[=]`, `[b]`) is allowed in `[\]` setup because setup may need to launch concurrent work that outlives setup and runs alongside the execution body — `(*) *All` in `[/]` cleanup collects the results. See [[concepts/pipelines/wrappers#Parallel Forking in Setup]].

**VALID:**
```polyglot
[ ] ✓ wrapper with (-) IO, [\]/[/] scopes, and composite [W] inside
{W} -W.DB.Transaction
   (-) <connStr;string
   (-) >txHandle;string

   [\]
      [ ] ✓ composite wrapper inside wrapper setup
      [W] -W.DB.Connection
         (-) $connStr << $connStr
         (-) $conn >> $conn
      [-] -DB.BeginTransaction
         (-) <conn << $conn
         (-) >handle >> $txHandle

   [/]
      [-] -DB.Commit
         (-) <handle << $txHandle
```

```polyglot
[ ] ✓ conditional inside wrapper setup — branching on input
{W} -W.Cache.Init
   (-) <backend;string
   (-) >cacheHandle;string

   [\]
      [?] $backend
         [?] =? "redis"
            [-] -Cache.Redis.Connect
               (-) >handle >> $cacheHandle
         [?] =? "memory"
            [-] -Cache.Memory.Init
               (-) >handle >> $cacheHandle
         [?] *?
            [-] -Cache.Default.Init
               (-) >handle >> $cacheHandle

   [/]
      [-] -Cache.Disconnect
         (-) <handle << $cacheHandle
```

```polyglot
[ ] ✓ error handler inside wrapper setup
{W} -W.Service.Init
   (-) <config;string
   (-) >serviceHandle;string

   [\]
      [-] -Service.Connect
         (-) <config << $config
         (-) >handle >> $serviceHandle
      [!] !Connect.Failed
         [-] -Log.Error
            (-) <msg << "Service connection failed"

   [/]
      [-] -Service.Disconnect
         (-) <handle << $serviceHandle
```

```polyglot
[ ] ✓ parallel fork in setup, collected in cleanup
{W} -W.Tracing
   (-) <traceId;string
   (-) >duration;string

   [\]
      [-] -Tracer.Open
         (-) <id << $traceId
         (-) >session >> $session

      [ ] parallel: timer runs concurrently with body
      [=] -Tracer.StartTimer
         (-) <session << $session
         (-) >handle >> $timerHandle

   [/]
      [ ] collect the timer started in setup
      (*) *All
         (*) << $timerHandle

      [-] -Tracer.StopTimer
         (-) <handle << $timerHandle
         (-) >elapsed >> $duration

      [-] -Tracer.Close
         (-) <session << $session
```

**INVALID:**
```polyglot
[ ] ✗ PGE01004 — [T] inside a wrapper
{W} -W.Bad
   (-) <input;string
   (-) >output;string

   [\]
      [T] -T.Call    [ ] ✗ PGE01004 — triggers not allowed in wrappers
```

```polyglot
[ ] ✗ PGE01004 — [Q] inside a wrapper
{W} -W.Bad
   [\]
      [Q] -Q.Default    [ ] ✗ PGE01004 — queues not allowed in wrappers
```

```polyglot
[ ] ✗ PGE01004 — [T] pipeline trigger inside a wrapper
{W} -W.Bad
   [T] -T.Call    [ ] ✗ PGE01004 — triggers not allowed in wrappers
```

**Open point:** None.

### See Also

- [[concepts/pipelines/wrappers|Wrappers]] — user-facing wrapper documentation (references PGE01004)
- [[concepts/macros|Macros (Retired)]] — macros retired; replaced by parameterized `##` schemas
