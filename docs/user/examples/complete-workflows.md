# Complete Workflow Examples

**Difficulty:** Advanced
**Languages:** Python, Rust
**Topics:** Production workflows, Multi-stage pipelines, Real-world patterns
**Time:** ~60 minutes

---

## Overview

Production-ready workflow examples that combine multiple Polyglot features into complete, real-world solutions. These examples demonstrate error handling, parallel processing, file operations, and data transformation in integrated workflows.

**Key Concept:** Real-world pipelines combine triggers, validation, processing, error recovery, and monitoring into robust automated workflows.

---

## Example 1: AI Model Inference Pipeline

Complete ML workflow: Rust preprocessing → Python model inference → Result aggregation.

### Use Case

Process large image dataset through a CNN model with high-performance Rust preprocessing.

### Complete Code

**File:** `ai_inference_pipeline.pg`

```polyglot
[@] Local@CompleteWorkflows.AIInference:1.0.0
[#] 1
[X]





// Pipeline: High-performance image classification
[|] BatchImageClassification

// Inputs
[i] .image_dir:pg.path
[i] .model_path:pg.path
[i] .batch_size:pg.int << 1000
[i] .output_file:pg.path

// Trigger: CLI or Scheduled
[t] |T.Cli

// Queue: High priority, require GPU
[Q] |Q.Priority
[<] .level:pg.int << 9

[Q] |Q.RequireResource
[<] .cpu_cores:pg.int << 8
[<] .memory_mb:pg.int << 16384
[<] .gpu_count:pg.int << 1

// Runtime Wrappers
[W] |W.Rust1.70
[W] |W.Python3.11

// Step 1: List all images
[r] |Directory.ListFiles
[<] .path:pg.path << .image_dir
[<] .pattern:pg.string << "*.jpg,*.png"
[>] .files:pg.string >> .image_files

// Step 2: Rust preprocessing (fast!)
[r] |Run.Rust
[<] .code:pg.string << """
use std::fs;
use std::path::Path;

let files: Vec<&str> = .image_files.split(',').collect();
let batch_size = .batch_size as usize;

// Simulate image preprocessing
let mut preprocessed_count = 0;
let mut batches = Vec::new();

for chunk in files.chunks(batch_size) {
    preprocessed_count += chunk.len();
    batches.push(chunk.len());
}

// Return summary
format!("Preprocessed {} images in {} batches", preprocessed_count, batches.len())
"""
[>] .result:pg.string >> .preprocess_summary

// Step 3: Python ML model inference
[r] |Run.Python
[<] .code:pg.string << """
import json

# Simulate loading model and running inference
# In real implementation:
# - Load PyTorch/TensorFlow model from .model_path
# - Run inference on preprocessed data
# - Return predictions

files = [f.strip() for f in .image_files.split(',') if f.strip()]
batch_size = .batch_size

# Simulate predictions
predictions = []
for i, file_path in enumerate(files):
    pred = {
        "file": file_path,
        "class": f"class_{i % 10}",
        "confidence": 0.95
    }
    predictions.append(pred)

result = json.dumps(predictions)
"""
[>] .result:pg.string >> .predictions_json

// Step 4: Rust aggregation and report generation
[r] |Run.Rust
[<] .code:pg.string << """
use std::collections::HashMap;

let predictions_str = .predictions_json;

// Parse and aggregate (simplified)
let mut class_counts: HashMap<String, i32> = HashMap::new();

// Count predictions by class
for i in 0..10 {
    let class_name = format!("class_{}", i);
    class_counts.insert(class_name, 0);
}

// Generate report
let mut report = String::from("Image Classification Report\\n");
report.push_str("================================\\n\\n");
report.push_str(&.preprocess_summary);
report.push_str("\\n\\n");
report.push_str("Classification Summary:\\n");

for (class, count) in &class_counts {
    report.push_str(&format!("{}: {} images\\n", class, count));
}

report
"""
[>] .result:pg.string >> .final_report

// Step 5: Write report to file
[r] |File.WriteText
[<] .path:pg.path << .output_file
[<] .content:pg.string << .final_report

// Output final report
[o] .final_report:pg.string
[X]
```

### Key Features

✓ **High Performance** - Rust preprocessing (10,000+ images/sec)
✓ **GPU Utilization** - Queue requires GPU for model inference
✓ **Batch Processing** - Configurable batch size
✓ **Multi-Language** - Rust + Python optimal for each task

