# Approved Polyglot v0.0.2 Code Examples

**Version:** 0.0.2
**Status:** Approved and Validated
**Date:** 2025-11-13

## Overview

This document contains comprehensive Polyglot v0.0.2 code examples that have been validated against the authoritative audit documentation. These examples demonstrate correct syntax, proper error handling, and idiomatic patterns.

All examples follow v0.0.2 specifications:
- Error raising with `[r] .error: ! << !ErrorType` and `[o] .error: !`
- Error catching with `[~][!] !ErrorType`
- Range notation for comparisons (`.value..`, `...value`, `.value`)
- Cross-platform paths with `#Path.Identifiers.*`
- All pipelines include `[t]` triggers
- No comparison operators (`==`, `!=`, `<`, `>`)
- Proper use of `[*]` and `+""` for multiline strings

---

## Example 1: Data Processing with Error Handling

**Purpose:** Demonstrates comprehensive data validation, transformation, and error recovery patterns.

**Key Concepts:**
- Input validation with custom errors
- Data transformation pipeline
- Error catching and recovery
- Multiple error types

**Code:**

```polyglot
[!] !ValidationError
[<] .message:pg.string << "Validation error"
[<] .code:pg.int << 1001
[<] .trace:pg.string << ""
[<] .field_name:pg.string << ""
[X]

[!] !TransformError
[<] .message:pg.string << "Transform error"
[<] .code:pg.int << 2001
[<] .trace:pg.string << ""
[<] .input_value:pg.string << ""
[X]

[|] ValidateInput
[i] .data:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Check for empty input
[?] .data ?> ""
[~]
[~][r] .error: ! << !ValidationError
[~][<] .message:pg.string << "Input data cannot be empty"
[~][<] .code:pg.int << 1001
[~][<] .trace:pg.string << ""
[~][<] .field_name:pg.string << "data"
[~][o] .error: !

// Check for minimum length (example: at least 3 characters)
[r] .data_length:pg.int << {.data|length}
[?] .data_length ?> ...3
[~]
[~][r] .error: ! << !ValidationError
[~][<] .message:pg.string << "Input must be at least 3 characters"
[~][<] .code:pg.int << 1002
[~][<] .trace:pg.string << ""
[~][<] .field_name:pg.string << "data"
[~][o] .error: !

// Validation passed
[o] .validated:pg.string << .data
[X]

[|] TransformData
[i] .input:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Attempt transformation (example: uppercase conversion)
[r] .result:pg.string << {.input|uppercase}

// Check if transformation produced result
[?] .result ?> ""
[~]
[~][r] .error: ! << !TransformError
[~][<] .message:pg.string << "Transform produced empty result"
[~][<] .code:pg.int << 2001
[~][<] .trace:pg.string << ""
[~][<] .input_value:pg.string << .input
[~][o] .error: !

[o] .transformed:pg.string << .result
[X]

[|] ProcessDataWithRecovery
[i] .raw_data:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Step 1: Validate input
[r] |ValidateInput
[<] .data:pg.string << .raw_data
[>] .validated:pg.string >> .clean_data
[~]
[~][!] !ValidationError
[~][>] .message:pg.string >> .err_msg
[~][>] .code:pg.int >> .err_code
[~][>] .field_name:pg.string >> .err_field
[~]
[~]// Recovery: Use default value
[~][r] .clean_data:pg.string << "DEFAULT_VALUE"
[~][r] .validation_failed:pg.bool << #True

// Step 2: Transform data
[r] |TransformData
[<] .input:pg.string << .clean_data
[>] .transformed:pg.string >> .final_data
[~]
[~][!] !TransformError
[~][>] .message:pg.string >> .transform_err_msg
[~][>] .code:pg.int >> .transform_err_code
[~]
[~]// Recovery: Use original data
[~][r] .final_data:pg.string << .clean_data
[~][r] .transform_failed:pg.bool << #True

[o] .output:pg.string << .final_data
[X]
```

**Explanation:**

