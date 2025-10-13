# Getting Started with Polyglot

[← Back to README](../README.md)

## Table of Contents
- [Installation](#installation)
- [Your First Pipeline](#your-first-pipeline)
- [Polyglot is an Automation Programming Language](#polyglot-is-an-automation-programming-language)
- [Basic Concepts](#basic-concepts)
- [Next Steps](#next-steps)

## Installation

Still in the production phase.

## Your First Pipeline

Let's create a simple "Hello World" pipeline.

### Step 1: Plan Your Automation

Before coding your automation, you must consider the following:
1. What are your pipeline triggers? 
2. What are your code bases and if there are any setting-up requirements they may need? In our case, we'll use Python and Rust codes with builtin functions.

### Step 2: Implement the Codes 

**hello.py**
```python
import click

@click.command()
@click.argument('name')
def hello(name: str):
    click.echo(f'Python says Hello, {name}!')
```

requirement.txt
```txt
click
```

**hello.rs**
```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "hello")]
#[command(about = "A simple hello program", long_about = None)]
struct Cli {
    /// Name to greet
    name: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Rust says Hello, {}!", cli.name);
}
```

### Step 3: Create a Polyglot File

Create a file named `hello.pg`:


```polyglot
\\ Define current module and imports
[@] com.example>HelloWorld
[X]

\\ Pipeline Definition
[|] SayHello
[i] name: pg\string
[t] |T.Cli
[<] .process: pg\string = "HelloWorld"

\\ New command-line terminal session
[w] |W.Cli

\\ Definitions\variable declations 
[D] py_file: pg\path = //FileDir//examples/hello.py
[D] py_dependency: pg\path = //FileDir//requirement.txt
[D] rs_file: pg\path = //FileDir//examples/hello.rs

\\ Say hello in Python
[f] |Cli.Python3.9
[<] .dependency: pg\path = py_dependency
[<] .file = py_file
[<] .arg.name: py\str = name

\\ Say hello in Rust
[f] |Cli.Rust1.8
[<] .file = rs_file
[<] .arg.name: rust\string = name

[>] None
[x]
```

### Step 4: Run the Pipeline

Compile and register `hello.pg`:
```bash
polyglot register hello.pg
>> [Registered] SayHello
```

Output will show compile errors if any, otherwise it will display registered pipelines.

The pipeline invocation is defined in `[t]`. For our pipeline, it's invoked by the command line as follows:
```bash
polyglot run "SayHello" --input name="Hasan"
>> Rust says Hello, Hasan!
>> Python says Hello, Hasan!
```

### What Just Happened?

Let's break down the pipeline:

**Module and imports:**
```polyglot
\\ Define current module and imports
[@] com.example>HelloWorld
\\[D] @Import.Some.PreDefined.Library
[X]
```

This defines this file's module and imports modules from predefined libraries.

**Pipeline structure:**
```polyglot
\\ Pipeline Definition
[|] SayHello
[t] ... Triggers
[i] ... Inputs
[\] ... Setup
... processes ...
[/] ... Cleanup
[>] Outputs
[x]
```

**Or using wrappers:**
```polyglot
\\ Pipeline Definition
[|] SayHello
[t] ... Triggers
[i] ... Inputs
[w] ... Wrappers (handle setup and cleanup)
... processes ...
[>] Outputs
[x]
```

**Trigger configuration:**
```polyglot
[t] |T.Cli
[<] .process = "HelloWorld"
```

This makes the pipeline invokable via:
```bash
polyglot run "SayHello" --input name="Hasan"
```

**Wrappers:**

Polyglot's standard library provides wrappers for common runtime environments. In this example:
```polyglot
\\ New command-line terminal session
[w] |W.Cli
```
Which is a wrapper creates a shell script of the pipline with Error handling and graceful exit to be run when pipline invoked.   

**Type conversions:**

Polyglot automatically converts between language-specific types:
- `name: py\str = name` converts `pg\string` → Python `str`
- `name: rust\string = name` converts `pg\string` → Rust `String`

Refer to the standard library documentation for available triggers, wrappers, and builtin functions.

## Polyglot is an Automation Programming Language

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

[|] ProcessFiles
[i] file_path: pg\string
[t] |T.Folder.FileCreated
[<] .folder: pg\path = //FileDir//data/
[<] .pattern: pg\array<path> = "*.csv"

[w] |W.Python3.11

[r] |U.Folder.FileCreated
[<] .folder: pg\path = //FileDir//data/
[<] .pattern: pg\array<path> = "*.csv"
[>] .new_files: pg\array<path> = new_files

[f] ~For.Each
[<] ... new_files
[>] ... new_file
[~]
[~][Q] |Q.CPU.Avaliable.MoreThan
[~][<] .threshold: pg\float = 50
[~]
[~][Q] |Q.RAM.Avaliable.MoreThan
[~][<] .threshold.mb: pg\float = 500
[~]
[~][r] |U.Console.Print
[~][<] .message: pg\string = "New file detected: {file_path}"
[~]
[~][r] |ProcessCSV
[~][<] .path: pg\path = file_path
[~][>] .status: pg\boolen = status
[~]
[~][j] ~Y.JoinAll
[~][<] ... status

[>] file_path
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
[D] @MyLib = @Hasan.MyLib.Example \\ import and alias
[X]

[|] GenerateReportschedule
[t] |T.Daily
[<] .time = T"02::" \\ every 2AM

[r] |U.Console.Print
[<] .message = "Generating daily report..."

[r] @MyLib|CreateReport
[>] .repot:py\str = report

[>] report
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

#### Defining Pipeline

Pipelines are the core unit of work in Polyglot. They have:
- **Inputs** (`[i]`) - Data the pipeline receives
- **Setup and cleanup** ({`[\]`,`[/]`} or `[W]`) - Setup preparation and cleanup
- **Triggers** (`[t]`) - Conditions that activate the pipeline
- **Execution** (`[r]`, `[f]`, `[b]`, `[?]`) - The actual work
- **Outputs** (`[o]`) - Data the pipeline returns

#### predefined pipeline

```ployglot
[r] |U.Some.Predifed.Pipeline
[<] .input_argument: datatype = value
[>] .output_argument: datatype 
```

Predefined pipelines always start with pipe character `|` and the name is camel-case, the dot '.' is used to group pipelines categorizes. 
However, pipelines names may not start with dot (`|.`).
It is mandatory to explicitly state the keywords and datatypes of the predefined pipeline's input and output arguments with a '.' dot at start.
Pipelines arguments are snake-case including dot `.`.

The only exception is Data race pipelines start with `|Y.` where only the variable from parallel pipline output is needed after `...` in place of argument keyword and type which are inferred.


```ployglot
[f] |Task1
[>] .ouput: datatype = out1

[f] |Task2
[>] .ouput: datatype = out2

[J] |Y.JoinAll
[<] ... out1
[<] ... out2

```

however, output is required like 

```ployglot
[f] |Task1
[>] .ouput: datatype = out1

[f] |Task2
[>] .ouput: datatype = out2

[J] |Y.JoinFirst
[<] ... out1
[<] ... out2
[>] .first: datatype = first
```

or 


```ployglot
[f] |Task1
[>] .ouput: datatype = out1

[f] |Task2
[>] .ouput: datatype = out2

[f] |Task3
[>] .ouput: datatype = out3

[J] |Y.JoinNth
[<] .n : pg\uint = 2
[<] ... out1
[<] ... out2
[<] ... out3
[>] .nth: datatype = secound
```

### 2. Triggers

Triggers define when a pipeline runs:

```polyglot
[t] |T.Call                          \\ Manual/CLI invocation

[t] |T.File.Modified
[<] .path = full\file\path.txt       \\ File Change events

[t] |T.Daily
[<] .time = "14:30"                   \\ Scheduled time

[t] |T.Cpu.LessThan
[<] .threshold = 90.0                 \\ Resource threshold
```

Multiple triggers can be combined—ALL must be satisfied for the pipeline to run.

### 3. Sequential Execution

By default, operations in `[r]` blocks run sequentially:

```polyglot
[r] |Step1
[>] .input_argument: datatype = result1

[r] |Step2
[<] .input_argument: datatype = result1
[>] .output_argument: datatype = result2

[r] |Step3
[<] .input_argument: datatype = result2
[>] .output_argument: datatype = result3
```

Step2 waits for Step1, Step3 waits for Step2.

### 4. Parallel Execution

Use `[f]` (fork) to run operations in parallel:

```polyglot
[f] |TaskA
[>] .output_argument: datatype = resultA

[f] |TaskB
[>] .output_argument: datatype = resultB

[f] |TaskC
[>] .output_argument: datatype = resultC

[j] |Y.JoinAll  \\ Wait for all to complete
[<] ... resultA
[<] ... resultB
[<] ... resultC

\\ Then you may use results variables
```

All three tasks run simultaneously, then join before continuing.

### 5. Error Handling

Handle errors explicitly:

```polyglot
[r] |RiskyOperation
[>] .output_argument: datatype = result

[~][!] !> py\!ValueError
[~][~][r] |U.Console.Print
[~][~][<] .message: pg\string = "Caught ValueError"
[~][~][r] |HandleError

[~][!] !> pg\!NetworkError
[~][~][r] |U.Console.Print
[~][~][<] .message: pg\string = "Network error"
[~][~][Q] |RetryOperation
[~][~][<] .retry.conunt: pg\uint = 6
```

### 6. Type System

Variables have types indicating their language and datatype:

```polyglot
name: pg\string        \\ Polyglot string
data: py\dict          \\ Python dictionary
numbers: rust\Vec<i32> \\ Rust vector of integers
```

Polyglot automatically converts between compatible types, implicitly runs 
``ployglot
[r] |U.Convert.Lang1.Datatype1.To.Lang2.Datatype2
[<] .in: Lang1\Datatype1 = in_value
[>] .out: Lang2\Datatype2 = out_value
``



## Complete Example

Here's a more complete example that demonstrates multiple concepts:

**data_processor.pg:**
```polyglot
[@] com.example>DataProcessor
\\ alias @Hasan>Examples>DataAnalysis > @Analysis
[D] @Analysis = @Hasan>Examples>DataAnalysis
[X]

[|] ProcessDataFile
[i] input_file: pg\path
[t] |T.File.Modified
[<] .path: pg\path = input_file

[Q] |Q.Priority
[<] .level: pg\uint = 2
[Q] |Q.CpuAvailable
[<] .threshold: pg\float = 75.0

[w] |W.Python3.11
[<] .requirments: pg\path = Path\to\requirments.txt

[\] |U.Log.Info
[<] .message: pg\string = f"Starting data processing for {input_file}"
[\] |S.Cli.Session.Start
[>] .cli.script: pg\path = script_file
[>] .pid: pg\string = session_pid

[r] |U.Data.CSV.Read
[<] .file: pg\path = input_file 
[>] .data: pg\string = raw_data
[~][!] !> py\!FileNotFoundError
[~][~][r] |U.Log.Error
[~][~][<] .message: pg\string = f"File not found: {input_file}"
[~][~][x] |Exit
[~][~][<] .code: pg\uint = 404

[r] @Analysis|ValidateData
[<] .raw_data: pg\string = raw_data
[>] .is_valid: pg\bool = isv_alid

[?] is_valid ?> False
[~][r] |U.Log.Error
[~][<] .message: pg\string = "Data validation failed"
[~][x] |Exit
[~][<] .code: pg\uint = 400

[f] @Analysis|CleanData
[<] .raw_data: pg\string = raw_data
[>] .clean_data: pg\string = clean_data

[f] @Analysis|CalculateStats
[<] .raw_data: py\string = raw_data
[>] .stats: py\dict = stats

[j] |Y.JoinAll
[<] ... clean_data
[<] ... stats

[r] @Analysis|SaveResults
[<] .clean_data: pg\string = clean_data
[<] .stats: py\dict = stats
[>] .output_file: pg\path = output_file

[/] |U.Log.Info
[<] message = "Processing complete: {output_file}"
[/] |C.Cli.Session.Close
[<] .script_file: pg\path
[<] .pid: pg\string = session_pid

[o] .output_file: pg\path = output_file
[x]
```

**Run it:**
```bash
polyglot watch data_processor.pg
```

Now polyglot trigger monitor file watcher will watch the CSV file, and it will process automatically if file has been modified.

## Next Steps

Now that you understand the basics, explore these topics:

1. **[Language Syntax](02-language-syntax.md)**—Complete syntax reference
2. **[Error Handling](06-error-handling.md)**—Robust error handling patterns
3. **[Flow Control](07-flow-control.md)**—Switch statements and conditionals
4. **[Standard Library](05-standard-library.md)**—All built-in pipelines
5. **[Queue System](08-queue-system.md)**—Resource management and queuing

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

1. **Start Simple**—Begin with sequential pipelines before using parallel execution
2. **Handle Errors**—Always use `[!]` blocks for operations that can fail
3. **Use Descriptive Names**—Clear pipeline and variable names make debugging easier
4. **Test Incrementally**—Build and test pipelines step by step
5. **Monitor Resources**—Use queue configuration to prevent resource exhaustion

### Getting Help

- **Documentation**—Read the complete docs in this repository
- **Examples**—Check the `examples/` directory (coming soon)
- **Community**—Join discussions on GitHub
- **Issues**—Report bugs or request features on GitHub Issues

---

[Next: Language Syntax →](02-language-syntax.md)