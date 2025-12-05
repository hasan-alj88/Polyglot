# Error Handling Examples

**Difficulty:** Intermediate
**Languages:** Python, Rust
**Topics:** !Error types, Validation, Retry strategies, Fallback patterns
**Time:** ~40 minutes

---

## Overview

Learn how to build resilient pipelines with proper error handling. These examples demonstrate validation, recovery strategies, and graceful degradation using Polyglot's `!Error` type system.

**Key Concept:** Polyglot uses `!Error` suffix to denote types that may contain errors, enabling explicit error handling at the type level.

---

## Example 1: File Validation

Validate file existence and readability before processing.

### Use Case

Process a data file only if it exists and is readable, otherwise return a clear error message.

### Complete Code

**File:** `validate_file.pg`

```polyglot
[@] Local@ErrorHandling.FileValidation:1.0.0
[#] 1
[X]





// Pipeline: Validate and read file with error handling
[|] ValidateAndReadFile

// Inputs
[i] .file_path:pg.path

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Check if file exists and is readable
[r] |File.Validate
[<] .path:pg.path << .file_path
[>] .valid:pg.bool!Error >> .is_valid
[>] .error_msg:pg.string >> .validation_error

// Handle validation result
[?] .is_valid
  // File is valid - read it
  [r] |File.ReadText
  [<] .path:pg.path << .file_path
  [>] .content:pg.string >> .file_content

  [o] .file_content:pg.string
[!]
  // File is invalid - return error
  [o] #Error
  [<] .message:pg.string << .validation_error
[X]
```

### Explanation

**Error Type Declaration:**
```polyglot
[>] .valid:pg.bool!Error >> .is_valid
```
- `:pg.bool!Error` means the result may be a boolean OR an error
- Forces explicit error handling

**Conditional Error Handling:**
```polyglot
[?] .is_valid
  // Success path
[!]
  // Error path
  [o] #Error
  [<] .message:pg.string << .validation_error
[X]
```
- `[?]` conditional block checks if validation succeeded
- `[!]` handles the error case
- `[o] #Error` returns an error to the caller

### Running

```bash
# Valid file
polyglot run ValidateAndReadFile --file_path "data.txt"

# Invalid file
polyglot run ValidateAndReadFile --file_path "missing.txt"
```

### Expected Output

**Success:**
```
[File contents displayed]
```

**Error:**
```
Error: File not found: missing.txt
```

---

## Example 2: Data Validation with Schema Checking

Validate CSV data against a schema before processing.

### Use Case

Ensure CSV file has required columns and valid data types before importing to database.

### Complete Code

**File:** `validate_schema.pg`

```polyglot
[@] Local@ErrorHandling.SchemaValidation:1.0.0
[#] 1
[X]





// Pipeline: Validate CSV schema and data quality
[|] ValidateCSVSchema

// Inputs
[i] .csv_file:pg.path
[i] .required_columns:pg.string  // Comma-separated list

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Read CSV file
[r] |File.ReadText
[<] .path:pg.path << .csv_file
[>] .content:pg.string >> .csv_content

// Validate schema and data
[r] |Run.Python
[<] .code:pg.string << """
import csv
from io import StringIO

csv_content = .csv_content
required_cols = .required_columns.split(',')

errors = []

# Parse CSV
try:
    csv_reader = csv.DictReader(StringIO(csv_content))
    rows = list(csv_reader)

    if not rows:
        errors.append("CSV file is empty")
    else:
        # Check required columns
        actual_cols = set(rows[0].keys())
        required_set = set(col.strip() for col in required_cols)
        missing_cols = required_set - actual_cols

        if missing_cols:
            errors.append(f"Missing required columns: {', '.join(missing_cols)}")

        # Validate data types (example: age should be integer)
        if 'age' in actual_cols:
            for i, row in enumerate(rows, start=2):
                try:
                    age = int(row['age'])
                    if age < 0 or age > 150:
                        errors.append(f"Row {i}: Invalid age value: {age}")
                except ValueError:
                    errors.append(f"Row {i}: Age must be an integer, got '{row['age']}'")

        # Check for empty required fields
        for i, row in enumerate(rows, start=2):
            for col in required_set:
                if col in row and not row[col].strip():
                    errors.append(f"Row {i}: Empty value in required column '{col}'")

except Exception as e:
    errors.append(f"Failed to parse CSV: {str(e)}")

# Return validation result
if errors:
    result_valid = False
    result_errors = '\\n'.join(errors)
else:
    result_valid = True
    result_errors = ""
"""
[>] .result_valid:pg.bool!Error >> .is_valid
[>] .result_errors:pg.string >> .error_list

// Handle validation result
[?] .is_valid
  // Schema is valid - proceed with processing
  [r] |Run.Python
  [<] .code:pg.string << """
result = f"Validation passed! CSV is ready for import."
"""
  [>] .result:pg.string >> .success_msg

  [o] .success_msg:pg.string
[!]
  // Schema validation failed
  [o] #Error
  [<] .message:pg.string << .error_list
[X]
```

