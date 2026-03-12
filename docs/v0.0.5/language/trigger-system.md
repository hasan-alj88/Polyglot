# Trigger System Guide

**Version:** v0.0.5
**Audience:** Polyglot Language Users
**Last Updated:** 2026-01-05
**Confidence:** Verified (V)

---

## Table of Contents

1. [Overview](#overview)
2. [Trigger Fundamentals](#trigger-fundamentals)
3. [Trigger Types](#trigger-types)
   - [CLI Triggers](#cli-triggers)
   - [Cron Triggers](#cron-triggers)
   - [Interval Triggers](#interval-triggers)
   - [Folder Triggers](#folder-triggers)
   - [HTTP Triggers](#http-triggers)
   - [Calendar Triggers](#calendar-triggers)
4. [Trigger I/O Wiring](#trigger-io-wiring)
5. [Wrappers and Sessions](#wrappers-and-sessions)
6. [Error Handling](#error-handling)
7. [Real-World Examples](#real-world-examples)
8. [Best Practices](#best-practices)
9. [Common Patterns](#common-patterns)
10. [See Also](#see-also)

---

## Overview

Triggers are the entry points to Polyglot pipelines, enabling automation through various event sources. They capture external events (CLI commands, HTTP requests, scheduled times, file changes) and transform them into pipeline executions.

### What Triggers Do

- **Capture Events:** Listen for CLI commands, cron schedules, file changes, HTTP requests
- **Provide Data:** Pass event data (arguments, timestamps, file paths, request bodies) to pipelines
- **Enable Automation:** Run pipelines automatically without manual invocation
- **Support Integration:** Connect Polyglot to external systems and workflows

### When to Use Triggers

Use triggers when you need to:
- Run pipelines on a schedule (daily backups, weekly reports)
- Respond to external events (new files, HTTP requests)
- Build CLI tools with argument parsing
- Automate recurring tasks (health checks, data processing)
- Create event-driven workflows

---

## Trigger Fundamentals

### Pipeline Structure with Triggers

Triggers are declared in pipelines using the `[t]` marker. The critical block ordering is:

```polyglot
{|} |PipelineName              %% Pipeline definition
[%] %Doc << "Description"

[t] |T.TriggerType             %% Trigger declaration
 |  <config << value
 |  >output >> <pipeline_input

[<] <pipeline_input:type       %% Pipeline inputs
[>] >pipeline_output:type      %% Pipeline outputs

[w] |W.WrapperType             %% Wrappers (optional)
 |  >session >> $session_var

[Q] Queue configuration        %% Queue (optional)

[r] ... execution statements   %% Pipeline logic
[>] >pipeline_output << $value

{x}                            %% Close pipeline
```

**Critical Rules:**
1. **Trigger** `[t]` comes AFTER pipeline definition `{|}`
2. **Inputs** `[<]` and **outputs** `[>]` come AFTER trigger
3. **Wrappers** `[w]` come AFTER inputs/outputs
4. **Queue** `[Q]` comes AFTER wrappers (optional)
5. **Execution** statements come AFTER all declarations

### Trigger I/O Markers

Triggers use the I/O marker ` | ` (space-pipe-space) for configuration and data flow:

```polyglot
[t] |T.Cli
 |  <cmd:string << "greet"           %% Configuration input
 |  <kwargs.name:string << <name     %% Wire trigger kwarg to pipeline input
 |  >args:serial >> <all_args        %% Wire trigger output to pipeline input
```

**Marker Syntax:**
- ` | ` (space-pipe-space) - Used for trigger I/O
- `~` (tilde) - Used for loop operations (NOT for triggers)

---

## Trigger Types

Polyglot v0.0.5 provides 6 trigger types for different automation scenarios.

### CLI Triggers

CLI triggers capture command-line arguments and execute pipelines.

#### Basic CLI Trigger

```polyglot
{@} @Local:Examples.SimpleCli:0.0.5.0
{x}

{|} |Greet
[%] %Doc << "Simple CLI trigger that greets a user"

[t] |T.Cli
 |  <cmd:string << "greet"
 |  <kwargs.name:string << <name

[<] <name:string
[>] >message:string

[w] |W.Console
 |  >session >> $console_session

[r] $greeting:string << "Hello, {$name}!"

[r] |U.Console"Hello, {$name}!"

[>] >message << $greeting
{x}
```

**Usage:**
```bash
polyglot run greet --name "Hasan"
```

**Output:**
```
Hello, Hasan!
```

#### Advanced CLI Trigger

CLI triggers can capture multiple kwargs and output all arguments as a serial:

```polyglot
{|} |ProcessCommand
[%] %Doc << "CLI trigger with advanced I/O wiring"

[t] |T.Cli
 |  <cmd:string << "process"
 |  <kwargs.action:string << <action
 |  <kwargs.target:string << <target
 |  >args:serial >> <command_args

[<] <action:string
[<] <target:string
[<] <command_args:serial

[>] >result:string
[>] >status:bool

[w] |W.Console
 |  >session >> $console_session

[r] $result_msg:string << "Action: {$action}, Target: {$target}"

[r] |U.Console"Executing: {$action} on {$target}"

%% Access args from serial
[r] $all_args:string << $command_args.all

[r] |U.Console"All arguments: {$all_args}"

[>] >result << $result_msg
[>] >status << -True
{x}
```

**Usage:**
```bash
polyglot run process --action "deploy" --target "production"
```

**Key Features:**
- Individual kwargs wire to specific inputs: `<kwargs.action:string << <action`
- Trigger outputs full serial with all args: `>args:serial >> <command_args`
- Multiple pipeline outputs supported
- Reserved enum `-True` for boolean values

---

### Cron Triggers

Cron triggers execute pipelines on a schedule using cron expressions.

#### Basic Cron Trigger

```polyglot
{|} |DailyBackup
[%] %Doc << "Run daily backup at midnight"

[t] |T.Cron
 |  <schedule:string << "0 0 * * *"
 |  >timestamp:dt >> <backup_time

[<] <backup_time:dt

[>] >status:bool
[>] >backup_path:path

[w] |W.File
 |  >session-Session-File >> $file_session

[r] $timestamp_str:string << |DT.Format
 |  <datetime:dt << $backup_time
 |  <format:string << "yyyy-MM-dd"
 |  >formatted >> $timestamp_str

[r] $backup_path:path << \\FileDir\\backups\\backup-{$timestamp_str}.tar.gz

[r] |U.File.Archive.Create
 |  <source:path << \\FileDir\\data\
 |  <destination:path << $backup_path
 |  >success:bool >> $success
   [!] !File.Archive.Error
      [>] >status << -False
   [!] !No.Error
      [>] >status << -True
   [!] !*
      [>] >status << -False

[>] >backup_path << $backup_path
{x}
```

**Cron Expression:** `"0 0 * * *"` (midnight every day)

#### Daily Trigger Alternative

For simpler daily scheduling, use the `|T.DT.Daily` helper:

```polyglot
[t] |T.DT.Daily"12AM"
 |  >timestamp:dt >> <backup_time
```

**Key Features:**
- Cron expressions for flexible scheduling
- Trigger outputs timestamp of execution
- File wrapper with reserved session enum: `-Session-File`
- Error handling with 3-part structure

---

### Interval Triggers

Interval triggers execute pipelines repeatedly at fixed time intervals.

#### Health Check Example

```polyglot
{#} #Status
[.] .Healthy:string << "healthy"
[.] .Unhealthy:string << "unhealthy"
[.] .Degraded:string << "degraded"
{x}

{|} |HealthCheck
[%] %Doc << "Check service health every 5 minutes"

[t] |T.Interval
 |  <duration:string << "5m"
 |  >timestamp:dt >> <check_time

[<] <check_time:dt

[>] >status#Status

[w] |W.HTTP
 |  >session-Session-HTTP >> $http_session

[r] $log_file:path << \\FileDir\\health.log

[w] |W.File
 |  <file:path << $log_file
 |  >session-Session-File >> $file_session

[r] |HTTP.Get
 |  <url:string << "http://localhost:8080/health"
 |  >status_code:uint >> $status_code
 |  >body:string >> $body
   [!] !HTTP.Error
      [r] $log_entry:string << "[{$check_time}] Service DOWN: HTTP error"

      [r] |U.File.Text.Append
       |  <file:path << $log_file
       |  <content:string << "{$log_entry}\n"

      [>] >status << #Status.Unhealthy
   [!] !No.Error
      [f] $status_code =? 200
         [r] $log_entry:string << "[{$check_time}] Service UP: {$body}"
         [>] >status << #Status.Healthy
      [f] $status_code =!? 200
         [r] $log_entry:string << "[{$check_time}] Service DEGRADED: Status {$status_code}"
         [>] >status << #Status.Degraded

      [r] |U.File.Text.Append
       |  <file:path << $log_file
       |  <content:string << "{$log_entry}\n"
   [!] !*
      [>] >status << #Status.Unhealthy
{x}
```

**Duration Format:** `"5m"` (5 minutes), `"1h"` (1 hour), `"30s"` (30 seconds)

**Key Features:**
- Duration string for interval configuration
- Multiple wrappers: HTTP + File
- File wrapper requires file path: `<file:path << $log_file`
- Use enums for status values, not strings
- Comparison operators: `=?` (equal), `=!?` (not equal)

---

### Folder Triggers

Folder triggers watch directories and execute pipelines when new files appear.

#### File Processing Example

```polyglot
{@} @Local:Examples.FolderWatcher:0.0.5.0
[@] @LogP << @Community:Log.Processer:0.0.5.0
{x}

{#} #ProcessResult
[.] .file:path
[.] .success:bool
[.] .message:string
{x}

{|} |ProcessNewLogs
[%] %Doc << "Process new log files as they arrive"

[t] |T.Folder.NewFiles
 |  <folder:path << \\FileDir\\incoming_logs\
 |  >files:array.path >> <new_files

[<] <new_files:array.path

[>] >results:array.serial
[>] >processed_count:uint

[r] $input_folder:path << \\FileDir\\incoming_logs\
[r] $output_folder:path << \\FileDir\\processed\

[w] |W.File
 |  <file:path << $input_folder
 |  >session-Session-File >> $file_session

[r] ~ForEach.Array
 ~  <array << $new_files
 ~  >item >> $file
   [r] |File.Read
    |  <path:path << $file
    |  >content:string >> $content
       [!] !File.Read.Error
          [r] |U.Console"Error reading {$file}"

          [*] *Into.Array
           *  <item.file:path << $file
           *  <item.success:bool << -False
           *  <item.message:string << "Read failed"
           *  >array >> $results

       [!] !No.Error
          [r] @LogP|ProcessLogContent
           |  <content:string << $content
           |  >result >> $processed

          [r] $output_path:path << "{$output_folder}\\{$file}"

          [r] |File.Write
           |  <path:path << $output_path
           |  <content:string << $processed

          [*] *Into.Array
           *  <item.file:path << $file
           *  <item.success:bool << -True
           *  <item.message:string << "Processed successfully"
           *  >array >> $results

       [!] !*
          [r] |U.Console"Unexpected error: {$file}"

          [*] *Into.Array
           *  <item.file:path << $file
           *  <item.success:bool << -False
           *  <item.message:string << "Unknown error"
           *  >array >> $results

[r] ~ForEach.Array
 ~  <array << $results
 ~  >item >> $current
   [f] $current.success =? -True
      [*] *Aggregate.Sum
       *  >inc:uint >> $success_count

[>] >results << $results
[>] >processed_count << $success_count
{x}
```

**Key Features:**
- Package import system: `[@] @alias << @Scope:Package:Version`
- Use imported pipelines: `@LogP|ProcessLogContent`
- Loop syntax (v0.0.5): `[r] ~ForEach.Array` with ` ~  <array <<` and ` ~  >item >>`
- Error handling: specific (`!File.Read.Error`), success (`!No.Error`), catch-all (`!*`)
- Pack operator: `[*] *Into.Array` for collection building
- Aggregate operator: `[*] *Aggregate.Sum` with `>inc` for counting
- Path concatenation: `"{$output_folder}\\{$file}"`

**Important:** Loop syntax in v0.0.5 uses `~` markers, NOT `>>` operator!

---

### HTTP Triggers

HTTP triggers create REST endpoints that execute pipelines when requests arrive.

#### REST API Example

```polyglot
{@} @Local:Examples.HTTPEndpoint:0.0.5.0
[@] @DB << @Local:Database.Utils:0.0.5.0
{x}

{#} #UserSchema
[.] .name:string
[.] .email:string
[.] .age:uint
{x}

{#} -HTTP-Server#APIServer
[A] #APIServer
[.] .host:string << "0.0.0.0"
[.] .port:uint << 8080
{x}

{|} |CreateUser
[%] %Doc << "HTTP endpoint to create user with validation"

[t] |T.HTTP.Endpoint
 |  <server-HTTP-Server << #APIServer
 |  <route:string << "/api/users"
 |  >request:serial >> <req

[<] <req:serial

[>] >response:serial

[w] |W.HTTP
 |  >session-Session-HTTP >> $http_session

[w] |W.DB.PostgreSQL
 |  <settings-DB-Settings.host:string << "localhost"
 |  <settings-DB-Settings.port:uint << 5432
 |  <settings-DB-Settings.database:string << "users_db"
 |  >session-Session-DB >> $db_session

[r] $method:string << $req.method
[r] $body:serial << $req.body

%% Check HTTP method using reserved enum
[f] $method =? -HTTP-Method-POST
   %% Validate body matches schema
   [f] $body #? #UserSchema
      [r] $email:string << $body.email
      [r] $name:string << $body.name
      [r] $age:uint << $body.age

      %% Validate email using regex operator
      [r] $email re? -Regex-Email
         [r] |DB.PostgreSQL.Insert
          |  <table:string << "users"
          |  <data:serial << $body
          |  >id:uint >> $user_id
            [!] !DB.Insert.Error
               [>] >response << {
               [+]  .status:uint << 500,
               [+]  .error:string << "Database error"
               [+] }
            [!] !No.Error
               [>] >response << {
               [+]  .status:uint << 201,
               [+]  .user_id:uint << $user_id,
               [+]  .message:string << "User created successfully"
               [+] }
            [!] !*
               [>] >response << {
               [+]  .status:uint << 500,
               [+]  .error:string << "Unexpected database error"
               [+] }

      [r] $email re!? -Regex-Email
         [>] >response << {
         [+]  .status:uint << 400,
         [+]  .error:string << "Invalid email format"
         [+] }

   [f] $body #!? #UserSchema
      [>] >response << {
      [+]  .status:uint << 400,
      [+]  .error:string << "Request body does not match UserSchema"
      [+] }

[f] $method =!? -HTTP-Method-POST
   [>] >response << {
   [+]  .status:uint << 405,
   [+]  .error:string << "Method not allowed. Use POST."
   [+] }
{x}
```

**Key Features:**
- Reserved schema extension: `{#} -HTTP-Server#Alias`
- Server configuration with `[A]` activation
- Schema validation: `#?` (match), `#!?` (mismatch)
- Regex validation: `re?` (match), `re!?` (no match)
- HTTP method enums: `-HTTP-Method-{GET, POST, PUT, DELETE}`
- Regex enums: `-Regex-Email`, `-Regex-path-Windows`, etc.
- Serial literal syntax (v0.0.5): `<< { [+] .field:type << value }`
- DB wrapper with settings schema

**Reserved HTTP Enums:**
- `-HTTP-Method-GET`
- `-HTTP-Method-POST`
- `-HTTP-Method-PUT`
- `-HTTP-Method-DELETE`

**Reserved Regex Enums:**
- `-Regex-Email`
- `-Regex-path-Windows`
- `-Regex-path-Unix`
- `-Regex-path-Polyglot`
- `-Regex-Alpha-Small`
- `-Regex-Alpha-Capital`
- `-Regex-Arabic`

---

### Calendar Triggers

Calendar triggers execute pipelines based on calendar events (business days, specific dates, date ranges).

#### Business Days Example

```polyglot
{@} @Local:Examples.CalendarBusinessDays:0.0.5.0
[@] @Notif << @Community:Notification.Service:0.0.5.0
{x}

{#} #ReportType
[.] .Daily:string << "daily"
[.] .Weekly:string << "weekly"
[.] .Monthly:string << "monthly"
{x}

{|} |GenerateBusinessReport
[%] %Doc << "Generate business reports on weekdays only (Monday-Friday)"

[t] |T.Calendar.BusinessWeek
 |  <start_time:string << "09:00"
 |  <timezone:string << "UTC"
 |  >date:dt >> <report_date
 |  >day_of_week:string >> <weekday

[<] <report_date:dt
[<] <weekday:string

[>] >report_path:path
[>] >status:bool

[w] |W.File
 |  >session-Session-File >> $file_session

[w] |W.DB.PostgreSQL
 |  <settings-DB-Settings.host:string << "localhost"
 |  <settings-DB-Settings.port:uint << 5432
 |  <settings-DB-Settings.database:string << "reports_db"
 |  >session-Session-DB >> $db_session

%% Format date for report filename
[r] $date_str:string << |DT.Format
 |  <datetime:dt << $report_date
 |  <format:string << "yyyy-MM-dd"
 |  >formatted >> $date_str

[r] $report_dir:path << \\FileDir\\reports\

%% Determine report type based on day of week
[f] $weekday =? "Monday"
   [r] $report_type#ReportType << #ReportType.Weekly
   [r] $report_name:string << "weekly-report-{$date_str}.pdf"

[f] $weekday =!? "Monday"
   [r] $report_type#ReportType << #ReportType.Daily
   [r] $report_name:string << "daily-report-{$date_str}.pdf"

[r] $report_path:path << "{$report_dir}\\{$report_name}"

%% Query data from database
[r] |DB.PostgreSQL.Query
 |  <query:string << "SELECT * FROM sales WHERE date = '{$date_str}'"
 |  >results:array.serial >> $sales_data
    [!] !DB.Query.Error
       [r] |U.Console"Database query failed for {$date_str}"

       [r] @Notif|SendAlert
        |  <message:string << "Report generation failed: DB error"
        |  <priority:string << "high"

       [>] >status << -False
       [>] >report_path << \\NoPath\\

    [!] !No.Error
       %% Check if we have data
       [r] $data_count:uint << |Array.Length
        |  <array << $sales_data
        |  >length >> $data_count

       [f] $data_count >? 0
          %% Generate report with data
          [r] |File.Write
           |  <path:path << $report_path
           |  <content:string << "Business Report - {$report_type}\nDate: {$date_str}\nRecords: {$data_count}"
           |  >success:bool >> $write_success

          [f] $write_success =? -True
             [r] @Notif|SendNotification
              |  <message:string << "Report generated: {$report_name}"
              |  <priority:string << "normal"

             [>] >status << -True
             [>] >report_path << $report_path

          [f] $write_success =!? -True
             [>] >status << -False
             [>] >report_path << \\NoPath\\

       [f] $data_count >!? 0
          %% No data for this date
          [r] |U.Console"No sales data for {$date_str}, skipping report"

          [>] >status << -True
          [>] >report_path << \\NoPath\\

    [!] !*
       [r] |U.Console"Unexpected error during report generation"

       [>] >status << -False
       [>] >report_path << \\NoPath\\
{x}
```

**Alternative Calendar Triggers:**

```polyglot
%% Specific date and time
[t] |T.Calendar.Date
 |  <date:string << "2026-01-15"
 |  <time:string << "14:00"
 |  >timestamp:dt >> <execution_time

%% Date range with interval
[t] |T.Calendar.Range
 |  <start_date:string << "2026-01-01"
 |  <end_date:string << "2026-01-31"
 |  <interval:string << "daily"
 |  >current_date:dt >> <date
```

**Key Features:**
- `|T.Calendar.BusinessWeek` excludes weekends automatically
- Trigger outputs date AND day of week for conditional logic
- Nested fork conditions for validation
- Comparison operators: `>?` (greater than), `>!?` (not greater / less than or equal)
- Package import for notifications
- Multiple validation layers

---

## Trigger I/O Wiring

Trigger I/O wiring connects trigger outputs to pipeline inputs, enabling data flow from external events into your pipeline logic.

### Basic Wiring Pattern

```polyglot
[t] |T.Cli
 |  <cmd:string << "command_name"
 |  <kwargs.param:type << <pipeline_input  %% Wire trigger kwarg to input
 |  >output:type >> <pipeline_input        %% Wire trigger output to input

[<] <pipeline_input:type
```

### Wiring Rules

1. **Trigger Configuration:** Use `<config:type << value` for trigger settings
2. **Kwargs Wiring:** Use `<kwargs.param:type << <input` to wire CLI kwargs to pipeline inputs
3. **Output Wiring:** Use `>output:type >> <input` to wire trigger outputs to pipeline inputs
4. **Pipeline Inputs:** Declare matching inputs with `[<] <input:type`

### Multiple Wiring Example

```polyglot
[t] |T.Cli
 |  <cmd:string << "process"
 |  <kwargs.action:string << <action      %% Wire action kwarg
 |  <kwargs.target:string << <target      %% Wire target kwarg
 |  >args:serial >> <all_args             %% Wire full args serial

[<] <action:string        %% Receives action kwarg
[<] <target:string        %% Receives target kwarg
[<] <all_args:serial      %% Receives all args
```

You can wire **both** individual kwargs AND the full serial of all arguments.

---

## Wrappers and Sessions

Wrappers provide runtime context and session management for pipelines. They are especially important for triggered pipelines that need process isolation.

### Wrapper Declaration

```polyglot
[w] |W.{Type}
 |  <config:type << value              %% Optional configuration
 |  >session-Session-{Type} >> $var    %% Session output
```

### Reserved Session Enums

All wrapper sessions use the reserved enum pattern `-Session-{Type}`:

- `-Session-File` - File wrapper sessions
- `-Session-DB` - Database wrapper sessions
- `-Session-HTTP` - HTTP wrapper sessions
- `-Session-Cli` - CLI/Console wrapper sessions

### Common Wrappers

#### Console Wrapper

Required for CLI-triggered pipelines to wrap the console session as a process for Queue manager:

```polyglot
[w] |W.Console
 |  >session >> $console_session
```

**Console Print Styles:**

```polyglot
%% Inline (recommended for ease of use)
[r] |U.Console"formatted string with {$var}"

%% With parameter
[r] |U.Console
 |  <message:string << $greeting
```

#### File Wrapper

Required for file operations, needs file path:

```polyglot
[w] |W.File
 |  <file:path << $log_file
 |  >session-Session-File >> $file_session
```

#### HTTP Wrapper

For HTTP operations:

```polyglot
[w] |W.HTTP
 |  >session-Session-HTTP >> $http_session
```

#### Database Wrapper

For database operations with configuration schema:

```polyglot
[w] |W.DB.PostgreSQL
 |  <settings-DB-Settings.host:string << "localhost"
 |  <settings-DB-Settings.port:uint << 5432
 |  <settings-DB-Settings.database:string << "users_db"
 |  >session-Session-DB >> $db_session
```

### Multiple Wrappers

Pipelines can have multiple wrappers:

```polyglot
[w] |W.HTTP
 |  >session-Session-HTTP >> $http_session

[w] |W.File
 |  <file:path << $log_file
 |  >session-Session-File >> $file_session

[w] |W.DB.PostgreSQL
 |  <settings-DB-Settings.host:string << "localhost"
 |  <settings-DB-Settings.port:uint << 5432
 |  >session-Session-DB >> $db_session
```

---

## Error Handling

Triggered pipelines must handle errors gracefully since they run automatically without user intervention.

### Error Block Structure

Error blocks in v0.0.5 have **3 parts**:

```polyglot
[r] |Operation
 |  <input << $value
 |  >output >> $result
   [!] !Specific.Error
      [r] ... handle specific error type

   [!] !No.Error
      [r] ... SUCCESS case (no error occurred)

   [!] !*
      [r] ... catch-all for OTHER errors
```

**Important:** `!No.Error` is the SUCCESS case, NOT `!*`!

### Error Types

- **Specific Errors:** `!File.Read.Error`, `!HTTP.Error`, `!DB.Insert.Error`
- **Success Case:** `!No.Error` - Executed when no error occurred
- **Catch-All:** `!*` - Catches any OTHER errors not explicitly handled

### Error Handling Example

```polyglot
[r] |HTTP.Get
 |  <url:string << "http://api.example.com/data"
 |  >status_code:uint >> $status
 |  >body:string >> $data
   [!] !HTTP.Error
      [r] |U.Console"HTTP request failed"
      [>] >status << -False

   [!] !No.Error
      [f] $status =? 200
         [r] |U.Console"Success: {$data}"
         [>] >status << -True
      [f] $status =!? 200
         [r] |U.Console"Unexpected status: {$status}"
         [>] >status << -False

   [!] !*
      [r] |U.Console"Unknown error occurred"
      [>] >status << -False
```

### Best Practices

1. **Always handle errors** in triggered pipelines
2. **Use specific error types** for known failures
3. **Always include `!No.Error`** for success case
4. **Always include `!*`** for unexpected errors
5. **Log errors** for debugging automated workflows
6. **Provide fallback values** for outputs

---

## Real-World Examples

### Example 1: Automated Daily Backups

```polyglot
{@} @Local:Production.DailyBackup:0.0.5.0
[@] @Compress << @Community:Compression.Utils:0.0.5.0
[@] @Notify << @Community:Slack.Notifier:0.0.5.0
{x}

{|} |DailyBackup
[%] %Doc << "Automated daily backup with compression and notification"

[t] |T.DT.Daily"02:00"
 |  >timestamp:dt >> <backup_time

[<] <backup_time:dt

[>] >success:bool
[>] >backup_size:uint

[w] |W.File
 |  >session-Session-File >> $file_session

[r] $date_str:string << |DT.Format
 |  <datetime:dt << $backup_time
 |  <format:string << "yyyy-MM-dd"
 |  >formatted >> $date_str

[r] $source:path << \\FileDir\\data\
[r] $backup_name:string << "backup-{$date_str}.tar.gz"
[r] $backup_path:path << "\\FileDir\\backups\\{$backup_name}"

[r] @Compress|CreateArchive
 |  <source:path << $source
 |  <destination:path << $backup_path
 |  <compression:string << "gzip"
 |  >size:uint >> $size
   [!] !Compress.Error
      [r] @Notify|SendAlert
       |  <channel:string << "#ops-alerts"
       |  <message:string << "❌ Backup failed: {$date_str}"
       |  <priority:string << "high"

      [>] >success << -False
      [>] >backup_size << 0

   [!] !No.Error
      [r] @Notify|SendMessage
       |  <channel:string << "#ops-logs"
       |  <message:string << "✅ Backup completed: {$backup_name} ({$size} bytes)"

      [>] >success << -True
      [>] >backup_size << $size

   [!] !*
      [r] @Notify|SendAlert
       |  <channel:string << "#ops-alerts"
       |  <message:string << "⚠️ Backup error: Unknown issue"
       |  <priority:string << "critical"

      [>] >success << -False
      [>] >backup_size << 0
{x}
```

### Example 2: File Processing Pipeline

```polyglot
{@} @Local:Production.LogProcessor:0.0.5.0
[@] @Parser << @Community:Log.Parser:0.0.5.0
[@] @DB << @Local:Database.Utils:0.0.5.0
{x}

{#} #LogLevel
[.] .Info:string << "INFO"
[.] .Warning:string << "WARNING"
[.] .Error:string << "ERROR"
{x}

{|} |ProcessLogs
[%] %Doc << "Process incoming log files and store in database"

[t] |T.Folder.NewFiles
 |  <folder:path << \\FileDir\\incoming_logs\
 |  >files:array.path >> <new_files

[<] <new_files:array.path

[>] >processed:uint
[>] >failed:uint

[w] |W.File
 |  >session-Session-File >> $file_session

[w] |W.DB.PostgreSQL
 |  <settings-DB-Settings.host:string << "localhost"
 |  <settings-DB-Settings.port:uint << 5432
 |  <settings-DB-Settings.database:string << "logs_db"
 |  >session-Session-DB >> $db_session

[r] ~ForEach.Array
 ~  <array << $new_files
 ~  >item >> $log_file
   [r] |File.Read
    |  <path:path << $log_file
    |  >content:string >> $content
       [!] !File.Read.Error
          [r] |U.Console"Failed to read: {$log_file}"

          [*] *Aggregate.Sum
           *  >inc:uint >> $failed_count

       [!] !No.Error
          [r] @Parser|ParseLog
           |  <content:string << $content
           |  >entries:array.serial >> $entries

          [r] ~ForEach.Array
           ~  <array << $entries
           ~  >item >> $entry
             [r] |DB.PostgreSQL.Insert
              |  <table:string << "logs"
              |  <data:serial << $entry
                 [!] !DB.Insert.Error
                    [r] |U.Console"DB insert failed for entry"
                 [!] !No.Error
                    [*] *Aggregate.Sum
                     *  >inc:uint >> $processed_count
                 [!] !*
                    [r] |U.Console"Unknown error during insert"

          %% Archive processed file
          [r] $archive_path:path << "\\FileDir\\archive\\{$log_file}"
          [r] |File.Move
           |  <source:path << $log_file
           |  <destination:path << $archive_path

       [!] !*
          [*] *Aggregate.Sum
           *  >inc:uint >> $failed_count

[>] >processed << $processed_count
[>] >failed << $failed_count
{x}
```

### Example 3: REST API with Validation

```polyglot
{@} @Local:API.UserService:0.0.5.0
[@] @Auth << @Community:Auth.JWT:0.0.5.0
[@] @DB << @Local:Database.Utils:0.0.5.0
{x}

{#} #User
[.] .id:uint
[.] .username:string
[.] .email:string
[.] .role:string
{x}

{#} -HTTP-Server#APIServer
[A] #APIServer
[.] .host:string << "0.0.0.0"
[.] .port:uint << 8080
{x}

{|} |GetUser
[%] %Doc << "Get user by ID with authentication"

[t] |T.HTTP.Endpoint
 |  <server-HTTP-Server << #APIServer
 |  <route:string << "/api/users/{id}"
 |  >request:serial >> <req

[<] <req:serial

[>] >response:serial

[w] |W.HTTP
 |  >session-Session-HTTP >> $http_session

[w] |W.DB.PostgreSQL
 |  <settings-DB-Settings.host:string << "localhost"
 |  <settings-DB-Settings.port:uint << 5432
 |  <settings-DB-Settings.database:string << "users_db"
 |  >session-Session-DB >> $db_session

%% Validate request method
[f] $req.method =!? -HTTP-Method-GET
   [>] >response << {
   [+]  .status:uint << 405,
   [+]  .error:string << "Method not allowed"
   [+] }

%% Verify authentication
[f] $req.method =? -HTTP-Method-GET
   [r] $auth_header:string << $req.headers.authorization

   [r] @Auth|VerifyToken
    |  <token:string << $auth_header
    |  >valid:bool >> $is_valid
    |  >user_id:uint >> $auth_user_id
      [!] !Auth.Invalid
         [>] >response << {
         [+]  .status:uint << 401,
         [+]  .error:string << "Unauthorized"
         [+] }

      [!] !No.Error
         [f] $is_valid =? -True
            [r] $user_id:uint << $req.params.id

            [r] |DB.PostgreSQL.Query
             |  <query:string << "SELECT * FROM users WHERE id = {$user_id}"
             |  >results:array.serial >> $results
                [!] !DB.Query.Error
                   [>] >response << {
                   [+]  .status:uint << 500,
                   [+]  .error:string << "Database error"
                   [+] }

                [!] !No.Error
                   [r] $result_count:uint << |Array.Length
                    |  <array << $results
                    |  >length >> $count

                   [f] $count >? 0
                      [r] $user:serial << $results.0

                      [>] >response << {
                      [+]  .status:uint << 200,
                      [+]  .data#User << $user
                      [+] }

                   [f] $count >!? 0
                      [>] >response << {
                      [+]  .status:uint << 404,
                      [+]  .error:string << "User not found"
                      [+] }

                [!] !*
                   [>] >response << {
                   [+]  .status:uint << 500,
                   [+]  .error:string << "Unexpected error"
                   [+] }

         [f] $is_valid =!? -True
            [>] >response << {
            [+]  .status:uint << 401,
            [+]  .error:string << "Invalid token"
            [+] }

      [!] !*
         [>] >response << {
         [+]  .status:uint << 500,
         [+]  .error:string << "Authentication service error"
         [+] }
{x}
```

---

## Best Practices

### 1. Always Use Wrappers

Triggered pipelines should always use appropriate wrappers for session management:

```polyglot
%% ✅ GOOD - Console wrapper for CLI trigger
[t] |T.Cli
 |  <cmd:string << "command"

[w] |W.Console
 |  >session >> $console_session
```

```polyglot
%% ❌ BAD - Missing wrapper
[t] |T.Cli
 |  <cmd:string << "command"

%% No wrapper!
```

### 2. Handle All Error Cases

Always implement exhaustive error handling:

```polyglot
%% ✅ GOOD - All cases covered
[r] |Operation
 |  <input << $value
 |  >output >> $result
   [!] !Operation.Error     %% Specific error
      [r] ... handle error
   [!] !No.Error            %% Success
      [r] ... process result
   [!] !*                   %% Catch-all
      [r] ... handle unexpected
```

```polyglot
%% ❌ BAD - Missing success and catch-all
[r] |Operation
 |  <input << $value
 |  >output >> $result
   [!] !Operation.Error
      [r] ... handle error
```

### 3. Use Reserved Enums

Always use reserved enums instead of strings for well-known values:

```polyglot
%% ✅ GOOD - Reserved enum
[f] $method =? -HTTP-Method-POST
   [r] ... handle POST

[f] $email re? -Regex-Email
   [r] ... email is valid
```

```polyglot
%% ❌ BAD - String literals
[f] $method =? "POST"
   [r] ... handle POST

[r] |U.String.Match
 |  <pattern << "email regex pattern"
 |  <text << $email
```

### 4. Log Important Events

Use console logging or notification services for debugging:

```polyglot
[r] |U.Console"Processing file: {$file_name}"

[r] |U.Console"Backup completed: {$backup_path} ({$size} bytes)"

[r] @Notif|SendAlert
 |  <message:string << "Critical error in {$pipeline_name}"
 |  <priority:string << "high"
```

### 5. Use Schema Validation

Validate data structures with schema comparison:

```polyglot
{#} #UserInput
[.] .name:string
[.] .email:string
[.] .age:uint
{x}

[f] $request_body #? #UserInput
   [r] ... process valid input
[f] $request_body #!? #UserInput
   [r] ... reject invalid input
```

### 6. Implement Graceful Degradation

Provide fallback values and default behaviors:

```polyglot
[r] |DB.Query
 |  <query:string << "SELECT * FROM cache"
 |  >results:array.serial >> $cached_data
   [!] !DB.Query.Error
      %% Fallback to empty array
      [r] $data:array.serial << []
   [!] !No.Error
      [r] $data:array.serial << $cached_data
   [!] !*
      [r] $data:array.serial << []
```

### 7. Use Enums for Status Values

Define enums for consistent status reporting:

```polyglot
{#} #ProcessStatus
[.] .Success:string << "success"
[.] .Failed:string << "failed"
[.] .Partial:string << "partial"
{x}

[>] >status#ProcessStatus << #ProcessStatus.Success
```

### 8. Package Imports for Reusability

Use package imports for common functionality:

```polyglot
[@] @Validator << @Community:Validation.Utils:0.0.5.0
[@] @Logger << @Community:Logging.Service:0.0.5.0
[@] @DB << @Local:Database.Utils:0.0.5.0

[r] @Validator|ValidateEmail
 |  <email:string << $user_email
```

---

## Common Patterns

### Pattern 1: CLI Command with Validation

```polyglot
[t] |T.Cli
 |  <cmd:string << "command"
 |  <kwargs.input:string << <input_value

[<] <input_value:string
[>] >result:string

[w] |W.Console
 |  >session >> $console_session

%% Validate input
[r] $input_value re? -Regex-Email
   [r] ... process valid email
[r] $input_value re!? -Regex-Email
   [r] |U.Console"Invalid email format"
   [>] >result << "error"
```

### Pattern 2: Scheduled Task with Notification

```polyglot
[@] @Notif << @Community:Notification.Service:0.0.5.0
{x}

[t] |T.Cron
 |  <schedule:string << "0 */6 * * *"  %% Every 6 hours
 |  >timestamp:dt >> <run_time

[r] |PerformTask
 |  >success:bool >> $success
   [!] !No.Error
      [r] @Notif|SendMessage
       |  <message:string << "Task completed successfully"
   [!] !*
      [r] @Notif|SendAlert
       |  <message:string << "Task failed"
       |  <priority:string << "high"
```

### Pattern 3: File Watcher with Batch Processing

```polyglot
[t] |T.Folder.NewFiles
 |  <folder:path << \\FileDir\\incoming\
 |  >files:array.path >> <new_files

[r] ~ForEach.Array
 ~  <array << $new_files
 ~  >item >> $file
   [r] |ProcessFile
    |  <path:path << $file
    |  >result >> $result
      [!] !No.Error
         [*] *Into.Array
          *  <item << $result
          *  >array >> $results
      [!] !*
         [r] |U.Console"Failed to process: {$file}"
```

### Pattern 4: REST API with JWT Auth

```polyglot
[@] @Auth << @Community:Auth.JWT:0.0.5.0
{x}

[t] |T.HTTP.Endpoint
 |  <route:string << "/api/protected"
 |  >request:serial >> <req

[r] $token:string << $req.headers.authorization

[r] @Auth|VerifyToken
 |  <token:string << $token
 |  >valid:bool >> $is_valid
   [!] !No.Error
      [f] $is_valid =? -True
         [r] ... process authorized request
      [f] $is_valid =!? -True
         [>] >response << {
         [+]  .status:uint << 401,
         [+]  .error:string << "Unauthorized"
         [+] }
   [!] !*
      [>] >response << {
      [+]  .status:uint << 500,
      [+]  .error:string << "Auth service error"
      [+] }
```

### Pattern 5: Business Days Report Generation

```polyglot
[t] |T.Calendar.BusinessWeek
 |  <start_time:string << "09:00"
 |  >date:dt >> <report_date
 |  >day_of_week:string >> <weekday

%% Monday = weekly report, others = daily
[f] $weekday =? "Monday"
   [r] |GenerateWeeklyReport
    |  <date:dt << $report_date
[f] $weekday =!? "Monday"
   [r] |GenerateDailyReport
    |  <date:dt << $report_date
```

---

## See Also

### Related Documentation

- **[Loop System Guide](loop-system.md)** - Unpack/pack operators, iteration patterns
- **[Error Handling Guide](error-handling.md)** - Error blocks, exhaustiveness, patterns
- **[Variable Lifecycle](variable-lifecycle.md)** - Immutability, default vs final states
- **[Standard Triggers YAML](../stdlib/standard-triggers.yaml)** - Complete trigger specifications

### Related Examples

- **[Hello World Multi-Runtime](../examples/hello-world-multi-runtime.pg)** - Basic CLI trigger example
- **File Processing Pipeline** - Folder trigger with loop processing
- **REST API Service** - HTTP trigger with validation

### Migration Guides

- **[v0.0.4 to v0.0.5 Migration](../migration-guide-v0.0.4-to-v0.0.5.md)** - Syntax changes
- **[What's New in v0.0.5](../whats-new-v0.0.5.md)** - New features overview

---

**Document Status:** ✅ Complete
**Training Session:** 2026-01-05
**Verified Examples:** 7
**Lines:** 1361

**Generated by:** Polly Language Expert
**For:** Scribe Documentation Architect
