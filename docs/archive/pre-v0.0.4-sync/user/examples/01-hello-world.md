---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/examples/01-hello-world.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Hello World Examples

**Version:** 0.0.2
**Complexity:** ★☆☆☆☆ (Basic) to ★★☆☆☆ (Elementary)

## Overview

The traditional "Hello World" program demonstrates the minimal syntax required to create a working Polyglot pipeline. This collection includes variations that progressively introduce key language features.

### What You'll Learn

- Basic pipeline structure
- Input and output declarations
- String literals and types
- Block markers and syntax
- Triggers and conditionals
- Error handling basics

---

## Example 1: Minimal Hello World

**Complexity:** ★☆☆☆☆ (Basic)

**Purpose:** The simplest possible Polyglot program. Demonstrates minimal pipeline structure.

**Key Concepts:**
- Pipeline definition with `[|]` and `[X]`
- Output declaration with `[o]`
- String type `:pg.string`
- Assignment operator `<<`

**Code:**

```polyglot
[|] HelloWorld
[i] #None
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] .message:pg.string << "Hello, World!"
[o] .message:pg.string
[X]
```

**Explanation:**

1. **`[|] HelloWorld`** - Defines a pipeline named `HelloWorld`
2. **`[o] .message:pg.string << "Hello, World!"`** - Declares an output field `.message` of type `:pg.string` and assigns the value `"Hello, World!"`
3. **`[X]`** - Closes the pipeline definition

**Output:**
```
.message = "Hello, World!"
```

**Key Takeaways:**
- ALL valid Polyglot code must start with a block element (here: `[|]`)
- Output fields are declared with `[o]`
- Type separator is backslash `\`, not forward slash
- Assignment uses `<<` (push INTO) operator

**See Also:**
- [Complete Syntax Reference](../language/01-syntax-complete.md)
- [Type System](../language/02-type-system.md)
- [Operators](../language/05-operators.md)

---

## Example 2: Hello World with Input

**Complexity:** ★☆☆☆☆ (Basic)

**Purpose:** Accept input and produce personalized output. Demonstrates input/output flow.

**Key Concepts:**
- Input declaration with `[i]`
- String interpolation (conceptual)
- Data flow from input to output
- Variable usage

**Code:**

```polyglot
[|] GreetUser
[i] .name:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] .greeting:pg.string << "Hello, {.name}!"
[o] .greeting:pg.string
[X]
```

**Explanation:**

1. **`[i] .name:pg.string`** - Declares an input field `.name` of type `:pg.string`
2. **`[o] .greeting:pg.string << "Hello, {.name}!"`** - Creates output with string interpolation

**Note:** String interpolation `{.name}` is syntax sugar that compiles to pipeline operations. See [DateTime System](../language/07-datetime-system.md) for details on how `{}` interpolation works (it's processed within `:pg.serial`).

**Usage Example:**
```
Input:  .name = "Alice"
Output: .greeting = "Hello, Alice!"
```

**Key Takeaways:**
- Input fields declared with `[i]` before they're used
- Variables referenced with dot notation (`.name`)
- String interpolation uses `{variable}` syntax
- Data flows explicitly from input to output

**See Also:**
- [Block Markers](../language/06-block-markers.md) - `[i]` and `[o]` details
- [Type System](../language/02-type-system.md) - String types

---

## Example 3: Hello World with Conditional

**Complexity:** ★★☆☆☆ (Elementary)

**Purpose:** Demonstrate conditional execution using triggers. Shows how to make pipelines respond to conditions.

**Key Concepts:**
- Trigger blocks with `[t]`
- Boolean conditions
- Conditional execution
- Sequential operations with `[r]`

**Code:**

```polyglot
[|] ConditionalGreeting
[i] .name:pg.string
[i] .should_greet:pg.bool
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[?] .should_greet ?> True
[~]
[~][r] |FormatGreeting
[~][<] .input_name:pg.string << .name
[~][>] .formatted:pg.string >> .greeting_message
[~]
[~][r] .result:pg.string << .greeting_message
[~][o] .result:pg.string

[?] .should_greet ?> False
[~]
[~][r] .result:pg.string << ""
[~][o] .result:pg.string

[X]