### Running

```bash
polyglot run ValidateCSVSchema \
  --csv_file "users.csv" \
  --required_columns "id,name,email,age"
```

### Input Data (`users.csv` - Invalid)

```csv
id,name,email
1,Alice,alice@example.com
2,Bob,
3,Charlie,charlie@example.com
```

### Expected Output (Error)

```
Error:
Missing required columns: age
Row 3: Empty value in required column 'email'
```

### Input Data (`users.csv` - Valid)

```csv
id,name,email,age
1,Alice,alice@example.com,28
2,Bob,bob@example.com,35
```

### Expected Output (Success)

```
Validation passed! CSV is ready for import.
```

---

## Example 3: Retry Strategy with Exponential Backoff

Retry failed operations with increasing delays.

### Use Case

Call an external API that may be temporarily unavailable. Retry up to 3 times with exponential backoff.

### Complete Code

**File:** `retry_pattern.pg`

```polyglot
[@] Local@ErrorHandling.RetryPattern:1.0.0
[#] 1
[X]





// Pipeline: Retry API call with exponential backoff
[|] RetryAPICall

// Inputs
[i] .api_url:pg.string
[i] .max_retries:pg.int << 3

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Retry loop with exponential backoff
[r] |Run.Python
[<] .code:pg.string << """
import time
import urllib.request
import urllib.error

url = .api_url
max_retries = .max_retries
retry_count = 0
success = False
last_error = ""

while retry_count < max_retries and not success:
    try:
        # Attempt API call
        with urllib.request.urlopen(url, timeout=5) as response:
            result_data = response.read().decode('utf-8')
            success = True
            result_success = True

    except urllib.error.URLError as e:
        retry_count += 1
        last_error = f"Attempt {retry_count}/{max_retries} failed: {str(e)}"

        if retry_count < max_retries:
            # Exponential backoff: 2^retry_count seconds
            wait_time = 2 ** retry_count
            print(f"{last_error}. Retrying in {wait_time} seconds...")
            time.sleep(wait_time)
        else:
            result_success = False
            result_data = ""

# Set result
if not success:
    result_success = False
    result_error = f"API call failed after {max_retries} retries. Last error: {last_error}"
else:
    result_error = ""
"""
[>] .result_success:pg.bool!Error >> .api_success
[>] .result_data:pg.string >> .api_response
[>] .result_error:pg.string >> .api_error

// Handle result
[?] .api_success
  // API call succeeded
  [o] .api_response:pg.string
[!]
  // API call failed after retries
  [o] #Error
  [<] .message:pg.string << .api_error
[X]
```

### Explanation

**Retry Logic:**
```python
while retry_count < max_retries and not success:
    try:
        # Attempt operation
    except Exception as e:
        retry_count += 1
        if retry_count < max_retries:
            wait_time = 2 ** retry_count  # Exponential backoff
            time.sleep(wait_time)
```

