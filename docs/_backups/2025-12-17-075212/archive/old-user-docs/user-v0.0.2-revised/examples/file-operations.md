# File Operations Examples

**Difficulty:** Beginner to Intermediate
**Languages:** Python, Rust
**Topics:** File I/O, Path handling, File watching, Directory operations
**Time:** ~25 minutes

---

## Overview

Learn how to work with files and directories in Polyglot pipelines. These examples demonstrate reading, writing, monitoring, and managing files using the `pg\path` type and file triggers.

**Key Concept:** Use `pg\path` for file paths and file triggers for automated file processing.

---

## Example 1: Basic File Reading and Writing

Read a text file, transform its contents, and write to a new file.

### Use Case

Convert all text in a file to uppercase and save to a new file.

### Complete Code

**File:** `file_transform.pg`

```polyglot
[@] Local@FileOps.BasicReadWrite:1.0.0
[#] 1
[X]





// Pipeline: Read, transform, and write file
[|] TransformFile

// Inputs
[i] .input_file: pg\path
[i] .output_file: pg\path

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Read input file
[r] |File.ReadText
[<] .path: pg\path << .input_file
[>] .content: pg\string >> .file_content

// Transform: Convert to uppercase
[r] |Run.Python
[<] .code: pg\string << """
result = .file_content.upper()
"""
[>] .result: pg\string >> .transformed_content

// Write to output file
[r] |File.WriteText
[<] .path: pg\path << .output_file
[<] .content: pg\string << .transformed_content

// Output success message
[o] .transformed_content: pg\string
[X]
```

### Explanation

**File Reading:**
```polyglot
[r] |File.ReadText
[<] .path: pg\path << .input_file
[>] .content: pg\string >> .file_content
```
- `|File.ReadText` is a standard library utility
- Input: `pg\path` type for file path
- Output: `pg\string` with file contents

**File Writing:**
```polyglot
[r] |File.WriteText
[<] .path: pg\path << .output_file
[<] .content: pg\string << .transformed_content
```
- `|File.WriteText` writes string to file
- Creates file if it doesn't exist
- Overwrites if file exists

### Running

```bash
polyglot run TransformFile \
  --input_file "input.txt" \
  --output_file "output.txt"
```

### Input Data (`input.txt`)

```
hello world
this is a test
```

### Expected Output (`output.txt`)

```
HELLO WORLD
THIS IS A TEST
```

---

## Example 2: File Watch Automation

Automatically process files when they appear in a directory.

### Use Case

Monitor a directory for new CSV files and automatically process them when they arrive.

### Complete Code

**File:** `file_watch.pg`

```polyglot
[@] Local@FileOps.FileWatch:1.0.0
[#] 1
[X]





// Pipeline: Automated file processing on file creation
[|] WatchAndProcess

// No inputs - triggered by file system events
[i] .watch_dir: pg\path << \\Data\\incoming\\
[i] .output_dir: pg\path << \\Data\\processed\\

// Trigger: File watch
[t] |T.FileWatch
[<] .path: pg\path << .watch_dir
[<] .pattern: pg\string << "*.csv"
[<] .event: pg\string << "create"

// Runtime Wrappers
[W] |W.Python3.11

// Get triggered file path
[r] |Trigger.GetFilePath
[>] .file_path: pg\path >> .new_file

// Read the new file
[r] |File.ReadText
[<] .path: pg\path << .new_file
[>] .content: pg\string >> .csv_content

// Process CSV
[r] |Run.Python
[<] .code: pg\string << """
import csv
from io import StringIO

# Parse CSV
csv_reader = csv.DictReader(StringIO(.csv_content))
rows = list(csv_reader)

# Count rows
row_count = len(rows)
result = f"Processed {row_count} rows from file"
"""
[>] .result: pg\string >> .process_summary

// Write summary to output directory
[r] |Path.GetFileName
[<] .path: pg\path << .new_file
[>] .filename: pg\string >> .original_filename

[r] |Run.Python
[<] .code: pg\string << """
import os
output_path = os.path.join(.output_dir, .original_filename.replace('.csv', '_summary.txt'))
result = output_path
"""
[>] .result: pg\path >> .summary_file

[r] |File.WriteText
[<] .path: pg\path << .summary_file
[<] .content: pg\string << .process_summary

// Output summary
[o] .process_summary: pg\string
[X]
```

