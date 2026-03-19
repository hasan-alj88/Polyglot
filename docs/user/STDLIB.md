---
audience: user
type: specification
updated: 2026-03-17
status: draft
---

# Polyglot Code -- Standard Library Reference

This document catalogs every built-in component of Polyglot Code.
Standard library items do NOT require an `[@]` import -- they are
available in every `.pg` file by default.

**Legend**

    .   fixed field navigation (subpackage, subtype, subfield)
    :   flexible field navigation
    <   input parameter
    >   output parameter
    ;   type annotation on a parameter
    (?) inferred / speculative -- not yet confirmed in specs

Items marked `(?)` are derived from examples or the scenario catalog
and have not been formally specified. All other items appear in at
least one spec file, lesson file, or canonical example.


## 1. Standard Library Pipelines (=)

Stdlib pipelines are called with `=` prefix and hierarchical dot
names. No `[@]` import is needed.

```
=File
   .Text
      .Read
         <path;path
         >content;string
      .Write
         <path;path
         <content;string
         >written;bool (?)
      .Append
         <path;path
         <content;string
         >written;bool (?)
   .Copy (?)
      <source;path
      <destination;path
      >copied;bool (?)
   .Move (?)
      <source;path
      <destination;path
      >moved;bool (?)
   .Delete (?)
      <path;path
      >deleted;bool (?)
   .List (?)
      <folder;path
      >files;array.path (?)
```

### System Pipelines (=Sys)

```
=Sys
   .OS
      >os;OS
      [ ] Returns the current operating system as #OS enum.
```


## 2. Triggers (=T)

Triggers are placed on `[t]` lines inside `{=}` pipeline definitions.

```
=T
   .Call
      [ ] Pipeline is invoked when called from another pipeline.
      [ ] No additional parameters.

   .Daily
      "time"
      [ ] Fires once per day at the specified time.
      [ ] Time is provided as an inline string argument.

   .Folder
      .NewFiles
         <Folder;path
         >NewFiles;array.path

   .Schedule (?)
      .Cron (?)
         <cron;string (?)
         [ ] Fires on a cron expression.

   .HTTP (?)
      .Webhook (?)
         <endpoint;string (?)
         <method;string (?)
         [ ] Fires on an incoming HTTP request.

   .File (?)
      .Created (?)
         <path;path (?)
         <pattern;string (?)
         [ ] Fires when a matching file is created.
```


## 3. Queue Configurations (=Q)

Queue modifiers are placed on `[Q]` lines.

```
=Q
   .Default
      [ ] Standard queue -- no special dispatch or kill rules.

   .Priority (?)
      <level;int (?)
      [ ] Sets pipeline execution priority.

   .DispatchIf (?)
      .CPU (?)
         .Available (?)
            .MoreThan (?)
               <threshold;float (?)
      .RAM (?)
         .Available (?)
            .MB (?)
               .MoreThan (?)
                  <threshold;float (?)

   .KillIf (?)
      .ExecutionTime (?)
         .MoreThan (?)
            <timeout;string (?)
      .CPU (?)
         .Usage (?)
            .MoreThan (?)
               <threshold;float (?)

   .ReplaceReTriggeredIf (?)
      .QueueTime (?)
         .MoreThan (?)
            <timeout;string (?)
```


## 4. Wrappers (=W)

Wrappers are placed on `[W]` lines. Each wrapper references a Macro
(`{M}`) that provides setup (`[\]`) and cleanup (`[/]`) around the
pipeline execution body. Macro IO (`[{]` input, `[}]` output) is
wired at the `[W]` line using `[=]` with `$` variables.

Execution order: `[t],[=]` -> `[Q]` -> `[\]` -> Body -> `[/]`

```
=W
   .Polyglot
      [ ] Pure Polyglot Code -- no external runtime, no setup/cleanup.
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


## 5. Expand Operators (~)

Expand operators iterate over a collection, producing a mini-pipeline
per item. The execution marker on the expand line controls
parallelism: `[p]` for parallel, `[r]` for sequential.

```
~ForEach
   .Array
      <Array
      >item
      .Enumerate
         <Array
         >index
         >item
   .Serial
      <Serial
      >key
      >item
   .Level
      <level
      >key
      >item
      [ ] The ~ suffix on the input path marks the iteration point:
      [ ]   <level << #SomeData.SubField.~