**Backoff Timing:**
- Attempt 1: Immediate
- Attempt 2: Wait 2 seconds
- Attempt 3: Wait 4 seconds
- Attempt 4: Wait 8 seconds

### Running

```bash
polyglot run RetryAPICall --api_url "https://api.example.com/data"
```

### Expected Output (Success after retry)

```
Attempt 1/3 failed: Connection timeout. Retrying in 2 seconds...
Attempt 2/3 failed: Connection timeout. Retrying in 4 seconds...
[API response data]
```

### Expected Output (Total failure)

```
Attempt 1/3 failed: Connection timeout. Retrying in 2 seconds...
Attempt 2/3 failed: Connection timeout. Retrying in 4 seconds...
Attempt 3/3 failed: Connection timeout. Retrying in 8 seconds...
Error: API call failed after 3 retries. Last error: Connection timeout
```

---

## Example 4: Fallback Pattern

Try primary data source, fall back to secondary if primary fails.

### Use Case

Fetch configuration from primary server; if unavailable, use backup server; if both fail, use default config.

### Complete Code

**File:** `fallback_pattern.pg`

```polyglot
[@] Local@ErrorHandling.FallbackPattern:1.0.0
[#] 1
[X]





// Pipeline: Multi-level fallback for configuration loading
[|] LoadConfigWithFallback

// Inputs
[i] .primary_url:pg.string
[i] .backup_url:pg.string
[i] .default_config:pg.string

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Try primary source
[r] |Run.Python
[<] .code:pg.string << """
import urllib.request
import urllib.error

try:
    with urllib.request.urlopen(.primary_url, timeout=3) as response:
        result_config = response.read().decode('utf-8')
        result_source = "primary"
        result_success = True
except Exception as e:
    result_success = False
    result_error = f"Primary failed: {str(e)}"
"""
[>] .result_success:pg.bool!Error >> .primary_success
[>] .result_config:pg.string >> .config_from_primary
[>] .result_source:pg.string >> .source_primary

// Check primary result
[?] .primary_success
  // Primary succeeded
  [o] .config_from_primary:pg.string
  [o] .source_primary:pg.string
[!]
  // Primary failed - try backup
  [r] |Run.Python
  [<] .code:pg.string << """
import urllib.request
import urllib.error

try:
    with urllib.request.urlopen(.backup_url, timeout=3) as response:
        result_config = response.read().decode('utf-8')
        result_source = "backup"
        result_success = True
except Exception as e:
    result_success = False
    result_error = f"Backup also failed: {str(e)}"
"""
  [>] .result_success:pg.bool!Error >> .backup_success
  [>] .result_config:pg.string >> .config_from_backup
  [>] .result_source:pg.string >> .source_backup

  // Check backup result
  [?] .backup_success
    // Backup succeeded
    [o] .config_from_backup:pg.string
    [o] .source_backup:pg.string
  [!]
    // Both failed - use default config
    [o] .default_config:pg.string
    [o] "default":pg.string
  [X]
[X]
```

### Explanation

**Nested Fallback:**
```polyglot
[?] .primary_success
  // Use primary
[!]
  [?] .backup_success
    // Use backup
  [!]
    // Use default
  [X]
[X]
```

Three-tier fallback: primary → backup → default

### Running

```bash
polyglot run LoadConfigWithFallback \
  --primary_url "https://config.example.com/app.json" \
  --backup_url "https://backup.example.com/app.json" \
  --default_config '{"mode":"safe","timeout":30}'
```

### Expected Output

**Primary available:**
```
{"mode":"production","timeout":60}
Source: primary
```

**Primary down, backup available:**
```
{"mode":"production","timeout":60}
Source: backup
```

**Both down:**
```
{"mode":"safe","timeout":30}
Source: default
```

---

## Example 5: Error Propagation

Propagate errors up the call chain for centralized handling.

### Use Case

Multi-step data pipeline where any step can fail. Propagate errors to top-level handler.

### Complete Code

**File:** `error_propagation.pg`

