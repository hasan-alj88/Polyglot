# Hello World Tutorial - Multi-Language Pipeline

**Version:** v0.0.4
**Difficulty:** Beginner
**Time:** 15 minutes
**Demonstrates:** CLI triggers, runtime wrappers, sequential execution, file I/O

---

## What You'll Build

A Polyglot pipeline that orchestrates three language runtimes (Python, Rust, JavaScript) to write "Hello World" messages to a log file, demonstrating cross-language integration.

**Run with:**
```bash
polyglot run helloworld
```

**Output:** A log file with greetings from all three languages!

---

## Complete Code

**File:** `HelloWorld.pg`

```polyglot
{@} @Local::Examples.HelloWorld:1.0.0.0
{x}



{|} |HelloWorld
[%] %Doc "Multi-language Hello World pipeline that writes to a log file"

[t] |T.CLI"helloworld"

[|] <log_path :pg.path <~ \\FileDir\\hello_world.log
[|] >error <~ !NoError

[w] |W.RT.Python3.9
(|) <requirements:pg.path << \\NoPath\\
(|) >env:pg.serial >> $py

[w] |W.RT.Rust
(|) <dependencies:pg.path << \\NoPath\\
(|) >env:pg.serial >> $rust

[w] |W.RT.JS
(|) <packages:pg.path << \\NoPath\\
(|) >env:pg.serial >> $js

[r] $timestamp :pg.dt << |DT.Now"iso8601"
[r] $header :pg.string << "=== Hello World Example - {$timestamp} ===\n"

[r] |U.File.Text.Append
(|) <file:pg.path << $log_path
(|) <content:pg.string << $header

[r] |U.RT.Python.Code
(|) <env:pg.serial << $py
(|) <kwargs.file:py.str << $log_path
(|) <code:pg.string << ""
[+] +"def log(file):"
[+] -"    with open(file, 'a') as f:"
[+] -"        f.write('Hello World from Python\\n')"
[+] -""
[+] +"log(file)"

[r] |U.RT.Rust.Code
(|) <env:pg.serial << $rust
(|) <kwargs.file:rust.String << $log_path
(|) <code:pg.string << ""
[+] +"use std::fs::OpenOptions;"
[+] +"use std::io::Write;"
[+] -""
[+] +"fn main() {"
[+] -"    let mut file = OpenOptions::new()"
[+] -"        .create(true)"
[+] -"        .append(true)"
[+] -"        .open(file)"
[+] -"        .expect(\"Failed to open file\");"
[+] -""
[+] -"    writeln!(file, \"Hello World from Rust\")"
[+] -"        .expect(\"Failed to write to file\");"
[+] +"}"

[r] |U.RT.JS.Code
(|) <env:pg.serial << $js
(|) <kwargs.file:js.string << $log_path
(|) <code:pg.string << ""
[+] +"const fs = require('fs');"
[+] -""
[+] +"fs.appendFileSync(file, 'Hello World from JavaScript\\n');"

[r] $footer :pg.string << "=== Completed Successfully ===\n"

[r] |U.File.Text.Append
(|) <file:pg.path << $log_path
(|) <content:pg.string << $footer

{x}
```

---

## Step-by-Step Breakdown

### Step 1: Package Declaration

```polyglot
{@} @Local::Examples.HelloWorld:1.0.0.0
{x}
```

**What this does:**
- Declares this file as part of the `Examples.HelloWorld` package
- Version `1.0.0.0` follows Semantic Versioning
- `@Local` means local development registry
- `{x}` closes the package block

**Note:** Package block must be separate from pipeline block with 3 blank lines between!

---

### Step 2: Pipeline Definition

```polyglot
{|} |HelloWorld
[%] %Doc "Multi-language Hello World pipeline that writes to a log file"
```

**What this does:**
- `{|}` opens a pipeline definition block
- `|HelloWorld` is the pipeline name
- `[%] %Doc` adds documentation metadata

---

### Step 3: CLI Trigger

```polyglot
[t] |T.CLI"helloworld"
```

**What this does:**
- `[t]` is the trigger marker
- `|T.CLI` is the CLI trigger pipeline
- `"helloworld"` is the trigger name

**How it works:** When you run `polyglot run helloworld`, the CLI searches for pipelines with this trigger and executes `|HelloWorld`.

---

### Step 4: Input/Output Parameters

