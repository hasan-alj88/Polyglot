# Data Processing Examples

**Difficulty:** Intermediate
**Languages:** Python, Rust
**Topics:** ETL pipelines, File I/O, Data transformation, pg\ types
**Time:** ~30 minutes

---

## Overview

Learn how to process data files with Polyglot pipelines. These examples demonstrate reading, transforming, and writing data using Polyglot native types.

**Note on Type Conversion:** Cross-language type conversion (e.g., `rs\Vec<T>` → `py\list`) is planned for a future version. For now, all examples use `:pg.` (Polyglot native) types for data exchange between pipeline stages.

---

## Example 1: CSV Data Transformation

Read a CSV file, filter rows, and write the results to a new file.

### Use Case

You have a CSV file with customer data and need to extract only active customers with purchases over $100.

### Complete Code

**File:** `filter_customers.pg`

```polyglot
[@] Local@DataProcessing.FilterCustomers:1.0.0
[#] 1
[X]





// Pipeline: Filter active customers from CSV
[|] FilterActiveCustomers

// Inputs
[i] .input_file:pg.path
[i] .output_file:pg.path
[i] .min_purchase:pg.int << 100

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Read CSV file
[r] |File.ReadCSV
[<] .path:pg.path << .input_file
[>] .data:pg.string >> .csv_content

// Filter data in Python
[r] |Run.Python
[<] .code:pg.string << """
import csv
from io import StringIO

# Parse CSV
csv_reader = csv.DictReader(StringIO(.csv_content))
filtered = []

# Filter: status='active' AND purchase >= min_purchase
for row in csv_reader:
    if row['status'] == 'active' and int(row['purchase']) >= .min_purchase:
        filtered.append(row)

# Convert back to CSV string
output = StringIO()
if filtered:
    writer = csv.DictWriter(output, fieldnames=filtered[0].keys())
    writer.writeheader()
    writer.writerows(filtered)

result = output.getvalue()
"""
[>] .result:pg.string >> .filtered_csv

// Write filtered data to output file
[r] |File.WriteText
[<] .path:pg.path << .output_file
[<] .content:pg.string << .filtered_csv

// Output summary
[o] .filtered_csv:pg.string
[X]
```

### Explanation

**Reading CSV:**
```polyglot
[r] |File.ReadCSV
[<] .path:pg.path << .input_file
[>] .data:pg.string >> .csv_content
```
- Reads CSV file as a `:pg.string`
- Standard library utility `|File.ReadCSV`

**Processing Data:**
```polyglot
[r] |Run.Python
[<] .code:pg.string << """..."""
[>] .result:pg.string >> .filtered_csv
```
- Python code receives CSV as `:pg.string`
- Filters rows based on criteria
- Returns filtered CSV as `:pg.string`

**Writing Output:**
```polyglot
[r] |File.WriteText
[<] .path:pg.path << .output_file
[<] .content:pg.string << .filtered_csv
```
- Writes `:pg.string` to file

### Running

```bash
polyglot run FilterActiveCustomers \
  --input_file "customers.csv" \
  --output_file "active_customers.csv" \
  --min_purchase 100
```

### Input Data (`customers.csv`)

```csv
id,name,status,purchase
1,Alice,active,150
2,Bob,inactive,200
3,Charlie,active,50
4,Diana,active,300
```

### Expected Output (`active_customers.csv`)

```csv
id,name,status,purchase
1,Alice,active,150
4,Diana,active,300
```

---

## Example 2: Log File Processing

Parse log files, extract errors, and generate a summary report.

### Use Case

Process application logs to extract all ERROR entries and count occurrences by error type.

### Complete Code

**File:** `process_logs.pg`

```polyglot
[@] Local@DataProcessing.LogAnalysis:1.0.0
[#] 1
[X]





// Pipeline: Extract and summarize errors from logs
[|] AnalyzeLogs

// Inputs
[i] .log_file:pg.path
[i] .output_report:pg.path

// Trigger: CLI or FileWatch
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Read log file
[r] |File.ReadText
[<] .path:pg.path << .log_file
[>] .content:pg.string >> .log_content

// Parse logs and extract errors
[r] |Run.Python
[<] .code:pg.string << """
import re
from collections import Counter

# Parse log lines
log_lines = .log_content.split('\\n')
errors = []

# Extract ERROR lines: "2024-01-15 ERROR [module] message"
error_pattern = r'ERROR \\[([^\\]]+)\\] (.+)'

for line in log_lines:
    match = re.search(error_pattern, line)
    if match:
        module = match.group(1)
        message = match.group(2)
        errors.append({'module': module, 'message': message})

# Count errors by module
error_counts = Counter(e['module'] for e in errors)

# Generate report
report = f"Error Analysis Report\\n"
report += f"{'='*50}\\n"
report += f"Total Errors: {len(errors)}\\n\\n"
report += "Errors by Module:\\n"
for module, count in error_counts.most_common():
    report += f"  {module}: {count}\\n"

result = report
"""
[>] .result:pg.string >> .error_report

// Write report to file
[r] |File.WriteText
[<] .path:pg.path << .output_report
[<] .content:pg.string << .error_report

// Output report
[o] .error_report:pg.string
[X]
```