### Explanation

**File Watch Trigger:**
```polyglot
[t] |T.FileWatch
[<] .path: pg\path << .watch_dir
[<] .pattern: pg\string << "*.csv"
[<] .event: pg\string << "create"
```
- Monitors directory for new files
- Pattern `*.csv` matches only CSV files
- Event `create` triggers on new file creation
- Other events: `modify`, `delete`, `move`

**Get Triggered File:**
```polyglot
[r] |Trigger.GetFilePath
[>] .file_path: pg\path >> .new_file
```
- Retrieves path of the file that triggered the pipeline

### Running

```bash
# Register and activate the file watch
polyglot register Local@FileOps.FileWatch:1.0.0
polyglot activate Local@FileOps.FileWatch:1.0.0

# Now the pipeline runs automatically when CSV files are created in the watch directory
```

### Expected Behavior

When `data.csv` is created in `\\Data\\incoming\\`:
1. Pipeline triggers automatically
2. Reads and processes the CSV
3. Writes summary to `\\Data\\processed\\data_summary.txt`

---

## Example 3: Directory Operations

List files in a directory and process each one.

### Use Case

Find all text files in a directory and count total words across all files.

### Complete Code

**File:** `directory_scan.pg`

```polyglot
[@] Local@FileOps.DirectoryScan:1.0.0
[#] 1
[X]





// Pipeline: Scan directory and process all files
[|] ScanAndCountWords

// Inputs
[i] .directory: pg\path
[i] .file_pattern: pg\string << "*.txt"

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// List files in directory
[r] |Directory.ListFiles
[<] .path: pg\path << .directory
[<] .pattern: pg\string << .file_pattern
[>] .files: pg\string >> .file_list  // Comma-separated list

// Process each file
[r] |Run.Python
[<] .code: pg\string << """
import os

files = .file_list.split(',')
total_words = 0
file_details = []

for file_path in files:
    file_path = file_path.strip()
    if not file_path:
        continue

    # Read file
    with open(file_path, 'r') as f:
        content = f.read()

    # Count words
    word_count = len(content.split())
    total_words += word_count

    filename = os.path.basename(file_path)
    file_details.append(f"{filename}: {word_count} words")

# Generate report
report = f"Total files: {len([f for f in files if f.strip()])}\\n"
report += f"Total words: {total_words}\\n\\n"
report += "Details:\\n"
report += "\\n".join(file_details)

result = report
"""
[>] .result: pg\string >> .word_count_report

// Output report
[o] .word_count_report: pg\string
[X]
```

### Explanation

**Directory Listing:**
```polyglot
[r] |Directory.ListFiles
[<] .path: pg\path << .directory
[<] .pattern: pg\string << .file_pattern
[>] .files: pg\string >> .file_list
```
- Lists all files matching pattern
- Returns comma-separated string of file paths
- Pattern supports wildcards: `*.txt`, `data_*.csv`

### Running

```bash
polyglot run ScanAndCountWords \
  --directory "\\Documents\\reports\\" \
  --file_pattern "*.txt"
```

### Expected Output

```
Total files: 5
Total words: 15432

Details:
report1.txt: 3200 words
report2.txt: 2100 words
report3.txt: 5000 words
report4.txt: 2632 words
report5.txt: 2500 words
```

---

## Example 4: Path Manipulation

Work with file paths: extract parts, join paths, check existence.

### Use Case

Parse a file path to extract directory, filename, and extension, then create a backup path.

### Complete Code

**File:** `path_operations.pg`