```polyglot
[@] Local@ErrorHandling.ErrorPropagation:1.0.0
[#] 1
[X]





// Pipeline: Multi-step pipeline with error propagation
[|] ProcessDataPipeline

// Inputs
[i] .input_file:pg.path
[i] .output_file:pg.path

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Step 1: Validate file
[r] |ValidateFile
[<] .path:pg.path << .input_file
[>] .valid:pg.bool!Error >> .file_valid
[>] .error:pg.string >> .validate_error

// Propagate validation error if failed
[?] .file_valid
  // Continue to step 2
[!]
  [o] #Error
  [<] .message:pg.string << f"Step 1 (Validation) failed: {.validate_error}"
  [X]  // Exit pipeline
[X]

// Step 2: Parse data
[r] |File.ReadText
[<] .path:pg.path << .input_file
[>] .content:pg.string >> .raw_data

[r] |Run.Python
[<] .code:pg.string << """
import json

try:
    data = json.loads(.raw_data)
    result_data = data
    result_success = True
    result_error = ""
except json.JSONDecodeError as e:
    result_success = False
    result_error = f"Invalid JSON: {str(e)}"
"""
[>] .result_success:pg.bool!Error >> .parse_success
[>] .result_data:pg.string >> .parsed_data
[>] .result_error:pg.string >> .parse_error

// Propagate parse error if failed
[?] .parse_success
  // Continue to step 3
[!]
  [o] #Error
  [<] .message:pg.string << f"Step 2 (Parsing) failed: {.parse_error}"
  [X]  // Exit pipeline
[X]

// Step 3: Transform data
[r] |Run.Python
[<] .code:pg.string << """
import json

try:
    data = json.loads(.parsed_data)

    # Validate required fields
    if 'records' not in data:
        raise ValueError("Missing 'records' field")

    # Transform
    transformed = []
    for record in data['records']:
        if 'id' not in record or 'value' not in record:
            raise ValueError(f"Record missing required fields: {record}")

        transformed.append({
            'id': record['id'],
            'value': record['value'] * 2  # Example transformation
        })

    result_data = json.dumps({'transformed': transformed})
    result_success = True
    result_error = ""

except Exception as e:
    result_success = False
    result_error = str(e)
"""
[>] .result_success:pg.bool!Error >> .transform_success
[>] .result_data:pg.string >> .transformed_data
[>] .result_error:pg.string >> .transform_error

// Propagate transform error if failed
[?] .transform_success
  // Continue to step 4
[!]
  [o] #Error
  [<] .message:pg.string << f"Step 3 (Transform) failed: {.transform_error}"
  [X]  // Exit pipeline
[X]

// Step 4: Write output
[r] |File.WriteText
[<] .path:pg.path << .output_file
[<] .content:pg.string << .transformed_data

// Success!
[o] .transformed_data:pg.string
[X]
```

### Explanation

**Error Propagation Pattern:**
```polyglot
[r] |SomeOperation
[>] .success:pg.bool!Error >> .op_success

[?] .op_success
  // Continue pipeline
[!]
  [o] #Error
  [<] .message:pg.string << "Operation failed: ..."
  [X]  // Exit early
[X]
```

Each step checks for errors and exits early if any step fails.

### Running

```bash
polyglot run ProcessDataPipeline \
  --input_file "data.json" \
  --output_file "transformed.json"
```

### Expected Output (Success)

```
{"transformed":[{"id":1,"value":200},{"id":2,"value":400}]}
```

### Expected Output (Error at step 2)

```
Error: Step 2 (Parsing) failed: Invalid JSON: Expecting value: line 1 column 1
```

### Expected Output (Error at step 3)

```
Error: Step 3 (Transform) failed: Missing 'records' field
```

---

## Example 6: Serial Load Block Error Handling

Handle errors from parallel file loading with `[s][!]` blocks.

### Use Case

Load multiple configuration files in parallel. Some files may be missing or invalid. Handle errors gracefully and continue with partial configuration.

### Complete Code

**File:** `serial_error_handling.pg`