### Running

```bash
polyglot run AnalyzeLogs \
  --log_file "app.log" \
  --output_report "error_summary.txt"
```

### Input Data (`app.log`)

```
2024-01-15 10:30:00 INFO [main] Application started
2024-01-15 10:31:22 ERROR [database] Connection timeout
2024-01-15 10:32:45 ERROR [auth] Invalid credentials
2024-01-15 10:33:12 ERROR [database] Query failed
2024-01-15 10:34:00 INFO [main] Processing request
2024-01-15 10:35:10 ERROR [auth] Token expired
```

### Expected Output (`error_summary.txt`)

```
Error Analysis Report
==================================================
Total Errors: 4

Errors by Module:
  database: 2
  auth: 2
```

---

## Example 3: ETL Pipeline - Extract, Transform, Load

Complete ETL pipeline: read JSON, transform data, load to CSV.

### Use Case

Extract user data from JSON API response, transform fields, and load into CSV for reporting.

### Complete Code

**File:** `etl_pipeline.pg`

```polyglot
[@] Local@DataProcessing.ETLPipeline:1.0.0
[#] 1
[X]





// Pipeline: JSON to CSV ETL
[|] JSONtoCSV_ETL

// Inputs
[i] .input_json:pg.path
[i] .output_csv:pg.path

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// EXTRACT: Read JSON file
[r] |File.ReadText
[<] .path:pg.path << .input_json
[>] .content:pg.string >> .json_content

// TRANSFORM: Convert JSON to CSV format
[r] |Run.Python
[<] .code:pg.string << """
import json
import csv
from io import StringIO

# Parse JSON
data = json.loads(.json_content)

# Transform: Extract relevant fields and compute derived values
transformed = []
for user in data['users']:
    transformed.append({
        'id': user['id'],
        'full_name': f"{user['first_name']} {user['last_name']}",
        'email': user['email'],
        'age': user['age'],
        'account_value': user.get('balance', 0) + user.get('credits', 0)
    })

# Convert to CSV
output = StringIO()
if transformed:
    writer = csv.DictWriter(output, fieldnames=transformed[0].keys())
    writer.writeheader()
    writer.writerows(transformed)

result = output.getvalue()
"""
[>] .result:pg.string >> .csv_data

// LOAD: Write to CSV file
[r] |File.WriteText
[<] .path:pg.path << .output_csv
[<] .content:pg.string << .csv_data

// Output final CSV
[o] .csv_data:pg.string
[X]
```

### Running

```bash
polyglot run JSONtoCSV_ETL \
  --input_json "users.json" \
  --output_csv "users_report.csv"
```

### Input Data (`users.json`)

```json
{
  "users": [
    {
      "id": 1,
      "first_name": "Alice",
      "last_name": "Smith",
      "email": "alice@example.com",
      "age": 28,
      "balance": 1000,
      "credits": 50
    },
    {
      "id": 2,
      "first_name": "Bob",
      "last_name": "Jones",
      "email": "bob@example.com",
      "age": 35,
      "balance": 2500,
      "credits": 0
    }
  ]
}
```

### Expected Output (`users_report.csv`)

```csv
id,full_name,email,age,account_value
1,Alice Smith,alice@example.com,28,1050
2,Bob Jones,bob@example.com,35,2500
```

---

## Example 4: Rust High-Performance Data Processing

Use Rust for fast data processing with pg\ types.

### Use Case

Process large text file: count word frequencies using Rust's performance.

### Complete Code

**File:** `word_count.pg`