```


## 6. Collect Operators (*)

Collectors are invoked with `[r]` (sequential) or `[p]` (parallel)
execution markers. Collector IO lines use `[*]` (matching the `*`
operator prefix). Collector outputs are accessible one level up from
the expand scope.

### *Into -- Collect into Collection

```
*Into
   .Array
      <item
      >Array
   .Serial
      <key
      <value
      >Serial
   .Level
      <key
      <value
      >Serial
```

### *Agg -- Reduce to Single Value

The namespace is `*Agg`, NOT `*Aggregate`.

```
*Agg
   .Sum
      <number
      >sum
   .Count
      <item
      >count
   .Average
      <number
      >average
   .Max
      <number
      >max
   .Min
      <number
      >min
   .Concatenate
      <string
      >result
```

### *Sync / *Race -- Parallel Variable Collectors

Used outside expand scopes to synchronise or race variables from parallel `[p]` pipeline calls.
Uses `[*] <<` (wait input) and `[*] >>` (collect output) — see IO spec.

```
*All
   [ ] Wait for ALL listed variables to be Final.
   [ ] Uses [*] << only — no [*] >>. All variables stay accessible after.
   [ ] No type constraint on inputs.
   [ ] Example:
   [ ]   [*] *All
   [ ]      [*] << $profile
   [ ]      [*] << $history

*First
   [ ] Sugar for *Nth with n=1.
   [ ] Takes first arriving value; all other inputs are cancelled.
   [ ] All [*] << inputs must be the same type. [*] >> output required.
   [*] << $candidate...
   [*] >> $winner

*Second
   [ ] Sugar for *Nth with n=2.
   [*] << $candidate...
   [*] >> $winner

*Nth
   [ ] Generic race form — wait for the Nth arrival.
   [ ] *First and *Second are named aliases for *Nth n=1/n=2.
   [*] <n;int            (which arrival to capture)
   [*] << $candidate...
   [*] >> $winner
```


## 7. Built-in Data Definitions (#)

### #Boolean

```
#Boolean
   [%] .description << "boolean type Enum"
   [%] .version << "1.0.0"
   .True
      [%] .alias << #True
   .False
      [%] .alias << #False
```

### #None

```
#None
   [ ] Represents the absence of a value.
```

### #OS

```
#OS
   [%] .description << "Operating system enum"
   [%] .version << "1.0.0"
   .Unix
      [%] .alias << #Unix
   .Windows
      [%] .alias << #Windows
```

### #path

```
#path
   [%] .description << "Cross-platform file system path"
   [%] .version << "1.0.0"
   [.] .Unix;string
   [.] .Windows;string
```

### #PipelineStatus

```
#PipelineStatus
   [%] .description << "Pipeline instance status"
   .AwaitTrigger
      [%] .alias << #AwaitTrigger
   .Disabled
      [%] .alias << #Disabled
   .Running
      [%] .alias << #Running
   .Failed
      [%] .alias << #Failed
```

### #VarState

```
#VarState
   [%] .description << "Variable lifecycle state"
   .Declared
      [%] .alias << #Declared
   .Default
      [%] .alias << #Default
   .Final
      [%] .alias << #Final
   .Failed
      [%] .alias << #Failed
   .Released
      [%] .alias << #Released
```


## 8. Built-in Error Namespaces (!)

Errors use the `!` prefix and hierarchical dot names. They appear
inside `[!]` blocks scoped under the `[r]` call that produces them.

### Standard Library Error Trees

Each stdlib pipeline exposes the errors it can raise:

```
=File.Text.Read
   !File.NotFound
   !File.ReadError

=File.Text.Write
   !File.NotFound
   !File.WriteError

=File.Text.Append
   !File.NotFound
   !File.WriteError
```

### Standard Error Namespaces

```
!File
   .NotFound
   .ReadError
   .WriteError (?)

!No
   .Input
   .Output (?)

!Timeout (?)
   :Connection (?)
      [ ] Uses flexible (:) field for specific timeout targets.

!Validation (?)
   .Error (?)