[|] FormatGreeting
[i] .input_name:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] .formatted:pg.string << "Hello, {.input_name}!"
[o] .formatted:pg.string
[X]
```

**Explanation:**

1. **`[t] |T.Call`** - Trigger declaration (required for all pipelines)
2. **`[?] .should_greet ?> True`** - Switch block for conditional execution
3. **`[~][r] |FormatGreeting`** - Nested operation within the True branch
4. **`[<]` and `[>]`** - Pass input INTO pipeline and pull output FROM pipeline
5. **`[?] .should_greet ?> False`** - Alternative branch when condition is false

**Usage Examples:**

```
Input:  .name = "Bob", .should_greet = true
Output: .result = "Hello, Bob!"

Input:  .name = "Bob", .should_greet = false
Output: (no output, greeting not executed)
```

**Key Takeaways:**
- `[?]` switch blocks enable conditional execution
- `[~]` marks nested operations within switch branches
- Each `[?]` branch handles a different condition value
- `<<` pushes data INTO, `>>` pulls data FROM

**See Also:**
- [Block Markers](../language/06-block-markers.md) - `[?]` and `[~]` details
- [Operators](../language/05-operators.md) - `<<` vs `>>` distinction

---

## Example 4: Hello World with Error Handling

**Complexity:** ★★☆☆☆ (Elementary)

**Purpose:** Demonstrate basic error handling. Shows how to catch and handle errors gracefully.

**Key Concepts:**
- Error types with `!Error`
- Error catching with `[!]` block AFTER operations
- Error field extraction
- Fallback behavior
- NO need to check for "no error" - default is success

**Code:**

```polyglot
[|] SafeGreeting
[i] .name:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Try to format greeting (might fail if name is empty)
[r] |ValidateAndGreet
[<] .input_name:pg.string << .name
[>] .greeting:pg.string >> .result_greeting
[~]
[~][!] !ValidationError
[~][>] .message:pg.string >> .err_msg
[~][>] .code:pg.int >> .err_code
[~]
[~]// Use fallback on error
[~][r] .result_greeting:pg.string << "Hello, Guest!"

// Use the result (either success or fallback)
[o] .result:pg.string << .result_greeting
[X]




// Validation pipeline that can fail
[|] ValidateAndGreet
[i] .input_name:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Check if name is empty
[r] |U.String.IsEmpty
[<] .value:pg.string << .input_name
[>] .is_empty:pg.bool >> .name_is_empty

// Throw error if empty, otherwise format greeting
[?] .name_is_empty ?> True
[~]
[~][r] .error: ! << !ValidationError
[~][<] .message:pg.string << "Name cannot be empty"
[~][<] .code:pg.int << 1001
[~][<] .trace:pg.string << ""
[~][o] .error: !

[?] .name_is_empty ?> False
[~]
[~][r] .greeting:pg.string << "Hello, {.input_name}!"
[~][o] .greeting:pg.string

[X]
```

**Explanation:**

1. **`[r] |ValidateAndGreet`** - Calls pipeline that might fail
2. **`[>] .greeting:pg.string >> .result_greeting`** - Pull output on success
3. **`[~]`** - Expansion marker for error handling block
4. **`[~][!] !ValidationError`** - Catch specific error type AFTER the operation
5. **`[~][>] .message:pg.string >> .err_msg`** - Extract error fields
6. **`[~][r] .result_greeting:pg.string << "Hello, Guest!"`** - Fallback value on error
7. **NO check for "no error" needed** - Default behavior is success; only catch when error occurs

**Error Raising (in ValidateAndGreet):**
- **`[~][r] .error: ! << !ValidationError`** - Create error variable of generic type `!` and assign specific error type
- **`[~][<] .message`, `.code`, `.trace`** - Assign required error fields
- **`[~][o] .error: !`** - Output the error (raises it to parent pipeline)

**Key Principle:** You don't check for "no error" - you only catch specific errors when they occur. If no error is thrown, execution continues normally.

**Usage Examples:**

```
Input:  .name = "Charlie"
Output: .result = "Hello, Charlie!"

Input:  .name = ""
Output: .result = "Hello, Guest!" (error caught and handled)
```

**Key Takeaways:**
- `!Error` types represent errors in Polyglot
- To raise an error: `[r] .error: ! << !ErrorType` then `[o] .error: !`
- Generic error type is `!` (exclamation mark alone)
- `[!]` block catches errors AFTER operations that might fail
- NO special "no error" value exists - absence of error means success
- Errors caught with `[~][!] !ErrorType` nested under operation
- Default behavior is success; only handle specific error cases
- Error types have three required fields: `.message`, `.code`, `.trace`

**See Also:**
- [Error Handling](../language/04-error-handling.md) - Complete error handling guide
- [Quick Language Reference](../audit/quick-language-reference.md#8-error-handling) - Error patterns

---

## Example 5: Hello World with Multiple Languages

**Complexity:** ★★☆☆☆ (Elementary)

**Purpose:** Demonstrate runtime wrappers for calling code in other languages.

**Key Concepts:**
- Runtime wrappers with `[W]`
- Multiple runtimes in one pipeline
- Cross-language integration
- Fixed vs dynamic version wrappers

**Code:**

```polyglot
[|] MultilingualGreeting
[i] .name:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Python wrapper: format message
[r] |PythonFormatter
[<] .name:pg.string << .name
[>] .py_greeting:pg.string >> .python_result