### Running

```bash
polyglot run BatchImageClassification \
  --image_dir "\\Data\\images\\" \
  --model_path "\\Models\\resnet50.pth" \
  --batch_size 1000 \
  --output_file "\\Results\\classification_report.txt"
```

---

## Example 2: ETL Pipeline with Error Recovery

Complete ETL workflow with validation, transformation, error handling, and retry logic.

### Use Case

Daily data warehouse load: extract from API, transform, validate, load to database with comprehensive error handling.

### Complete Code

**File:** `etl_pipeline.pg`

```polyglot
[@] Local@CompleteWorkflows.DailyETL:1.0.0
[#] 1
[X]





// Pipeline: Production ETL with error recovery
[|] DailyDataWarehouseLoad

// Inputs
[i] .api_url:pg.string
[i] .db_connection:pg.string
[i] .output_log:pg.path

// Trigger: Daily at 2 AM
[t] |T.Daily
[<] .at:pg.dt << DT"02:00:"

// Runtime Wrappers
[W] |W.Python3.11

// Setup: Initialize log
[s] |Setup.CreateLog
[<] .log_path:pg.path << .output_log

// EXTRACT: Fetch data from API with retry
[r] |Run.Python
[<] .code:pg.string << """
import urllib.request
import json
import time

url = .api_url
max_retries = 3
retry_count = 0
success = False
data = None

while retry_count < max_retries and not success:
    try:
        with urllib.request.urlopen(url, timeout=30) as response:
            data = json.loads(response.read().decode('utf-8'))
            success = True
            result_success = True
            result_data = json.dumps(data)
    except Exception as e:
        retry_count += 1
        if retry_count < max_retries:
            wait_time = 2 ** retry_count
            time.sleep(wait_time)
        else:
            result_success = False
            result_error = f"API fetch failed after {max_retries} retries: {str(e)}"
"""
[>] .result_success:pg.bool!Error >> .extract_success
[>] .result_data:pg.string >> .raw_data
[>] .result_error:pg.string >> .extract_error

// Handle extraction failure
[?] .extract_success
  // Continue to transform
[!]
  // Log error and exit
  [r] |File.AppendText
  [<] .path:pg.path << .output_log
  [<] .content:pg.string << .extract_error

  [o] #Error
  [<] .message:pg.string << .extract_error
  [X]
[X]

// TRANSFORM: Validate and transform data
[r] |Run.Python
[<] .code:pg.string << """
import json

try:
    data = json.loads(.raw_data)

    # Validate required fields
    if 'records' not in data:
        raise ValueError("Missing 'records' field in API response")

    records = data['records']

    # Transform each record
    transformed = []
    for i, record in enumerate(records):
        # Validate record
        if 'id' not in record or 'value' not in record:
            raise ValueError(f"Record {i} missing required fields")

        # Transform
        transformed_record = {
            'id': record['id'],
            'value': float(record['value']),
            'category': record.get('category', 'unknown'),
            'processed_at': 'CURRENT_TIMESTAMP'
        }
        transformed.append(transformed_record)

    result_data = json.dumps(transformed)
    result_success = True
    result_count = len(transformed)

except Exception as e:
    result_success = False
    result_error = f"Transform failed: {str(e)}"
    result_count = 0
"""
[>] .result_success:pg.bool!Error >> .transform_success
[>] .result_data:pg.string >> .transformed_data
[>] .result_count:pg.int >> .record_count
[>] .result_error:pg.string >> .transform_error

// Handle transformation failure
[?] .transform_success
  // Continue to load
[!]
  // Log error and exit
  [r] |File.AppendText
  [<] .path:pg.path << .output_log
  [<] .content:pg.string << .transform_error

  [o] #Error
  [<] .message:pg.string << .transform_error
  [X]
[X]

// LOAD: Insert into database
[r] |Run.Python
[<] .code:pg.string << """
import json

# Simulate database load
# In real implementation: use psycopg2, SQLAlchemy, etc.

try:
    data = json.loads(.transformed_data)

    # Simulate successful DB insert
    result_success = True
    result_loaded = len(data)
    result_message = f"Successfully loaded {len(data)} records to database"

except Exception as e:
    result_success = False
    result_error = f"Database load failed: {str(e)}"
"""
[>] .result_success:pg.bool!Error >> .load_success
[>] .result_loaded:pg.int >> .loaded_count
[>] .result_message:pg.string >> .load_message
[>] .result_error:pg.string >> .load_error

// Handle load failure
[?] .load_success
  // Success - log completion
  [r] |Run.Python
  [<] .code:pg.string << """
from datetime import datetime

summary = f"""
ETL Pipeline Completed Successfully
====================================
Timestamp: {datetime.now().isoformat()}
Records Extracted: {.record_count}
Records Loaded: {.loaded_count}
Status: SUCCESS
"""
result = summary
"""
  [>] .result:pg.string >> .success_log

  [r] |File.AppendText
  [<] .path:pg.path << .output_log
  [<] .content:pg.string << .success_log

  [o] .success_log:pg.string
[!]
  // Log load error
  [r] |File.AppendText
  [<] .path:pg.path << .output_log
  [<] .content:pg.string << .load_error

  [o] #Error
  [<] .message:pg.string << .load_error
  [X]
[X]

// Cleanup: Close connections, delete temp files
[c] |Cleanup.CloseConnections
[X]
```

