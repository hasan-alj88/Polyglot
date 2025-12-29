# Trigger I/O Wiring Guide

**Version:** v0.0.4
**Category:** Language Guide
**Audience:** Intermediate users
**Prerequisites:** Basic triggers, pipeline I/O

---

## Overview

Triggers can **output data** that gets wired directly to pipeline inputs. This enables **reactive patterns** where triggers not only activate pipelines but also provide the data to process.

**Key Concepts:**
- Trigger output parameters: `>output:type`
- Output-to-input wiring: `>trigger_out >> <pipeline_in`
- Pipeline input declaration: `[|] <input:type`
- Reactive data flow

---

## Basic Syntax

### Trigger with Output

```polyglot
[t] |T.TriggerName
(|) <trigger_config << value
(|) >output_data:type >> <pipeline_input

[|] <pipeline_input:type
```

**Pattern:**
1. Trigger declaration: `[t] |T.TriggerName`
2. Trigger configuration inputs: `(|) <config << value`
3. Trigger output wired to pipeline: `(|) >output >> <input`
4. Pipeline declares the input: `[|] <input:type`

---

## Simple Example: Folder Monitoring

### Monitor Folder for New Files

```polyglot
{@} @Local:Examples.FolderMonitor:0.0.0.1
{x}


{|} |ProcessNewFiles
[%] %Doc << "Process files when they appear in folder"

[t] |T.Folder.NewFiles
(|) <folder:pg.path << \\FileDir\\logs\
(|) >new_files:pg.array.pg.path >> <files

[|] <files:pg.array.pg.path

[p] ~ForEach.Array
(~) <array << $files
(~) >item >> $file
   [r] |File.Process
   (|) <path << $file
{x}
```

**What happens:**
1. `|T.Folder.NewFiles` watches `\\FileDir\\logs\` folder
2. When new files appear, trigger fires
3. Trigger outputs array of file paths: `>new_files`
4. Output wired to pipeline input: `>> <files`
5. Pipeline receives and processes the files

---

## The `|T.Folder.NewFiles` Trigger

### Syntax

```polyglot
[t] |T.Folder.NewFiles
(|) <folder:pg.path << \\Path\
(|) >new_files:pg.array.pg.path >> <pipeline_input
```

**Input:**
- `<folder:pg.path` - Folder to monitor (with trailing `\`)

**Output:**
- `>new_files:pg.array.pg.path` - Array of new file paths

**Behavior:**
- Monitors folder continuously
- Fires when new files detected
- Outputs all new file paths since last check
- Folder path must end with `\` (folder convention)

### Example with Filtering

```polyglot
[t] |T.Folder.NewFiles
(|) <folder:pg.path << \\FileDir\\uploads\
(|) >new_files:pg.array.pg.path >> <uploads

[|] <uploads:pg.array.pg.path

// Filter for .txt files only
[p] ~ForEach.Array
(~) <array << $uploads
(~) >item >> $file
   [r] $extension :pg.string << |Path.GetExtension"{$file}"

   [f] $extension =? ".txt"
      [r] |ProcessTextFile
      (|) <path << $file
   {x}
{x}
```

---

## Output-to-Input Wiring

### Wiring Syntax

```polyglot
>trigger_output:type >> <pipeline_input
```

**Left Side (Trigger Output):**
- `>trigger_output` - Output parameter from trigger
- Must match trigger's declared output

**Right Side (Pipeline Input):**
- `<pipeline_input` - Input parameter of pipeline
- Declared later with `[|] <pipeline_input:type`

**Type Requirements:**
- Types must match exactly
- Including nested types: `pg.array.pg.path` not `pg.array.path`

### Wiring Examples

**Simple Type:**
```polyglot
[t] |T.CLI"command"
(|) >result:pg.string >> <input

[|] <input:pg.string
```

**Array Type:**
```polyglot
[t] |T.Folder.NewFiles
(|) <folder:pg.path << \\Path\
(|) >new_files:pg.array.pg.path >> <files

[|] <files:pg.array.pg.path
```

**Serial Type:**
```polyglot
[t] |T.HTTP.Request
(|) <url:pg.string << "https://api.example.com/data"
(|) >response:pg.serial >> <data