```polyglot
[@] Local@ErrorHandling.SerialLoadErrors:1.0.0
[#] 1
[X]




// Pipeline: Load configs with comprehensive error handling
[|] LoadConfigsWithErrorHandling

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Serial Load: Load multiple configs in parallel
[s] .required_config << JSON"\\Config\\app.json"
[s] .optional_config1 << JSON"\\Config\\feature-flags.json"
[s] .optional_config2 << JSON"\\Config\\experimental.json"
[s] .all_plugins << JSON"\\Plugins\\*.json".FilenameKey

// Automatic join - all loads attempted before error handling

// Variable-level critical check
[!] .required_config.error =? !File.NotFound
[r] |U.Log.Fatal
[<] .msg:pg.string << "Required configuration file missing!"
[o] !ConfigurationError
[X]

// Scope-level error handling for optional configs
[s][!] !File.NotFound
[r] |U.Log.Warn
[<] .msg:pg.string << "Optional config file not found, using defaults"

[s][!] !JSON.ParseError
[r] |U.Log.Error
[<] .msg:pg.string << "Invalid JSON in config file"
[r] |Run.Python
[<] .code:pg.string << """
# Log which file failed
result = "Check configuration files for syntax errors"
"""
[>] .result:pg.string >> .error_hint
[r] |U.Log.Error
[<] .msg:pg.string << .error_hint

// Process loaded configurations
[r] |Run.Python
[<] .code:pg.string << """
import json

# Check each config and use defaults if needed
def get_config(var, default):
    if var == "#None.ErrorState":
        return default
    return var

required = get_config(.required_config, {})
optional1 = get_config(.optional_config1, {"features": []})
optional2 = get_config(.optional_config2, {"experiments": []})
plugins = get_config(.all_plugins, {})

# Merge all configurations
final_config = {
    **required,
    "features": optional1.get("features", []),
    "experiments": optional2.get("experiments", []),
    "plugins": list(plugins.keys()) if isinstance(plugins, dict) else []
}

# Generate status report
status = {
    "required_loaded": .required_config != "#None.ErrorState",
    "optional1_loaded": .optional_config1 != "#None.ErrorState",
    "optional2_loaded": .optional_config2 != "#None.ErrorState",
    "plugins_loaded": .all_plugins != "#None.ErrorState",
    "config": final_config
}

result = json.dumps(status, indent=2)
"""
[>] .result:pg.string >> .config_status

[o] .config_status:pg.string
[X]
```

### Explanation

**Variable-Level Error Checking:**
```polyglot
[!] .required_config.error =? !File.NotFound
[r] |U.Log.Fatal
[o] !ConfigurationError
[X]
```
- Check specific variable's error state
- `.error` field contains error type
- Fail immediately if critical config missing
- Variable-level takes precedence over scope-level

**Scope-Level Error Handling:**
```polyglot
[s][!] !File.NotFound
[r] |U.Log.Warn
[<] .msg:pg.string << "Optional config file not found, using defaults"
```
- Catches errors from ALL `[s]` blocks at this scope
- Applies to `optional_config1`, `optional_config2`, and `all_plugins`
- Does NOT trigger for `required_config` (handled at variable-level)

**Error-Carrying Variables:**
```polyglot
def get_config(var, default):
    if var == "#None.ErrorState":
        return default
    return var
```
- Variables hold data on success
- Variables hold `#None.ErrorState` on failure
- Application continues with defaults for failed loads
- Partial success model: use what loaded successfully

**Partial Success Scenario:**
- `required_config` loads ✓
- `optional_config1` loads ✓
- `optional_config2` fails (missing) → Use default
- `all_plugins` loads ✓
- Application continues with 3 out of 4 configs

### Running

```bash
polyglot run LoadConfigsWithErrorHandling
```

### Expected Output (All Success)

```json
{
  "required_loaded": true,
  "optional1_loaded": true,
  "optional2_loaded": true,
  "plugins_loaded": true,
  "config": {
    "app_name": "MyApp",
    "features": ["feature1", "feature2"],
    "experiments": ["exp1"],
    "plugins": ["auth", "logging", "metrics"]
  }
}
```