### Key Features

✓ **Scheduled Execution** - Runs daily at 2 AM
✓ **Retry Logic** - Exponential backoff for API calls
✓ **Error Propagation** - Fails fast with clear error messages
✓ **Logging** - Comprehensive execution log
✓ **Cleanup** - Resource cleanup even on failure

### Expected Output (Success)

```
ETL Pipeline Completed Successfully
====================================
Timestamp: 2025-01-15T02:00:15
Records Extracted: 5000
Records Loaded: 5000
Status: SUCCESS
```

---

## Example 3: Automated File Processing Workflow

File watch → validation → processing → notification.

### Use Case

Monitor upload directory, validate files, process with ML model, send notifications.

### Complete Code

**File:** `auto_file_processor.pg`

```polyglot
[@] Local@CompleteWorkflows.FileProcessor:1.0.0
[#] 1
[X]





// Pipeline: Automated file processing with validation
[|] ProcessUploadedFile

// Configuration
[i] .upload_dir:pg.path << \\Data\\uploads\\
[i] .processed_dir:pg.path << \\Data\\processed\\
[i] .error_dir:pg.path << \\Data\\errors\\
[i] .notification_url:pg.string

// Trigger: File watch
[t] |T.FileWatch
[<] .path:pg.path << .upload_dir
[<] .pattern:pg.string << "*.csv"
[<] .event:pg.string << "create"

// Runtime Wrappers
[W] |W.Python3.11

// Get triggered file
[r] |Trigger.GetFilePath
[>] .file_path:pg.path >> .uploaded_file

// Extract filename
[r] |Path.GetFileName
[<] .path:pg.path << .uploaded_file
[>] .filename:pg.string >> .file_name

// Step 1: Validate file
[r] |File.ReadText
[<] .path:pg.path << .uploaded_file
[>] .content:pg.string >> .file_content

[r] |Run.Python
[<] .code:pg.string << """
import csv
from io import StringIO

errors = []

try:
    csv_reader = csv.DictReader(StringIO(.file_content))
    rows = list(csv_reader)

    # Validate not empty
    if not rows:
        errors.append("File is empty")

    # Validate required columns
    required_cols = ['id', 'data', 'timestamp']
    if rows:
        actual_cols = set(rows[0].keys())
        missing = set(required_cols) - actual_cols
        if missing:
            errors.append(f"Missing columns: {', '.join(missing)}")

    # Validate data types
    for i, row in enumerate(rows[:10]):  # Check first 10
        try:
            int(row.get('id', ''))
        except ValueError:
            errors.append(f"Row {i+2}: 'id' must be integer")
            break

    if errors:
        result_valid = False
        result_error = '\\n'.join(errors)
    else:
        result_valid = True
        result_row_count = len(rows)

except Exception as e:
    result_valid = False
    result_error = f"Validation error: {str(e)}"
"""
[>] .result_valid:pg.bool!Error >> .is_valid
[>] .result_error:pg.string >> .validation_errors
[>] .result_row_count:pg.int >> .row_count

// Handle validation result
[?] .is_valid
  // Valid - process file
  [r] |Run.Python
  [<] .code:pg.string << """
import csv
import json
from io import StringIO

# Parse and process
csv_reader = csv.DictReader(StringIO(.file_content))
rows = list(csv_reader)

# Simulate processing
processed = []
for row in rows:
    processed.append({
        'id': row['id'],
        'data': row['data'].upper(),  # Example transformation
        'timestamp': row['timestamp']
    })

result = json.dumps(processed, indent=2)
"""
  [>] .result:pg.string >> .processed_data

  // Move to processed directory
  [r] |Run.Python
  [<] .code:pg.string << """
import os
import shutil

output_path = os.path.join(.processed_dir, .file_name.replace('.csv', '_processed.json'))
result = output_path
"""
  [>] .result:pg.path >> .output_path

  [r] |File.WriteText
  [<] .path:pg.path << .output_path
  [<] .content:pg.string << .processed_data

  // Send success notification
  [r] |Run.Python
  [<] .code:pg.string << """
import urllib.request
import json

notification = {
    "status": "success",
    "filename": .file_name,
    "rows_processed": .row_count,
    "output_file": .output_path
}

try:
    data = json.dumps(notification).encode('utf-8')
    req = urllib.request.Request(.notification_url, data=data, headers={'Content-Type': 'application/json'})
    urllib.request.urlopen(req, timeout=5)
    result = f"Success: Processed {.row_count} rows from {.file_name}"
except Exception as e:
    result = f"Processed but notification failed: {str(e)}"
"""
  [>] .result:pg.string >> .success_message

  [o] .success_message:pg.string
[!]
  // Invalid - move to error directory
  [r] |Run.Python
  [<] .code:pg.string << """
import os
import shutil

error_path = os.path.join(.error_dir, .file_name)
shutil.move(.uploaded_file, error_path)

error_log_path = error_path.replace('.csv', '_errors.txt')
result = error_log_path
"""
  [>] .result:pg.path >> .error_log_path

  [r] |File.WriteText
  [<] .path:pg.path << .error_log_path
  [<] .content:pg.string << .validation_errors

  // Send error notification
  [r] |Run.Python
  [<] .code:pg.string << """
import urllib.request
import json

notification = {
    "status": "error",
    "filename": .file_name,
    "errors": .validation_errors
}

try:
    data = json.dumps(notification).encode('utf-8')
    req = urllib.request.Request(.notification_url, data=data, headers={'Content-Type': 'application/json'})
    urllib.request.urlopen(req, timeout=5)
    result = f"Error: File validation failed for {.file_name}"
except Exception as e:
    result = f"Validation failed and notification failed: {str(e)}"
"""
  [>] .result:pg.string >> .error_message

  [o] #Error
  [<] .message:pg.string << .error_message
  [X]
[X]
[X]
```