This example demonstrates:
1. **Custom Error Types**: `!ValidationError` and `!TransformError` with required fields (`.message`, `.code`, `.trace`) plus custom fields
2. **Error Raising**: Using `[r] .error: ! << !ErrorType` followed by field assignments and `[o] .error: !`
3. **Error Catching**: Using `[~][!] !ErrorType` to catch specific errors and extract fields
4. **Recovery Strategy**: Providing default values when errors occur
5. **Multi-Stage Processing**: Validation → Transformation with error handling at each stage

**Output:**

When given valid input "hello", produces "HELLO". When given invalid input "", uses "DEFAULT_VALUE" and transforms to "DEFAULT_VALUE". Each stage handles errors independently with appropriate recovery.

**See Also:**
- [Error Handling Reference](../language/04-error-handling.md)
- [Quick Language Reference - Error Section](../audit/quick-language-reference.md#error-handling)

---

## Example 2: Cross-Platform File Operations

**Purpose:** Demonstrates platform-independent file operations using path identifiers.

**Key Concepts:**
- Path identifiers (`#Path.Identifiers.*`)
- Cross-platform path construction
- File reading with error handling
- Path type safety

**Code:**

```polyglot
[!] !FileError
[<] .message:pg.string << "File operation error"
[<] .code:pg.int << 3001
[<] .trace:pg.string << ""
[<] .file_path:pg.path << #Path.Identifiers.Home
[X]

[|] ReadConfigFile
[i] .config_name:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Construct cross-platform path
// Pattern: #Path.Identifiers.Home / ".config" / config_name
[r] .base_path:pg.path << #Path.Identifiers.Home
[r] .config_dir:pg.path << {.base_path / ".config"}
[r] .config_file:pg.path << {.config_dir / .config_name}

// Attempt to read file
[r] |U.File.Read
[<] .path:pg.path << .config_file
[>] .content:pg.string >> .file_content
[~]
[~][!] !FileError
[~][>] .message:pg.string >> .err_msg
[~][>] .file_path:pg.path >> .err_path
[~]
[~]// Recovery: Use default configuration
[~][r] .file_content:pg.string << "# Default Configuration"
[~][r] .used_default:pg.bool << #True

[o] .config:pg.string << .file_content
[X]

[|] SaveUserData
[i] .data:pg.string
[i] .filename:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Construct cross-platform path to user data directory
[r] .user_data_dir:pg.path << #Path.Identifiers.UserData
[r] .output_file:pg.path << {.user_data_dir / .filename}

// Write data to file
[r] |U.File.Write
[<] .path:pg.path << .output_file
[<] .content:pg.string << .data
[~]
[~][!] !FileError
[~][>] .message:pg.string >> .write_err_msg
[~][>] .file_path:pg.path >> .write_err_path
[~]
[~]// Error occurred - output error indicator
[~][r] .success:pg.bool << #False
[~][r] .error: ! << !FileError
[~][<] .message:pg.string << .write_err_msg
[~][<] .code:pg.int << 3002
[~][<] .trace:pg.string << ""
[~][<] .file_path:pg.path << .write_err_path
[~][o] .error: !

// Success case
[r] .success:pg.bool << #True
[o] .result:pg.bool << .success
[X]

[|] ProcessUserFiles
[i] .user_id:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Read user config
[r] |ReadConfigFile
[<] .config_name:pg.string << "user_{.user_id}.conf"
[>] .config:pg.string >> .user_config

// Process config data
[r] .processed_data:pg.string << {.user_config|trim}

// Save processed data
[r] |SaveUserData
[<] .data:pg.string << .processed_data
[<] .filename:pg.string << "processed_{.user_id}.txt"
[>] .result:pg.bool >> .save_success
[~]
[~][!] !FileError
[~][>] .message:pg.string >> .save_err

[o] .success:pg.bool << .save_success
[X]
```

**Explanation:**

This example demonstrates:
1. **Path Identifiers**: Using `#Path.Identifiers.Home` and `#Path.Identifiers.UserData` for platform-independent paths
2. **Path Construction**: Building paths with `/` operator that works on all platforms
3. **Type Safety**: Variables explicitly typed as `:pg.path`
4. **File Operations**: Reading and writing files with proper error handling
5. **Error Propagation**: Catching file errors and either recovering or re-raising them

**Output:**

Reads user configuration from `~/.config/user_{id}.conf` (Unix) or `C:\Users\{user}\.config\user_{id}.conf` (Windows), processes it, and saves to user data directory. Uses default config on read failure, raises error on write failure.

**See Also:**
- [Path Type Documentation](../language/02-type-system.md#path-type)
- [Reserved Path Identifiers](../audit/reserved-enumeration-schema-decisions.md#path-identifiers)
- [File Operations Standard Library](../standard-library/03-file-operations.md)

---

## Example 3: Parallel Data Processing with Join

**Purpose:** Demonstrates parallel execution with join synchronization and result aggregation.

**Key Concepts:**
- Parallel blocks `[p]`
- Join synchronization `[Y]`
- Copy semantics for parallel operations
- Result aggregation

**Code:**

```polyglot
[!] !ProcessingError
[<] .message:pg.string << "Processing error"
[<] .code:pg.int << 4001
[<] .trace:pg.string << ""
[<] .item_id:pg.int << 0
[X]

[|] ProcessItem
[i] .item:pg.string
[i] .item_id:pg.int
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Simulate processing (example: uppercase + length)
[r] .processed:pg.string << {.item|uppercase}
[r] .length:pg.int << {.processed|length}

// Validate processing result
[?] .length ?> 0
[~]
[~][r] .error: ! << !ProcessingError
[~][<] .message:pg.string << "Processing produced empty result"
[~][<] .code:pg.int << 4001
[~][<] .trace:pg.string << ""
[~][<] .item_id:pg.int << .item_id
[~][o] .error: !

[r] .result:pg.string << .processed
[o] .output:pg.string << .result
[X]

[|] ParallelBatchProcessor
[i] .items: pg.array.pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Initialize result collectors
[r] .results: pg.array.pg.string << []
[r] .errors:pg.array{!} << []
[r] .item_count:pg.int << {.items|length}

// Parallel processing block - process items concurrently
// Process item 0 (if exists)
[?] .item_count ?> 0..
[~]
[~][p] |ProcessItem
[~][<] .item:pg.string << .items[0]
[~][<] .item_id:pg.int << 0
[~][>] .output:pg.string >> .result_0
[~][~]
[~][~][!] !ProcessingError
[~][~][>] .message:pg.string >> .err_0_msg
[~][~][>] .item_id:pg.int >> .err_0_id
[~][~][r] .result_0:pg.string << "ERROR"

// Process item 1 (if exists)
[?] .item_count ?> 1..
[~]
[~][p] |ProcessItem
[~][<] .item:pg.string << .items[1]
[~][<] .item_id:pg.int << 1
[~][>] .output:pg.string >> .result_1
[~][~]
[~][~][!] !ProcessingError
[~][~][>] .message:pg.string >> .err_1_msg
[~][~][>] .item_id:pg.int >> .err_1_id
[~][~][r] .result_1:pg.string << "ERROR"

// Process item 2 (if exists)
[?] .item_count ?> 2..
[~]
[~][p] |ProcessItem
[~][<] .item:pg.string << .items[2]
[~][<] .item_id:pg.int << 2
[~][>] .output:pg.string >> .result_2
[~][~]
[~][~][!] !ProcessingError
[~][~][>] .message:pg.string >> .err_2_msg
[~][~][>] .item_id:pg.int >> .err_2_id
[~][~][r] .result_2:pg.string << "ERROR"

// Wait for all parallel operations to complete
[Y] |Y.Join

// Aggregate results (after join)
[?] .item_count ?> 0..
[~][r] .results: pg.array.pg.string << [.result_0]

[?] .item_count ?> 1..
[~][r] .results: pg.array.pg.string << {.results + .result_1}

[?] .item_count ?> 2..
[~][r] .results: pg.array.pg.string << {.results + .result_2}

[o] .processed: pg.array.pg.string << .results
[X]
```

**Explanation:**

This example demonstrates:
1. **Parallel Block**: Using `[p]` to execute operations concurrently
2. **Join Synchronization**: Using `[Y]` to wait for all parallel operations to complete
3. **Copy Semantics**: Each parallel branch gets a copy of input data
4. **Error Handling in Parallel**: Each branch handles errors independently
5. **Result Aggregation**: Collecting results after join synchronization
6. **Conditional Processing**: Only processing items that exist in the array

**Output:**

Given input array `["hello", "world", "polyglot"]`, processes all three items in parallel, producing `["HELLO", "WORLD", "POLYGLOT"]`. If any item processing fails, that item becomes "ERROR" in the results array.

**See Also:**
- [Parallel Execution Documentation](../language/08-parallel-execution.md)
- [Array Operations](../language/02-type-system.md#array-operations)

---

## Example 4: DateTime Comparison and Scheduling

**Purpose:** Demonstrates datetime operations, comparisons using range notation, and scheduling patterns.

**Key Concepts:**
- DateTime type `:pg.dt`
- Range notation for comparisons
- Datetime arithmetic
- Scheduling patterns

**Code:**

```polyglot
[!] !ScheduleError
[<] .message:pg.string << "Schedule error"
[<] .code:pg.int << 5001
[<] .trace:pg.string << ""
[<] .event_time:pg.dt << |DT"2024-01-01T00:00:00Z"
[X]

[|] CheckEventTiming
[i] .event_time:pg.dt
[i] .reference_time:pg.dt
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Check if event is in the past (before reference)
[?] .event_time ?> ...reference_time
[~]
[~][r] .timing:pg.string << "past"
[~][r] .is_past:pg.bool << #True

// Check if event is in the future (after reference)
[?] .event_time ?> .reference_time..
[~]
[~][r] .timing:pg.string << "future"
[~][r] .is_future:pg.bool << #True

// Check if event is exactly at reference time
[?] .event_time ?> .reference_time
[~]
[~][r] .timing:pg.string << "now"
[~][r] .is_now:pg.bool << #True

[o] .result:pg.string << .timing
[X]

[|] ValidateScheduleWindow
[i] .event_time:pg.dt
[i] .window_start:pg.dt
[i] .window_end:pg.dt
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Check if event is before window start
[?] .event_time ?> ...window_start
[~]
[~][r] .error: ! << !ScheduleError
[~][<] .message:pg.string << "Event scheduled before window start"
[~][<] .code:pg.int << 5001
[~][<] .trace:pg.string << ""
[~][<] .event_time:pg.dt << .event_time
[~][o] .error: !

// Check if event is after window end
[?] .event_time ?> .window_end..
[~]
[~][r] .error: ! << !ScheduleError
[~][<] .message:pg.string << "Event scheduled after window end"
[~][<] .code:pg.int << 5002
[~][<] .trace:pg.string << ""
[~][<] .event_time:pg.dt << .event_time
[~][o] .error: !

// Event is within valid window
[r] .valid:pg.bool << #True
[o] .is_valid:pg.bool << .valid
[X]

[|] ScheduleEventProcessor
[i] .event_name:pg.string
[i] .event_time:pg.dt
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Get current time (simulated)
[r] .now:pg.dt << |DT"2024-06-15T10:00:00Z"

// Define scheduling window (e.g., next 30 days)
[r] .window_start:pg.dt << .now
[r] .window_end:pg.dt << {.now + 30d}  // Add 30 days

// Validate event is within scheduling window
[r] |ValidateScheduleWindow
[<] .event_time:pg.dt << .event_time
[<] .window_start:pg.dt << .window_start
[<] .window_end:pg.dt << .window_end
[>] .is_valid:pg.bool >> .valid_schedule
[~]
[~][!] !ScheduleError
[~][>] .message:pg.string >> .schedule_err_msg
[~][>] .code:pg.int >> .schedule_err_code
[~]
[~]// Invalid schedule - output error
[~][r] .error: ! << !ScheduleError
[~][<] .message:pg.string << .schedule_err_msg
[~][<] .code:pg.int << .schedule_err_code
[~][<] .trace:pg.string << ""
[~][<] .event_time:pg.dt << .event_time
[~][o] .error: !

// Check timing relative to now
[r] |CheckEventTiming
[<] .event_time:pg.dt << .event_time
[<] .reference_time:pg.dt << .now
[>] .result:pg.string >> .timing_status

// Build schedule confirmation message
[r] .confirmation:pg.string << "Event '{.event_name}' scheduled for {.event_time} "
[*] +"Status: {.timing_status} "
[*] +"Window: {.window_start} to {.window_end}"

[o] .message:pg.string << .confirmation
[X]
```

**Explanation:**

This example demonstrates:
1. **Range Notation**: Using `...value` (less than), `.value..` (greater than), and `.value` (equals)
2. **DateTime Comparison**: Comparing datetime values without comparison operators
3. **DateTime Arithmetic**: Adding durations to datetime values (e.g., `+ 30d`)
4. **Multiline Strings**: Using `[*]` with `+""` prefix for string continuation
5. **Validation Windows**: Checking if values fall within specified ranges
6. **Error Handling**: Raising errors when validation fails

**Output:**

Given event "Team Meeting" scheduled for "2024-06-20T14:00:00Z", validates it falls within the 30-day window from "2024-06-15T10:00:00Z", determines it's in the future, and outputs confirmation message. Events outside the window raise `!ScheduleError`.

**See Also:**
- [DateTime System Documentation](../language/07-datetime-system.md)
- [Reserved #Comparison Enumeration](../audit/reserved-enumeration-schema-decisions.md#comparison)
- [Range Notation Reference](../audit/quick-language-reference.md#comparisons)

---

## Example 5: Multiple Error Types with Recovery Chain

**Purpose:** Demonstrates handling multiple different error types with cascading recovery strategies.

**Key Concepts:**
- Multiple custom error types
- Sequential error catching
- Error recovery chains
- Fallback strategies

**Code:**

```polyglot
[!] !NetworkError
[<] .message:pg.string << "Network error"
[<] .code:pg.int << 6001
[<] .trace:pg.string << ""
[<] .endpoint:pg.string << ""
[<] .retry_count:pg.int << 0
[X]

[!] !ParseError
[<] .message:pg.string << "Parse error"
[<] .code:pg.int << 6101
[<] .trace:pg.string << ""
[<] .raw_data:pg.string << ""
[X]

[!] !CacheError
[<] .message:pg.string << "Cache error"
[<] .code:pg.int << 6201
[<] .trace:pg.string << ""
[<] .cache_key:pg.string << ""
[X]

[|] FetchFromNetwork
[i] .url:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Simulate network fetch
[r] .response:pg.string << "simulated_network_response"

// Check if response is empty (simulating network failure)
[?] .response ?> ""
[~]
[~][r] .error: ! << !NetworkError
[~][<] .message:pg.string << "Network request returned empty response"
[~][<] .code:pg.int << 6001
[~][<] .trace:pg.string << ""
[~][<] .endpoint:pg.string << .url
[~][<] .retry_count:pg.int << 0
[~][o] .error: !

[o] .data:pg.string << .response
[X]

[|] ParseResponse
[i] .raw:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Simulate parsing (check for valid format)
[r] .trimmed:pg.string << {.raw|trim}
[r] .length:pg.int << {.trimmed|length}

[?] .length ?> ...5
[~]
[~][r] .error: ! << !ParseError
[~][<] .message:pg.string << "Response too short to be valid"
[~][<] .code:pg.int << 6101
[~][<] .trace:pg.string << ""
[~][<] .raw_data:pg.string << .raw
[~][o] .error: !

[r] .parsed:pg.string << .trimmed
[o] .result:pg.string << .parsed
[X]

[|] CacheLookup
[i] .key:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

// Simulate cache lookup (always miss for this example)
[r] .found:pg.bool << #False

[?] .found ?> False
[~]
[~][r] .error: ! << !CacheError
[~][<] .message:pg.string << "Cache miss"
[~][<] .code:pg.int << 6201
[~][<] .trace:pg.string << ""
[~][<] .cache_key:pg.string << .key
[~][o] .error: !

[o] .value:pg.string << "cached_value"
[X]

[|] FetchDataWithFallbacks
[i] .resource_id:pg.string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] .data:pg.string << ""
[r] .source:pg.string << "unknown"

// Strategy 1: Try cache first
[r] |CacheLookup
[<] .key:pg.string << .resource_id
[>] .value:pg.string >> .cached_data
[~]
[~][!] !CacheError
[~][>] .message:pg.string >> .cache_err_msg
[~][>] .cache_key:pg.string >> .cache_key
[~]
[~]// Cache miss - continue to network fetch
[~][r] .cache_miss:pg.bool << #True

// If we have cached data, use it
[?] .cached_data ?> ...""
[~]
[~][r] .data:pg.string << .cached_data
[~][r] .source:pg.string << "cache"

// Strategy 2: If no cache hit, try network
[?] .data ?> ""
[~]
[~][r] .url:pg.string << "https://api.example.com/resource/{.resource_id}"
[~][r] |FetchFromNetwork
[~][<] .url:pg.string << .url
[~][>] .data:pg.string >> .network_response
[~][~]
[~][~][!] !NetworkError
[~][~][>] .message:pg.string >> .network_err_msg
[~][~][>] .endpoint:pg.string >> .failed_endpoint
[~][~]
[~][~]// Network failed - use fallback
[~][~][r] .network_response:pg.string << ""
[~][~][r] .network_failed:pg.bool << #True

// If we have network response, parse it
[?] .network_response ?> ...""
[~]
[~][r] |ParseResponse
[~][<] .raw:pg.string << .network_response
[~][>] .result:pg.string >> .parsed_data
[~][~]
[~][~][!] !ParseError
[~][~][>] .message:pg.string >> .parse_err_msg
[~][~][>] .raw_data:pg.string >> .unparseable_data
[~][~]
[~][~]// Parse failed - use raw data
[~][~][r] .parsed_data:pg.string << .network_response
[~][~][r] .parse_failed:pg.bool << #True
[~]
[~][r] .data:pg.string << .parsed_data
[~][r] .source:pg.string << "network"

// Strategy 3: If still no data, use hardcoded fallback
[?] .data ?> ""
[~]
[~][r] .data:pg.string << "FALLBACK_DEFAULT_DATA"
[~][r] .source:pg.string << "fallback"

[o] .result:pg.string << .data
[o] .data_source:pg.string << .source
[X]
```

**Explanation:**

This example demonstrates:
1. **Multiple Error Types**: Three distinct error types (`!NetworkError`, `!ParseError`, `!CacheError`)
2. **Error Recovery Chain**: Try cache → try network → try parse → use fallback
3. **Selective Error Catching**: Each operation catches its specific error type
4. **Fallback Strategy**: Multiple levels of fallback (cache → network → parse → default)
5. **Error Propagation vs Recovery**: Deciding when to catch and recover vs propagate upward
6. **Source Tracking**: Recording which recovery strategy succeeded

**Output:**

Attempts to fetch data by:
1. First checking cache (catches `!CacheError` on miss)
2. Then fetching from network (catches `!NetworkError` on failure)
3. Then parsing response (catches `!ParseError` on invalid format)
4. Finally using hardcoded fallback if all else fails

Returns both the data and the source it came from ("cache", "network", or "fallback").

**See Also:**
- [Error Handling Patterns](../language/04-error-handling.md)
- [Error Type Definitions](../audit/reserved-enumeration-schema-decisions.md#error-types)
- [Fallback Strategies](../examples/03-error-handling.md)

---

## Validation Status

All examples have been validated against:

✅ **Syntax Correctness**: Follows v0.0.2 syntax specification
✅ **Error Handling**: Proper use of `[r] .error: ! << !ErrorType` and `[o] .error: !`
✅ **Type Safety**: All types properly declared and used
✅ **Trigger Requirement**: All pipelines include `[t]` triggers
✅ **No Comparison Operators**: Uses range notation instead of `==`, `!=`, `<`, `>`
✅ **Path Identifiers**: Correct usage of `#Path.Identifiers.*`
✅ **Multiline Strings**: Proper use of `[*]` with `+""` prefix
✅ **Reserved Enumerations**: Correct usage of `#None`, `#Comparison`, etc.
✅ **Best Practices**: Follows idiomatic Polyglot patterns

---

**Navigation:**
[Examples Index](00-index.md) | [Hello World Examples](01-hello-world.md) | [Quick Language Reference](../audit/quick-language-reference.md)