// Node wrapper: add emoji
[r] |NodeEmojifier
[<] .message:pg.string << .python_result
[>] .final:pg.string >> .node_result

[r] .greeting:pg.string << .node_result
[o] .greeting:pg.string
[X]




// Python formatting
[|] PythonFormatter
[i] .name:pg.string
[t] |T.Call
[W] |W.Python3.11  // Wrapper applies to entire pipeline

[r] |U.Python.Run
[<] .py:pg.string << "lambda _: f'¡Hola, {.name}!'"
[>] .out:pg.serial >> .out

// Unpack
[r] .py_greeting:pg.string << .out{0}
[o] .py_greeting:pg.string
[X]




// Node emoji addition
[|] NodeEmojifier
[i] .message:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup  // No special runtime needed

[r] .final:pg.string << "{.message} 👋"
[o] .final:pg.string
[X]
```

**Explanation:**

1. **`[W] |W.Python3.11`** - Establishes Python 3.11 runtime wrapper context
2. **`|PythonFormatter`** - Runs Python code within wrapper scope
3. **`[W] |W.Node20`** - Switches to Node.js 20 runtime wrapper
4. **`|NodeEmojifier`** - Runs Node.js code within wrapper scope
5. **Results flow through pipeline:** Input → Python → Node → Output

**Usage Example:**
```
Input:  .name = "Diana"
Output: .greeting = "¡Hola, Diana! 👋"
```

**Key Takeaways:**
- `[W]` block establishes runtime wrapper context
- Fixed version wrappers (e.g., `|W.Python3.11`) specify exact version
- Multiple runtimes can be used in same pipeline
- Each wrapper creates isolated execution context
- Supported runtimes: Python, Node, Rust, Go, Ruby, Deno

**See Also:**
- [Runtime Wrappers](../standard-library/01-runtime-wrappers.md) - Complete wrapper documentation
- [Standard Library Overview](../standard-library/00-overview.md)

---

## Example 6: Hello World with Parallel Execution

**Complexity:** ★★★☆☆ (Intermediate)

**Purpose:** Demonstrate parallel execution for concurrent greeting generation.

**Key Concepts:**
- Parallel blocks with `[p]`
- Join operations with `[Y]`
- Copy semantics
- Synchronization

**Code:**

```polyglot
[|] ParallelGreetings
[i] .names: pg.array.pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Generate greetings in parallel for each name
[p] |GreetInEnglish
[<] .name_list: pg.array.pg.string << .names
[~][r] |FormatEnglish
[<] .name:pg.string << .name_list[*]
[>] .greeting:pg.string >> english_greetings

[p] |GreetInSpanish
[<] .name_list: pg.array.pg.string << .names
[~][r] |FormatSpanish
[<] .name:pg.string << .name_list[*]
[>] .greeting:pg.string >> spanish_greetings

[p] |GreetInFrench
[<] .name_list: pg.array.pg.string << .names
[~][r] |FormatFrench
[<] .name:pg.string << .name_list[*]
[>] .greeting:pg.string >> french_greetings

// Synchronize all parallel results
[Y] |Y.JoinAll
[>] english_greetings
[>] spanish_greetings
[>] french_greetings

// Combine results
[r] |CombineGreetings
[<] .english: pg.array.pg.string << english_greetings
[<] .spanish: pg.array.pg.string << spanish_greetings
[<] .french: pg.array.pg.string << french_greetings
[>] .all:pg.string >> combined_output

[o] .greetings:pg.string << combined_output
[X]

// Helper pipelines
[|] FormatEnglish
[i] .name:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] .greeting:pg.string << "Hello, {.name}!"
[o] .greeting:pg.string
[X]




[|] FormatSpanish
[i] .name:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] .greeting:pg.string << "¡Hola, {.name}!"
[o] .greeting:pg.string
[X]




