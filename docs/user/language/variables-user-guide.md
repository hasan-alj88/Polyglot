# Variables in Polyglot: User Guide

**Last Updated:** 2025-11-24
**Audience:** Polyglot developers
**Level:** Beginner to Intermediate

---

## Table of Contents

1. [Introduction](#introduction)
2. [The Async-Centric Difference](#the-async-centric-difference)
3. [Variable Assignment Syntax](#variable-assignment-syntax)
4. [When Are Variables Ready?](#when-are-variables-ready)
5. [Error Handling](#error-handling)
6. [Practical Examples](#practical-examples)
7. [Common Patterns](#common-patterns)
8. [FAQ](#faq)

---

## Introduction

In Polyglot, variables work differently than traditional programming languages because **Polyglot is async-centric**. This guide shows you the practical syntax and patterns you need to write effective Polyglot code.

### What You Need to Know

Three core concepts:

1. **How to assign variables** (three operators)
2. **When variables are ready to use** (at `[i]` blocks)
3. **How to handle errors** (`.errors` field)

That's it. The runtime handles all the async coordination for you.

---

## The Async-Centric Difference

**Traditional languages:** Variables have values immediately.
```javascript
const name = "Alice";  // Available now
```

**Polyglot:** Variables transition to ready state asynchronously.
```polyglot
[r] |FetchUser
[>] .name:pg.string >> .user_name  # Becomes ready when pipeline completes
```

**The Magic:** Polyglot **automatically waits** for variables to be ready. You never write `await`.

---

## Variable Assignment Syntax

Polyglot has **three ways** to assign variables:

### 1. Schema-Only Declaration

**Use when:** Data comes from a pipeline later

```polyglot
[#] UserProfile
[<] .name:pg.string          # No default, will be populated
[<] .email:pg.string         # No default, will be populated
[<] .age:pg.int              # No default, will be populated
[X]
```

**What happens:** Fields have no value until explicitly populated by a pipeline.

---

### 2. Default Assignment `<~` or `~>`

**Use when:** You want sensible defaults that can be overridden

```polyglot
[#] ServerConfig
[<] .timeout:pg.int <~ 30           # Default: 30 seconds
[<] .max_retries:pg.int <~ 3        # Default: 3 retries
[<] .port:pg.int <~ 8080            # Default: port 8080
[X]
```

**Bidirectional operators:**
```polyglot
[<] .field: Type <~ value    # Default from left
[>] .field: Type ~> .var     # Default to right
```

**What happens:**
- At `[i]` blocks, defaults kick in if not overridden
- You can override once during instantiation
- After first use, becomes immutable

**When to use:**
- Configuration values
- Fallback defaults
- Optional parameters

---

### 3. Constant Assignment `<<` or `>>`

**Use when:** Value never changes

```polyglot
[#] AppMetadata
[<] .version:pg.string << "1.0.0"    # Always "1.0.0"
[<] .name:pg.string << "MyApp"       # Always "MyApp"
[<] .build:pg.string << "2025-11-24" # Always this date
[X]
```

**Bidirectional operators:**
```polyglot
[<] .field: Type << value    # Constant from left
[>] .field: Type >> .var     # Async assignment to right
```

**What happens:** Value is immutable and always ready.

**When to use:**
- Version numbers
- Application names
- API keys (from environment)
- Fixed configuration

---

## When Are Variables Ready?

### Simple Rule: Variables Are Ready at `[i]` Blocks

```polyglot
[|] ProcessUser
[i] .user_name:pg.string
[i] .user_age:pg.int
[t] |T.Call

[W] |W.Python3.11

# All [i] variables are READY here
[r] |greet_user
[<] .name:pg.string << .user_name    # ✅ Ready to use
[<] .age:pg.int << .user_age          # ✅ Ready to use

[o] #None
[X]
```

**Key insight:** Polyglot pipelines won't trigger until all `[i]` variables are ready.

---

### Automatic Waiting

When you pass a variable to a pipeline, **Polyglot waits automatically** if needed:

```polyglot
[r] |FetchUser
[<] .id:pg.string << "user123"
[>] .profile: #UserProfile >> .user_data

# Polyglot automatically waits for .user_data to be ready
[r] |ProcessProfile
[<] .data: #UserProfile << .user_data   # No await needed!
[>] .result:pg.string >> .processed
```

**You never write `await`.** The runtime handles it.

---

## Error Handling

### Checking for Errors

Every variable has a `.errors` field:

```polyglot
[r] |FetchUserFromAPI
[<] .user_id:pg.string << "user123"
[>] .user: #UserProfile >> .user_data
[>] .errors:pg.array{!} >> .fetch_errors

# Check if operation failed
[?] .user_data.state =? #Variables.States.Faulted
[~][r] |U.Log.Error
[~][<] .msg:pg.string << "Failed to fetch user: {.fetch_errors}"
[~][o] #None

# Success path
[?] .user_data.state =? #Variables.States.Ready
[~][r] |ProcessUser
[~][<] .user: #UserProfile << .user_data
[~][o] .processed_result:pg.string
```

---

### Error Handling Patterns

#### Pattern 1: Catch Specific Errors

```polyglot
[r] |CallExternalAPI
[<] .endpoint:pg.string << "https://api.example.com/data"
[>] .response:pg.string >> .api_response
[>] .errors:pg.array{!} >> .api_errors
[~]
[~][!] !pg.Network.Timeout
[~][>] .message:pg.string >> .timeout_msg
[~][~][r] |U.Log.Warn
[~][~][<] .msg:pg.string << "API timeout, using cached data"
[~][~]
[~][~][r] |GetCachedData
[~][~][>] .cached:pg.string >> .fallback_data
[~][~][o] .fallback_data:pg.string
[~]
[~][!] !pg.Network.ConnectionFailed
[~][>] .message:pg.string >> .connection_msg
[~][~][r] |U.Log.Error
[~][~][<] .msg:pg.string << "Connection failed: {.connection_msg}"
[~][~][o] #None

# Success path
[o] .api_response:pg.string
```

#### Pattern 2: Check State Explicitly

```polyglot
[r] |RiskyOperation
[>] .result:pg.string >> .operation_result
[>] .errors:pg.array{!} >> .operation_errors

# Explicit state checking
[?] .operation_result.state =? #Variables.States.Ready
[~][r] |ProcessSuccess
[~][<] .data:pg.string << .operation_result

[?] .operation_result.state =? #Variables.States.Faulted
[~][r] |ProcessFailure
[~][<] .errors:pg.array{!} << .operation_errors

[?] *?
[~][r] |U.Log.Warn
[~][<] .msg:pg.string << "Unexpected state"

[o] #None
```

---

## Practical Examples

### Example 1: Configuration with Defaults

```polyglot
[#] DatabaseConfig
[<] .host:pg.string <~ "localhost"
[<] .port:pg.int <~ 5432
[<] .database:pg.string <~ "myapp"
[<] .timeout:pg.int <~ 30
[<] .max_connections:pg.int <~ 10
[X]

[|] ConnectToDatabase
[i] #None
[t] |T.Call

[W] |W.Python3.11

# Config with defaults is ready
[i] .config: #DatabaseConfig << #DatabaseConfig

[r] |establish_connection
[<] .host:pg.string << .config.host
[<] .port:pg.int << .config.port
[<] .db:pg.string << .config.database
[>] .connection:pg.string >> .db_conn

[o] .db_conn:pg.string
[X]
```

---

### Example 2: API Call with Error Handling

```polyglot
[|] FetchWeatherData
[i] .city:pg.string
[t] |T.Call

[W] |W.Python3.11

[r] |call_weather_api
[<] .location:pg.string << .city
[>] .weather:pg.string >> .weather_data
[>] .errors:pg.array{!} >> .api_errors
[~]
[~][!] !pg.Network.Timeout
[~][~][r] |U.Log.Warn
[~][~][<] .msg:pg.string << "API timeout for {.city}"
[~][~]
[~][~]# Use cached weather data
[~][~][r] |get_cached_weather
[~][~][<] .location:pg.string << .city
[~][~][>] .cached:pg.string >> .cached_weather
[~][~]
[~][~][o] .cached_weather:pg.string

# Success path
[r] |parse_weather_data
[<] .raw:pg.string << .weather_data
[>] .parsed:pg.string >> .result

[o] .result:pg.string
[X]
```

---

### Example 3: Multiple Parallel Operations

```polyglot
[|] FetchDashboardData
[i] #None
[t] |T.Call

[W] |W.Python3.11

# Launch parallel operations
[p] |fetch_user_stats
[>] .stats:pg.string >> .user_stats

[p] |fetch_notifications
[>] .notifications: pg.array.pg.string >> .user_notifications

[p] |fetch_recent_activity
[>] .activity: pg.array.pg.string >> .recent_activity

# Wait for all to complete
[Y] |Y.Join
[<] .user_stats
[<] .user_notifications
[<] .recent_activity

# Check if all succeeded
[?] .user_stats.state =? #Variables.States.Ready
[~][?] .user_notifications.state =? #Variables.States.Ready
[~][~][?] .recent_activity.state =? #Variables.States.Ready
[~][~][~]# All ready - build full dashboard
[~][~][~][r] |build_dashboard
[~][~][~][<] .stats:pg.string << .user_stats
[~][~][~][<] .notifications: pg.array.pg.string << .user_notifications
[~][~][~][<] .activity: pg.array.pg.string << .recent_activity
[~][~][~][>] .dashboard:pg.string >> .full_dashboard
[~][~][~]
[~][~][~][o] .full_dashboard:pg.string

# Partial failure handling
[?] *?
[~][r] |build_partial_dashboard
[~][<] .stats:pg.string << .user_stats
[~][o] #None

[o] #None
[X]
```

---

## Common Patterns

### Pattern: Configuration Objects

**Problem:** Need config with sensible defaults

**Solution:** Use default assignment `<~`

```polyglot
[#] AppConfig
[<] .log_level:pg.string <~ "INFO"
[<] .debug_mode: #Boolean <~ #False
[<] .api_timeout:pg.int <~ 30
[<] .max_retries:pg.int <~ 3
[X]
```

---

### Pattern: Optional Parameters

**Problem:** Some inputs are optional

**Solution:** Schema-only for required, defaults for optional

```polyglot
[#] SearchParams
[<] .query:pg.string                    # Required
[<] .page:pg.int <~ 1                   # Optional, default 1
[<] .page_size:pg.int <~ 20             # Optional, default 20
[<] .sort_by:pg.string <~ "relevance"   # Optional, default "relevance"
[X]
```

---

### Pattern: Fallback on Error

**Problem:** Need fallback when API fails

**Solution:** Catch error, provide alternative

```polyglot
[r] |FetchLiveData
[>] .data:pg.string >> .live_data
[>] .errors:pg.array{!} >> .fetch_errors
[~]
[~][!] !pg.Network.*
[~][~][r] |GetCachedData
[~][~][>] .cached:pg.string >> .fallback_data
[~][~][o] .fallback_data:pg.string

[o] .live_data:pg.string
```

---

### Pattern: Conditional Processing

**Problem:** Different logic based on state

**Solution:** Use state checking with `[?]`

```polyglot
[r] |ProcessData
[>] .result:pg.string >> .processed_data

[?] .processed_data.state =? #Variables.States.Ready
[~][r] |SaveToDatabase
[~][<] .data:pg.string << .processed_data

[?] .processed_data.state =? #Variables.States.Faulted
[~][r] |LogError
[~][<] .errors:pg.array{!} << .processed_data.errors
```

---

## FAQ

### Q: Do I need to worry about variable states?

**A:** No. The compiler handles states. You just need to know:
- Variables are ready at `[i]` blocks
- How to handle errors (check `.errors` field)
- The three assignment operators

---

### Q: What's the difference between `<~` and `<<`?

**A:**
- `<~` = Default (can override once)
- `<<` = Constant (never changes)

Use `<~` for config values you might override. Use `<<` for things like version numbers.

---

### Q: When do I need to check `.state`?

**A:** Only for advanced error handling. Most of the time, use error blocks `[!]` instead.

```polyglot
# Simple: Use error blocks
[r] |DoSomething
[>] .result:pg.string >> .data
[~][!] !SomeError
[~][~][r] |HandleError

# Advanced: Check state explicitly
[?] .data.state =? #Variables.States.Faulted
[~][r] |ComplexErrorHandling
```

---

### Q: How do I know if a pipeline failed?

**A:** Check the output variable's state or use error blocks:

```polyglot
# Method 1: Error blocks (recommended)
[r] |RiskyOperation
[>] .result:pg.string >> .output
[~][!] !pg.Network.Timeout
[~][~][r] |HandleTimeout

# Method 2: State checking
[?] .output.state =? #Variables.States.Faulted
[~][r] |HandleFailure
```

---

### Q: Can I reassign a variable?

**A:** No. Once a variable is ready, it's immutable. But you can assign to a NEW variable:

```polyglot
[r] |GetData
[>] .result:pg.string >> .original_data

# Can't reassign .original_data
# But can create new variable
[r] |TransformData
[<] .input:pg.string << .original_data
[>] .result:pg.string >> .transformed_data  # New variable
```

---

### Q: What about nested fields?

**A:** Nested fields work the same way:

```polyglot
[#] Address
[<] .street:pg.string <~ ""
[<] .city:pg.string <~ ""
[X]

[#] User
[<] .name:pg.string
[<] .address: #Address << #Address
[X]

# Access nested fields
[i] .user: #User << #User
[r] |LogCity
[<] .city:pg.string << .user.address.city
```

---

### Q: Do I ever write `await`?

**A:** No. Polyglot is async-centric - the runtime waits automatically. That's the magic.

---

## Summary

**Three things to remember:**

1. **Assignment Operators:**
   - Schema-only (no operator) = No default, populate later
   - Default `<~` or `~>` = Sensible default, can override once
   - Constant `<<` or `>>` = Never changes

2. **Variables are ready at `[i]` blocks** - Polyglot won't trigger until ready

3. **Error handling:**
   - Use error blocks `[!]` for specific errors
   - Check `.state` for advanced patterns
   - Access `.errors` field for error details

**That's it.** The runtime handles all async coordination. Write your logic, and Polyglot makes it work.

---

## Next Steps

- Read: [Enumerations User Guide](./enumerations-user-guide.md)
- Read: [Pipelines User Guide](./pipelines-user-guide.md)
- Read: [Error Handling Deep Dive](./error-handling.md)
- Try: [Interactive Tutorial](../tutorials/variables-tutorial.md)

---

**Need more details?** See [Variable States Technical Specification](../../technical/variable-states-specification.md) (for implementers)