[|] <data:pg.serial
```

---

## The `|T.CLI` Trigger

### Syntax

```polyglot
[t] |T.CLI"command_name"
(|) >args:pg.array.pg.string >> <arguments
(|) >flags:pg.serial >> <options
```

**Input (via name):**
- `"command_name"` - CLI command to trigger on

**Outputs:**
- `>args:pg.array.pg.string` - Command line arguments
- `>flags:pg.serial` - Command line flags as serial data

### Example: CLI Command Handler

```polyglot
{|} |HandleProcess
[%] %Doc << "Handle 'process' command from CLI"

[t] |T.CLI"process"
(|) >args:pg.array.pg.string >> <files
(|) >flags:pg.serial >> <options

[|] <files:pg.array.pg.string
[|] <options:pg.serial

[r] $verbose :pg.bool << $options.verbose

[p] ~ForEach.Array
(~) <array << $files
(~) >item >> $file
   [f] $verbose =? #True
      [r] |Log.Info"Processing {$file}"
   {x}

   [r] |File.Process
   (|) <path << $file
{x}
```

**CLI Usage:**
```bash
polyglot run process file1.txt file2.txt --verbose
```

---

## Multiple Trigger Outputs

### Wiring Multiple Outputs

```polyglot
[t] |T.HTTP.Request
(|) <url:pg.string << "https://api.example.com"
(|) >status:pg.int >> <http_status
(|) >body:pg.string >> <response_body
(|) >headers:pg.serial >> <response_headers

[|] <http_status:pg.int
[|] <response_body:pg.string
[|] <response_headers:pg.serial

[f] $http_status =? 200
   [r] |ProcessResponse
   (|) <body << $response_body
   (|) <headers << $response_headers
{x}

[f] *?
   [r] |HandleError
   (|) <status << $http_status
{x}
```

**Pattern:**
- Each trigger output wired to separate pipeline input
- Pipeline declares all inputs
- Can use outputs independently

---

## Complete Example: Log File Processor

### Scenario
Monitor log folder, process new log files, generate summaries using LLM, append to rolling log folder.

```polyglot
{@} @Local:Examples.LogProcessor:0.0.0.1
{x}


