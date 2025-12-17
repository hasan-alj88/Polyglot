---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/getting-started.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Getting Started with Polyglot

This guide will help you install Polyglot, create your first pipeline, and understand the basic workflow. By the end, you'll have a running automation pipeline.

## Installation

Polyglot consists of two components:
1. **Polyglot Service** - The runtime that executes pipelines
2. **Polyglot CLI** - Command-line tool for compiling and managing pipelines

### Prerequisites

- **Rust:** 1.70 or later (for CLI and service
- **Python:** 3.8+ (if using Python wrappers
- **Node.js:** 16+ (if using Node.js wrappers
- **Go:** 1.19+ (if using Go wrappers

### Option 1: Docker (Recommended

The easiest way to run Polyglot is with Docker:

```bash
# Pull the Polyglot service image
docker pull polyglot/service:latest

# Run the service
docker run -d \
  --name polyglot-service \
  -p 8080:8080 \
  -v /var/lib/polyglot:/data \
  polyglot/service:latest

# Install the CLI
cargo install polyglot-cli
```

### Option 2: Manual Installation

Build from source:

```bash
# Clone the repository
git clone https://github.com/polyglot-lang/polyglot.git
cd polyglot

# Build the service
cd service
cargo build --release

# Install the service binary
sudo cp target/release/polyglot-service /usr/local/bin/

# Build and install the CLI
cd ../cli
cargo install --path .

# Start the service
polyglot-service start
```

### Verify Installation

```bash
# Check CLI version
polyglot --version

# Check service status
polyglot status
```

You should see:
```
Polyglot CLI v0.1.0
Service Status: Running
  - Trigger Monitor: Active
  - Queue Manager: Active (0 jobs
  - Runner: Active (4 workers
```

## Your First Pipeline: Async File Watcher

Polyglot is an **async-centric** language for pipeline orchestration. Instead of a traditional "Hello World" that prints to console, let's create a pipeline that **watches for events** and **responds asynchronously**.

We'll build a file watcher that:
- Monitors a folder for new files
- Says hello to each new file in a log
- Stops automatically after 10 hellos
- Runs in the background (event-driven

### 1. Create the Pipeline

Create a file called `file_watcher.pg`:

```polyglot
[@] @Local::Examples.FileWatcherHello:1.0.0.0
[X]



[|] |HelloToFiles
[t] |T.File.Created
[<] <path:pg.string << "/tmp/polyglot-demo/*.txt"
[>] >file_path:pg.path >> .new_file
[o] !No.Output

[W] |W.Polyglot.Scope

// Count hellos (line count in log file
[r] .count:pg.int << |U.File.LineCount"hello.log"

// If under limit, say hello
[?] .count <? 10
[~][r] .filename:pg.string << |U.File.Basename"{.new_file"
[~][r] .timestamp:pg.string << |DT.Format"{|DT.Now'', 'HH:mm:ss'"
[~][r] .message:pg.string << "[{.timestamp] Hello to file #{.count + 1: {.filename"
[~]
[~][r] |U.File.AppendLine
[~][<] <file:pg.path << "hello.log"
[~][<] <line:pg.string << .message
[~]
[~][r] |U.Log.Info"{.message"
[~]

// Stop after 10
[?] .count =>? 10
[~][r] |U.Log.Info"Reached 10 hellos! Pipeline complete."
[~][r] |U.Pipeline.Deactivate"@Local::Examples.FileWatcherHello:1.0.0.0|HelloToFiles"
[~]

[o] !No.Output
[X]
```

**What's happening here:**
- `[t] |T.File.Created` - Event-driven trigger (runs when files are created
- `[<] <path:pg.string << "/tmp/polyglot-demo/*.txt"` - Watch pattern
- `[r] .count:pg.int << |U.File.LineCount"hello.log"` - Track state via file
- `[?] .count <? 10` - Conditional logic (stop at 10
- `|U.File.AppendLine` - Async file I/O operation
- `|DT.Now''` - Async datetime operation
- `|U.Pipeline.Deactivate` - Self-deactivation when complete

### 2. Compile the Pipeline

```bash
polyglot compile file_watcher.pg
```

Output:
```
✓ Compiled file_watcher.pg
✓ Generated IR
✓ IR Saved in database
Package Info:
- @Local::Examples.FileWatcherHello:1.0.0.0
- Files                     : 1
- Pipeline(s               : 1
- Enumeration definitions   : 0
- Error definitions         : 0
- Macro definitions         : 0
- Status                    : Inactive
Pipelines:
✓ @Local::Examples.FileWatcherHello:1.0.0.0|HelloToFiles
```

### 3. Create Demo Directory and Activate

```bash
# Create demo directory
mkdir -p /tmp/polyglot-demo

# Create alias for convenience
polyglot alias @Local::Examples.FileWatcherHello:1.0.0.0 Demo

# Activate the pipeline (automatically registers if needed
polyglot activate @Demo|HelloToFiles
```

Output:
```
@Local::Examples.FileWatcherHello:1.0.0.0|HelloToFiles
✓ Checking registration...
✓ Pipeline registered in Local Registry
✓ Inactive -> Active
✓ Watching: /tmp/polyglot-demo/*.txt
```

**Note:** `activate` automatically checks if the pipeline is registered and registers it if needed - you don't need a separate `register` command!

The pipeline is now running in the background, waiting for events!

### 4. Watch the Log (Terminal 1

Open a terminal and stream the log file:

```bash
tail -f hello.log
```

This will show new log entries in real-time as files are created.

### 5. Trigger Events (Terminal 2

In a second terminal, create some files:

```bash
# Create test files
touch /tmp/polyglot-demo/test1.txt
touch /tmp/polyglot-demo/test2.txt
touch /tmp/polyglot-demo/data.txt
```

**Watch Terminal 1!** You'll see:
```
[14:23:45] Hello to file #1: test1.txt
[14:23:47] Hello to file #2: test2.txt
[14:23:49] Hello to file #3: data.txt
```

### 6. Create More Files

```bash
# Create files 4-12
for i in {4..12; do
  touch /tmp/polyglot-demo/file$i.txt
  sleep 1
done
```

Watch the log stream - after the 10th file:
```
[14:24:01] Hello to file #10: file10.txt
Reached 10 hellos! Pipeline complete.
```

The pipeline automatically **deactivates itself**!

### 7. Verify Pipeline Status

```bash
polyglot status @Demo|HelloToFiles
```

Output:
```
@Local::Examples.FileWatcherHello:1.0.0.0|HelloToFiles
Status                : Inactive
Total executions today: 10
```

**Congratulations!** You've created your first **async, event-driven** Polyglot pipeline!

---

## What You Just Learned

This example demonstrates Polyglot's **async-centric** paradigm:

### ✅ Event-Driven Execution
- Pipeline runs in **background**, not one-shot execution
- **Triggered by events** (file creation, not manual calls
- Shows the `|T.File.Created` trigger system

### ✅ Async State Management
- State tracked via **async file operations** (`.count << |U.File.LineCount`
- Variables transition through states: **Declared → Ready**
- Each file event = new async execution context

### ✅ Conditional Flow Control
- `[?]` blocks for **async branching**
- Pipeline makes decisions based on state
- Auto-deactivation when condition met

### ✅ Async I/O Operations
- **File operations**: `|U.File.AppendLine`, `|U.File.Basename`
- **DateTime operations**: `|DT.Now''`, `|DT.Format`
- All operations are **non-blocking** and **composable**

### ✅ Pipeline Lifecycle
- **Activation**: Pipeline starts monitoring
- **Execution**: Responds to events asynchronously
- **Deactivation**: Self-terminates when complete

### Why This Is Different from Traditional Programming

| Traditional "Hello World" | Polyglot Async "Hello World" |
|--------------------------|------------------------------|
| One-shot execution | Background event loop |
| Synchronous print | Async file I/O + logging |
| No state management | Stateful counting |
| Manual invocation | Event-driven triggers |
| Completes immediately | Runs until conditions met |
| Console output only | File + log + self-management |

**Polyglot is not about printing to console - it's about orchestrating async operations in response to events.**

---

## More Automation Examples

Let's create a pipeline that runs on a schedule.

### Daily Greeting Pipeline

Create `daily_greeting.pg`:

```polyglot
[@] @Local::DailyGreeting:1.0.0.0
[X]



[|] |DailyGreeting
[t] |T.DT.Daily
[<] <hour:pg.int << 9
[<] <minute:pg.int << 0

[W] |W.RT.Python3.14

// Get current date using Python datetime
[r] |RT.Python.Run.Function
[<] <function:pg.string << "datetime.datetime.now"
[>] >output:pg.serial >> .now

[r] |RT.Python.Run.Function
[<] <function:pg.string << "datetime.datetime.strftime"
[<] <args:pg.serial << {.now, "%Y-%m-%d"
[>] >output:pg.serial >> .today_result

[r] .today:pg.string << .today_result

// Create message
[r] .message:pg.string << "Good morning! Today is {.today"

// Print to console using standard output pipeline
[r] |U.Log.Info
[<] <message:pg.string << .message

[o] .message:pg.string
[X]
```

**New Concepts:**
- `[t] |T.DT.Daily` - Time-based trigger (runs daily
- `[<] <hour:pg.int << 9` - Trigger configuration for 9 AM
- `[W] |W.RT.Python3.14` - Declare Python 3.14 runtime wrapper (required for all Python operations
- `[r] |RT.Python.Run.Function` - Call Python function via runtime wrapper
  - **Under the hood:** Imports the specified function and calls it with `**kwargs` derived from the `<args>` input
  - **Returns:** The function's return value via `:pg.serial` output
  - **Critical:** Only works if `[W] |W.RT.PythonX.XX` wrapper is declared (where X.XX matches your Python version
  - **Requirements:** The specified module must be available in Python's import path
  - **Alternative:** Use `|RT.Python.Run.File` to execute a Python script file (equivalent to `python file.py` from command line
- `[<] <function:pg.string << "datetime.datetime.now"` - Specify Python function to call (module.function format
- `:pg.serial` - Universal data interchange format for cross-language data

### Compile and Activate

```bash
# Compile the pipeline
polyglot compile daily_greeting.pg

# Activate (automatically registers if needed
polyglot activate daily_greeting
```

Output:
```
✓ Checking registration...
✓ Pipeline registered: daily_greeting
✓ Trigger activated: T.Daily (09:00
✓ Next run: 2025-12-03 09:00:00
```

**Note:** You can also explicitly register first with `polyglot register daily_greeting.pg daily_greeting`, but `activate` does this automatically.

The pipeline will now run automatically every day at 9 AM!

### Test Without Waiting

Don't want to wait until 9 AM? Run it manually:

```bash
polyglot run daily_greeting.pg daily_greeting
```

Output:
```
Good morning! Today is 2025-12-02
{
  "message": "Good morning! Today is 2025-12-02"

```

## Cross-Language Example

Now let's use multiple languages in one pipeline.

### Python + Rust Pipeline

Create `multi_lang.pg`:

```polyglot
[@] @Local::FastFibonacci:1.0.0.0
[X]

[|] |FastFibonacci
[i] .n:pg.int
[t] |T.Call
[o] .result:pg.int
[o] .message:pg.string

[W] |W.RT.Rust1.8
[W] |W.RT.Python3.14

// Use Rust for fast computation
[r] |RT.Rust.Run.File
[<] <file:pg.path << \\FileDir\\rust\\fibonacci.rs
[<] <input:pg.serial << {.n
[>] >output:pg.serial >> .fib_result

[r] .fib_value:pg.int << .fib_result

// Use Python for formatting (simpler than Rust
[r] |RT.Python.Run.Function
[<] <function:pg.string << "format_message"
[<] <args:pg.serial << {.n, .fib_value
[>] >output:pg.serial >> .format_result

[r] .message:pg.string << .format_result

[r] .result:pg.int << .fib_value

[o] .result:pg.int
[o] .message:pg.string
[X]
```

Create `rust/fibonacci.rs` (runtime wrapper receives JSON input:

```rust
use serde_json::Value;

fn main( {
    // Read input from stdin (provided by Polyglot runtime
    let mut input = String::new(;
    std::io::stdin(.read_line(&mut input.unwrap(;
    let data: Value = serde_json::from_str(&input.unwrap(;

    let n = data["n"].as_i64(.unwrap(;

    // Compute Fibonacci
    let result = if n <= 1 {
        n
     else {
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        
        b
    ;

    // Output as JSON to stdout
    println!("{", result;

```

Create `python/format.py` (for string formatting:

```python
# polyglot_input is automatically available as a dict
def format_message(n, fib_value:
    return f"The {nth Fibonacci number is {fib_value"

# If called as function via RT.Python.Run.Function
if __name__ == "__main__":
    # Set polyglot_output for runtime to capture
    polyglot_output = format_message(
        polyglot_input['n'],
        polyglot_input['fib_value']
    
```

**Compile and run:**

```bash
# Compile and run pipeline
polyglot compile multi_lang.pg
polyglot run multi_lang.pg FastFibonacci --input '{"n": 20'
```

Output:
```
{
  "result": 6765,
  "message": "The 20th Fibonacci number is 6765"

```

**What Happened:**
1. Polyglot called the Rust function for fast computation
2. Polyglot called Python for easy string formatting
3. Results combined seamlessly

## CLI Workflow Summary

The typical Polyglot workflow for **event-driven pipelines**:

```bash
# 1. Write .pg file with event trigger
vim my_pipeline.pg

# 2. Compile to IR database
polyglot compile my_pipeline.pg

# 3. Create alias (optional, for convenience
polyglot alias @Local::MyPackage:1.0.0.0 MyAlias

# 4. Activate the pipeline (auto-registers + starts monitoring
# Note: 'activate' checks registration and registers if needed
polyglot activate @MyAlias|PipelineName

# 5. Monitor execution in real-time
tail -f log_file.log        # For file-based logging
# OR
polyglot logs PipelineName  # For service logs

# 6. Check pipeline status
polyglot status @MyAlias|PipelineName

# 7. Deactivate when done (or let it self-deactivate
polyglot deactivate @MyAlias|PipelineName
```

**For manual-call pipelines** (using `[t] |T.Call` trigger:

```bash
# Compile
polyglot compile my_pipeline.pg

# Run directly (one-shot execution
polyglot run my_pipeline.pg pipeline_name --input '{...'
```

**Example from our file watcher:**
```bash
polyglot compile file_watcher.pg
polyglot alias @Local::Examples.FileWatcherHello:1.0.0.0 Demo
polyglot activate @Demo|HelloToFiles
tail -f hello.log
# In another terminal: touch /tmp/polyglot-demo/test.txt
```

## Service Management

### Start/Stop Service

```bash
# Start service
polyglot-service start

# Stop service
polyglot-service stop

# Restart service
polyglot-service restart

# Check status
polyglot status
```

### View Service Logs

```bash
# Tail all service logs
polyglot logs --service

# View specific pipeline logs
polyglot logs my_pipeline

# View failed executions only
polyglot logs my_pipeline --failed
```

### Monitor Queues

```bash
# View queue status
polyglot queue status

# View pending jobs
polyglot queue list

# View failed jobs
polyglot queue failed

# Retry failed job
polyglot queue retry <job-id>
```

## Common Commands Reference

| Command | Description |
|---------|-------------|
| `polyglot compile <file>` | Compile .pg file to IR database |
| `polyglot run <file> <pipeline> [--input ...]` | Run pipeline manually |
| `polyglot register <file> <pipeline>` | Register pipeline with service (optional - `activate` does this |
| `polyglot activate <pipeline>` | Activate pipeline triggers (auto-registers if needed |
| `polyglot deactivate <pipeline>` | Deactivate pipeline triggers |
| `polyglot alias <package> <alias>` | Create short alias for package |
| `polyglot list` | List all registered pipelines |
| `polyglot logs <pipeline>` | View pipeline execution logs |
| `polyglot status` | Check service status |
| `polyglot queue status` | View queue statistics |

## Example: File Monitoring Pipeline

Let's create a practical pipeline that processes uploaded files.

Create `process_uploads.pg`:

```polyglot
[@] @Local::ProcessUpload:1.0.0.0
[X]

[|] |ProcessUpload
[t] |T.File.Created
[<] <path:pg.string << "/uploads/*.csv"
[>] >file_path:pg.path >> .file_path
[o] .processed_path:pg.path

[W] |W.RT.Python3.14
[<] <requirements:pg.file << \\FileDir\\python\\requirements.txt

[W] |W.RT.Rust1.8

// Read CSV with Python (pandas
[r] |RT.Python.Run.Function
[<] <function:pg.string << "pandas.read_csv"
[<] <args:pg.serial << {.file_path
[>] >output:pg.serial >> .data

// Process with Rust (faster - remove duplicates
[r] |RT.Rust.Run.File
[<] <file:pg.path << \\FileDir\\rust\\cleaner.rs
[<] <input:pg.serial << .data
[>] >output:pg.serial >> .cleaned

// Validate with Rust
[r] |RT.Rust.Run.File
[<] <file:pg.path << \\FileDir\\rust\\validator.rs
[<] <input:pg.serial << .cleaned
[>] >output:pg.serial >> .validated

// Get basename for output path
[r] |U.File.Basename
[<] <path:pg.path << .file_path
[>] >basename:pg.string >> .filename

[r] .output_path:pg.path << "/processed/{.filename"

// Save result with Python pandas
[r] |RT.Python.Run.Function
[<] <function:pg.string << "pandas.DataFrame.to_csv"
[<] <args:pg.serial << {.validated, .output_path

[r] .processed_path:pg.path << .output_path

[o] .processed_path:pg.path
[X]
```

**What This Does:**
1. Watches `/uploads/` for new CSV files
2. When a CSV appears, automatically processes it
3. Reads with pandas (easy
4. Cleans and validates with Rust (fast
5. Saves processed CSV to `/processed/`

**Compile and activate:**

```bash
polyglot compile process_uploads.pg
polyglot activate process_upload  # Auto-registers if needed
```

Now any CSV file dropped in `/uploads/` will be processed automatically!

**Test it:**

```bash
# Copy a test file
cp test.csv /uploads/

# Check logs
polyglot logs process_upload
```

## Example: API Integration

Create an hourly pipeline that fetches data from an API and stores it.

Create `api_monitor.pg`:

```polyglot
[@] @Local::FetchMetrics:1.0.0.0
[X]

[|] |FetchMetrics
[t] |T.DT.Hourly
[<] <minute:pg.int << 0
[i] .api_url:pg.url
[i] .db_connection:pg.string
[o] .rows_inserted:pg.int

[W] |W.RT.Python3.14
[<] <requirements:pg.file << \\FileDir\\python\\requirements.txt

[W] |W.RT.Rust1.8

[W] |W.DB.Postgresql
[>] >session:pg.db >> .db_session

// Fetch from API (using standard API utility
[r] |U.API.Rest.Get
[<] <url:pg.url << .api_url
[>] >response:pg.string >> .raw_response

// Parse JSON response
[r] |RT.Python.Run.Function
[<] <function:pg.string << "json.loads"
[<] <args:pg.serial << {.raw_response
[>] >output:pg.serial >> .data

// Transform data (Rust - fast processing
[r] |RT.Rust.Run.File
[<] <file:pg.path << \\FileDir\\rust\\transformer.rs
[<] <input:pg.serial << .data
[>] >output:pg.serial >> .transformed

// Insert to database
[r] |DB.BulkInsert
[<] <session:pg.db << .db_session
[<] <table:pg.string << "metrics"
[<] <data:pg.serial << .transformed
[>] >result:pg.int >> .rows_inserted

[o] .rows_inserted:pg.int
[X]
```

**Activate with configuration:**

```bash
polyglot activate api_monitor.pg FetchMetrics \
  --config api_url=https://api.example.com/metrics \
  --config db_connection=postgresql://localhost/metrics
```

**Note:** `activate` auto-registers with the provided configuration.

Metrics will be fetched and stored every hour!

## Debugging Tips

### 1. Compile Errors

```bash
polyglot compile my_pipeline.pg
```

If compilation fails, you'll see detailed error messages:

```
Error: Type mismatch at line 15
  Expected: Integer
  Found: String

  .count << "not a number"
            ^^^^^^^^^^^^^^^
```

### 2. Runtime Errors

```bash
# View recent errors
polyglot logs my_pipeline --failed

# View specific execution
polyglot logs my_pipeline --execution <id>
```

### 3. Dry Run

```bash
# Validate without executing
polyglot run my_pipeline.pg pipeline_name --dry-run --input '{...'
```

### 4. Verbose Output

```bash
# Show detailed execution trace
polyglot run my_pipeline.pg pipeline_name --verbose --input '{...'
```

## Troubleshooting Common Issues

### Installation Problems

#### Docker: "Cannot connect to Docker daemon"

**Problem:** `docker run` fails with connection error.

**Solution:**
```bash
# Start Docker daemon
sudo systemctl start docker

# Or on macOS
open -a Docker

# Verify Docker is running
docker ps
```

#### CLI: "polyglot: command not found"

**Problem:** After `cargo install polyglot-cli`, the command isn't found.

**Solution:**
```bash
# Ensure Cargo bin is in PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Or add to ~/.zshrc if using zsh

# Verify installation
which polyglot
```

#### Service: "Service failed to start"

**Problem:** `polyglot-service start` fails immediately.

**Solution:**
```bash
# Check port 8080 isn't already in use
sudo lsof -i :8080
# If something is using it, kill that process or change Polyglot's port

# Check service logs for specific error
journalctl -u polyglot-service -n 50

# Verify PostgreSQL is running (if using PostgreSQL backend)
systemctl status postgresql
```

### First Pipeline Issues

#### "Package declaration must be first block"

**Problem:** Compilation fails with block order error.

**Incorrect:**
```polyglot
[|] |MyPipeline        // ERROR: Pipeline before package
[@] @Local::Test:1.0.0.0
```

**Correct:**
```polyglot
[@] @Local::Test:1.0.0.0  // Package ALWAYS first
[X]                        // Close package block

[|] |MyPipeline            // Then pipeline definitions
```

#### "Unexpected whitespace before type colon"

**Problem:** Warning about type annotation formatting.

**Non-Canonical (triggers warning):**
```polyglot
[r] .count :pg.int << 42   // Warning: space before colon
```

**Canonical:**
```polyglot
[r] .count:pg.int << 42    // No space before colon
```

**Note:** This is a warning, not an error - your code will still compile.

#### "Pipeline not found"

**Problem:** `polyglot run` can't find your pipeline.

**Solution:**
```bash
# Make sure pipeline name matches definition
# File: example.pg
# [|] |MyPipeline  <- This is the pipeline name

# Run with EXACT name (case-sensitive)
polyglot run example.pg MyPipeline --input '{}'

# List available pipelines in file
polyglot compile example.pg --list-pipelines
```

### Runtime Issues

#### Trigger Never Fires

**Problem:** Event-driven pipeline doesn't run.

**Checklist:**
```bash
# 1. Verify pipeline is activated
polyglot list --triggers
# Should show your pipeline with "Status: Active"

# 2. Check trigger monitor is running
polyglot status --service trigger-monitor
# Should show "Status: Running"

# 3. For file triggers, verify path exists and is writable
# For time triggers, check system time is correct
date

# 4. Check trigger monitor logs
polyglot logs --service trigger-monitor --follow
```

#### "No workers available"

**Problem:** Jobs sit in queue but don't execute.

**Solution:**
```bash
# Check runner service status
polyglot status --service runner

# If stopped, restart it
sudo systemctl restart polyglot-runner

# Check worker count
polyglot status --verbose
# Should show "Workers: X/Y (X active, Y total)"

# Increase workers if needed (edit /etc/polyglot/runner.yaml)
num_workers: 8  # Increase from 4
```

### Language Runtime Issues

#### Python: "Module not found"

**Problem:** Pipeline fails with Python import error.

**Solution:**
```bash
# Verify Python runtime is configured
cat /etc/polyglot/runner.yaml
# Check python.virtualenv path exists

# For wrapper-specific dependencies, use [W] block:
[W] |W.RT.Python3.14
[<] <requirements:pg.file << \\FileDir\\requirements.txt
# This installs dependencies before execution
```

#### Rust: "Binary not found"

**Problem:** Rust wrapper fails to find compiled binary.

**Solution:**
```bash
# Rust binaries are pre-compiled during registration
# Re-register to rebuild:
polyglot unregister my_pipeline
polyglot register my_pipeline.pg my_pipeline

# Check compilation logs
polyglot logs my_pipeline --compilation
```

### Permission Errors

#### "Permission denied" on /var/lib/polyglot

**Problem:** Service can't write to data directory.

**Solution:**
```bash
# Create directory with correct permissions
sudo mkdir -p /var/lib/polyglot
sudo chown -R $USER:$USER /var/lib/polyglot

# Or run service as polyglot user
sudo useradd -r polyglot
sudo chown -R polyglot:polyglot /var/lib/polyglot
```

### Getting Help

If you're still stuck:

1. **Check service logs:** `polyglot logs --service`
2. **Enable debug mode:** `POLYGLOT_LOG=debug polyglot run ...`
3. **Review examples:** See [examples/](examples/) for working code
4. **Read advanced docs:** [Polyglot Service Guide](polyglot-service.md) for service troubleshooting

**Common Gotchas:**
- Pipeline names are case-sensitive
- Package declaration `[@] ... [X]` must be first in file
- All variables need operator prefix (`.var`, `|pipe`, `#enum`, etc.)
- Triggers require `polyglot activate` before they'll fire
- No whitespace before type colon (`.var:type` not `.var :type`)

---

## Next Steps

Now that you have Polyglot running:

1. **Understand Async:** Read [Async-Centric Language](async-centric-language.md to understand how Polyglot handles async operations
2. **Learn States:** Study [Variable State System](variable-state-system.md for deep understanding
3. **Master Syntax:** Explore [Syntax Overview](syntax/overview.md for complete language reference
4. **See Examples:** Check [Automation Workflows](examples/automation-workflows.md for real-world patterns
5. **Advanced Features:** Learn [Parallel Execution](advanced/parallel-execution.md and [DateTime System](advanced/datetime-system.md

## Quick Reference

### Event-Driven Pipeline Structure

```polyglot
[@] @Registry::PackageName:Version
[X]

[|] |PipelineName
[t] |T.Event.Trigger      // Event trigger (file, time, etc.
[<] <config:pg.type      // Trigger configuration
[>] >event_data >> .data  // Event data output
[o] .output:pg.type      // Pipeline outputs

[W] |W.Polyglot.Scope     // Wrapper (or runtime wrapper

// Pipeline body - async operations
[r] .variable:pg.type << |AsyncOperation""
[r] .result:pg.type << |ProcessData"{.variable"

[o] .output:pg.type
[X]
```

### Manual-Call Pipeline Structure

```polyglot
[@] @Registry::PackageName:Version
[X]

[|] |PipelineName
[i] .input:pg.type       // Inputs
[t] |T.Call               // Manual call trigger
[o] .output:pg.type      // Outputs

[W] |W.Polyglot.Scope

[r] .result:pg.type << .input

[o] .output:pg.type
[X]
```

### Running Pipelines

```bash
# Event-driven (background execution
polyglot compile file.pg
polyglot activate @Package|PipelineName
tail -f log.txt

# Manual call (one-time execution
polyglot run file.pg pipeline_name --input '{"key": "value"'
```

### Service Commands

```bash
polyglot status          # Check service health
polyglot list            # List pipelines
polyglot logs <name>     # View logs
polyglot queue status    # Check queue
```

**You're ready to start building with Polyglot!**