```polyglot
[@] Local@FileOps.PathManipulation:1.0.0
[#] 1
[X]





// Pipeline: Path manipulation and backup file creation
[|] CreateBackup

// Inputs
[i] .source_file: pg\path

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Extract path components
[r] |Path.GetDirectory
[<] .path: pg\path << .source_file
[>] .directory: pg\path >> .dir

[r] |Path.GetFileName
[<] .path: pg\path << .source_file
[>] .filename: pg\string >> .name

[r] |Path.GetExtension
[<] .path: pg\path << .source_file
[>] .extension: pg\string >> .ext

// Create backup path
[r] |Run.Python
[<] .code: pg\string << """
import os
from datetime import datetime

# Get base name without extension
base_name = .name.replace(.ext, '')

# Create backup filename with timestamp
timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
backup_name = f"{base_name}_backup_{timestamp}{.ext}"

# Join with directory
backup_path = os.path.join(.dir, backup_name)
result = backup_path
"""
[>] .result: pg\path >> .backup_path

// Read source file
[r] |File.ReadText
[<] .path: pg\path << .source_file
[>] .content: pg\string >> .file_content

// Write to backup path
[r] |File.WriteText
[<] .path: pg\path << .backup_path
[<] .content: pg\string << .file_content

// Return backup path
[o] .backup_path: pg\path
[X]
```

### Explanation

**Path Component Extraction:**
```polyglot
[r] |Path.GetDirectory
[<] .path: pg\path << .source_file
[>] .directory: pg\path >> .dir

[r] |Path.GetFileName
[<] .path: pg\path << .source_file
[>] .filename: pg\string >> .name

[r] |Path.GetExtension
[<] .path: pg\path << .source_file
[>] .extension: pg\string >> .ext
```

For path `\\Data\\files\\report.txt`:
- Directory: `\\Data\\files\\`
- Filename: `report.txt`
- Extension: `.txt`

### Running

```bash
polyglot run CreateBackup --source_file "\\Data\\report.txt"
```

### Expected Output

```
\\Data\\report_backup_20250115_143022.txt
```

---

## Example 5: Batch File Processing

Process multiple files in batch with progress tracking.

### Use Case

Convert multiple CSV files to JSON format.

### Complete Code

**File:** `batch_convert.pg`

```polyglot
[@] Local@FileOps.BatchConvert:1.0.0
[#] 1
[X]





// Pipeline: Batch CSV to JSON conversion
[|] BatchCSVtoJSON

// Inputs
[i] .input_dir: pg\path
[i] .output_dir: pg\path

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// List all CSV files
[r] |Directory.ListFiles
[<] .path: pg\path << .input_dir
[<] .pattern: pg\string << "*.csv"
[>] .files: pg\string >> .csv_files

// Process batch
[r] |Run.Python
[<] .code: pg\string << """
import csv
import json
import os
from io import StringIO

files = [f.strip() for f in .csv_files.split(',') if f.strip()]
converted_count = 0
errors = []

for csv_path in files:
    try:
        # Read CSV
        with open(csv_path, 'r') as f:
            csv_content = f.read()

        # Parse CSV
        csv_reader = csv.DictReader(StringIO(csv_content))
        rows = list(csv_reader)

        # Convert to JSON
        json_data = json.dumps(rows, indent=2)

        # Create output path
        filename = os.path.basename(csv_path)
        json_filename = filename.replace('.csv', '.json')
        output_path = os.path.join(.output_dir, json_filename)

        # Write JSON
        with open(output_path, 'w') as f:
            f.write(json_data)

        converted_count += 1

    except Exception as e:
        errors.append(f"{csv_path}: {str(e)}")

# Generate summary
summary = f"Batch conversion complete\\n"
summary += f"Total files: {len(files)}\\n"
summary += f"Converted: {converted_count}\\n"
summary += f"Errors: {len(errors)}\\n"

if errors:
    summary += "\\nError details:\\n"
    summary += "\\n".join(errors)

result = summary
"""
[>] .result: pg\string >> .conversion_summary

// Output summary
[o] .conversion_summary: pg\string
[X]
```

### Running