```polyglot
[@] Local@DataProcessing.WordCount:1.0.0
[#] 1
[X]





// Pipeline: High-performance word frequency counter
[|] CountWords

// Inputs
[i] .input_file:pg.path

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Rust1.70

// Read file
[r] |File.ReadText
[<] .path:pg.path << .input_file
[>] .content:pg.string >> .file_content

// Count words in Rust (fast!)
[r] |Run.Rust
[<] .code:pg.string << """
use std::collections::HashMap;

let text = .file_content;

// Split by whitespace and count
let mut word_counts: HashMap<String, usize> = HashMap::new();

for word in text.split_whitespace() {
    let word_lower = word.to_lowercase()
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_string();

    if !word_lower.is_empty() {
        *word_counts.entry(word_lower).or_insert(0) += 1;
    }
}

// Convert to sorted string output
let mut counts: Vec<_> = word_counts.iter().collect();
counts.sort_by(|a, b| b.1.cmp(a.1));

let mut result = String::from("Word Frequency Report\\n");
result.push_str(&format!("{'='*30}\\n"));

for (word, count) in counts.iter().take(10) {
    result.push_str(&format!("{}: {}\\n", word, count));
}

// Return as pg\string
result
"""
[>] .result:pg.string >> .word_report

// Output report
[o] .word_report:pg.string
[X]
```

### Running

```bash
polyglot run CountWords --input_file "document.txt"
```

### Expected Output

```
Word Frequency Report
==============================
the: 45
and: 32
data: 28
processing: 21
...
```

---

## Key Concepts

### Using pg\ Types for Data Exchange

**Current Version (v0.0.2):**
- Use `:pg.string` for text data exchange
- Use `:pg.int`, `:pg.bool` for simple values
- Use `:pg.path` for file paths

**Example:**
```polyglot
// Read returns pg\string
[r] |File.ReadText
[>] .content:pg.string >> .data

// Process with pg\string
[r] |Run.Python
[<] .input:pg.string << .data
[>] .result:pg.string >> .output
```

### Cross-Language Type Conversion (Future Feature)

**Status:** To Be Determined (TBD)

Future versions will support automatic type conversion:
```polyglot
// FUTURE: Direct type conversion
[r] |Run.Rust
[>] .data: rs\Vec<String> >> .rust_data

[r] |Run.Python
[<] .data: py\list << .rust_data  // Auto-convert rs\Vec → py\list
```

For now, serialize to `:pg.string` (JSON, CSV, etc.) for cross-language data exchange.

---

## ETL Pattern Summary

**Extract → Transform → Load:**

1. **Extract** - Read data from source
   ```polyglot
   [r] |File.ReadText
   [>] .content:pg.string >> .raw_data
   ```

2. **Transform** - Process data
   ```polyglot
   [r] |Run.Python  // or Run.Rust
   [<] .input:pg.string << .raw_data
   [>] .result:pg.string >> .transformed_data
   ```

3. **Load** - Write to destination
   ```polyglot
   [r] |File.WriteText
   [<] .content:pg.string << .transformed_data
   ```

---

## Performance Tips

1. **Use Rust for Heavy Processing**
   - Large file parsing
   - Complex string operations
   - High-frequency counting/aggregation

2. **Use Python for Flexibility**
   - JSON/CSV libraries
   - Complex data transformations
   - Quick prototyping

3. **Use pg\string for Exchange**
   - Serialize complex data to JSON
   - Use CSV for tabular data
   - Keep data as strings between stages

---

## Common Patterns

### Pattern 1: File → Process → File
```polyglot
[r] |File.ReadText → [r] |Run.Python → [r] |File.WriteText
```

### Pattern 2: Multiple Transformations
```polyglot
[r] |File.ReadText
  → [r] |Run.Rust     // Fast filtering
  → [r] |Run.Python   // Complex analysis
  → [r] |File.WriteText
```

### Pattern 3: Scheduled ETL
```polyglot
[t] |T.Daily
[<] .at:pg.dt << |DT"02:00:"

[r] |File.ReadText
[r] |Run.Python
[r] |File.WriteText
```

---

## Next Steps

1. **Error Handling** - [Error Handling Examples](error-handling.md)
   - Validate data with error types
   - Handle malformed files

2. **Parallel Processing** - [Parallel Execution Examples](parallel-execution.md)
   - Process multiple files concurrently
   - Partition large datasets

3. **File Operations** - [File Operations Examples](file-operations.md)
   - File watching and automation
   - Advanced file I/O patterns

---

## See Also

- [Hello World Examples](hello-world.md) - Basic syntax
- [Type System Documentation](../language/02-type-system.md) - All pg\ types
- [File Operations Utilities](../standard-library/03-utilities.md) - File I/O functions
- [Examples Index](README.md) - All examples

---

**Last Updated:** 2025-11-15
**Note:** Cross-language type conversion is planned for future versions. Current examples use pg\ types only.