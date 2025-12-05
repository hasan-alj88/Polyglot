# Hello World Examples

**Difficulty:** Beginner
**Languages:** Python, Rust
**Topics:** Basic syntax, Multi-language execution, Runtime wrappers
**Time:** ~10 minutes

---

## Overview

Your first Polyglot pipeline! These examples demonstrate the basic structure of a Polyglot program with multiple language runtimes.

**Key Concept:** Polyglot lets you run Python and Rust code in the same pipeline, with proper runtime wrappers for each language.

---

## Example 1: Simple Multi-Language Hello World

The simplest possible multi-language pipeline - Python and Rust both say hello.

### Complete Code

**File:** `hello.pg`

```polyglot
[@] Local@MyFirstApp.HelloWorld:1.0.0
[#] 1
[X]





// Pipeline: Multi-language Hello World
[|] SayHello

// Trigger: Command-line activation
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11
[W] |W.Rust1.70

// Python says hello
[r] |Run.Python
[<] .code: py\str << "print('Hello from Python!')"

// Rust says hello
[r] |Run.Rust
[<] .code: rs\String << "println!(\"Hello from Rust!\");"

// No outputs
[o] #None
[X]
```

### Explanation

**Package Declaration (Lines 1-3):**
```polyglot
[@] Local@MyFirstApp.HelloWorld:1.0.0
[#] 1
[X]
```
- Every `.pg` file must start with a package block
- `[@]` declares the package namespace and version
- `[#] 1` indicates this is file #1 (for multi-file packages)
- `[X]` terminates the package block

**Pipeline Definition (Line 9):**
```polyglot
[|] SayHello
```
- `[|]` marks the start of a pipeline
- `SayHello` is the pipeline name

**Trigger (Line 12):**
```polyglot
[t] |T.Cli
```
- `|T.Cli` means this pipeline is triggered manually via command line
- Without a trigger, the pipeline won't execute

**Runtime Wrappers (Lines 15-16):**
```polyglot
[W] |W.Python3.11
[W] |W.Rust1.70
```
- **Important:** These set up the language runtime environments
- `|W.Python3.11` prepares Python 3.11 with `uv`
- `|W.Rust1.70` prepares Rust 1.70 compiler
- Wrappers are required before running code in that language

**Python Run Block (Lines 19-20):**
```polyglot
[r] |Run.Python
[<] .code: py\str << "print('Hello from Python!')"
```
- `[r]` marks a run block (execute code)
- `|Run.Python` executes Python code
- `.code` input receives a Python string (`py\str`)
- `<<` is the assignment operator

**Rust Run Block (Lines 23-24):**
```polyglot
[r] |Run.Rust
[<] .code: rs\String << "println!(\"Hello from Rust!\");"
```
- Similar structure to Python block
- `.code` receives a Rust String (`rs\String`)
- Note the escaped quotes: `\"`

**Output (Line 27):**
```polyglot
[o] #None
```
- `[o]` marks the output section
- `#None` means this pipeline produces no outputs
- It just prints to console

### Running the Example

Once Polyglot CLI is available:

```bash
# Compile the .pg file
polyglot compile hello.pg

# Register with the daemon
polyglot register Local@MyFirstApp.HelloWorld:1.0.0

# Activate triggers
polyglot activate Local@MyFirstApp.HelloWorld:1.0.0

# Run the pipeline
polyglot run SayHello
```

### Expected Output

```
Hello from Python!
Hello from Rust!
```

---

## Example 2: Hello World with Name Input

A slightly more advanced version that accepts a name parameter.

### Complete Code

**File:** `hello_name.pg`

```polyglot
[@] Local@MyFirstApp.HelloWithName:1.0.0
[#] 1
[X]





// Pipeline: Personalized multi-language greeting
[|] GreetByName

// Input: Name to greet
[i] .name:pg.string

// Trigger: Command-line with parameters
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11
[W] |W.Rust1.70

// Python greeting
[r] |Run.Python
[<] .code: py\str << f"print(f'Hello from Python, {.name}!')"

// Rust greeting
[r] |Run.Rust
[<] .code: rs\String << f"println!(\"Hello from Rust, {.name}!\");"

// No outputs
[o] #None
[X]
```

### Explanation

**Input Declaration:**
```polyglot
[i] .name:pg.string
```
- `[i]` declares an input parameter
- `.name` is the variable (dot prefix required)
- `:pg.string` is a Polyglot native string type
- This value will be passed when triggering the pipeline

