# Error Handling

[← Back to README](../README.md)

## Table of Contents
- [Error Handling Philosophy](#error-handling-philosophy)
- [Error Handler Syntax](#error-handler-syntax)
- [Standard Library Error Types](#standard-library-error-types)
- [Error Propagation Rules](#error-propagation-rules)
- [Recovery Patterns](#recovery-patterns)
- [Best Practices](#best-practices)

## Error Handling Philosophy

Polyglot treats errors as **first-class citizens** in the pipeline flow. Every operation that can fail should explicitly handle potential errors using the `[!]` error handler block.

**Key Principles:**
- Error handlers are scoped to their immediate parent element
- Unhandled errors propagate upward to parent pipelines
- Errors in forked branches are isolated
- Setup/cleanup errors have special handling rules

## Error Handler Syntax

Basic pattern:
```polyglot
[r] |RiskyOperation
[~] << input: py\dict
[~] >> output: py\dict

[~][!] !> py\!ValueError
[~][~][r] |HandleValueError

[~][!] !> py\!TypeError
[~][~][r] |HandleTypeError
```

**Syntax Breakdown:**
- `[!]` - Declares an error handler
- `!>` - Error type matching operator
- Error handlers execute **only if** the specific error type occurs
- Multiple handlers can be chained for different error types

## Standard Library Error Types

### Polyglot Core Errors (`pg\!`)

```polyglot
pg\!NetworkError          \\ Network connectivity issues
pg\!TimeoutError          \\ Operation timeout
pg\!ResourceError         \\ System resource exhaustion
pg\!OOMError              \\ Out of Memory
pg\!CpuThrottleError      \\ CPU limit reached
pg\!DiskFullError         \\ Disk space exhausted
pg\!PermissionError       \\ File/system permission denied
pg\!ConfigError           \\ Configuration error
pg\!ValidationError       \\ Input validation failed
pg\!DependencyError       \\ Missing dependency
pg\!RuntimeError          \\ Generic runtime error
pg\!NotImplementedError   \\ Feature not implemented
pg\!ConversionError       \\ Type conversion failed
```

### Python Errors (`py\!`)

All Python built-in exceptions available:
```polyglot
py\!ValueError            \\ Invalid value
py\!TypeError             \\ Type mismatch
py\!KeyError              \\ Missing dict key
py\!IndexError            \\ List/array index out of bounds
py\!AttributeError        \\ Missing attribute
py\!ImportError           \\ Module import failed
py\!IOError               \\ Input/output error
py\!OSError               \\ Operating system error
py\!ZeroDivisionError     \\ Division by zero
py\!AssertionError        \\ Assertion failed
py\!RuntimeError          \\ Generic runtime error
py\!MemoryError           \\ Out of memory
py\!FileNotFoundError     \\ File doesn't exist
py\!PermissionError       \\ Permission denied
py\!TimeoutError          \\ Operation timed out
```

### Rust Errors (`rust\!`)

Mapped from Result<T, E> and panic scenarios:
```polyglot
rust\!Panic               \\ Rust panic (unwrap on None/Err)
rust\!IOError             \\ std::io::Error
rust\!ParseError          \\ Parsing failures
rust\!SerdeError          \\ Serialization/deserialization
rust\!NoneError           \\ Option::None unwrapped
rust\!BorrowError         \\ RefCell borrow violation
rust\!LockError           \\ Mutex/RwLock poisoned
rust\!ThreadError         \\ Thread spawn/join failure
```

### JavaScript/Node Errors (`js\!`)

```polyglot
js\!Error                 \\ Generic error
js\!TypeError             \\ Type error
js\!ReferenceError        \\ Undefined reference
js\!SyntaxError           \\ Syntax error
js\!RangeError            \\ Value out of range
js\!URIError              \\ URI handling error
js\!EvalError             \\ Eval() error
js\!AggregateError        \\ Multiple errors
```

### C++ Errors (`cpp\!`)

```polyglot
cpp\!Exception            \\ std::exception
cpp\!RuntimeError         \\ std::runtime_error
cpp\!LogicError           \\ std::logic_error
cpp\!BadAlloc             \\ std::bad_alloc
cpp\!BadCast              \\ std::bad_cast
cpp\!OutOfRange           \\ std::out_of_range
cpp\!InvalidArgument      \\ std::invalid_argument
```

## Error Propagation Rules

### Rule 1: Unhandled Errors Propagate Upward

```polyglot
[|] ParentPipeline
[t] |T.Call
[r] |ChildPipeline >> result

[!] !> py\!ValueError
[~][r] |U.Log << "Child pipeline failed with ValueError"
[~][o] >> error_message: pg\string

[o] >> result
[x]

[|] ChildPipeline
[t] |T.Call
[r] |RiskyOperation
\\ No error handler - error propagates to parent
[o] >> result
[x]
```

### Rule 2: Forked Branches Handle Errors Independently

```polyglot
[|] ParallelProcessing
[t] |T.Call

[f] |Branch1
[~][r] |Operation1
[~][!] !> pg\!NetworkError
[~][~][r] |U.Log << "Branch1 network error"
[~][~][x] |Exit << 0  \\ Branch1 exits independently

[f] |Branch2
[~][r] |Operation2
\\ Branch2 continues even if Branch1 fails

[j] |JoinAll
[o] >> results
[x]
```

### Rule 3: Setup/Cleanup Errors are Critical

```polyglot
[|] ResourcePipeline
[t] |T.Call
[w] |W.PostgreSQL

[\] |U.Database.Connect << connection_string
[!] !> pg\!NetworkError
[~][r] |U.Log.Error << "Cannot connect to database"
[~][x] |Exit << 503  \\ Setup failure - pipeline cannot continue

[r] |U.Database.Query << "SELECT * FROM users"

[/] |U.Database.Disconnect
[!] !> pg\!NetworkError
[~][r] |U.Log.Warning << "Cleanup failed, connection may leak"
\\ Cleanup errors logged but don't fail the pipeline

[x]
```

## Recovery Patterns

### Pattern 1: Retry with Exponential Backoff

```polyglot
[r] |U.Network.HttpGet << url >> response

[!] !> pg\!NetworkError
[~][r] |U.Error.RetryWithBackoff
[~][~] << max_attempts: 5
[~][~] << base_delay: T"2"      \\ 2 seconds
[~][~] << max_delay: T"60"      \\ 60 seconds
[~][~] << strategy: #RetryStrategy.Exponential
[~][r] |U.Network.HttpGet << url >> response
```

### Pattern 2: Fallback to Alternative

```polyglot
[r] |PrimaryDataSource << query >> data

[!] !> pg\!TimeoutError
[~][r] |U.Log.Warning << "Primary source timed out, using cache"
[~][r] |CacheDataSource << query >> data
```

### Pattern 3: Graceful Degradation

```polyglot
[r] |EnhancedFeature << input >> output

[!] !> pg\!ResourceError
[~][r] |U.Log << "Resources low, falling back to basic feature"
[~][r] |BasicFeature << input >> output
```

### Pattern 4: Error Aggregation in Parallel

```polyglot
[|] ParallelWithErrors
[t] |T.Call

[f] |Task1
[~][r] |RiskyOp1
[~][!] !> py\!ValueError
[~][~][r] |U.Log << "Task1 error"
[~][~][o] >> error1: pg\string

[f] |Task2
[~][r] |RiskyOp2
[~][!] !> py\!TypeError
[~][~][r] |U.Log << "Task2 error"
[~][~][o] >> error2: pg\string

[j] |JoinAll

[r] |U.AggregateErrors << error1 << error2 >> error_report

[o] >> error_report
[x]
```

### Pattern 5: Circuit Breaker

```polyglot
[|] CircuitBreakerExample
[i] service_url: pg\string
[t] |T.Call

[r] |CheckCircuitState << service_url >> is_open: pg\bool

[?] is_open ?> True
[~][r] |U.Log << "Circuit is open, using fallback"
[~][r] |FallbackService >> result
[~][x] |Exit << 0

[r] |CallService << service_url >> result

[!] !> pg\!NetworkError
[~][r] |IncrementFailureCount << service_url
[~][r] |CheckIfShouldOpenCircuit << service_url >> should_open: pg\bool
[~][?] should_open ?> True
[~][~][r] |OpenCircuit << service_url
[~][r] |FallbackService >> result

[o] >> result
[x]
```

## Best Practices

### 1. Always Handle Expected Errors

```polyglot
\\ ❌ Bad: No error handling
[r] |ParseJson << json_string >> data

\\ ✅ Good: Handle expected errors
[r] |ParseJson << json_string >> data

[!] !> py\!ValueError
[~][r] |U.Log.Error << "Invalid JSON format"
[~][x] |Exit << 400
```

### 2. Be Specific with Error Types

```polyglot
\\ ❌ Bad: Catching too broad
[!] !> pg\!RuntimeError
[~][r] |HandleAllErrors

\\ ✅ Good: Handle specific errors differently
[!] !> py\!ValueError
[~][r] |HandleInvalidInput

[!] !> py\!KeyError
[~][r] |HandleMissingKey

[!] !> pg\!NetworkError
[~][r] |RetryWithBackoff
```

### 3. Log Before Exiting

```polyglot
\\ ❌ Bad: Silent failure
[!] !> pg\!ValidationError
[~][x] |Exit << 422

\\ ✅ Good: Log context
[!] !> pg\!ValidationError
[~][r] |U.Log.Error << "Validation failed: {error_details}"
[~][x] |Exit << 422
```

### 4. Clean Up Resources in Error Paths

```polyglot
\\ ✅ Good: Cleanup even on error
[r] |U.Database.Connect << connection_string >> db_conn

[r] |U.Database.Query << "SELECT * FROM users" >> results

[!] !> pg\!NetworkError
[~][r] |U.Database.Disconnect << db_conn  \\ Cleanup
[~][r] |U.Log.Error << "Database query failed"
[~][x] |Exit << 500

[r] |U.Database.Disconnect << db_conn
```

### 5. Don't Swallow Errors Silently

```polyglot
\\ ❌ Bad: Ignoring errors
[!] !> py\!ValueError
[~]\\ Do nothing

\\ ✅ Good: At minimum, log them
[!] !> py\!ValueError
[~][r] |U.Log.Warning << "Caught ValueError, continuing"
```

### 6. Provide Context in Error Messages

```polyglot
\\ ❌ Bad: Generic message
[!] !> py\!FileNotFoundError
[~][r] |U.Log.Error << "File not found"

\\ ✅ Good: Include context
[!] !> py\!FileNotFoundError
[~][r] |U.Log.Error << "File not found: {file_path}"
[~][r] |U.Log.Error << "Attempted to read at: {timestamp}"
```

## Common Error Handling Scenarios

### File Operations

```polyglot
[|] SafeFileRead
[i] file_path: pg\string
[t] |T.Call

[r] |U.System.File.Text.Read << file_path >> content

[!] !> py\!FileNotFoundError
[~][r] |U.Log.Error << "File does not exist: {file_path}"
[~][x] |Exit << 404

[!] !> py\!PermissionError
[~][r] |U.Log.Error << "Permission denied: {file_path}"
[~][x] |Exit << 403

[!] !> py\!IOError
[~][r] |U.Log.Error << "I/O error reading: {file_path}"
[~][x] |Exit << 500

[o] >> content
[x]
```

### Network Operations

```polyglot
[|] RobustHttpRequest
[i] url: pg\string
[t] |T.Call

[r] |U.Network.HttpGet << url >> response

[!] !> pg\!NetworkError
[~][r] |U.Log.Warning << "Network error, retrying..."
[~][r] |U.Error.RetryWithBackoff
[~][~] << max_attempts: 3
[~][~] << base_delay: T"5"
[~][r] |U.Network.HttpGet << url >> response

[!] !> pg\!TimeoutError
[~][r] |U.Log.Error << "Request timed out: {url}"
[~][x] |Exit << 504

[o] >> response
[x]
```

### Database Operations

```polyglot
[|] SafeDatabaseQuery
[i] query: pg\string
[t] |T.Call

[w] |W.PostgreSQL

[\] |U.Database.Connect << connection_string >> db_conn

[!] !> pg\!NetworkError
[~][r] |U.Log.Error << "Cannot connect to database"
[~][x] |Exit << 503

[r] |U.Database.Query << query >> results

[!] !> pg\!TimeoutError
[~][r] |U.Database.Disconnect << db_conn
[~][r] |U.Log.Error << "Query timed out"
[~][x] |Exit << 504

[!] !> py\!ValueError
[~][r] |U.Database.Disconnect << db_conn
[~][r] |U.Log.Error << "Invalid query syntax"
[~][x] |Exit << 400

[/] |U.Database.Disconnect << db_conn

[o] >> results
[x]
```

### Type Conversion

```polyglot
[|] SafeTypeConversion
[i] data: py\object
[t] |T.Call

[r] |U.Convert.Py.Dict.To.Rust.HashMap << data >> rust_data

[!] !> pg\!ConversionError
[~][r] |U.Log.Error << "Cannot convert Python dict to Rust HashMap"
[~][r] |U.Log.Error << "Data structure: {data_structure}"
[~][x] |Exit << 422

[o] >> rust_data
[x]
```

## Error Handling in Parallel Branches

### Independent Error Handling

```polyglot
[|] ParallelWithIndependentErrors
[t] |T.Call

[f] |TaskA
[~][r] |RiskyOpA
[~][!] !> py\!ValueError
[~][~][r] |U.Log << "TaskA failed, using default"
[~][~][o] >> result_a: pg\string << "DEFAULT_A"

[f] |TaskB
[~][r] |RiskyOpB
[~][!] !> py\!TypeError
[~][~][r] |U.Log << "TaskB failed, using default"
[~][~][o] >> result_b: pg\string << "DEFAULT_B"

[j] |JoinAll
\\ Both branches return results even if they errored

[o] >> result_a
[o] >> result_b
[x]
```

### Fail-Fast Pattern

```polyglot
[|] ParallelFailFast
[i] critical_flag: pg\bool
[t] |T.Call

[f] |CriticalTask
[~][r] |ImportantOperation
[~][!] !> pg\!RuntimeError
[~][~][r] |U.Log.Critical << "Critical task failed!"
[~][~][x] |Exit << 1  \\ Exit entire pipeline

[f] |NonCriticalTask
[~][r] |OptionalOperation
[~][!] !> pg\!RuntimeError
[~][~][r] |U.Log.Warning << "Non-critical task failed"
[~][~]\\ Continue execution

[j] |JoinAll
[x]
```

## Testing Error Handlers

Error handlers should be tested to ensure they work correctly:

```polyglot
[|] TestErrorHandling
[t] |T.Call

\\ Simulate error condition
[r] |SimulateNetworkError

[!] !> pg\!NetworkError
[~][r] |U.Log << "Error handler triggered correctly"
[~][r] |U.Assert << True  \\ Test passes
[~][o] >> test_result: pg\bool << True

[r] |U.Assert << False  \\ Should not reach here
[o] >> test_result: pg\bool << False
[x]
```

## Standard Library Error Utilities

```polyglot
\\ Retry mechanisms
|U.Error.Retry << operation << max_attempts: pg\int
|U.Error.RetryWithBackoff << operation << max_attempts << base_delay << max_delay << strategy

\\ Graceful shutdown
|U.Error.GracefulShutdown << cleanup_tasks: pg\list

\\ Error aggregation
|U.Error.Aggregate << errors: pg\list >> summary: pg\dict

\\ Error logging
|U.Error.Log << error: pg\!Any << level: #LogLevel << context: pg\dict
```

---

[Next: Flow Control →](07-flow-control.md)