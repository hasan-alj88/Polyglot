# Trigger System Technical Guide

**Version:** v0.0.5
**Audience:** System Implementers, Architects
**Last Updated:** 2026-01-05
**Confidence:** Verified (V)

---

## Table of Contents

1. [Overview](#overview)
2. [Trigger Lifecycle](#trigger-lifecycle)
3. [Runtime Integration](#runtime-integration)
4. [I/O Mechanism](#io-mechanism)
5. [Scheduling and Polling](#scheduling-and-polling)
6. [Error Propagation](#error-propagation)
7. [Wrapper Integration](#wrapper-integration)
8. [Performance Considerations](#performance-considerations)
9. [Implementation Notes](#implementation-notes)

---

## Overview

The Polyglot trigger system provides a unified abstraction for event-driven pipeline execution. Triggers transform external events into pipeline invocations with typed data flow from event sources to pipeline inputs.

### Architecture Principles

1. **Event Abstraction:** Uniform interface across heterogeneous event sources
2. **Type Safety:** Strongly typed I/O between triggers and pipelines
3. **Process Isolation:** Wrapper-based session management for execution context
4. **Lazy Activation:** Triggers activate only when pipelines are deployed
5. **Queue Integration:** Optional queue-based execution for load management

### Key Components

- **Trigger Engine:** Event source polling and subscription management
- **I/O Wiring Layer:** Type-safe data flow from triggers to pipelines
- **Session Manager:** Wrapper-based execution context creation
- **Scheduler:** Time-based trigger orchestration
- **File Watcher:** Filesystem event monitoring
- **HTTP Server:** Request routing and handler dispatch

---

## Trigger Lifecycle

### Lifecycle States

```
┌─────────────┐
│  DECLARED   │  Pipeline contains [t] block
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  VALIDATED  │  Type checking, I/O wiring validation
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  REGISTERED │  Trigger engine creates event listener
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   ACTIVE    │  Listening for events
└──────┬──────┘
       │
       ├──────→  EVENT RECEIVED
       │         ↓
       │    ┌─────────────┐
       │    │  EXECUTING  │  Pipeline runs with trigger data
       │    └──────┬──────┘
       │           │
       │           ▼
       │    ┌─────────────┐
       │    │  COMPLETED  │  Execution finished, back to ACTIVE
       │    └─────────────┘
       │
       ▼
┌─────────────┐
│  STOPPED    │  Pipeline undeployed, listener destroyed
└─────────────┘
```

### Declaration Phase

When a pipeline with `[t]` marker is parsed:

1. **Syntax Validation:** Trigger type, configuration syntax, I/O declarations checked
2. **Block Ordering Verification:** Ensures `[t]` → `[<][>]` → `[w]` → `[Q]` → execution order
3. **I/O Type Checking:** Validates trigger outputs match pipeline input types
4. **Wrapper Compatibility:** Verifies required wrappers for trigger type

### Registration Phase

When pipeline is deployed:

1. **Trigger Engine Lookup:** Finds appropriate trigger engine (CLI, HTTP, Scheduler, FileWatcher)
2. **Configuration Application:** Applies trigger-specific settings (route, schedule, folder, etc.)
3. **Listener Creation:** Creates event listener/subscriber
4. **I/O Binding:** Establishes data flow channels from trigger to pipeline
5. **Activation:** Starts listening for events

### Execution Phase

When event is received:

1. **Event Capture:** Trigger engine receives external event
2. **Data Extraction:** Extracts typed data from event (args, timestamp, request body, file paths)
3. **I/O Wiring:** Maps extracted data to pipeline inputs per wiring configuration
4. **Session Creation:** Wrapper creates execution context
5. **Pipeline Invocation:** Pipeline executes with trigger-provided inputs
6. **Completion:** Pipeline outputs are captured, session cleaned up
7. **Return to Active:** Trigger returns to listening state

---

## Runtime Integration

### Trigger Engines

Polyglot runtime provides 6 specialized trigger engines:

#### CLI Trigger Engine

**Event Source:** Command-line invocation via `polyglot run <command>`

**Initialization:**
```
1. Parse pipeline declarations
2. Register command names with CLI router
3. Build argument parser for each command's kwargs
```

**Execution Flow:**
```
User runs: polyglot run greet --name "Alice"
         ↓
CLI Router matches "greet" → finds |Greet pipeline
         ↓
Argument Parser extracts kwargs: {name: "Alice"}
         ↓
I/O Wiring maps <kwargs.name> → <name> pipeline input
         ↓
Console Wrapper creates session
         ↓
Pipeline executes with inputs: {name: "Alice"}
         ↓
Output printed to console
```

**Key Implementation Details:**
- Commands are registered at deployment time
- Argument parsing uses trigger's `<kwargs.*` declarations
- Console wrapper required for process management

#### Cron/Daily Trigger Engine

**Event Source:** System scheduler (cron daemon or equivalent)

**Initialization:**
```
1. Parse cron expression or daily time
2. Calculate next execution timestamp
3. Register with system scheduler
```

**Execution Flow:**
```
Scheduler ticks to scheduled time
         ↓
Cron Engine receives wake-up signal
         ↓
Captures current timestamp
         ↓
I/O Wiring maps timestamp → <pipeline_input>
         ↓
Pipeline executes with timestamp
         ↓
Next execution time calculated
```

**Key Implementation Details:**
- Uses system cron or internal scheduler
- Timestamp provided as `:dt` type
- Execution is best-effort (may skip if system down)

#### Interval Trigger Engine

**Event Source:** Internal timer with fixed interval

**Initialization:**
```
1. Parse duration string ("5m", "1h", "30s")
2. Start interval timer
3. Register callback
```

**Execution Flow:**
```
Timer expires every <duration>
         ↓
Interval Engine receives timer callback
         ↓
Captures current timestamp
         ↓
I/O Wiring maps timestamp → <pipeline_input>
         ↓
Pipeline executes
         ↓
Timer resets for next interval
```

**Key Implementation Details:**
- Interval is fixed, not cron-based
- Timer continues even if execution takes longer than interval
- Multiple executions may run concurrently unless queue prevents it

#### Folder Trigger Engine

**Event Source:** Filesystem watcher monitoring directory

**Initialization:**
```
1. Parse folder path
2. Create filesystem watcher (inotify/FSEvents/ReadDirectoryChangesW)
3. Register for NEW_FILE events
```

**Execution Flow:**
```
New file appears in monitored folder
         ↓
Filesystem watcher fires event
         ↓
Folder Engine accumulates file paths
         ↓
Batch timeout reached OR file count threshold reached
         ↓
I/O Wiring maps array.path → <new_files>
         ↓
Pipeline executes with file array
         ↓
Watcher continues monitoring
```

**Key Implementation Details:**
- Uses OS-specific file watching mechanisms
- Files are batched to reduce execution frequency
- File paths are absolute `:path` types
- File wrapper required for file access

#### HTTP Trigger Engine

**Event Source:** HTTP server receiving requests

**Initialization:**
```
1. Parse server configuration (host, port)
2. Start HTTP server (if not already running)
3. Register route handler
```

**Execution Flow:**
```
HTTP request arrives: POST /api/users
         ↓
HTTP Server routes to registered handler
         ↓
HTTP Engine extracts request data (method, headers, body, params)
         ↓
Creates request serial: {method, headers, body, params}
         ↓
I/O Wiring maps request serial → <req>
         ↓
Pipeline executes with request data
         ↓
Pipeline output (response serial) returned to client
```

**Key Implementation Details:**
- Single HTTP server instance serves multiple endpoints
- Request body parsed based on Content-Type
- Response serial must contain `.status:uint` field
- HTTP wrapper required for session management

#### Calendar Trigger Engine

**Event Source:** Calendar-based scheduler

**Initialization:**
```
1. Parse calendar configuration (BusinessWeek, Date, Range)
2. Calculate next occurrence
3. Register with scheduler
```

**Execution Flow:**
```
Calendar date/time reached
         ↓
Calendar Engine receives scheduled trigger
         ↓
Captures current date and day_of_week
         ↓
I/O Wiring maps date → <report_date>, day_of_week → <weekday>
         ↓
Pipeline executes with calendar data
         ↓
Next occurrence calculated
```

**Key Implementation Details:**
- BusinessWeek automatically excludes weekends
- Timezone-aware scheduling
- Outputs both `:dt` timestamp and `:string` day name

---

## I/O Mechanism

### Type-Safe Data Flow

Trigger I/O wiring establishes compile-time verified data flow channels:

```polyglot
[t] |T.Cli
 |  <cmd:string << "greet"
 |  <kwargs.name:string << <name     %% Wire trigger output → pipeline input
 |  >args:serial >> <all_args        %% Wire trigger output → pipeline input

[<] <name:string          %% Pipeline input receives trigger data
[<] <all_args:serial      %% Pipeline input receives trigger data
```

### Wiring Compilation

**Step 1: Parse I/O Declarations**
```
Trigger declares:
  <kwargs.name:string << <name
  >args:serial >> <all_args

Pipeline declares:
  [<] <name:string
  [<] <all_args:serial
```

**Step 2: Type Checking**
```
Verify:
  - <kwargs.name:string> matches <name:string> ✓
  - >args:serial> matches <all_args:serial> ✓
```

**Step 3: Code Generation**
```
Generate data flow:
  trigger_output.kwargs.name → pipeline_input.name
  trigger_output.args → pipeline_input.all_args
```

### Reserved Enum Flow

Reserved enums (like `-Session-File`) flow through I/O wiring:

```polyglot
[w] |W.File
 |  <file:path << $log_file
 |  >session-Session-File >> $file_session

Runtime:
  1. Wrapper creates session object
  2. Session tagged with -Session-File enum
  3. Assigned to $file_session variable
  4. Variable available in pipeline scope
```

### Schema Validation Flow

Schema comparison (`#?`) in triggered pipelines:

```polyglot
[f] $request_body #? #UserSchema
   [r] ... process valid input

Runtime:
  1. $request_body contains serial from trigger
  2. #UserSchema defines expected structure
  3. Comparison validates field presence and types
  4. Fork condition branches based on result
```

---

## Scheduling and Polling

### Scheduler Architecture

```
┌─────────────────────────────────────────┐
│        Scheduler (Central)              │
│                                         │
│  ┌──────────┐  ┌──────────┐  ┌────────┐│
│  │ Cron     │  │ Daily    │  │Calendar││
│  │ Queue    │  │ Queue    │  │ Queue  ││
│  └────┬─────┘  └────┬─────┘  └───┬────┘│
│       │             │             │     │
└───────┼─────────────┼─────────────┼─────┘
        │             │             │
        ▼             ▼             ▼
   Execute        Execute        Execute
   Pipeline       Pipeline       Pipeline
```

### Cron Scheduler

**Algorithm:**
```
1. Parse cron expression: "0 0 * * *"
2. Calculate next run time using cron logic
3. Sleep until next run time
4. Execute pipeline with current timestamp
5. Goto step 2
```

**Cron Expression Support:**
```
┌─ minute (0-59)
│ ┌─ hour (0-23)
│ │ ┌─ day of month (1-31)
│ │ │ ┌─ month (1-12)
│ │ │ │ ┌─ day of week (0-7, 0=Sunday)
│ │ │ │ │
* * * * *
```

### Interval Scheduler

**Algorithm:**
```
1. Parse duration: "5m" → 300000ms
2. Start timer for duration
3. On timer expiration:
   a. Execute pipeline with current timestamp
   b. Reset timer
   c. Goto step 3
```

**Drift Handling:**
- Uses absolute timestamps to calculate next execution
- Compensates for execution time to maintain interval

### File Watcher Polling

**Batching Strategy:**
```
Configuration:
  - Batch window: 100ms
  - Max batch size: 100 files

Algorithm:
  1. Filesystem event received (new file)
  2. Add to batch buffer
  3. Start/reset batch timer (100ms)
  4. If batch size reaches 100 OR timer expires:
     a. Execute pipeline with file array
     b. Clear batch buffer
     c. Goto step 1
```

**Why Batching:**
- Reduces pipeline executions for burst file creation
- More efficient than per-file execution
- Configurable trade-off between latency and throughput

---

## Error Propagation

### Error Handling Flow

```
Trigger receives event
        ↓
I/O wiring extracts data
        ↓
Pipeline executes
        ├─→ Error in pipeline statement
        │         ↓
        │   [!] block catches error
        │         ↓
        │   Pipeline completes with error outputs
        │         ↓
        └─→ Trigger logs error (optional)
                  ↓
            Trigger returns to active state
```

### Error Types

**Trigger-Level Errors:**
- Configuration errors (invalid cron expression, missing folder)
- Network errors (HTTP server bind failure)
- System errors (file watcher creation failure)

**Pipeline-Level Errors:**
- Runtime errors (file not found, DB connection failed)
- Type errors (schema mismatch, invalid data)
- Logic errors (handled by `[!]` blocks)

### Error Recovery

**Trigger Engine Behavior:**
```
If trigger error:
  - Log error
  - Attempt recovery (retry, exponential backoff)
  - If unrecoverable: disable trigger, alert operator

If pipeline error:
  - Pipeline's [!] blocks handle errors
  - Pipeline outputs may indicate failure
  - Trigger continues to active state
```

**Best Practice:**
Pipelines should handle all errors internally using `[!]` blocks. Trigger engine should only handle infrastructure-level failures.

---

## Wrapper Integration

### Wrapper Lifecycle in Triggered Pipelines

```
Trigger receives event
        ↓
I/O wiring prepares inputs
        ↓
Wrapper.CreateSession()
        ├─→ Console: Attach to CLI process
        ├─→ File: Open file handles
        ├─→ HTTP: Create HTTP client context
        └─→ DB: Establish connection pool
        ↓
Pipeline executes with session
        ↓
Wrapper.CloseSession()
        ├─→ Console: Flush output
        ├─→ File: Close handles
        ├─→ HTTP: Clean up connections
        └─→ DB: Return connections to pool
        ↓
Trigger returns to active
```

### Session Management

**Session Creation:**
```rust
// Pseudocode
fn create_session(wrapper_type: WrapperType, config: WrapperConfig) -> Session {
    match wrapper_type {
        WrapperType::Console => {
            let console = attach_to_console();
            Session::Console(console)
        }
        WrapperType::File => {
            let file_handle = open_file(config.file_path)?;
            Session::File(file_handle)
        }
        WrapperType::HTTP => {
            let http_client = create_http_client(config.settings);
            Session::HTTP(http_client)
        }
        WrapperType::DB => {
            let db_conn = connect_to_database(config.settings)?;
            Session::DB(db_conn)
        }
    }
}
```

**Session Cleanup:**
```rust
// Pseudocode
fn close_session(session: Session) {
    match session {
        Session::Console(console) => console.flush(),
        Session::File(handle) => handle.close(),
        Session::HTTP(client) => client.shutdown(),
        Session::DB(conn) => connection_pool.return(conn),
    }
}
```

### Reserved Session Enums

Reserved session enums are compile-time known variants:

```
-Session-File   → maps to Session::File(FileHandle)
-Session-DB     → maps to Session::DB(DbConnection)
-Session-HTTP   → maps to Session::HTTP(HttpClient)
-Session-Cli    → maps to Session::Console(ConsoleHandle)
```

Runtime enforces that wrapper output types match reserved enum expectations.

---

## Performance Considerations

### Trigger Overhead

**Typical Latencies:**
```
CLI Trigger:      < 1ms (direct invocation)
HTTP Trigger:     < 5ms (request routing)
Cron Trigger:     ± 1s (scheduler granularity)
Interval Trigger: ± 100ms (timer accuracy)
Folder Trigger:   50-500ms (filesystem event + batching)
Calendar Trigger: ± 1s (scheduler granularity)
```

### Concurrency Models

**CLI Trigger:**
- One execution per command invocation
- Multiple concurrent commands possible (separate processes)

**HTTP Trigger:**
- One execution per request
- Concurrent requests handled by HTTP server thread pool
- Queue can limit concurrency if needed

**Cron/Interval/Calendar Triggers:**
- Single-threaded by default (one execution at a time)
- If execution exceeds interval, next execution waits
- Queue can manage backlog

**Folder Trigger:**
- Batch processing reduces concurrent executions
- Files processed in batches, not individually

### Queue Integration

**Optional Queue:** `[Q]` marker enables queue-based execution

```polyglot
[t] |T.HTTP.Endpoint
 |  <route:string << "/api/process"

[Q] max_concurrent: 5, timeout: 30s

%% Pipeline executes with queue management
```

**Queue Behavior:**
- Limits concurrent pipeline executions
- Queues excess requests
- Provides timeout and backpressure handling
- Useful for rate-limiting and resource management

### Resource Management

**File Descriptors:**
- File wrappers consume FDs
- Folder triggers watching many directories consume FDs
- Monitor and configure OS limits

**Memory:**
- HTTP request bodies held in memory
- Folder trigger batches held in memory
- Large batch sizes or request bodies may cause memory pressure

**Network Connections:**
- HTTP server maintains connection pool
- DB wrapper maintains connection pool
- Configure pool sizes based on expected load

---

## Implementation Notes

### Trigger Registration

Triggers are registered when pipelines are deployed:

```
polyglot deploy my-pipeline.pg
        ↓
Parse pipeline
        ↓
Find [t] block
        ↓
Trigger Engine registration:
  - CLI: Add command to CLI router
  - HTTP: Add route to HTTP server
  - Cron/Interval/Calendar: Add to scheduler queue
  - Folder: Start filesystem watcher
        ↓
Pipeline is ACTIVE
```

### Multi-Trigger Limitation

**Current Version:** Each pipeline can have exactly ONE trigger.

**Rationale:** Simplifies execution model and I/O wiring.

**Future:** Multiple triggers per pipeline may be supported with explicit trigger-to-input mapping.

### Trigger Composition

Pipelines can be composed using package imports:

```polyglot
%% shared-processor.pg
{|} |ProcessData
[<] <input:string
[>] >output:string
[r] ... processing logic
{x}

%% cli-frontend.pg
[@] @Processor << @Local:SharedProcessor:0.0.5.0
{x}

{|} |CliEntry
[t] |T.Cli
 |  <cmd:string << "process"
 |  <kwargs.input:string << <input

[<] <input:string
[>] >result:string

[r] @Processor|ProcessData
 |  <input:string << $input
 |  >output:string >> $result

[>] >result << $result
{x}
```

This pattern allows triggered "entry point" pipelines to delegate to shared processing logic.

### Testing Triggers

**Unit Testing:**
- Test pipeline logic independently of trigger
- Mock trigger inputs as pipeline inputs

**Integration Testing:**
- CLI: Invoke via `polyglot run <command>`
- HTTP: Send HTTP requests to endpoints
- Cron/Interval: Override scheduler for faster execution
- Folder: Create test files in watched directory

**Load Testing:**
- HTTP triggers: Use load testing tools (wrk, ab, hey)
- Folder triggers: Create file bursts, verify batching
- Queue: Verify backpressure and timeout behavior

---

## See Also

### Related Documentation

- **[Trigger System User Guide](trigger-system.md)** - Usage patterns and examples
- **[Trigger Dev Reference](trigger-dev-reference.md)** - Formal specifications
- **[Standard Triggers YAML](../stdlib/standard-triggers.yaml)** - Complete trigger specs

### Implementation References

- Trigger Engine Architecture (internal docs)
- Scheduler Design (internal docs)
- Wrapper System Design (internal docs)

---

**Document Status:** ✅ Complete
**Training Session:** 2026-01-05
**Lines:** 586

**Generated by:** Polly Language Expert
**For:** Scribe Documentation Architect