[|] FormatFrench
[i] .name:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] .greeting:pg.string << "Bonjour, {.name}!"
[o] .greeting:pg.string
[X]




[|] CombineGreetings
[i] .name:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Call all three formatter pipelines
[r] |FormatEnglish
[<] .name:pg.string << .name
[>] .greeting:pg.string >> .english_greeting

[r] |FormatSpanish
[<] .name:pg.string << .name
[>] .greeting:pg.string >> .spanish_greeting

[r] |FormatFrench
[<] .name:pg.string << .name
[>] .greeting:pg.string >> .french_greeting

// Combine all results
[r] .all:pg.string << "English: {.english_greeting}, Spanish: {.spanish_greeting}, French: {.french_greeting}"
[o] .all:pg.string
[X]
```

**Explanation:**

1. **`[p]` blocks** - Create parallel execution contexts (mini-pipelines)
2. **`[~]` expansion** - Indicates nested operations within parallel blocks
3. **`.name_list[*]`** - Array iteration (conceptual syntax)
4. **`[Y] |Y.Join`** - Synchronization point that waits for all parallel blocks
5. **`[>]` in join** - Pulls results from parallel contexts (NOT `[<]`)
6. **All three language formats execute concurrently**

**Usage Example:**
```
Input:  .names = ["Alice", "Bob"]
Output: .greetings = "English: [Hello, Alice!, Hello, Bob!], Spanish: [¡Hola, Alice!, ¡Hola, Bob!], French: [Bonjour, Alice!, Bonjour, Bob!]"
```

**Key Takeaways:**
- `[p]` blocks execute in parallel (concurrently)
- `[Y]` join blocks synchronize parallel results
- Use `[>]` (not `[<]`) within join blocks to pull results
- Copy semantics ensure thread safety
- `[~]` required for nested operations in parallel contexts
- Parallel execution improves performance for independent operations

**See Also:**
- [Parallel Execution](../language/08-parallel-execution.md) - Complete parallel programming guide
- [Join Operations](../standard-library/05-join-operations.md) - Synchronization details
- [Expansion Operator](../language/09-expansion-operator.md) - Understanding `[~]`

---

## Common Patterns Summary

### Pattern 1: Minimal Pipeline
```polyglot
[|] PipelineName
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[o] .output: type << value
[X]
```

### Pattern 2: Input to Output
```polyglot
[|] PipelineName
[i] .input: type
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[o] .output: type << .input
[X]
```

### Pattern 3: Conditional Execution
```polyglot
[?] .condition ?> True
[~][r] |HandleTrue

[?] .condition ?> False
[~][r] |HandleFalse
```

### Pattern 4: Error Handling
```polyglot
[r] |MayFail
[>] .result:pg.string >> .success_result
[~]
[~][!] !SomeError
[~][>] .message:pg.string >> .err_msg
[~]
[~][r] .success_result:pg.string << "fallback value"

// Continue with result (either success or fallback)
[r] .output:pg.string << .success_result
[o] .output:pg.string
```

### Pattern 5: Pipeline Call
```polyglot
[r] |OtherPipeline
[<] .input_field: type << value
[>] .output_field: type >> result
```

## Progression Path

After mastering these Hello World examples:

1. **Next:** [Data Processing Examples](02-data-processing.md) - Learn data transformation
2. **Then:** [Error Handling Examples](03-error-handling.md) - Advanced error patterns
3. **Finally:** [Parallel Execution Examples](04-parallel-execution.md) - Concurrent programming

## Quick Reference

| Feature | Example | Complexity |
|---------|---------|------------|
| Basic Pipeline | Example 1 | ★☆☆☆☆ |
| Input/Output | Example 2 | ★☆☆☆☆ |
| Conditionals | Example 3 | ★★☆☆☆ |
| Error Handling | Example 4 | ★★☆☆☆ |
| Runtime Wrappers | Example 5 | ★★☆☆☆ |
| Parallel Execution | Example 6 | ★★★☆☆ |

## See Also

- [Complete Syntax Reference](../language/01-syntax-complete.md) - All syntax details
- [Type System](../language/02-type-system.md) - Understanding types
- [Block Markers](../language/06-block-markers.md) - All block markers explained
- [Operators](../language/05-operators.md) - `<<`, `>>`, and other operators
- [Examples Index](00-index.md) - All examples

---

**Navigation:**
← [Examples Index](00-index.md) | [Data Processing Examples →](02-data-processing.md)