**Using Input in Code:**
```polyglot
[<] .code: py\str << f"print(f'Hello from Python, {.name}!')"
```
- The `{.name}` syntax interpolates the input value
- Works in both Python and Rust code strings

### Running with Input

```bash
polyglot run GreetByName --name "Alice"
```

### Expected Output

```
Hello from Python, Alice!
Hello from Rust, Alice!
```

---

## Example 3: Hello World with Output

Return a greeting string instead of printing.

### Complete Code

**File:** `hello_output.pg`

```polyglot
[@] Local@MyFirstApp.HelloWithOutput:1.0.0
[#] 1
[X]





// Pipeline: Multi-language greeting with output
[|] GenerateGreeting

// Input: Name to greet
[i] .name:pg.string << "World"

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11
[W] |W.Rust1.70

// Python generates greeting
[r] |Run.Python
[<] .code: py\str << f"result = f'Hello from Python, {.name}!'"
[>] .greeting: py\str >> .python_greeting

// Rust generates greeting
[r] |Run.Rust
[<] .code: rs\String << f"let result = format!(\"Hello from Rust, {}!\", \"{.name}\");"
[>] .greeting: rs\String >> .rust_greeting

// Combine and output
[r] |Run.Python
[<] .code: py\str << f"result = '{.python_greeting}\\n{.rust_greeting}'"
[>] .result: py\str >> .final_greeting

// Output the combined greeting
[o] .final_greeting:pg.string
[X]
```

### Explanation

**Input with Default:**
```polyglot
[i] .name:pg.string << "World"
```
- `<< "World"` provides a default value
- If no input is provided, "World" is used

**Output Assignment:**
```polyglot
[>] .greeting: py\str >> .python_greeting
```
- `[>]` marks an output assignment
- `>>` is the output operator
- `.python_greeting` stores the result for later use

**Final Output:**
```polyglot
[o] .final_greeting:pg.string
```
- Returns the final greeting string
- Can be captured or displayed

### Running

```bash
# With default name
polyglot run GenerateGreeting

# With custom name
polyglot run GenerateGreeting --name "Alice"
```

### Expected Output

```
Hello from Python, World!
Hello from Rust, World!
```

Or with custom name:

```
Hello from Python, Alice!
Hello from Rust, Alice!
```

---

## Key Takeaways

1. **Package Block Required:** Every `.pg` file starts with `[@]`, `[#]`, `[X]`
2. **Runtime Wrappers Are Important:** Use `[W] |W.Python3.11` and `[W] |W.Rust1.70` before running code
3. **Variables Need Dots:** All variables start with `.` (e.g., `.name`, `.greeting`)
4. **Type Format:** Use backslash separator: `py\str`, `rs\String`, `:pg.string`
5. **Triggers Required:** Use `[t] |T.Cli` for manual execution
6. **Assignment Operators:**
   - `<<` for input assignment
   - `>>` for output assignment

---

## Common Mistakes

❌ **Missing package block:**
```polyglot
// WRONG - Missing [@], [#], [X]
[|] SayHello
```

✅ **Correct:**
```polyglot
[@] Local@MyApp:1.0.0
[#] 1
[X]



[|] SayHello
```

---

❌ **Missing runtime wrappers:**
```polyglot
// WRONG - No [W] blocks
[r] |Run.Python
[<] .code: py\str << "print('Hello')"
```

✅ **Correct:**
```polyglot
[W] |W.Python3.11

[r] |Run.Python
[<] .code: py\str << "print('Hello')"
```

---

❌ **Variable without dot prefix:**
```polyglot
// WRONG
[i] name:pg.string
```

✅ **Correct:**
```polyglot
[i] .name:pg.string
```

---

❌ **Wrong type separator:**
```polyglot
// WRONG - Using forward slash
[i] .name: pg/string
```

✅ **Correct:**
```polyglot
[i] .name:pg.string
```

---

## Next Steps

Now that you've mastered Hello World, try:

1. **Add More Languages** - Try JavaScript or C++ wrappers
2. **File Operations** - [File Operations Examples](file-operations.md)
3. **Data Processing** - [Data Processing Examples](data-processing.md)
4. **Error Handling** - [Error Handling Examples](error-handling.md)

---

## See Also

- [Quick Start Guide](../language/00-quick-start.md) - Comprehensive beginner guide
- [Complete Syntax Reference](../language/01-syntax-complete.md) - All syntax rules
- [Runtime Wrappers Catalog](../standard-library/01-runtime-wrappers.md) - All available wrappers
- [Examples Index](README.md) - All examples organized by topic

---

**Last Updated:** 2025-11-15