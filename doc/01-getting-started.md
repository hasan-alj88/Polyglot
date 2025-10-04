# Getting Started with Polyglot

[← Back to README](../README.md)

## Table of Contents
- [Installation](#installation)
- [Your First Pipeline](#your-first-pipeline)
- [Running Pipelines](#running-pipelines)
- [Basic Concepts](#basic-concepts)
- [Next Steps](#next-steps)

## Installation

**Note:** Polyglot is currently in the brainstorming phase. Installation instructions will be available once the MVP is complete.

### Planned Installation Methods

**Planned** Means Not available Yet

**Via Install Script (Linux/macOS):**
```bash
curl -sSL https://polyglot.io/install.sh | sh
```

**Via Package Managers:**
```bash
# Homebrew (macOS)
brew install polyglot

# APT (Debian/Ubuntu)
sudo apt install polyglot

# DNF (Fedora)
sudo dnf install polyglot
```

## Your First Pipeline

Let's create a simple "Hello World" pipeline.

### Step 1: Create a File

Create a file named `hello.pg`:

```polyglot
[@] com.example>HelloWorld
[X]

[|] SayHello
[i] name: pg\string
[t] |T.Cli << "HelloWorld"
[r] |U.Console.Print << "Hello, {name}!"

[o] >> name
[x]
```

### Step 2: Run the Pipeline

Compile and register `hello.pg`
```bash
ployglot register hello.pg
>> [Registed] SayHello
```
Output will show Compile errors if any else it will display registered pipelines.
The pipline invocation is defined in `[t]` for our pipline its invoked by commandline as follows
```bash
polyglot run "SayHello" --input name="Hasan"
>> Hello, Hasan!
```

### What Just Happened?

Let's break down the pipeline:

```polyglot
[@] com.example>HelloWorld  \\ Namespace declaration
[X]                          \\ End of imports

[|] SayHello                 \\ Pipeline definition
[i] name: pg\string          \\ Input parameter of type string
[t] |T.Call                  \\ Trigger: can be called directly

[r] |U.Console.Print << "Hello, {name}!"  \\ Print message

[o] >> name                  \\ Output the name
[x]                          \\ End pipeline
```

## Polyglot is an automation programming language

### Direct Invocation

Run a pipeline with the CLI:

```bash
polyglot run myfile.pg --input param1="value1" --input param2=123
```

### File Watch Trigger

Create a pipeline that runs when a file changes:

```polyglot
[@] com.example>FileWatcher
[X]

[|] ProcessFile
[i] file_path: pg\string
[t] |T.FileCreated << "data/*.csv"

[w] |W.Python3.11

[r] |U.Console.Print << "New file detected: {file_path}"
[r] |ProcessCSV << file_path

[o] >> file_path
[x]
```

Start watching:

```bash
polyglot watch myfile.pg
```

Now when you create a CSV file in the `data/` directory, the pipeline automatically runs.

### Scheduled Execution

Run a pipeline daily at 2 AM:

```polyglot
[@] com.example>DailyReport
[X]

[|] GenerateReport
[t] |T.Daily << "02:00"

[r] |U.Console.Print << "Generating daily report..."
[r] |CreateReport >> report

[o] >> report
[x]
```

Start the scheduler:

```bash
polyglot start  # Runs as daemon
polyglot status # Check status
polyglot stop   # Stop daemon
```

## Basic Concepts

### 1. Pipelines

Pipelines are the core unit of work in Polyglot. They have:
- **Inputs** (`[i]`) - Data the pipeline receives
- **Triggers** (`[t]`) - Conditions that activate the pipeline
- **Execution** (`[r]`) - The actual work
- **Outputs** (`[o]`) - Data the pipeline returns

### 2. Triggers

Triggers define when a pipeline runs:

```polyglot
[t] |T.Call                        \\ Manual/CLI invocation
[t] |T.FileChanged << "*.txt"      \\ File system events
[t] |T.Daily << "14:30"            \\ Scheduled time
[t] |T.CpuHigh << 90.0             \\ Resource threshold
```

Multiple triggers can be combined - ALL must be satisfied for the pipeline to run.

### 3. Sequential Execution

By default, operations in `[r]` blocks run sequentially:

```polyglot
[r] |Step1 >> result1
[r] |Step2 << result1 >> result2
[r] |Step3 << result2 >> result3
```

Step2 waits for Step1, Step3 waits for Step2.

### 4. Parallel Execution

Use `[f]` (fork) to run operations in parallel:

```polyglot
[f] |TaskA >> resultA
[f] |TaskB >> resultB
[f] |TaskC >> resultC
[j] |JoinAll  \\ Wait for all to complete
```

All three tasks run simultaneously, then join before continuing.

### 5. Error Handling

Handle errors explicitly:

```polyglot
[r] |RiskyOperation >> result

[!] !> py\!ValueError
[~][r] |U.Console.Print << "Caught ValueError"
[~][r] |HandleError

[!] !> pg\!NetworkError
[~][r] |U.Console.Print << "Network error"
[~][r] |RetryOperation
```

### 6. Type System

Variables have types indicating their language and datatype:

```polyglot
name: pg\string       \\ Polyglot string
data: py\dict         \\ Python dictionary
numbers: rust\Vec<i32> \\ Rust vector of integers
```

Polyglot automatically converts between compatible types.

## Complete Example

Here's a more complete example that demonstrates multiple concepts:

**data_processor.pg:**
```polyglot
[@] com.example>DataProcessor
[X]

[|] ProcessDataFile
[i] input_file: pg\string
[t] |T.FileCreated << "data/*.csv"

[Q] |Q.Priority << 2
[Q] |Q.CpuAvailable << 75.0

[w] |W.Python3.11

[\] |U.Log << "Starting data processing for {input_file}"

[r] |ReadCSV << input_file >> raw_data

[!] !> py\!FileNotFoundError
[~][r] |U.Log.Error << "File not found: {input_file}"
[~][x] |Exit << 404

[r] |ValidateData << raw_data >> is_valid

[?] is_valid ?> False
[~][r] |U.Log.Error << "Data validation failed"
[~][x] |Exit << 400

[f] |CleanData << raw_data >> clean_data
[f] |CalculateStats << raw_data >> stats
[j] |JoinAll

[r] |SaveResults << clean_data << stats >> output_file

[/] |U.Log << "Processing complete: {output_file}"

[o] >> output_file
[x]

[|] ReadCSV
[i] file_path: pg\string
[t] |T.Call
[w] |With.Python3.11

[r] |PythonReadCSV << file_path >> data
[o] >> data
[x]

[|] ValidateData
[i] data: py\dict
[t] |T.Call

[r] |CheckDataStructure << data >> valid
[o] >> valid
[x]

[|] CleanData
[i] data: py\dict
[t] |T.Call

[r] |PythonCleanData << data >> cleaned
[o] >> cleaned
[x]

[|] CalculateStats
[i] data: py\dict
[t] |T.Call

[r] |PythonCalculateStats << data >> statistics
[o] >> statistics
[x]

[|] SaveResults
[i] clean_data: py\dict
[i] stats: py\dict
[t] |T.Call

[r] |PythonSaveToFile << clean_data << stats >> file_path
[o] >> file_path
[x]
```

**Run it:**
```bash
polyglot watch data_processor.pg
```

Now drop a CSV file into the `data/` directory and watch it process automatically.

## Next Steps

Now that you understand the basics, explore these topics:

1. **[Language Syntax](02-language-syntax.md)** - Complete syntax reference
2. **[Error Handling](06-error-handling.md)** - Robust error handling patterns
3. **[Flow Control](07-flow-control.md)** - Switch statements and conditionals
4. **[Standard Library](05-standard-library.md)** - All built-in pipelines
5. **[Queue System](08-queue-system.md)** - Resource management and queuing

### Common Patterns to Learn

**Pattern 1: ETL Pipeline**
Extract data, transform it, load it somewhere else.

**Pattern 2: File Processing**
Watch directories, process files as they arrive.

**Pattern 3: Scheduled Tasks**
Run maintenance, reports, backups on schedule.

**Pattern 4: Event-Driven Workflows**
Respond to system events, webhooks, message queues.

**Pattern 5: Multi-Language Integration**
Use Python for data prep, Rust for computation, Node for APIs.

### Tips for Success

1. **Start Simple** - Begin with sequential pipelines before using parallel execution
2. **Handle Errors** - Always use `[!]` blocks for operations that can fail
3. **Use Descriptive Names** - Clear pipeline and variable names make debugging easier
4. **Test Incrementally** - Build and test pipelines step by step
5. **Monitor Resources** - Use queue configuration to prevent resource exhaustion

### Getting Help

- **Documentation** - Read the complete docs in this repository
- **Examples** - Check the `examples/` directory (coming soon)
- **Community** - Join discussions on GitHub
- **Issues** - Report bugs or request features on GitHub Issues

---

[Next: Language Syntax →](02-language-syntax.md)