```bash
polyglot run BatchCSVtoJSON \
  --input_dir "\\Data\\csv\\" \
  --output_dir "\\Data\\json\\"
```

### Expected Output

```
Batch conversion complete
Total files: 10
Converted: 10
Errors: 0
```

---

## Example 6: Serial Load Block - Parallel Configuration Loading

Load multiple configuration files in parallel using the `[s]` Serial Load Block.

### Use Case

Application startup needs to load multiple JSON configuration files. Use `[s]` blocks to load them all in parallel instead of sequentially for faster startup time.

### Complete Code

**File:** `serial_load_configs.pg`

```polyglot
[@] Local@FileOps.SerialLoadConfigs:1.0.0
[#] 1
[X]




// Pipeline: Load multiple configs in parallel
[|] LoadApplicationConfigs

// Inputs
[i] .config_dir: pg\path

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Serial Load: Load multiple configs in parallel
[s] .app_config << JSON"\\Config\\app.json"
[s] .db_config << JSON"\\Config\\database.json"
[s] .api_config << JSON"\\Config\\api.json"
[s] .cache_config << JSON"\\Config\\cache.json"

// Automatic join happens here - all loads complete before continuing

// Check for critical config errors
[s][!] !File.NotFound
[r] |U.Log.Error
[<] .msg: pg\string << "Critical config file missing"
[o] !ConfigurationError

[s][!] !JSON.ParseError
[r] |U.Log.Error
[<] .msg: pg\string << "Invalid JSON in config file"
[o] !ConfigurationError

// Use loaded configurations
[r] |Run.Python
[<] .code: pg\string << """
import json

# All configs are available as variables
app = .app_config if .app_config != "#None.ErrorState" else {}
db = .db_config if .db_config != "#None.ErrorState" else {}
api = .api_config if .api_config != "#None.ErrorState" else {}
cache = .cache_config if .cache_config != "#None.ErrorState" else {}

# Combine configurations
combined = {
    "application": app,
    "database": db,
    "api": api,
    "cache": cache,
    "loaded": {
        "app": app != {},
        "db": db != {},
        "api": api != {},
        "cache": cache != {}
    }
}

result = json.dumps(combined, indent=2)
"""
[>] .result: pg\string >> .merged_config

// Output merged configuration
[o] .merged_config: pg\string
[X]
```

### Explanation

**Serial Load Blocks:**
```polyglot
[s] .app_config << JSON"\\Config\\app.json"
[s] .db_config << JSON"\\Config\\database.json"
[s] .api_config << JSON"\\Config\\api.json"
[s] .cache_config << JSON"\\Config\\cache.json"
```
- All four files load **in parallel**
- Automatic join after all loads complete
- Much faster than sequential loading

**Scope-Level Error Handling:**
```polyglot
[s][!] !File.NotFound
[r] |U.Log.Error
[<] .msg: pg\string << "Critical config file missing"
```
- Catches `!File.NotFound` errors from **any** `[s]` block at this scope
- Applies to all four config loads
- Different error types can have different handlers

**Error-Carrying Variables:**
```polyglot
app = .app_config if .app_config != "#None.ErrorState" else {}
```
- Variables hold data on success
- Variables hold `#None.ErrorState` on failure
- Check `.error` field for error details
- Partial success: some files load, others fail

### Running

```bash
polyglot run LoadApplicationConfigs
```

### Expected Output (All Success)

```json
{
  "application": {"name": "MyApp", "version": "1.0"},
  "database": {"host": "localhost", "port": 5432},
  "api": {"endpoint": "https://api.example.com"},
  "cache": {"ttl": 3600},
  "loaded": {
    "app": true,
    "db": true,
    "api": true,
    "cache": true
  }
}
```

### Expected Output (Partial Failure)

If `api.json` is missing:
```
Error: Critical config file missing
Cause: !File.NotFound
```

**Performance:** 4x faster than sequential loading (4 files loaded concurrently).

---

## Example 7: Serial Load with Wildcards and Filters