```


## 9. Basic Types

Bare type names -- no namespace prefix required.

```
string       Text value
int          Integer number
float        Floating-point number
bool         Boolean (use #Boolean.True / #Boolean.False)
path         File system path
array        Ordered collection, keys from 0
serial       Dynamic key-value structure (flexible fields)
```

### Live Type Modifier

`live` is a type modifier, not a standalone type. It marks metadata
fields managed by the Polyglot runtime. Users read via `%` accessor
but never assign.

```
live #PipelineStatus     Pipeline instance status
live array.error         Accumulated errors
live #Boolean            Success flag
live int                 Counter (instanceCount, triggerCount, etc.)
live #VarState           Variable lifecycle state
live string              Timestamp or duration
live array.path          File references
```

### Element-Typed Arrays

Arrays specify element type via dot notation:

```
array.string       Array of strings
array.int          Array of integers
array.float        Array of floats
array.path         Array of paths
array.bool         Array of booleans
array.UserRecord   Array of user-defined #UserRecord (no # in type position)
```

### User-Defined Types

Declared with `{#}` blocks, referenced with `#` prefix:

```
#UserRecord        Standalone reference
array.UserRecord   As array element type (# dropped after .)
```


## 10. Implicit Live Metadata Fields

Every Polyglot object carries implicit `live` metadata fields populated
by the runtime. Read via the `%` accessor. Never assign to these.

### Pipeline `{=}` Live Fields

```
=PipelineName%status;live #PipelineStatus
   [ ] Current instance status (AwaitTrigger, Disabled, Running, Failed)
=PipelineName%errors;live array.error
   [ ] Accumulated errors from this pipeline
=PipelineName%isSuccess;live #Boolean
   [ ] Whether last run completed without error
=PipelineName%instanceCount;live int
   [ ] Number of active instances
=PipelineName%lastRun;live string
   [ ] Timestamp of last execution
=PipelineName%duration;live string
   [ ] Duration of current or last run
=PipelineName%triggerCount;live int
   [ ] Total number of times triggered
```

### Variable `$` Live Fields

```
$variableName%state;live #VarState
   [ ] Current lifecycle state (Declared, Default, Final, Failed, Released)
```

### Data `{#}` Live Fields

```
#DataName%lastModified;live string
   [ ] Timestamp of last modification
#DataName%files;live array.path
   [ ] Files that define or reference this type
#DataName%errors;live array.error
   [ ] Compile or validation errors related to this type
#DataName%usageCount;live int
   [ ] Number of pipelines referencing this type
```


## 11. Block Element Registry (Quick Reference)

Definition elements (top-level):

```
{@}   Package declaration (mandatory first block)
{#}   Data definition
{=}   Pipeline definition
{M}   Macro definition
{ }   Comment
```

Block elements (inside definitions):

```
[@]   Import package / multi-file package reference
[=]   Pipeline IO line
[~]   Expand IO line
[*]   Collect IO line
[*] <<  Wait input (inside [*] blocks) — waits for Final; var stays accessible
[*] >>  Collect output (inside [*] race blocks) — inputs cancelled; output survives
[r]   Run in series
[p]   Run in parallel
[b]   Run in background (fire and forget)
[#]   Load serialized data into typed structure
[?]   Conditional switch
[!]   Error handling
[t]   Trigger
[Q]   Queue
[W]   Wrapper
[\]   Setup scope
[/]   Cleanup scope
[{]   From outer scope (macros)
[}]   To outer scope (macros)
[.]   Fixed subfield access
[:]   Flexible subfield access
[%]   Metadata (description, version, authors, license, alias, info)
[&]   AND
[+]   OR
[-]   NOT
[^]   XOR
[ ]   Comment
```


## 12. Assignment Operators

```
<<    Final push   (right to left)   $x << "value"
>>    Final pull   (left to right)   >out >> $x
<~    Default      (right to left)   .field <~ "value"
~>    Default      (left to right)   >out ~> ""
```


## 13. Comparison and Range Operators

```
=?    Equal
>?    Greater than
<?    Less than
>=?   Greater or equal
<=?   Less or equal
=!?   Not equal
<!?   Not less than (negated)
>!?   Not greater than (negated)
<=!?  Not less-or-equal (negated)
>=!?  Not greater-or-equal (negated)
*?    Wildcard (else / catch-all)
?[    Closed range start     a ?[10, 20]
?(    Open range start       a ?(10, 20)
```