{#} #LLM.Config;LogAnalyzer
[A] #LLMConfig
[s] |YAML.Load"\\FileDir\\llm.yaml"
   [.] .api_key:pg.string << .api.key
   [.] .model:pg.string << .model.name
[s][!] !*
{x}

{#} #Folders.Rolling;LogSummary
[A] #Summary
   [%] %Scope: #Scope << #Pipelines;Process;Logs
[.] .file_size_mb:pg.float << 300
[.] .folder:pg.path << \\FileDir\\summaries\
[.] .file_type:#FileTypes << #Text
{x}


{|} |Process;Logs
[%] %Doc << "Monitor logs folder and generate summaries"

[t] |T.Folder.NewFiles
(|) <folder:pg.path << \\FileDir\\logs\
(|) >new_files:pg.array.pg.path >> <files

[|] <files:pg.array.pg.path

[w] |W.Polyglot.Scope

[p] ~ForEach.Array
(~) <array << $files
(~) >item >> $file
   [r] |LLM.Query
   (|) <attachments:pg.path << {$file}
   (|) <prompt:pg.string << "Summarize errors and warnings in this log file"
   (|) <config:#LLM.Config << #LLMConfig
   (|) >result:pg.string >> $summary

   [r] |Folder.Rolling.Append
   (|) <config:#Folders.Rolling << #Summary
   (|) <new_content:pg.string << $summary
      [!] $success:pg.bool << !
         [?] !NoError ? #True
         [?] !* ? #False

[*] *Into.Array
(*) <item << $success

[r] |U.Boolean.All
(|) <array:pg.array.pg.bool << $success
(|) >result:pg.bool >> $all_success

[f] $all_success =? #True
   [|] >error << !NoError
[f] $all_success =? #False
   [|] >error << !Pipeline.Task.Failed
{x}
```

**Data Flow:**
1. `|T.Folder.NewFiles` monitors `\\FileDir\\logs\`
2. New files trigger pipeline execution
3. File paths array wired to `<files` input
4. Loop processes each file with LLM
5. Summaries appended to rolling folder
6. Success states collected and checked

---

## Trigger Types Reference

### `|T.Folder.NewFiles`
**Purpose:** Monitor folder for new files

**Inputs:**
- `<folder:pg.path` - Folder to watch

**Outputs:**
- `>new_files:pg.array.pg.path` - Array of new file paths

**Example:**
```polyglot
[t] |T.Folder.NewFiles
(|) <folder:pg.path << \\FileDir\\uploads\
(|) >new_files:pg.array.pg.path >> <files
```

### `|T.CLI`
**Purpose:** Trigger on CLI command

**Inputs:**
- Command name (via string parameter)

**Outputs:**
- `>args:pg.array.pg.string` - Arguments
- `>flags:pg.serial` - Flags/options

**Example:**
```polyglot
[t] |T.CLI"serve"
(|) >args:pg.array.pg.string >> <ports
(|) >flags:pg.serial >> <options
```

### `|T.HTTP.Request`
**Purpose:** HTTP endpoint trigger

**Inputs:**
- `<url:pg.string` - Endpoint URL
- `<method:pg.string` - HTTP method

**Outputs:**
- `>status:pg.int` - HTTP status code
- `>body:pg.string` - Response body
- `>headers:pg.serial` - Response headers

**Example:**
```polyglot
[t] |T.HTTP.Request
(|) <url:pg.string << "https://api.example.com/webhook"
(|) <method:pg.string << "POST"
(|) >status:pg.int >> <http_status
(|) >body:pg.string >> <data
```

### `|T.Schedule.Cron`
**Purpose:** Time-based scheduling

**Inputs:**
- `<schedule:pg.string` - Cron expression

**Outputs:**
- `>timestamp:pg.dt` - Trigger timestamp
- `>iteration:pg.int` - Iteration count

**Example:**
```polyglot
[t] |T.Schedule.Cron
(|) <schedule:pg.string << "0 * * * *"  // Every hour
(|) >timestamp:pg.dt >> <run_time
```

---

## Advanced Patterns

### Pattern 1: Conditional Trigger Processing

```polyglot
[t] |T.Folder.NewFiles
(|) <folder:pg.path << \\FileDir\\uploads\
(|) >new_files:pg.array.pg.path >> <files

[|] <files:pg.array.pg.path

[r] $count :pg.int << |Array.Length
(|) <array << $files

[f] $count >? 0
   [r] |Notify"Received {$count} new files"

   [p] ~ForEach.Array
   (~) <array << $files
   (~) >item >> $file
      [r] |ProcessFile
      (|) <path << $file
   {x}
{x}
```

### Pattern 2: Trigger Output Transformation

```polyglot
[t] |T.CLI"convert"
(|) >args:pg.array.pg.string >> <raw_args

[|] <raw_args:pg.array.pg.string

// Transform args to paths
[p] ~ForEach.Array
(~) <array << $raw_args
(~) >item >> $arg
   [r] $path :pg.path << |Path.FromString"{$arg}"

[*] *Into.Array
(*) <item << $path

[r] $paths :pg.array.pg.path << $path

// Process converted paths
[p] ~ForEach.Array
(~) <array << $paths
(~) >item >> $file
   [r] |Convert
   (|) <input << $file
{x}
```

### Pattern 3: Multi-Trigger Coordination

```polyglot
{|} |FileProcessor
[t] |T.Folder.NewFiles
(|) <folder:pg.path << \\FileDir\\queue\
(|) >new_files:pg.array.pg.path >> <queued_files

[|] <queued_files:pg.array.pg.path
[r] |ProcessQueue
(|) <files << $queued_files
{x}

{|} |ManualTrigger
[t] |T.CLI"process-now"
(|) >args:pg.array.pg.string >> <files_to_process

[|] <files_to_process:pg.array.pg.string

[p] ~ForEach.Array
(~) <array << $files_to_process
(~) >item >> $file_str
   [r] $file :pg.path << |Path.FromString"{$file_str}"
   [r] |ProcessFile
   (|) <path << $file
{x}
{x}
```

---

## Best Practices

### ✅ 1. Use Full Type Paths

```polyglot
// ✅ GOOD: Full nested type
(|) >new_files:pg.array.pg.path >> <files
[|] <files:pg.array.pg.path
```

```polyglot
// ❌ WRONG: Abbreviated type (will error)
(|) >new_files:pg.array.path >> <files
[|] <files:pg.array.path
```

### ✅ 2. Folder Paths Need Trailing Backslash

```polyglot
// ✅ GOOD: Folder with trailing \
(|) <folder:pg.path << \\FileDir\\logs\
```

```polyglot
// ❌ WRONG: Missing trailing \ (treated as file)
(|) <folder:pg.path << \\FileDir\\logs
```

### ✅ 3. Declare All Pipeline Inputs

```polyglot
// ✅ GOOD: All wired inputs declared
[t] |T.HTTP.Request
(|) >status:pg.int >> <http_status
(|) >body:pg.string >> <response

[|] <http_status:pg.int
[|] <response:pg.string
```

```polyglot
// ❌ WRONG: Missing input declaration
[t] |T.HTTP.Request
(|) >status:pg.int >> <http_status
(|) >body:pg.string >> <response

[|] <http_status:pg.int
// Missing <response declaration!
```

### ✅ 4. Handle Empty Trigger Output

```polyglot
// ✅ GOOD: Check for empty arrays
[t] |T.Folder.NewFiles
(|) <folder:pg.path << \\FileDir\\uploads\
(|) >new_files:pg.array.pg.path >> <files

[|] <files:pg.array.pg.path

[r] $count :pg.int << |Array.Length
(|) <array << $files

[f] $count =? 0
   [r] |Log.Info"No new files"
{x}

[f] $count >? 0
   [r] |ProcessFiles
   (|) <files << $files
{x}
```

---

## Troubleshooting

### Issue 1: Type Mismatch Error

**Error:** Type mismatch between trigger output and pipeline input

```polyglot
// ❌ WRONG
[t] |T.Folder.NewFiles
(|) >new_files:pg.array.pg.path >> <files

[|] <files:pg.array.pg.string  // Wrong type!
```

**Solution:** Match types exactly:

```polyglot
// ✅ RIGHT
[t] |T.Folder.NewFiles
(|) >new_files:pg.array.pg.path >> <files

[|] <files:pg.array.pg.path  // Correct type
```

### Issue 2: Missing Input Declaration

**Error:** Undefined input parameter

```polyglot
// ❌ WRONG: Wired but not declared
[t] |T.CLI"command"
(|) >args:pg.array.pg.string >> <arguments
// Missing: [|] <arguments:...
```

**Solution:** Declare all wired inputs:

```polyglot
// ✅ RIGHT
[t] |T.CLI"command"
(|) >args:pg.array.pg.string >> <arguments

[|] <arguments:pg.array.pg.string
```

### Issue 3: Folder Not Found

**Error:** Folder monitoring fails to start

```polyglot
// ❌ WRONG: Folder doesn't exist
[t] |T.Folder.NewFiles
(|) <folder:pg.path << \\FileDir\\nonexistent\
```

**Solution:** Ensure folder exists or create it first:

```polyglot
// ✅ RIGHT: Create folder if needed
[r] |Folder.EnsureExists"\\FileDir\\uploads\"

[t] |T.Folder.NewFiles
(|) <folder:pg.path << \\FileDir\\uploads\
(|) >new_files:pg.array.pg.path >> <files
```

---

## Quick Reference

```
┌─────────────────────────────────────────────┐
│ TRIGGER I/O WIRING                          │
├─────────────────────────────────────────────┤
│                                             │
│  BASIC PATTERN                              │
│  [t] |T.TriggerName                         │
│  (|) <config << value                       │
│  (|) >output:type >> <pipeline_input        │
│                                             │
│  [|] <pipeline_input:type                   │
│                                             │
│  WIRING SYNTAX                              │
│  >trigger_out:type >> <pipeline_in          │
│                                             │
│  COMMON TRIGGERS                            │
│  |T.Folder.NewFiles - File monitoring       │
│  |T.CLI"cmd" - Command line                 │
│  |T.HTTP.Request - HTTP endpoint            │
│  |T.Schedule.Cron - Time-based              │
│                                             │
│  TYPE REQUIREMENTS                          │
│  - Types must match exactly                 │
│  - Use full paths: pg.array.pg.path         │
│  - Folders end with \                       │
│                                             │
│  BEST PRACTICES                             │
│  - Declare all wired inputs                 │
│  - Handle empty outputs                     │
│  - Use full type annotations                │
│  - Check folder existence                   │
│                                             │
└─────────────────────────────────────────────┘
```

---

## See Also

- [Pipeline I/O](../syntax/io-operators.md) - Input/output operators
- [Loop System](../control-flow/loops.md) - Processing arrays
- [Error Handling](../error-handling/basics.md) - Error patterns
- [Pipeline Composition](../advanced/pipeline-composition.md) - Chaining pipelines

---

**Generated by:** Polly (Polyglot Language Expert)
**For:** User Documentation
**Version:** v0.0.4
**Last Updated:** 2025-12-28
**Confidence:** ✅ Verified - All patterns from session-2025-12-27-trigger-io-advanced