Use wildcards to load multiple files matching a pattern with filtering.

### Use Case

Load all JSON plugin configurations from a directory, excluding test files.

### Complete Code

**File:** `serial_load_wildcard.pg`

```polyglot
[@] Local@FileOps.SerialLoadWildcard:1.0.0
[#] 1
[X]




// Pipeline: Load plugins with wildcard and filtering
[|] LoadPluginConfigs

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Serial Load: Load all plugin configs, exclude tests
[s] .plugins << JSON"\\Plugins\\*.json".ExcludeFileName"*test*"

// Automatic join after all matching files loaded

// Error handling for missing directory or parse errors
[s][!] !File.NotFound
[r] |U.Log.Warn
[<] .msg: pg\string << "Plugin directory not found, using defaults"

[s][!] !JSON.ParseError
[r] |U.Log.Error
[<] .msg: pg\string << "Invalid plugin configuration detected"

// Process loaded plugins
[r] |Run.Python
[<] .code: pg\string << """
import json

# .plugins contains merged plugin configurations
plugins = .plugins if .plugins != "#None.ErrorState" else {}

if isinstance(plugins, dict):
    plugin_count = len(plugins)
    plugin_names = list(plugins.keys())
    result = f"Loaded {plugin_count} plugins: {', '.join(plugin_names)}"
else:
    result = "No plugins loaded"
"""
[>] .result: pg\string >> .summary

[o] .summary: pg\string
[X]
```

### Explanation

**Wildcard with Chained Filter:**
```polyglot
[s] .plugins << JSON"\\Plugins\\*.json".ExcludeFileName"*test*"
```
- `*.json` - Matches all JSON files in directory
- `.ExcludeFileName"*test*"` - Filters out any file with "test" in name
- Loads: `plugin1.json`, `plugin2.json`
- Excludes: `plugin1.test.json`, `test-config.json`

**Combination Strategy:**
- Multiple files → merged into single object
- Default strategy: FilenameKey (keys based on filenames)
- Alternative: `.Index`, `.Merge`, `.Concat`, `.FlatMap`

### Running

```bash
polyglot run LoadPluginConfigs
```

### File Structure

```
\Plugins\
  ├── auth.json
  ├── logging.json
  ├── metrics.json
  └── auth.test.json    ← Excluded by filter
```

### Expected Output

```
Loaded 3 plugins: auth, logging, metrics
```

**Performance:** All matching files load in parallel. 10 plugins load as fast as 1 plugin.

---

## Example 8: Chained Literal Pipelines

Combine multiple transformations in serial load path.

### Use Case

Load environment-specific configurations with complex filtering and key extraction.

### Complete Code

**File:** `chained_literal_pipeline.pg`

```polyglot
[@] Local@FileOps.ChainedLiterals:1.0.0
[#] 1
[X]




// Pipeline: Load configs with chained transformations
[|] LoadEnvironmentConfigs

// Inputs
[i] .environment: #Environment  // #Production, #Development, #Testing

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Chained literal pipeline: Filter → Extract keys from filenames
[s] .env_configs << JSON"\\Config\\{.environment}\\*.json"
                     .ExcludeFileName"*.backup"
                     .ExcludeFileName"*.tmp"
                     .FilenameKey

// Load shared configs separately
[s] .shared << JSON"\\Config\\shared.json"

// Automatic join

// Error handling
[s][!] !File.NotFound
[r] |U.Log.Warn
[<] .msg: pg\string << "Some config files missing"

// Merge environment and shared configs
[r] |Run.Python
[<] .code: pg\string << """
import json

env = .env_configs if .env_configs != "#None.ErrorState" else {}
shared = .shared if .shared != "#None.ErrorState" else {}

# Merge: environment configs override shared
merged = {**shared, **env}

result = json.dumps({
    "environment": ".environment",
    "config_count": len(merged),
    "configs": merged
}, indent=2)
"""
[>] .result: pg\string >> .final_config

[o] .final_config: pg\string
[X]
```

### Explanation