```polyglot
[|] <log_path :pg.path <~ \\FileDir\\hello_world.log
[|] >error <~ !NoError
```

**Input parameter:**
- `<log_path` - Input parameter (the `<` means input)
- `:pg.path` - Type annotation (path type)
- `<~` - Default assignment (can be overridden)
- `\\FileDir\\hello_world.log` - Path literal syntax

**Output parameter:**
- `>error` - Output parameter (the `>` means output)
- `<~` - Default value (can be updated in pipeline)
- `!NoError` - Error type (the `!` prefix)

**Override via CLI:**
```bash
polyglot run helloworld --log_path="\\Custom\\my_log.txt"
```

---

### Step 5: Initialize Runtime Wrappers

```polyglot
[w] |W.RT.Python3.9
(|) <requirements:pg.path << \\NoPath\\
(|) >env:pg.serial >> $py
```

**What this does:**
- `[w]` is the wrapper marker
- `|W.RT.Python3.9` - Python 3.9 runtime wrapper (version-specific!)
- `(|)` markers define pipeline call parameters (different from `[|]`!)
- `<requirements:pg.path` - Input parameter for Python requirements file
- `<<` - Final assignment operator (immutable)
- `\\NoPath\\` - Special "no path" value
- `>env:pg.serial` - Output parameter
- `>> $py` - Stores environment in `$py` variable for later use

**Repeat for Rust and JavaScript:**
```polyglot
[w] |W.RT.Rust
(|) <dependencies:pg.path << \\NoPath\\
(|) >env:pg.serial >> $rust

[w] |W.RT.JS
(|) <packages:pg.path << \\NoPath\\
(|) >env:pg.serial >> $js
```

**Key concept:** Runtime wrappers output a `:pg.serial` environment that contains the runtime configuration.

---

### Step 6: Create Header with Timestamp

```polyglot
[r] $timestamp :pg.dt << |DT.Now"iso8601"
[r] $header :pg.string << "=== Hello World Example - {$timestamp} ===\n"
```

**What this does:**
- `[r]` - Sequential execution marker
- `$timestamp` - Variable name (the `$` prefix)
- `:pg.dt` - DateTime type
- `<<` - Final assignment (pull from right)
- `|DT.Now` - DateTime utility pipeline
- `"iso8601"` - Format parameter
- `{$timestamp}` - Variable substitution in string

**Result:** Header with current timestamp like:
```
=== Hello World Example - 2025-12-26T16:45:23Z ===
```

---

### Step 7: Write Header to File

```polyglot
[r] |U.File.Text.Append
(|) <file:pg.path << $log_path
(|) <content:pg.string << $header
```

**What this does:**
- `|U.File.Text.Append` - Utility pipeline for appending text to files
- `(|)` - Pipeline call parameters
- `<file` - Named input parameter
- `<content` - Named input parameter
- Both use `<<` for final assignment

**Key concept:** File operations use language-agnostic utilities (not wrapper-specific).

---

### Step 8: Execute Python Code

```polyglot
[r] |U.RT.Python.Code
(|) <env:pg.serial << $py
(|) <kwargs.file:py.str << $log_path
(|) <code:pg.string << ""
[+] +"def log(file):"
[+] -"    with open(file, 'a') as f:"
[+] -"        f.write('Hello World from Python\\n')"
[+] -""
[+] +"log(file)"
```

**What this does:**
- `|U.RT.Python.Code` - Runtime code execution utility
- `<env:pg.serial << $py` - Uses Python environment from Step 5
- `<kwargs.file:py.str` - Passes `$log_path` as Python `file` variable (type `py.str`)
- `<code:pg.string` - The code to execute
- `[+]` markers build multiline code:
  - `[+] +"text"` - New line
  - `[+] -"text"` - New line (indentation is inside the string!)

**Resulting Python code:**
```python
def log(file):
    with open(file, 'a') as f:
        f.write('Hello World from Python\n')

log(file)
```

**Note:** The `file` variable comes from `<kwargs.file:py.str`!

---

### Step 9: Execute Rust and JavaScript Code

Same pattern for Rust and JavaScript:

**Rust:**
```polyglot
[r] |U.RT.Rust.Code
(|) <env:pg.serial << $rust
(|) <kwargs.file:rust.String << $log_path
(|) <code:pg.string << ""
[+] +"use std::fs::OpenOptions;"
[+] +"use std::io::Write;"
[+] -""
[+] +"fn main() { ... }"
```

