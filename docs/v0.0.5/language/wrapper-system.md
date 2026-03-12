# Wrapper System Guide - v0.0.5

**Version:** 0.0.5
**Last Updated:** 2026-01-11
**Status:** Official Documentation

---

## Table of Contents

1. [Introduction](#introduction)
2. [What Are Wrappers?](#what-are-wrappers)
3. [Universal Wrapper Pattern](#universal-wrapper-pattern)
4. [Database Wrappers](#database-wrappers)
5. [Runtime Wrappers](#runtime-wrappers)
6. [HTTP Wrappers](#http-wrappers)
7. [File System Wrappers](#file-system-wrappers)
8. [Error Handling](#error-handling)
9. [Resource Management](#resource-management)
10. [Best Practices](#best-practices)
11. [Common Mistakes](#common-mistakes)
12. [Complete Examples](#complete-examples)

---

## Introduction

Wrappers are Polyglot's solution for **resource lifecycle management**. They provide automatic setup, cleanup, and error handling for resources like database connections, runtime environments, HTTP clients, and file locks.

### Key Benefits

- **Automatic Resource Cleanup** - No manual connection closing or resource freeing
- **Consistent Error Handling** - Unified error patterns across all resource types
- **Pipeline Isolation** - Resources scoped to pipeline lifetime
- **Type Safety** - Reserved schema validation for resource handles

---

## What Are Wrappers?

Wrappers use the `[w]` marker and manage resources that need:

1. **Initialization** - Setup (connect to database, create HTTP client)
2. **Lifetime** - Exist throughout pipeline execution
3. **Cleanup** - Automatic teardown when pipeline exits

### Wrapper vs. Regular Pipeline

| Aspect | Wrapper `[w]` | Pipeline `[r]` |
|--------|--------------|----------------|
| **Lifecycle** | Entire pipeline | Single execution |
| **Cleanup** | Automatic | Not applicable |
| **Error Scope** | Can exit pipeline | Continues unless `[x]` |
| **Position** | Before execution | Anywhere in execution |
| **Resource Management** | Yes | No |

---

## Universal Wrapper Pattern

All wrappers follow this structure:

```polyglot
[w] |W.{Category}.{Type}
 |  <input_params...>
 |  >output-ReservedSchema >> $variable
   [!] !{Category}.{ErrorType}
      [>] >error_output
         [.] .field << value
      [x]  %% Exit pipeline on critical error
   [!] !*
      |U.Do.Nothing  %% Continue on success
```

### Critical Rules

1. **ALL `[w]` markers MUST come BEFORE execution markers** (`[r]`, `[f]`, `[p]`, etc.)
2. **Error handlers are INSIDE the wrapper block** (indented)
3. **Outputs use reserved schemas** (`>field-SchemaName`)
4. **Resources auto-cleanup on `{x}`** (pipeline exit)

### Pipeline Structure Order

```
{|} → [t] → [<] → [>] → [w] → [r][p][*][b][f][s][>]
     └─────────────────┘       └──────────────────┘
          Metadata             Execution Section
         & Wrappers
```

---

## Database Wrappers

### W.DB.Connect - Generic Database Connection

**Purpose:** Connect to any SQL database with connection parameters.

**Reserved Schema:** `-DB-Connection`

**Basic Usage:**

```polyglot
[w] |W.DB.Connect
 |  <host:string << "localhost"
 |  <port:int << 5432
 |  <database:string << "myapp"
 |  <user:string << "admin"
 |  <password:string << "secret"
 |  >db-DB-Connection >> $dbConnection
   [!] !DB.ConnectionError
      [>] >result
         [.] .success:bool << -False
         [.] .error:string << "Database connection failed"
      [x]
   [!] !DB.AuthenticationError
      [>] >result
         [.] .success:bool << -False
         [.] .error:string << "Authentication failed"
      [x]
   [!] !*
      |U.Do.Nothing
```

### W.DB.Postgresql - PostgreSQL-Specific

**Purpose:** Connect using schema-based configuration.

**Reserved Schema:** `-DB-Connection`

**Usage:**

```polyglot
{#} #DatabaseConfig
[A] #ProductionDB
[s] << |YAML.Load"\\FileDir\\db-config.yaml"
   [.] .host:string << .database.host
   [.] .port:int << .database.port
   [.] .database:string << .database.name
   [.] .user:string << .database.credentials.user
   [.] .password:string << .database.credentials.password
{x}

{|} |QueryData
[w] |W.DB.Postgresql
 |  <db << #ProductionDB
 |  >db-DB-Connection >> $dbConnection
   [!] !DB.ConnectionError
      [>] >error << "PostgreSQL connection failed"
      [x]
   [!] !*
      |U.Do.Nothing
{x}
```

### Executing Database Queries

**Pipeline:** `|DB.Query`

**Pattern:**

```polyglot
%% Build SQL query with interpolation
[r] $userId << 42
[r] $query << |SQL"SELECT * FROM users WHERE id = ?"

%% Execute query
[r] |DB.Query
 |  <connection-DB-Connection << $dbConnection
 |  <query:string << $query
 |  <args:array.string << ($userId)
 |  >rows:array.serial >> $results
 |  >row_count:uint >> $rowCount
   [!] !DB.QueryError
      [>] >error << "Query execution failed"
      [x]
   [!] !*
      |U.Do.Nothing
```

**Key Points:**
- SQL queries use `|SQL"..."` literal
- Connection: `<connection-DB-Connection`
- Arguments: `<args:array.string` (all parameters as strings)
- Results: `>rows:array.serial` (array of row objects)

---

## Runtime Wrappers

Runtime wrappers enable executing Python, Rust, or JavaScript code from Polyglot pipelines.

### Pattern Overview

1. **Create runtime environment** with `[w] |W.RT.{Language}`
2. **Execute code** with `[r] |RT.{Language}.Code`
3. **Pass data** via environment variables (all `:string`)
4. **Multi-line code** using `[c]` marker

### W.RT.Python - Python Runtime

**Reserved Schema:** `-RTenv-python`

**Complete Example:**

```polyglot
{|} |ProcessData
[t] |T.Cli
 |  <cmd:string << process
 |  >kwargs.input_file:string >> <input_file

[<] <input_file:string
[>] >result:serial

%% Create Python environment
[w] |W.RT.Python
 |  <version:string << "3.11"
 |  >environment-RTenv-python >> $pyEnv
   [!] !RT.Python.InitError
      [>] >result
         [.] .success:bool << -False
         [.] .error:string << "Python initialization failed"
      [x]
   [!] !*
      |U.Do.Nothing

%% Execute Python code
[r] |RT.Python.Code
 |  <env.lang-RTenv-python << $pyEnv
 |  <env.vars.input_file:string << $input_file
 |  <env.vars.output_file:string << "\tmp\output.json"
 |  <code:string << |Python""
[c] import os
[c] import json
[c]
[c] input_file = os.environ['input_file']
[c] output_file = os.environ['output_file']
[c]
[c] with open(input_file, 'r') as f:
[c]     data = json.load(f)
[c]
[c] processed = {k: v.upper() for k, v in data.items()}
[c]
[c] with open(output_file, 'w') as f:
[c]     json.dump(processed, f, indent=2)
[c]
[c] print("Processing complete")
 |  >exit_code:uint >> $exitCode
   [!] !RT.Python.Error
      [>] >result
         [.] .success:bool << -False
         [.] .error:string << "Python execution failed"
      [x]
   [!] !*
      [>] >result
         [.] .success:bool << -True
         [.] .message:string << "Processing complete"

{x}
```

### W.RT.Rust - Rust Runtime

**Reserved Schema:** `-RTenv-rust`

**Example:**

```polyglot
[w] |W.RT.Rust
 |  <version:string << "1.75"
 |  >environment-RTenv-rust >> $rustEnv
   [!] !RT.Rust.InitError
      [>] >error << "Rust initialization failed"
      [x]
   [!] !*
      |U.Do.Nothing

[r] |RT.Rust.Code
 |  <env.lang-RTenv-rust << $rustEnv
 |  <env.vars.output_path:string << $outputPath
 |  <code:string << |Rust""
[c] use std::env;
[c] use std::fs;
[c]
[c] let path = env::var("output_path").unwrap();
[c] let content = "Hello from Rust!";
[c] fs::write(path, content).expect("Failed to write");
 |  >exit_code:uint >> $exitCode
   [!] !RT.Rust.Error
      [>] >error << "Rust execution failed"
      [x]
   [!] !*
      |U.Do.Nothing
```

### W.RT.JavaScript - JavaScript/Node.js Runtime

**Reserved Schema:** `-RTenv-javascript`

**Example:**

```polyglot
[w] |W.RT.JavaScript
 |  <version:string << "20"
 |  >environment-RTenv-javascript >> $jsEnv
   [!] !RT.JavaScript.InitError
      [>] >error << "JavaScript initialization failed"
      [x]
   [!] !*
      |U.Do.Nothing

[r] |RT.JavaScript.Code
 |  <env.lang-RTenv-javascript << $jsEnv
 |  <env.vars.config_path:string << $configPath
 |  <code:string << |JavaScript""
[c] const fs = require('fs');
[c] const configPath = process.env.config_path;
[c]
[c] const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
[c] console.log(`Loaded config: ${config.name}`);
 |  >exit_code:uint >> $exitCode
   [!] !RT.JavaScript.Error
      [>] >error << "JavaScript execution failed"
      [x]
   [!] !*
      |U.Do.Nothing
```

### Environment Variables - Critical Rules

**ALL environment variables MUST be `:string` type:**

```polyglot
%% CORRECT
<env.vars.port:string << "5432"
<env.vars.debug:string << "true"
<env.vars.count:string << "100"

%% WRONG - Will cause errors
<env.vars.port:int << 5432      %% ❌ Not :string
<env.vars.debug:bool << -True   %% ❌ Not :string
```

**Why?** Environment variables are stored as strings in the OS. Runtime code performs type conversion:

```python
# Python - convert in code
port = int(os.environ['port'])
debug = os.environ['debug'] == 'true'
```

```rust
// Rust - convert in code
let port: u16 = env::var("port").unwrap().parse().unwrap();
let debug: bool = env::var("debug").unwrap() == "true";
```

---

## HTTP Wrappers

### W.HTTP.Client - HTTP Client with Connection Pooling

**Purpose:** Create HTTP client for making requests.

**Reserved Schema:** `-HTTP-Client`

**Complete Example:**

```polyglot
{|} |FetchUserProfile
[t] |T.Cli
 |  <cmd:string << fetch-profile
 |  >kwargs.user_id:uint >> <user_id

[<] <user_id:uint
[>] >profile:serial

%% Initialize HTTP client
[w] |W.HTTP.Client
 |  <base_url:string << "https://api.example.com"
 |  <timeout:int << 30
 |  >client-HTTP-Client >> $httpClient
   [!] !HTTP.ClientError
      [>] >profile
         [.] .success:bool << -False
         [.] .error:string << "Failed to create HTTP client"
      [x]
   [!] !*
      |U.Do.Nothing

%% Make GET request with route interpolation
[r] |HTTP.Get
 |  <client-HTTP-Client << $httpClient
 |  <route:string << "\api\v2\users\{$user_id}\profile"
 |  >results:serial >> $response
   [!] !HTTP.RequestError
      [>] >profile
         [.] .success:bool << -False
         [.] .error:string << "API request failed"
      [x]
   [!] !*
      |U.Do.Nothing

%% Return successful result
[>] >profile
   [.] .success:bool << -True
   [.] .data << $response

{x}
```

### String Interpolation in Routes

**Pattern:** `"\text\{$variable}\more"`

```polyglot
[r] $userId << 42
[r] $postId << 100

%% Interpolate variables into route
[r] $route << "\api\users\{$userId}\posts\{$postId}"
%% Result: "\api\users\42\posts\100"

[r] |HTTP.Get
 |  <client-HTTP-Client << $client
 |  <route:string << $route
 |  >results:serial >> $data
```

### HTTP Request Methods

All HTTP methods follow the same pattern:

```polyglot
[r] |HTTP.Get       %% GET request
[r] |HTTP.Post      %% POST request
[r] |HTTP.Put       %% PUT request
[r] |HTTP.Delete    %% DELETE request
[r] |HTTP.Patch     %% PATCH request
```

### Response Validation

Use `#?` operator to validate responses:

```polyglot
%% Define expected response schema
{#} #UserProfile
[A] #ValidProfile
[s] << {:}
   [.] .id:uint
   [.] .name:string
   [.] .email:string
{x}

%% Validate API response
[r] |HTTP.Get
 |  <client-HTTP-Client << $client
 |  <route:string << "\users\{$userId}"
 |  >results:serial >> $response

%% Validate structure
[r] $validated << $response #? #UserProfile
```

---

## File System Wrappers

### W.File.Lock - Exclusive File Access

**Purpose:** Acquire file lock for safe concurrent access.

**Reserved Schema:** `-File-Lock`

**Usage:**

```polyglot
{|} |ProcessLogFile
[<] <log_path:path

%% Acquire exclusive file lock
[w] |W.File.Lock
 |  <path:path << $log_path
 |  >lock-File-Lock >> $fileLock
   [!] !File.LockError
      [>] >error << "Could not acquire file lock"
      [x]
   [!] !*
      |U.Do.Nothing

%% Safe to read/write - lock held
[r] |File.Read
 |  <path:path << $log_path
 |  >content:string >> $logData

[r] |File.Write
 |  <path:path << $log_path
 |  <content:string << $updatedData

%% Lock automatically released on {x}
{x}
```

**Key Points:**
- Lock automatically released when pipeline exits
- Prevents concurrent access by other processes
- Use for critical file operations

---

## Error Handling

### Universal Error Pattern

All wrappers support error handlers **inside the wrapper block**:

```polyglot
[w] |W.{Category}.{Type}
 |  <inputs...>
 |  >output-Schema >> $var
   [!] !{Specific}.Error
      %% Handle specific error
      [>] >error_output
      [x]  %% Exit pipeline if critical
   [!] !{Another}.Error
      %% Handle another error type
      [>] >error_output
      [x]
   [!] !*
      %% Success case - continue execution
      |U.Do.Nothing
```

### Error Handler Rules

1. **Indentation:** Error handlers are INSIDE wrapper block (3 spaces)
2. **Exit Control:** Use `[x]` to exit pipeline on critical errors
3. **Success Case:** Always include `[!] !*` for success continuation
4. **Order:** Specific errors first, `!*` last

### Common Error Types by Category

**Database:**
- `!DB.ConnectionError` - Connection failed
- `!DB.AuthenticationError` - Auth failed
- `!DB.QueryError` - Query execution failed
- `!DB.TimeoutError` - Operation timed out

**Runtime:**
- `!RT.{Language}.InitError` - Runtime initialization failed
- `!RT.{Language}.Error` - Code execution failed
- `!RT.{Language}.CompileError` - Compilation failed (Rust)

**HTTP:**
- `!HTTP.ClientError` - Client initialization failed
- `!HTTP.RequestError` - Request failed
- `!HTTP.TimeoutError` - Request timed out
- `!HTTP.StatusError` - Non-2xx status code

**File:**
- `!File.LockError` - Could not acquire lock
- `!File.NotFoundError` - File not found
- `!File.PermissionError` - Permission denied

---

## Resource Management

### Automatic Cleanup

Wrappers automatically clean up resources when the pipeline exits (`{x}`):

```polyglot
{|} |ProcessWithResources

%% Acquire multiple resources
[w] |W.DB.Connect >> $db
[w] |W.HTTP.Client >> $http
[w] |W.File.Lock >> $lock

%% Use resources
[r] |DB.Query...
[r] |HTTP.Get...
[r] |File.Write...

%% ALL resources automatically cleaned up here
{x}
```

**Cleanup Order:** Resources cleaned up in **reverse order** of acquisition (stack-based).

### Cleanup Guarantees

| Resource | Cleanup Action |
|----------|----------------|
| **Database** | Connections closed, transactions rolled back |
| **Runtime** | Environments terminated, temp files removed |
| **HTTP** | Connections closed, pooled connections returned |
| **File** | Locks released, handles closed |

### Pipeline Exit Points

Resources cleaned up on:

1. **Normal completion** - Reaching `{x}`
2. **Error exit** - `[x]` in error handler
3. **Early return** - `[>]` with `[x]`

---

## Best Practices

### 1. Always Provide Error Handlers

```polyglot
%% GOOD - Error handling
[w] |W.DB.Connect
 |  >db-DB-Connection >> $db
   [!] !DB.ConnectionError
      [>] >error << "Connection failed"
      [x]
   [!] !*
      |U.Do.Nothing

%% RISKY - No error handling (connection failures unhandled)
[w] |W.DB.Connect
 |  >db-DB-Connection >> $db
```

### 2. Order Wrappers Logically

```polyglot
%% GOOD - Config first, then resources
[w] |W.Config.Load >> $config
[w] |W.DB.Connect
 |  <host << $config.db_host
 |  >db-DB-Connection >> $db

%% WRONG - Can't use $config (not declared yet)
[w] |W.DB.Connect
 |  <host << $config.db_host  %% ❌ $config not available
[w] |W.Config.Load >> $config
```

### 3. Use Schema Validation

```polyglot
%% Validate external data
[r] |HTTP.Get
 |  <client-HTTP-Client << $client
 |  <route:string << "\data"
 |  >results:serial >> $data

%% Validate before use
[r] $validated << $data #? #ExpectedSchema

[f] $validated ?= -False
   [>] >error << "Invalid data structure"
   [x]
```

### 4. Minimize Wrapper Scope

```polyglot
%% GOOD - Short-lived connection
{|} |QuickQuery
[w] |W.DB.Connect >> $db
[r] |DB.Query...
{x}

%% RISKY - Long-lived connection (holds resources)
{|} |LongRunningPipeline
[w] |W.DB.Connect >> $db
%% ... many operations ...
%% ... connection held entire time ...
{x}
```

### 5. Group Related Operations

```polyglot
%% GOOD - Separate pipelines for separate resources
{|} |DatabaseOps
[w] |W.DB.Connect >> $db
[r] |DB.Query...
{x}

{|} |HTTPOps
[w] |W.HTTP.Client >> $client
[r] |HTTP.Get...
{x}

%% AVOID - Mixing unrelated resources
{|} |MixedOps
[w] |W.DB.Connect >> $db
[w] |W.HTTP.Client >> $client
%% Holding both resources unnecessarily
{x}
```

---

## Common Mistakes

### ❌ Mistake 1: Wrapper After Execution

```polyglot
%% WRONG - Wrapper after execution marker
{|} |BadOrder
[r] $value << 42          %% Execution marker
[w] |W.DB.Connect >> $db  %% ❌ COMPILE ERROR
{x}
```

**Fix:**
```polyglot
%% CORRECT - All wrappers before execution
{|} |GoodOrder
[w] |W.DB.Connect >> $db  %% Wrapper first
[r] $value << 42          %% Execution after
{x}
```

### ❌ Mistake 2: Wrong Output Type

```polyglot
%% WRONG - Using :serial instead of reserved schema
[w] |W.DB.Connect
 |  >db:serial >> $db  %% ❌ Not type-safe
```

**Fix:**
```polyglot
%% CORRECT - Using reserved schema
[w] |W.DB.Connect
 |  >db-DB-Connection >> $db  %% ✅ Type-safe
```

### ❌ Mistake 3: Environment Variable Types

```polyglot
%% WRONG - Using non-string types
[r] |RT.Python.Code
 |  <env.lang-RTenv-python << $pyEnv
 |  <env.vars.port:int << 5432      %% ❌ Must be :string
 |  <env.vars.debug:bool << -True   %% ❌ Must be :string
```

**Fix:**
```polyglot
%% CORRECT - All environment variables as :string
[r] |RT.Python.Code
 |  <env.lang-RTenv-python << $pyEnv
 |  <env.vars.port:string << "5432"    %% ✅ String
 |  <env.vars.debug:string << "true"   %% ✅ String
```

### ❌ Mistake 4: Missing Success Handler

```polyglot
%% INCOMPLETE - No success handler
[w] |W.DB.Connect
 |  >db-DB-Connection >> $db
   [!] !DB.ConnectionError
      [>] >error << "Failed"
      [x]
   %% ❌ Missing [!] !* handler
```

**Fix:**
```polyglot
%% COMPLETE - Success handler included
[w] |W.DB.Connect
 |  >db-DB-Connection >> $db
   [!] !DB.ConnectionError
      [>] >error << "Failed"
      [x]
   [!] !*
      |U.Do.Nothing  %% ✅ Continue on success
```

### ❌ Mistake 5: Manual Cleanup

```polyglot
%% WRONG - Attempting manual cleanup (not needed)
[r] |DB.Close
 |  <connection-DB-Connection << $db  %% ❌ Not necessary
```

**Fix:**
```polyglot
%% CORRECT - Let wrapper handle cleanup automatically
%% Just exit the pipeline
{x}  %% ✅ Automatic cleanup
```

---

## Complete Examples

### Example 1: Multi-Runtime Data Pipeline

Process data with Python, transform with Rust, upload with HTTP:

```polyglot
{@} @Local:DataPipeline:1.0.0
{x}

{|} |ProcessAndUpload
[t] |T.Cli
 |  <cmd:string << process-upload
 |  >kwargs.input_file:string >> <input_file

[<] <input_file:string
[>] >result:serial

%% ALL WRAPPERS FIRST
[w] |W.RT.Python
 |  <version:string << "3.11"
 |  >environment-RTenv-python >> $pyEnv
   [!] !RT.Python.InitError
      [>] >result
         [.] .success:bool << -False
         [.] .stage:string << "python_init"
      [x]
   [!] !*
      |U.Do.Nothing

[w] |W.RT.Rust
 |  <version:string << "1.75"
 |  >environment-RTenv-rust >> $rustEnv
   [!] !RT.Rust.InitError
      [>] >result
         [.] .success:bool << -False
         [.] .stage:string << "rust_init"
      [x]
   [!] !*
      |U.Do.Nothing

[w] |W.HTTP.Client
 |  <base_url:string << "https://api.example.com"
 |  <timeout:int << 60
 |  >client-HTTP-Client >> $httpClient
   [!] !HTTP.ClientError
      [>] >result
         [.] .success:bool << -False
         [.] .stage:string << "http_init"
      [x]
   [!] !*
      |U.Do.Nothing

%% EXECUTION SECTION

%% 1. Parse CSV with Python
[r] |RT.Python.Code
 |  <env.lang-RTenv-python << $pyEnv
 |  <env.vars.input_file:string << $input_file
 |  <env.vars.temp_json:string << "\tmp\data.json"
 |  <code:string << |Python""
[c] import csv
[c] import json
[c] import os
[c]
[c] input_file = os.environ['input_file']
[c] temp_json = os.environ['temp_json']
[c]
[c] with open(input_file, 'r') as f:
[c]     reader = csv.DictReader(f)
[c]     data = list(reader)
[c]
[c] with open(temp_json, 'w') as f:
[c]     json.dump(data, f)
 |  >exit_code:uint >> $pyExit
   [!] !RT.Python.Error
      [>] >result
         [.] .success:bool << -False
         [.] .stage:string << "python_parse"
      [x]
   [!] !*
      |U.Do.Nothing

%% 2. Transform data with Rust
[r] |RT.Rust.Code
 |  <env.lang-RTenv-rust << $rustEnv
 |  <env.vars.input_json:string << "\tmp\data.json"
 |  <env.vars.output_json:string << "\tmp\transformed.json"
 |  <code:string << |Rust""
[c] use std::env;
[c] use std::fs;
[c] use serde_json::{Value, json};
[c]
[c] let input = env::var("input_json").unwrap();
[c] let output = env::var("output_json").unwrap();
[c]
[c] let data: Vec<Value> = serde_json::from_str(
[c]     &fs::read_to_string(input).unwrap()
[c] ).unwrap();
[c]
[c] let transformed: Vec<Value> = data.iter()
[c]     .map(|item| json!({"processed": true, "data": item}))
[c]     .collect();
[c]
[c] fs::write(output, serde_json::to_string(&transformed).unwrap()).unwrap();
 |  >exit_code:uint >> $rustExit
   [!] !RT.Rust.Error
      [>] >result
         [.] .success:bool << -False
         [.] .stage:string << "rust_transform"
      [x]
   [!] !*
      |U.Do.Nothing

%% 3. Upload via HTTP
[r] |File.Read
 |  <path:path << "\tmp\transformed.json"
 |  >content:string >> $jsonData

[r] |HTTP.Post
 |  <client-HTTP-Client << $httpClient
 |  <route:string << "\api\data\upload"
 |  <body:string << $jsonData
 |  >results:serial >> $uploadResponse
   [!] !HTTP.RequestError
      [>] >result
         [.] .success:bool << -False
         [.] .stage:string << "http_upload"
      [x]
   [!] !*
      [>] >result
         [.] .success:bool << -True
         [.] .stage:string << "complete"
         [.] .response << $uploadResponse

{x}
```

### Example 2: Database Transaction Pattern

Query database, validate, process:

```polyglot
{@} @Local:UserManagement:1.0.0
{x}

{#} #DatabaseConfig
[A] #ProductionDB
[s] << |YAML.Load"\\FileDir\\db.yaml"
   [.] .host:string << .database.host
   [.] .port:int << .database.port
   [.] .database:string << .database.name
   [.] .user:string << .database.user
   [.] .password:string << .database.password
{x}

{|} |UpdateUserStatus
[t] |T.Cli
 |  <cmd:string << update-status
 |  >kwargs.user_id:uint >> <user_id
 |  >kwargs.new_status:string >> <new_status

[<] <user_id:uint
[<] <new_status:string
[<] <db_config#DatabaseConfig
[>] >result:serial

%% Connect to database
[w] |W.DB.Connect
 |  <host:string << $db_config.host
 |  <port:int << $db_config.port
 |  <database:string << $db_config.database
 |  <user:string << $db_config.user
 |  <password:string << $db_config.password
 |  >db-DB-Connection >> $dbConnection
   [!] !DB.ConnectionError
      [>] >result
         [.] .success:bool << -False
         [.] .error:string << "Database connection failed"
      [x]
   [!] !*
      |U.Do.Nothing

%% Check user exists
[r] $checkQuery << |SQL"SELECT id FROM users WHERE id = ?"

[r] |DB.Query
 |  <connection-DB-Connection << $dbConnection
 |  <query:string << $checkQuery
 |  <args:array.string << ($user_id)
 |  >rows:array.serial >> $checkResults
 |  >row_count:uint >> $userExists
   [!] !DB.QueryError
      [>] >result
         [.] .success:bool << -False
         [.] .error:string << "Query failed"
      [x]
   [!] !*
      |U.Do.Nothing

%% Validate user exists
[f] $userExists ?= 0
   [>] >result
      [.] .success:bool << -False
      [.] .error:string << "User not found"
   [x]

%% Update user status
[r] $updateQuery << |SQL"UPDATE users SET status = ? WHERE id = ?"

[r] |DB.Query
 |  <connection-DB-Connection << $dbConnection
 |  <query:string << $updateQuery
 |  <args:array.string << ($new_status, $user_id)
 |  >rows:array.serial >> $updateResults
 |  >row_count:uint >> $rowsAffected
   [!] !DB.QueryError
      [>] >result
         [.] .success:bool << -False
         [.] .error:string << "Update failed"
      [x]
   [!] !*
      |U.Do.Nothing

%% Return success
[>] >result
   [.] .success:bool << -True
   [.] .rows_affected:uint << $rowsAffected
   [.] .message:string << "User status updated"

{x}
```

---

## See Also

- [Runtime Orchestration Guide](../quick-reference/runtime-orchestration.md)
- [Loop System Guide](./loop-system.md)
- [Error Handling Guide](./error-handling.md)
- [Standard Wrappers Reference](../stdlib/standard-wrappers.yaml)
- [Reserved Schemas](../stdlib/reserved-enums.yaml)
- [Hello World Example](../examples/hello-world-multi-runtime.pg)

---

**Status:** ✅ Official Documentation
**Version:** 0.0.5
**Last Updated:** 2026-01-11
**Contributors:** Scribe Documentation Architect, Polly Language Expert