**Chained Literal Pipeline:**
```polyglot
[s] .env_configs << JSON"\\Config\\{.environment}\\*.json"
                     .ExcludeFileName"*.backup"
                     .ExcludeFileName"*.tmp"
                     .FilenameKey
```
- `{.environment}` - Variable interpolation in path
- `.ExcludeFileName"*.backup"` - First filter
- `.ExcludeFileName"*.tmp"` - Second filter
- `.FilenameKey` - Use filename as object keys

**Transformation Chain:**
1. Load all `.json` files from environment directory
2. Exclude `.backup` files
3. Exclude `.tmp` files
4. Merge using FilenameKey strategy

### Running

```bash
polyglot run LoadEnvironmentConfigs --environment Production
```

### File Structure

```
\Config\
  ├── shared.json
  ├── Production\
  │   ├── database.json
  │   ├── api.json
  │   ├── database.backup     ← Excluded
  │   └── api.tmp             ← Excluded
  └── Development\
      ├── database.json
      └── api.json
```

### Expected Output

```json
{
  "environment": "Production",
  "config_count": 3,
  "configs": {
    "timeout": 30,
    "database": {"host": "prod.example.com"},
    "api": {"endpoint": "https://api.prod.example.com"}
  }
}
```

---

## Key Concepts

### 1. Path Type

**pg\path Type:**
```polyglot
[i] .file_path: pg\path
[i] .directory: pg\path
```
- Platform-independent path handling
- Supports both absolute and relative paths
- Windows: `\\Data\\file.txt`
- Unix: `/data/file.txt`

### 2. File I/O Operations

**Reading Files:**
```polyglot
[r] |File.ReadText     // Read as string
[r] |File.ReadBytes    // Read as binary
[r] |File.ReadLines    // Read as array of lines
```

**Writing Files:**
```polyglot
[r] |File.WriteText    // Write string
[r] |File.WriteBytes   // Write binary
[r] |File.AppendText   // Append to file
```

### 3. File Triggers

**File System Events:**
```polyglot
[t] |T.FileWatch
[<] .path: pg\path << .directory
[<] .pattern: pg\string << "*.csv"
[<] .event: pg\string << "create"
```

**Event Types:**
- `create` - New file created
- `modify` - File content changed
- `delete` - File removed
- `move` - File moved/renamed

### 4. Path Operations

**Standard Library Functions:**
```polyglot
|Path.GetDirectory    // Extract directory
|Path.GetFileName     // Extract filename
|Path.GetExtension    // Extract extension
|Path.Join            // Join path components
|Path.Exists          // Check if path exists
```

### 5. Serial Load Blocks (`[s]`)

**Purpose:** Load serialized data files in parallel with automatic synchronization

**Basic Syntax:**
```polyglot
[s] .variable << Format"path"
```

**Supported Formats:**
- `JSON"path"` - JSON files
- `YAML"path"` - YAML files
- `TOML"path"` - TOML files
- `XML"path"` - XML files

**Key Features:**
- **Parallel Execution:** All `[s]` blocks at same scope load concurrently
- **Automatic Join:** Runtime waits for all loads before continuing
- **Error-Carrying Variables:** Variables hold data OR `#None.ErrorState`
- **Scope-Level Error Handling:** `[s][!]` catches errors from all serial blocks
- **Wildcard Support:** `*.json` loads multiple files
- **Chained Filters:** `.ExcludeFileName"pattern"` for filtering
- **Combination Strategies:** `.FilenameKey`, `.Index`, `.Merge`, `.Concat`, `.FlatMap`

**Three-Step Execution:**
```
Step 1: Collect paths (expand wildcards, apply filters)
Step 2: Load in parallel (concurrent I/O)
Step 3: Assign to variables (automatic join)
```

**Performance:**
- Multiple files load as fast as single file
- Near-linear speedup (4 files ≈ 4x faster)
- Ideal for application startup, configuration loading

---

## Common Patterns