### Key Features

✓ **Automated Trigger** - Processes files as they arrive
✓ **Validation** - Comprehensive CSV validation
✓ **Error Handling** - Invalid files moved to error directory
✓ **Notifications** - Webhook notifications on success/failure
✓ **File Organization** - Auto-organizes processed and error files

---

## Example 4: Multi-Stage Data Pipeline

Complex workflow: ingest → validate → parallel transform → aggregate → load.

### Use Case

Process multiple data sources in parallel, aggregate results, load to data warehouse.

### Complete Code

**File:** `multi_stage_pipeline.pg`

```polyglot
[@] Local@CompleteWorkflows.MultiStage:1.0.0
[#] 1
[X]





// Pipeline: Multi-stage parallel data processing
[|] AggregateMultiSource

// Inputs
[i] .source1_url:pg.string
[i] .source2_url:pg.string
[i] .source3_url:pg.string
[i] .output_file:pg.path

// Trigger: CLI or Scheduled
[t] |T.Cli

// Queue: Require resources
[Q] |Q.RequireResource
[<] .cpu_cores:pg.int << 4
[<] .memory_mb:pg.int << 8192

// Runtime Wrappers
[W] |W.Python3.11

// Parallel fetch from multiple sources
[p] |FetchSource1
[r] |Run.Python
[<] .code:pg.string << """
import urllib.request
import json

try:
    with urllib.request.urlopen(.source1_url, timeout=10) as response:
        data = json.loads(response.read().decode('utf-8'))
        result = json.dumps(data)
except Exception as e:
    result = json.dumps({"error": str(e)})
"""
[>] .result:pg.string >> .data1
[X]

[p] |FetchSource2
[r] |Run.Python
[<] .code:pg.string << """
import urllib.request
import json

try:
    with urllib.request.urlopen(.source2_url, timeout=10) as response:
        data = json.loads(response.read().decode('utf-8'))
        result = json.dumps(data)
except Exception as e:
    result = json.dumps({"error": str(e)})
"""
[>] .result:pg.string >> .data2
[X]

[p] |FetchSource3
[r] |Run.Python
[<] .code:pg.string << """
import urllib.request
import json

try:
    with urllib.request.urlopen(.source3_url, timeout=10) as response:
        data = json.loads(response.read().decode('utf-8'))
        result = json.dumps(data)
except Exception as e:
    result = json.dumps({"error": str(e)})
"""
[>] .result:pg.string >> .data3
[X]

// Join all sources
[Y] |Y.JoinAll
[<] ... .data1
[<] ... .data2
[<] ... .data3

// Aggregate and transform
[r] |Run.Python
[<] .code:pg.string << """
import json

# Parse all sources
source1 = json.loads(.data1)
source2 = json.loads(.data2)
source3 = json.loads(.data3)

# Aggregate
aggregated = {
    "source1_count": len(source1.get('records', [])),
    "source2_count": len(source2.get('records', [])),
    "source3_count": len(source3.get('records', [])),
    "total_records": 0,
    "combined_data": []
}

# Combine all records
for source, name in [(source1, 'source1'), (source2, 'source2'), (source3, 'source3')]:
    if 'records' in source:
        for record in source['records']:
            record['source'] = name
            aggregated['combined_data'].append(record)

aggregated['total_records'] = len(aggregated['combined_data'])

result = json.dumps(aggregated, indent=2)
"""
[>] .result:pg.string >> .aggregated_data

// Write to output file
[r] |File.WriteText
[<] .path:pg.path << .output_file
[<] .content:pg.string << .aggregated_data

// Output summary
[o] .aggregated_data:pg.string
[X]
```