### Expected Output (Partial Failure - optional2 missing)

**Console:**
```
Warning: Optional config file not found, using defaults
```

**Output:**
```json
{
  "required_loaded": true,
  "optional1_loaded": true,
  "optional2_loaded": false,
  "plugins_loaded": true,
  "config": {
    "app_name": "MyApp",
    "features": ["feature1", "feature2"],
    "experiments": [],
    "plugins": ["auth", "logging", "metrics"]
  }
}
```

### Expected Output (Critical Failure - required config missing)

```
Fatal: Required configuration file missing!
Error: !ConfigurationError
```

Pipeline exits immediately without processing optional configs.

---

## Example 7: Multiple Error Types with Precedence

Handle different error types with appropriate responses.

### Use Case

Load data files with different criticality levels. Critical files must load, optional files can fail gracefully.

### Complete Code

**File:** `error_precedence.pg`

```polyglot
[@] Local@ErrorHandling.ErrorPrecedence:1.0.0
[#] 1
[X]




// Pipeline: Demonstrate error precedence
[|] LoadDataWithPrecedence

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Load critical and optional data files
[s] .critical_data << JSON"\\Data\\critical.json"
[s] .user_data << JSON"\\Data\\users.json"
[s] .cache_data << JSON"\\Data\\cache.json"

// Variable-level handler for critical file (highest precedence)
[~][!] !File.NotFound
[~][?] .critical_data.error =? !File.NotFound
[~]  [r] |U.Log.Fatal
[~]  [<] .msg:pg.string << "Critical data file is missing - cannot continue"
[~]  [o] !DataError
[~]  [X]  // Exit pipeline immediately
[~][X]

// Scope-level handler for non-critical files (lower precedence)
[s][!] !File.NotFound
[r] |U.Log.Warn
[<] .msg:pg.string << "Non-critical data file missing, continuing with defaults"

// Different handler for parse errors
[s][!] !JSON.ParseError
[r] |U.Log.Error
[<] .msg:pg.string << "Invalid JSON detected"
[r] |Run.Python
[<] .code:pg.string << """
# Determine which file failed
errors = []
if .critical_data == "#None.ErrorState" and .critical_data.error == "!JSON.ParseError":
    errors.append("critical.json")
if .user_data == "#None.ErrorState" and .user_data.error == "!JSON.ParseError":
    errors.append("users.json")
if .cache_data == "#None.ErrorState" and .cache_data.error == "!JSON.ParseError":
    errors.append("cache.json")

result = f"Files with parse errors: {', '.join(errors)}"
"""
[>] .result:pg.string >> .parse_errors
[r] |U.Log.Error
[<] .msg:pg.string << .parse_errors

// Process with available data
[r] |Run.Python
[<] .code:pg.string << """
import json

critical = .critical_data if .critical_data != "#None.ErrorState" else {}
users = .user_data if .user_data != "#None.ErrorState" else []
cache = .cache_data if .cache_data != "#None.ErrorState" else {}

result = json.dumps({
    "status": "running",
    "critical_loaded": critical != {},
    "users_count": len(users) if isinstance(users, list) else 0,
    "cache_available": cache != {}
}, indent=2)
"""
[>] .result:pg.string >> .status

[o] .status:pg.string
[X]
```

### Explanation

**Error Precedence Hierarchy:**
```
1. Variable-level [~][!] (highest - for critical files)
2. Scope-level [s][!] (medium - for all serial blocks)
3. Implicit notification (lowest - no handler defined)
```

**Critical File Protection:**
```polyglot
[~][!] !File.NotFound
[~][?] .critical_data.error =? !File.NotFound
[~]  [o] !DataError
[~]  [X]  // Exit immediately
```
- Nested block with variable check
- Only triggers for `.critical_data`
- Takes precedence over general `[s][!]` handler
- Exits pipeline before continuing

