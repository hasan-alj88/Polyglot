# Standard Library Reference

[← Back to README](../README.md)

## Table of Contents
- [Naming Conventions](#naming-conventions)
- [Trigger Pipelines](#trigger-pipelines-t)
- [Wrapper Pipelines](#wrapper-pipelines-w)
- [Validation Wrappers](#validation-wrappers-with)
- [Utility Pipelines](#utility-pipelines-u)
- [Queue Management](#queue-management-q)
- [Join Operations](#join-operations)

## Naming Conventions

The standard library uses hierarchical full names with short aliases for common operations.

**Pipeline Name Prefixes:**

| Prefix    | Kind                   | Requirements                               | Purpose                                         |
|-----------|------------------------|--------------------------------------------|-------------------------------------------------|
| `\|T.`    | **Trigger**            | Must return exactly one boolean            | Trigger pipelines that activate other pipelines |
| `\|W.`    | **Wrapper**            | Must be a macro with setup/cleanup         | Wrapper macros defining full resource lifecycle |
| `\|With.` | **Validation Wrapper** | Must be a macro that validates environment | Checks if environment already exists            |
| `\|U.`    | **Utility**            | Utility commands/pipelines                 | Variety of utility functions                    |
| `\|Q.`    | **Queue**              | Used in `[Q]` blocks                       | Queue management and condition pipelines        |

**Note:** The standard library is **always implicitly imported** and doesn't require `[@]` declarations.

## Trigger Pipelines (`|T.`)

### File System Triggers

```polyglot
|T.System.File.Changed << file_path: pg\string >> changed: pg\bool
|T.System.File.Created << file_path: pg\string >> created: pg\bool
|T.System.File.Deleted << file_path: pg\string >> deleted: pg\bool
|T.System.File.Modified << file_path: pg\string >> modified: pg\bool
|T.System.Directory.Changed << dir_path: pg\string >> changed: pg\bool

\\ Aliases:
|T.FileChanged → |T.System.File.Changed
|T.FileCreated → |T.System.File.Created
|T.FileDeleted → |T.System.File.Deleted
|T.DirChanged → |T.System.Directory.Changed
```

**Example:**
```polyglot
[|] ProcessNewFiles
[t] |T.FileCreated << "data/*.csv"
[r] |ProcessCSV
[x]
```

### Schedule Triggers

```polyglot
|T.Schedule.Cron << cron_expr: pg\string >> triggered: pg\bool
|T.Schedule.Interval << interval: pg\Datetime >> triggered: pg\bool
|T.Schedule.Once << datetime: pg\Datetime >> triggered: pg\bool
|T.Schedule.Daily << time: pg\string >> triggered: pg\bool
|T.Schedule.Weekly << day: pg\string << time: pg\string >> triggered: pg\bool

\\ Aliases:
|T.Cron → |T.Schedule.Cron
|T.Every → |T.Schedule.Interval
|T.Daily → |T.Schedule.Daily
```

**Example:**
```polyglot
[|] DailyReport
[t] |T.Daily << "02:00"  \\ 2 AM daily
[r] |GenerateReport
[x]
```

### System Resource Triggers

```polyglot
|T.System.Resource.Cpu.Above << threshold: pg\float >> triggered: pg\bool
|T.System.Resource.Cpu.Below << threshold: pg\float >> triggered: pg\bool
|T.System.Resource.Memory.Above << threshold: pg\float >> triggered: pg\bool
|T.System.Resource.Memory.Below << threshold: pg\float >> triggered: pg\bool
|T.System.Resource.DiskSpace.Below << threshold: pg\float >> triggered: pg\bool
|T.System.Resource.Network.Local.Available >> available: pg\bool
|T.System.Resource.Network.Internet.Available >> available: pg\bool
|T.System.Resource.Network.Traffic.Above << threshold: pg\float >> triggered: pg\bool

\\ Aliases:
|T.CpuHigh → |T.System.Resource.Cpu.Above
|T.CpuLow → |T.System.Resource.Cpu.Below
|T.MemoryLow → |T.System.Resource.Memory.Below
```

**Example:**
```polyglot
[|] AlertHighCpu
[t] |T.CpuHigh << 90.0  \\ Trigger when CPU > 90%
[r] |SendAlert
[x]
```

### External Triggers

```polyglot
|T.Network.HttpRequest << endpoint: pg\string >> triggered: pg\bool
|T.Network.WebhookReceived << webhook_id: pg\string >> triggered: pg\bool
|T.Message.QueueReceived << queue_name: pg\string >> triggered: pg\bool
|T.Message.KafkaMessage << topic: pg\string >> triggered: pg\bool
|T.Message.RabbitMQMessage << queue: pg\string >> triggered: pg\bool

\\ Aliases:
|T.Http → |T.Network.HttpRequest
|T.Webhook → |T.Network.WebhookReceived
```

### Application Triggers

```polyglot
|T.Polyglot.Application.Cli >> triggered: pg\bool
|T.Polyglot.Application.Api >> triggered: pg\bool
|T.Polyglot.Pipeline.Called >> triggered: pg\bool
|T.Polyglot.Pipeline.Has.Trigger.Called >> has_trigger: pg\bool

\\ Aliases:
|T.Cli → |T.Polyglot.Application.Cli
|T.Call → |T.Polyglot.Pipeline.Called
```

### Validation Triggers

```polyglot
|T.Check.Python.Version.Equal << version: pg\string >> matches: pg\bool
|T.Check.Python.Version.LessThan << version: pg\string >> less: pg\bool
|T.Check.Python.Version.MoreThan << version: pg\string >> more: pg\bool
|T.Check.Python.Modules.Requirements << req_file: pg\string >> satisfied: pg\bool

|T.Check.Rust.Version.Equal << version: pg\string >> matches: pg\bool
|T.Check.Node.Version.Equal << version: pg\string >> matches: pg\bool

|T.Check.File.Exists << file_path: pg\string >> exists: pg\bool
|T.Check.File.Modified.After << file: pg\string << date: pg\Datetime >> after: pg\bool
|T.Check.Directory.Exists << dir_path: pg\string >> exists: pg\bool

|T.Assert << condition: pg\bool  \\ Throws error if false
```

## Wrapper Pipelines (`|W.`)

### Language Runtime Wrappers

```polyglot
|W.Runtime.Python.3.10
|W.Runtime.Python.3.11
|W.Runtime.Python.3.12
|W.Runtime.Node.18
|W.Runtime.Node.20
|W.Runtime.Rust.1.70
|W.Runtime.Cpp.11
|W.Runtime.Cpp.17
|W.Runtime.Cpp.20

\\ Aliases:
|W.Python3.10 → |W.Runtime.Python.3.10
|W.Python3.11 → |W.Runtime.Python.3.11
|W.Python3.12 → |W.Runtime.Python.3.12
|W.Node → |W.Runtime.Node.20
|W.Rust → |W.Runtime.Rust.1.70
|W.Cpp17 → |W.Runtime.Cpp.17
```

**Example:**
```polyglot
[|] PythonPipeline
[t] |T.Call
[w] |W.Python3.11  \\ Sets up Python 3.11 runtime
[r] |SomePythonOperation
[x]
```

### Database Wrappers

```polyglot
|W.Database.PostgreSQL.Connection << conn_str: pg\string
|W.Database.MySQL.Connection << conn_str: pg\string
|W.Database.MongoDB.Connection << conn_str: pg\string
|W.Database.Redis.Connection << conn_str: pg\string
|W.Database.SQLite.Connection << db_path: pg\string

\\ Aliases:
|W.PostgreSQL → |W.Database.PostgreSQL.Connection
|W.MySQL → |W.Database.MySQL.Connection
|W.Redis → |W.Database.Redis.Connection
```

### File System Wrappers

```polyglot
|W.System.File.Bytes.Read << file_path: pg\string
|W.System.File.Bytes.Write << file_path: pg\string
|W.System.File.Bytes.Append << file_path: pg\string

|W.System.File.Text.Read << file_path: pg\string
|W.System.File.Text.Write << file_path: pg\string
|W.System.File.Text.Append << file_path: pg\string

|W.System.File.Log.Read << file_path: pg\string
|W.System.File.Log.Write << file_path: pg\string
|W.System.File.Log.Append << file_path: pg\string

|W.System.Directory.Temporary  \\ Creates temp dir, deletes afterward
```

### Network Wrappers

```polyglot
|W.Network.Http.Server << port: pg\int
|W.Network.Http.Client
|W.Network.WebSocket.Server << port: pg\int
|W.Network.WebSocket.Client

\\ Aliases:
|W.HttpServer → |W.Network.Http.Server
|W.HttpClient → |W.Network.Http.Client
```

### Timer and Recording Wrappers

```polyglot
|W.Timer  \\ Times execution
|W.Record.Timer  \\ Records timing metrics
|W.Record.RunStatus  \\ Records execution status
```

## Validation Wrappers (`|With.`)

Validation wrappers check if an environment is already set up. They don't perform setup, only validation.

```polyglot
|With.Runtime.Python.3.10
|With.Runtime.Node.20
|With.Runtime.Rust.1.70
|With.Database.PostgreSQL.Connection
|With.FileSystem.File.Read.Handle
|With.FileSystem.File.Write.Handle
|With.FileSystem.File.Append.Handle

\\ Aliases:
|With.Python3.10 → |With.Runtime.Python.3.10
|With.Node → |With.Runtime.Node.20
|With.PostgreSQL → |With.Database.PostgreSQL.Connection
```

**Example:**
```polyglot
[|] MainPipeline
[t] |T.Call
[w] |W.Python3.10  \\ Sets up Python
[r] |SubPipeline
[x]

[|] SubPipeline
[t] |T.Call
[w] |With.Python3.10  \\ Validates Python already set up
[r] |UsePythonOperation
[x]
```

## Utility Pipelines (`|U.`)

### System Operations

```polyglot
|U.System.Resource.Cpu.Get.Usage >> usage: pg\float
|U.System.Resource.Cpu.Get.UsagePercentage >> percent: pg\float
|U.System.Resource.Memory.Get.Usage >> usage: pg\int
|U.System.Resource.Memory.Get.UsagePercentage >> percent: pg\float

|U.System.Resource.Disk.Get.Space.Total >> total: pg\int
|U.System.Resource.Disk.Get.Space.Left >> left: pg\int
|U.System.Resource.Disk.Get.Space.LeftPercentage >> percent: pg\float
|U.System.Resource.Disk.Get.Space.Used >> used: pg\int
|U.System.Resource.Disk.Get.Space.UsedPercentage >> percent: pg\float

|U.System.Get.Network.IP >> ip: pg\string
|U.System.Get.Network.DownloadRate >> rate: pg\float
|U.System.Get.Network.UploadRate >> rate: pg\float

|U.System.OperatingSystem.Get >> os: #OperatingSystem

|U.System.File.Open << path: pg\string >> handle: pg\handle
|U.System.File.Close << handle: pg\handle
|U.System.File.Type << path: pg\string >> type: pg\string
|U.System.File.Extension << path: pg\string >> ext: pg\string

|U.System.Execute.Command << cmd: pg\string >> output: pg\string
|U.System.Execute.Shell << script: pg\string >> output: pg\string

\\ Aliases:
|U.CpuUsage → |U.System.Resource.Cpu.Get.UsagePercentage
|U.MemoryUsage → |U.System.Resource.Memory.Get.UsagePercentage
|U.RunShell → |U.System.Execute.Shell
|U.RunCommand → |U.System.Execute.Command
```

### Boolean Triggers as Utilities

All file and resource triggers are also available as utilities:

```polyglot
|U.System.File.Changed << file_path: pg\string >> changed: pg\bool
|U.System.File.Created << file_path: pg\string >> created: pg\bool
|U.System.File.Exists << file_path: pg\string >> exists: pg\bool

|U.System.Resource.Cpu.Above << threshold: pg\float >> above: pg\bool
|U.System.Resource.Cpu.Below << threshold: pg\float >> below: pg\bool
|U.System.Resource.Memory.Above << threshold: pg\float >> above: pg\bool
|U.System.Resource.Memory.Below << threshold: pg\float >> below: pg\bool
```

### Logging

```polyglot
|U.Log << log_level: #LogLevel << message: pg\string
|U.Log.Debug << message: pg\string
|U.Log.Info << message: pg\string
|U.Log.Warning << message: pg\string
|U.Log.Error << message: pg\string
|U.Log.Critical << message: pg\string

[#] LogLevel
[D] Debug
[D] Info
[D] Warning
[D] Error
[D] Critical
[x]
```

### Type Conversion

Format: `|U.Convert.{lang1}.{type1}.To.{lang2}.{type2}`

```polyglot
\\ Python ↔ Polyglot
|U.Convert.Py.String.To.Pg.String << input: py\str >> output: pg\string
|U.Convert.Pg.String.To.Py.String << input: pg\string >> output: py\str
|U.Convert.Py.Dict.To.Pg.Dict << input: py\dict >> output: pg\dict
|U.Convert.Py.List.To.Pg.List << input: py\list >> output: pg\list

\\ Python ↔ Rust
|U.Convert.Py.String.To.Rust.String << input: py\str >> output: rust\String
|U.Convert.Rust.String.To.Py.String << input: rust\String >> output: py\str
|U.Convert.Py.List.To.Rust.Vec << input: py\list >> output: rust\Vec

\\ C++ ↔ Polyglot (std::string aliased as Cpp.string)
|U.Convert.Cpp.String.To.Pg.String << input: cpp\string >> output: pg\string
|U.Convert.Pg.String.To.Cpp.String << input: pg\string >> output: cpp\string

\\ Rust ↔ Polyglot
|U.Convert.Rust.String.To.Pg.String << input: rust\String >> output: pg\string
|U.Convert.Pg.String.To.Rust.String << input: pg\string >> output: rust\String

\\ Range utilities
|U.FloatRange << start: pg\float << end: pg\float << step: pg\float >> range: pg\list
|U.IntRange << start: pg\int << end: pg\int << step: pg\int >> range: pg\list
```

### Error Handling

```polyglot
|U.Error.Handle.OOM  \\ Out of Memory handler
|U.Error.Handle.CpuThrottle
|U.Error.Handle.Timeout
|U.Error.Handle.ResourceLimit
|U.Error.Retry << operation << max_attempts: pg\int
|U.Error.RetryWithBackoff
[~] << operation
[~] << max_attempts: pg\int
[~] << base_delay: pg\Datetime
[~] << max_delay: pg\Datetime
[~] << strategy: #RetryStrategy
|U.Error.GracefulShutdown << cleanup_tasks: pg\list

\\ Aliases:
|U.Retry → |U.Error.Retry
|U.Shutdown → |U.Error.GracefulShutdown
```

### Console Operations

```polyglot
|U.Console.Print << message: pg\string
|U.Console.Clear
|U.Console.RewriteLine << message: pg\string

\\ Alias:
|U.Print → |U.Console.Print
```

### General Utilities

```polyglot
|U.Sleep << duration: pg\Datetime  \\ Do nothing for specified time
|U.Exit << exit_code: pg\int  \\ Exit pipeline with code
```

### Boolean Operations

```polyglot
|U.Boolean.And << a: pg\bool << b: pg\bool >> result: pg\bool
|U.Boolean.Or << a: pg\bool << b: pg\bool >> result: pg\bool
|U.Boolean.Not << a: pg\bool >> result: pg\bool
|U.Boolean.Xor << a: pg\bool << b: pg\bool >> result: pg\bool
|U.Boolean.True >> result: pg\bool
|U.Boolean.False >> result: pg\bool
```

### Comparison Operations

```polyglot
|U.Compare.Equal << a: any << b: any >> result: pg\bool
|U.Compare.NotEqual << a: any << b: any >> result: pg\bool
|U.Compare.GreaterThan << a: number << b: number >> result: pg\bool
|U.Compare.LessThan << a: number << b: number >> result: pg\bool
|U.Compare.GreaterOrEqual << a: number << b: number >> result: pg\bool
|U.Compare.LessOrEqual << a: number << b: number >> result: pg\bool
```

### String Operations

```polyglot
|U.String.Contains << haystack: pg\string << needle: pg\string >> result: pg\bool
|U.String.StartsWith << text: pg\string << prefix: pg\string >> result: pg\bool
|U.String.EndsWith << text: pg\string << suffix: pg\string >> result: pg\bool
|U.String.IsEmpty << text: pg\string >> result: pg\bool
|U.String.ToUpperCase << text: pg\string >> result: pg\string
|U.String.ToLowerCase << text: pg\string >> result: pg\string
|U.String.Trim << text: pg\string >> result: pg\string
|U.String.Split << text: pg\string << delimiter: pg\string >> result: pg\list
|U.String.Join << parts: pg\list << delimiter: pg\string >> result: pg\string
```

### Collection Operations

```polyglot
|U.Collection.Contains << collection: any << item: any >> result: pg\bool
|U.Collection.IsEmpty << collection: any >> result: pg\bool
|U.Collection.Size << collection: any >> size: pg\int
|U.Collection.First << collection: any >> item: any
|U.Collection.Last << collection: any >> item: any
|U.Collection.Filter << collection: any << predicate >> filtered: any
|U.Collection.Map << collection: any << transform >> mapped: any
```

## Queue Management (`|Q.`)

### Queue Configuration

```polyglot
|Q.Priority << priority: pg\int
|Q.Type << type: #QueueType
|Q.MaxSize << size: pg\int
|Q.Timeout << duration: pg\Datetime
|Q.ParallelExecution << count: pg\int
|Q.ToQueue << queue: #Queue
|Q.Halt
|Q.Resume

[#] QueueType
[D] FIFO      \\ First In First Out
[D] LIFO      \\ Last In First Out
[D] Priority  \\ Priority-based
[D] Deadline  \\ Deadline-based
[x]

[#] Queue
[D] Default
[~][Q] |Q.Type << #QueueType.Priority
[~][Q] |Q.Timeout << T"5:"  \\ 5 minutes
[~][Q] |Q.MaxSize << 1000
[x]
```

### Resource Conditions

```polyglot
|Q.Resource.Available.CPU.Get >> available: pg\float
|Q.Resource.Available.CPU.MoreThan << threshold: pg\float >> result: pg\bool
|Q.Resource.Available.CPU.LessThan << threshold: pg\float >> result: pg\bool
|Q.Resource.Available.CPU.InRange << min: pg\float << max: pg\float >> result: pg\bool

|Q.Resource.Usage.CPU.Get >> usage: pg\float
|Q.Resource.Usage.CPU.MoreThan << threshold: pg\float >> result: pg\bool
|Q.Resource.Usage.CPU.LessThan << threshold: pg\float >> result: pg\bool
|Q.Resource.Usage.CPU.InRange << min: pg\float << max: pg\float >> result: pg\bool

|Q.Resource.Available.Memory.Get >> available: pg\int
|Q.Resource.Available.Memory.MoreThan << threshold: pg\int >> result: pg\bool
|Q.Resource.Available.Memory.LessThan << threshold: pg\int >> result: pg\bool

|Q.Resource.Usage.Memory.Get >> usage: pg\int
|Q.Resource.Usage.Memory.MoreThan << threshold: pg\int >> result: pg\bool

|Q.Resource.Available.DiskSpace.Get >> available: pg\int
|Q.Resource.Usage.DiskSpace.Get >> used: pg\int

\\ Aliases:
|Q.CpuAvailable → |Q.Resource.Available.CPU.Get
|Q.MemoryAvailable → |Q.Resource.Available.Memory.Get
```

### Concurrency Control

```polyglot
|Q.Concurrency.MaxPipelines << max: pg\int
|Q.Concurrency.MaxPerType << type: pg\string << max: pg\int
|Q.Concurrency.RateLimit << rate: pg\int << period: pg\Datetime
|Q.Concurrency.Throttle << rate: pg\float
|Q.Concurrency.BackPressure << threshold: pg\float

\\ Aliases:
|Q.MaxConcurrent → |Q.Concurrency.MaxPipelines
|Q.RateLimit → |Q.Concurrency.RateLimit
```

### Retry and Failure

```polyglot
|Q.Retry.MaxAttempts << attempts: pg\int
|Q.Retry.Strategy << strategy: #RetryStrategy
|Q.Retry.BackoffBase << duration: pg\Datetime
|Q.Retry.BackoffMax << duration: pg\Datetime
|Q.DeadLetterQueue << queue_name: pg\string
|Q.FailureHandler << handler_pipeline

[#] RetryStrategy
[D] Immediate    \\ Retry immediately
[D] Fixed        \\ Fixed delay
[D] Exponential  \\ Exponential backoff
[D] Linear       \\ Linear increase
[x]
```

### Kill Conditions

```polyglot
|Q.Kill.Exceeds.Usage.CPU << threshold: pg\float
|Q.Kill.Exceeds.Usage.Memory << threshold: pg\float
|Q.Kill.Timeout.Execution << duration: pg\Datetime
|Q.Kill.Strategy << strategy: #KillStrategy
|Q.Kill.GracePeriod << duration: pg\Datetime
|Q.Kill.CustomHandler << handler_pipeline

\\ Aliases:
|Q.Kill.MemoryLimit → |Q.Kill.Exceeds.Usage.Memory
|Q.Kill.CPULimit → |Q.Kill.Exceeds.Usage.CPU
|Q.Kill.ExecTimeout → |Q.Kill.Timeout.Execution

[#] KillStrategy
[D] Graceful        \\ SIGTERM then SIGKILL
[D] Immediate       \\ SIGKILL immediately
[D] CustomHandler   \\ User-defined cleanup
[D] Degrade         \\ Pause, free resources, resume
[x]
```

### Monitoring

```polyglot
|Q.Monitoring.EmitMetrics << enabled: pg\bool
|Q.Monitoring.MetricsInterval << interval: pg\Datetime
|Q.Monitoring.TrackLatency << enabled: pg\bool
|Q.Monitoring.AlertThreshold << threshold: pg\float
```

## Join Operations

```polyglot
|JoinAll  \\ Wait for all forked branches
|JoinFirst  \\ Return when first branch completes
|JoinLast  \\ Return when last branch completes
|JoinNth << n: pg\int  \\ Return when nth branch completes
```

**Example:**
```polyglot
[f] |Branch1
[f] |Branch2
[f] |Branch3
[j] |JoinAll  \\ Wait for all three
```

---

[Next: Error Handling →](06-error-handling.md)