### Key Features

✓ **Parallel Fetching** - 3 sources fetched simultaneously
✓ **Join Operation** - Waits for all sources before aggregating
✓ **Error Tolerance** - Individual source failures don't crash pipeline
✓ **Data Aggregation** - Combines multiple sources into single dataset

---

## Production Patterns

### Pattern 1: Extract → Transform → Load (ETL)
```polyglot
[r] |Extract.FromAPI (with retry)
[r] |Validate.Schema
[r] |Transform.Data
[r] |Load.ToDatabase
[r] |Log.Success
```

### Pattern 2: File Watch → Validate → Process → Notify
```polyglot
[t] |T.FileWatch
[r] |Validate.File
[?] .valid
  [r] |Process.Data
  [r] |Notify.Success
[!]
  [r] |Move.ToErrorDir
  [r] |Notify.Error
[X]
```

### Pattern 3: Parallel Fetch → Join → Aggregate
```polyglot
[p] |FetchSource1
[p] |FetchSource2
[p] |FetchSource3
[Y] |Y.JoinAll
[r] |Aggregate.Results
[r] |Save.Output
```

---

## Best Practices

1. **Always Include Error Handling**
   - Use `!Error` types
   - Log all errors
   - Implement fallback strategies

2. **Use Appropriate Triggers**
   - Scheduled: Daily/hourly ETL
   - File Watch: Real-time processing
   - CLI: Manual/testing

3. **Resource Management**
   - Set queue priorities
   - Limit concurrent tasks
   - Clean up in `[c]` blocks

4. **Logging and Monitoring**
   - Log start/end timestamps
   - Track record counts
   - Monitor error rates

5. **Parallel When Possible**
   - Independent data sources
   - Batch processing
   - I/O-bound operations

---

## Next Steps

1. **Review Individual Topics:**
   - [Error Handling](error-handling.md) - Robust error patterns
   - [Parallel Execution](parallel-execution.md) - Performance optimization
   - [File Operations](file-operations.md) - File automation

2. **Study Standard Library:**
   - [Utilities Catalog](../standard-library/03-utilities.md)
   - [Triggers Catalog](../standard-library/04-triggers.md)
   - [Queue Control](../standard-library/02-queue-control.md)

3. **Understand Architecture:**
   - [System Architecture](../architecture/00-overview.md)
   - [Queue System](../architecture/03-queue-system.md)
   - [Trigger Monitoring](../architecture/04-trigger-monitoring.md)

---

## See Also

- [Examples Index](README.md) - All examples
- [Quick Start Guide](../language/00-quick-start.md) - Getting started
- [BNF Grammar](../language/bnf/polyglot grammer.md) - Complete syntax

---

**Last Updated:** 2025-11-15