**Multiple Error Type Handling:**
```polyglot
[s][!] !File.NotFound
  // Handle missing files

[s][!] !JSON.ParseError
  // Handle invalid JSON
```
- Different handlers for different error types
- Each error type handled appropriately
- Critical vs non-critical differentiation

### Running

```bash
polyglot run LoadDataWithPrecedence
```

### Expected Output

**All files valid:**
```json
{
  "status": "running",
  "critical_loaded": true,
  "users_count": 150,
  "cache_available": true
}
```

**Critical file missing:**
```
Fatal: Critical data file is missing - cannot continue
Error: !DataError
```

**Non-critical file missing:**
```
Warning: Non-critical data file missing, continuing with defaults
{
  "status": "running",
  "critical_loaded": true,
  "users_count": 0,
  "cache_available": false
}
```

---

## Key Concepts

### 1. Error Types

**Declaration:**
```polyglot
[>] .result:pg.string!Error >> .output
```
- `!Error` suffix indicates the value may be an error
- Forces explicit error handling in consuming code

### 2. Error Handling Blocks

**Conditional error checking:**
```polyglot
[?] .success_flag
  // Success path
[!]
  // Error path
[X]
```

**Returning errors:**
```polyglot
[o] #Error
[<] .message:pg.string << "Error description"
```

### 3. Error Strategies

| Strategy | Use Case | Example |
|----------|----------|---------|
| **Validation** | Check preconditions | File exists, schema valid |
| **Retry** | Transient failures | Network calls, DB queries |
| **Fallback** | Multiple data sources | Primary → Backup → Default |
| **Propagation** | Multi-step pipelines | Early exit on error |
| **Recovery** | Graceful degradation | Use cached data on failure |

---

## Common Patterns

### Pattern 1: Validate → Process → Handle Error
```polyglot
[r] |Validate
[>] .valid:pg.bool!Error

[?] .valid
  [r] |Process
[!]
  [o] #Error
[X]
```

### Pattern 2: Try → Retry → Fail
```polyglot
[r] |TryOperation (attempt 1)
[?] .success → Done
[!]
  [r] |TryOperation (attempt 2)
  [?] .success → Done
  [!]
    [o] #Error
  [X]
[X]
```

### Pattern 3: Try Primary → Fallback → Default
```polyglot
[r] |TryPrimary
[?] .success → Use primary
[!]
  [r] |TryBackup
  [?] .success → Use backup
  [!]
    Use default
  [X]
[X]
```

---

## Best Practices

1. **Always Handle !Error Types**
   - Never ignore `!Error` results
   - Use `[?]` conditional to check success

2. **Fail Fast with Validation**
   - Validate inputs early
   - Exit immediately on validation failure

3. **Provide Clear Error Messages**
   - Include context (which step failed)
   - Include details (what went wrong)
   - Include hints (how to fix)

4. **Use Appropriate Strategy**
   - Retry: Transient network/DB errors
   - Fallback: Multiple data sources available
   - Propagate: Multi-step pipelines
   - Validate: User inputs, file schemas

5. **Log Errors for Debugging**
   - Log error details before propagating
   - Include timestamps and context
   - Use structured logging

---

## Next Steps

1. **Parallel Execution** - [Parallel Execution Examples](parallel-execution.md)
   - Error handling in parallel workflows
   - Partial failure scenarios

2. **Complete Workflows** - [Complete Workflows](complete-workflows.md)
   - Production error handling patterns
   - Monitoring and alerting

3. **Error Handling Reference** - [Error Handling Documentation](../language/04-error-handling.md)
   - Complete error type system
   - Advanced error patterns

---

## See Also

- [Data Processing Examples](data-processing.md) - Data validation patterns
- [Type System](../language/02-type-system.md) - Understanding !Error types
- [Serial Load Blocks](../language/06-block-markers.md#s---serial-load-block) - `[s]` block syntax
- [Serial Error Handling](../language/04-error-handling.md#serial-block-error-handling-s) - `[s][!]` complete reference
- [File Operations](file-operations.md) - Serial load file examples
- [Examples Index](README.md) - All examples

---

**Last Updated:** 2025-11-19