### Pattern 1: Read → Process → Write
```polyglot
[r] |File.ReadText
  → [r] |Run.Python  // Process
  → [r] |File.WriteText
```

### Pattern 2: File Watch → Auto-Process
```polyglot
[t] |T.FileWatch
  → [r] |Trigger.GetFilePath
  → [r] |File.ReadText
  → [r] |ProcessData
  → [r] |File.WriteText
```

### Pattern 3: Directory Scan → Batch Process
```polyglot
[r] |Directory.ListFiles
  → [r] |ForEach.File  // Process each
  → [r] |Aggregate.Results
```

### Pattern 4: Parallel Config Loading
```polyglot
[s] .config1 << JSON"file1.json"
[s] .config2 << JSON"file2.json"
[s] .config3 << JSON"file3.json"
// Automatic join
[s][!] !File.NotFound  // Handle errors
  → [r] |UseDefaults
[r] |MergeConfigs      // Use loaded configs
```

### Pattern 5: Wildcard Load with Filtering
```polyglot
[s] .files << JSON"*.json".ExcludeFileName"*test*"
// Loads all .json files except tests
[s][!] !JSON.ParseError
  → [r] |LogInvalidFiles
[r] |ProcessLoadedFiles
```

---

## Best Practices

1. **Always Use pg\path Type**
   - Platform-independent
   - Handles path separators correctly

2. **Validate File Existence**
   - Check before reading
   - Use `|Path.Exists` utility
   - Handle missing files gracefully

3. **Use File Triggers for Automation**
   - Monitor directories instead of polling
   - More efficient than scheduled checks
   - Real-time processing

4. **Handle File Errors**
   - Wrap file operations in error handling
   - Log failed file operations
   - Don't crash on single file failure

5. **Clean Up Temporary Files**
   - Use cleanup blocks `[c]`
   - Delete temp files after processing
   - Avoid filling disk space

6. **Use Serial Load Blocks for Multiple Files**
   - Load configs in parallel with `[s]` blocks
   - Much faster than sequential loading
   - Automatic error handling with `[s][!]`
   - Check for `#None.ErrorState` before using variables

7. **Leverage Wildcards and Filters**
   - Use `*.json` for batch loading
   - Chain filters: `.ExcludeFileName"*test*"`
   - Choose appropriate combination strategy

---

## File Path Formatting

**Windows Paths:**
```polyglot
[i] .file: pg\path << \\Data\\files\\report.txt
[i] .dir: pg\path << C:\\Users\\Alice\\Documents\\
```

**Unix Paths:**
```polyglot
[i] .file: pg\path << /data/files/report.txt
[i] .dir: pg\path << /home/alice/documents/
```

**Relative Paths:**
```polyglot
[i] .file: pg\path << ..\\data\\file.txt
[i] .current: pg\path << .\\config.json
```

**Special Path Variables:**
```polyglot
\\FileDir\\       // Directory of current .pg file
\\TempDir\\       // System temp directory
\\HomeDir\\       // User home directory
\\WorkDir\\       // Current working directory
```

---

## Next Steps

1. **Data Processing** - [Data Processing Examples](data-processing.md)
   - Process file contents
   - CSV/JSON transformation

2. **Error Handling** - [Error Handling Examples](error-handling.md)
   - File validation
   - Error recovery

3. **Complete Workflows** - [Complete Workflows](complete-workflows.md)
   - Production file processing patterns
   - Complex automation

---

## See Also

- [Examples Index](README.md) - All examples
- [File Utilities](../standard-library/utilities.md) - All file operations
- [Trigger Catalog](../standard-library/triggers.md) - All trigger types
- [Type System](../language/type-system.md) - pg\path type details
- [Serial Load Blocks](../language/block-markers.md#s---serial-load-block) - Complete `[s]` syntax reference
- [Serial Error Handling](../language/error-handling.md#serial-block-error-handling-s) - `[s][!]` error patterns
- [Runtime Execution](../architecture/runtime-execution.md#serial-load-block-execution) - Parallel execution model

---

**Last Updated:** 2025-11-19