**JavaScript:**
```polyglot
[r] |U.RT.JS.Code
(|) <env:pg.serial << $js
(|) <kwargs.file:js.string << $log_path
(|) <code:pg.string << ""
[+] +"const fs = require('fs');"
[+] -""
[+] +"fs.appendFileSync(file, 'Hello World from JavaScript\\n');"
```

---

### Step 10: Write Footer

```polyglot
[r] $footer :pg.string << "=== Completed Successfully ===\n"

[r] |U.File.Text.Append
(|) <file:pg.path << $log_path
(|) <content:pg.string << $footer
```

Same pattern as Step 7!

---

## Expected Output

After running `polyglot run helloworld`, the file `hello_world.log` contains:

```
=== Hello World Example - 2025-12-26T16:45:23Z ===
Hello World from Python
Hello World from Rust
Hello World from JavaScript
=== Completed Successfully ===
```

---

## Key Concepts Explained

### 1. **Assignment Operators**

| Operator | Direction | State | Use When |
|----------|-----------|-------|----------|
| `<<` | Pull ← | Final | Value won't change |
| `<~` | Pull ← | Default | Might be overridden once |
| `>>` | Push → | Final | Push immutable value |
| `~>` | Push → | Default | Push default value |

**Examples:**
```polyglot
[r] $name :pg.string << "Alice"        // Final - immutable
[|] <timeout :pg.int <~ 30             // Default - can override
[r] "Alice" >> $name                   // Same as <<
[|] 30 ~> <timeout                     // Same as <~
```

### 2. **Parameter Markers**

| Marker | Use | Example |
|--------|-----|---------|
| `[|]` | Pipeline definition I/O | `[|] <input :pg.string` |
| `(|)` | Pipeline call parameters | `(|) <param :pg.int << 5` |

### 3. **Path Literals**

```polyglot
\\FileDir\\hello.log     // Regular path
\\NoPath\\               // Special "no path" value
```

### 4. **Type Annotations**

```polyglot
:pg.string      // String type
:pg.int         // Integer type
:pg.bool        // Boolean type
:pg.path        // Path type
:pg.dt          // DateTime type
:pg.serial      // Serial data type
:pg.array.X     // Array of type X
```

### 5. **Code Building with `[+]`**

```polyglot
[+] +"def hello():"           // New line
[+] -"    print('hi')"        // New line (indentation IN string)
```

**Output:**
```python
def hello():
    print('hi')
```

---

## Common Customizations

### Change Log Location

```bash
polyglot run helloworld --log_path="\\Custom\\location.log"
```

### Use Different Python Version

```polyglot
[w] |W.RT.Python3.11        // Change version
```

### Add More Languages

```polyglot
[w] |W.RT.Ruby
(|) <gems:pg.path << \\NoPath\\
(|) >env:pg.serial >> $ruby

[r] |U.RT.Ruby.Code
(|) <env:pg.serial << $ruby
(|) <kwargs.file:ruby.String << $log_path
(|) <code:pg.string << ""
[+] +"File.open(file, 'a') { |f| f.puts 'Hello from Ruby!' }"
```

---

## Troubleshooting

### Error: "Pipeline not found"
- Check trigger name matches: `[t] |T.CLI"helloworld"`
- Run: `polyglot run helloworld` (not `hello_world`)

### Error: "Permission denied"
- Check write permissions for log file directory
- Try different path: `--log_path="\\Home\\log.txt"`

### Error: "Runtime not available"
- Ensure Python 3.9, Rust, and Node.js are installed
- Runtime wrappers must be registered

---

## Next Steps

After completing this tutorial:

1. **Learn More Syntax**
   - [Variable Lifecycle Guide](./variable-lifecycle.md)
   - [Operator Reference](./operators-reference.md)
   - [Type System](./type-system.md)

2. **Try More Examples**
   - [Parallel Execution](./parallel-hello-world.md)
   - [Error Handling](./error-handling-tutorial.md)
   - [File Processing](./file-processing.md)

3. **Build Real Pipelines**
   - [Database Operations](./database-tutorial.md)
   - [API Integration](./api-tutorial.md)
   - [Data Transformation](./data-pipeline.md)

---

**Generated by:** Polly (Polyglot Language Expert)
**For:** User Documentation
**Version:** v0.0.4
**Last Updated:** 2025-12-26
