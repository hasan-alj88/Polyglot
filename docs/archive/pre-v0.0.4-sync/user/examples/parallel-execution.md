---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/examples/parallel-execution.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Parallel Execution Examples

**Difficulty:** Intermediate to Advanced
**Languages:** Python, Rust
**Topics:** Fork/join patterns, Concurrent processing, Resource optimization
**Time:** ~35 minutes

---

## Overview

Learn how to execute multiple tasks concurrently using Polyglot's parallel execution features. These examples demonstrate fork/join patterns, parallel data processing, and performance optimization.

**Key Concept:** Use `[p]` blocks for parallel execution and `[Y]` (join) to collect results.

---

## Example 1: Basic Parallel Processing

Execute multiple independent tasks concurrently.

### Use Case

Process three independent data files simultaneously instead of sequentially.

### Complete Code

**File:** `basic_parallel.pg`

```polyglot
[@] Local@Parallel.BasicForkJoin:1.0.0
[#] 1
[X]





// Pipeline: Process multiple files in parallel
[|] ProcessFilesParallel

// Inputs
[i] .file1:pg.path
[i] .file2:pg.path
[i] .file3:pg.path

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Fork: Process files in parallel
[p] |File.ReadText
[<] .path:pg.path << .file1
[>] .content:pg.string >> .data1
[~]
[~][r] |Run.Python
[~][<] .code:pg.string << """
result = len(.data1.split('\\n'))
"""
[>] .result:pg.int >> .lines1
[X]

[p] |File.ReadText
[<] .path:pg.path << .file2
[>] .content:pg.string >> .data2
[~]
[~][r] |Run.Python
[~][<] .code:pg.string << "
[~][*] +"result = len(.data2.split('\\n'))"
[~][>] .result:pg.int >> .lines2
[~][X]

[p] |File.ReadText
[<] .path:pg.path << .file3
[>] .content:pg.string >> .data3
[~]
[~][r] |Run.Python
[~][<] .code:pg.string << "
[~][*] +"result = len(.data3.split('\\n'))"
[~][>] .result:pg.int >> .lines3
[~][X]

// Join: Wait for all parallel tasks to complete
[Y] |Y.JoinAll
[<] ... .lines1
[<] ... .lines2
[<] ... .lines3

// Combine results
[r] |Run.Python
[<] .code:pg.string << "
[~][*] +"total = .lines1 + .lines2 + .lines3"
[~][*] +"result = f"Total lines: {total} (File1: {."[~][*] +"lines1}, File2: {.lines2}, File3: {.[~][*] "
[~][*]+"lines3})"

[>] .result:pg.string >> .summary

[o] .summary:pg.string
[X]
```

### Explanation

**Parallel Blocks:**
```polyglot
[p] |ProcessFile1
  // Task 1 code
[X]

[p] |ProcessFile2
  // Task 2 code
[X]

[p] |ProcessFile3
  // Task 3 code
[X]
```
- Each `[p]` block executes concurrently
- Tasks run independently in parallel

**Join Operation:**
```polyglot
[Y] |Y.JoinAll
[<] ... .lines1
[<] ... .lines2
[<] ... .lines3
```
- `[Y]` waits for all parallel tasks to complete
- `...` expansion operator collects results
- Pipeline continues only after all tasks finish

### Running

```bash
polyglot run ProcessFilesParallel \
  --file1 "data1.txt" \
  --file2 "data2.txt" \
  --file3 "data3.txt"
```

### Expected Output

```
Total lines: 1500 (File1: 500, File2: 600, File3: 400)
```

**Performance:** 3x faster than sequential processing (assuming I/O-bound tasks).

---

## Example 2: Parallel Data Partitioning

Split a large dataset into chunks and process them in parallel.

### Use Case

Process a large CSV file by splitting it into partitions and processing each partition concurrently.

### Complete Code

**File:** `parallel_partitions.pg`

```polyglot
[@] Local@Parallel.DataPartitioning:1.0.0
[#] 1
[X]





// Pipeline: Partition and process data in parallel
[|] PartitionAndProcess

// Inputs
[i] .input_file:pg.path
[i] .partition_count:pg.int << 4

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Read and partition data
[r] |File.ReadText
[<] .path:pg.path << .input_file
[>] .content:pg.string >> .file_content

[r] |Run.Python
[<] .code:pg.string << """
import csv
from io import StringIO

# Parse CSV
csv_reader = csv.DictReader(StringIO(.file_content))
rows = list(csv_reader)

# Partition into chunks
partition_count = .partition_count
chunk_size = len(rows) // partition_count
partitions = []

for i in range(partition_count):
    start = i * chunk_size
    end = start + chunk_size if i < partition_count - 1 else len(rows)
    partition = rows[start:end]

    # Convert partition back to CSV string
    output = StringIO()
    if partition:
        writer = csv.DictWriter(output, fieldnames=partition[0].keys())
        writer.writeheader()
        writer.writerows(partition)
        partitions.append(output.getvalue())

result_p1 = partitions[0] if len(partitions) > 0 else ""
result_p2 = partitions[1] if len(partitions) > 1 else ""
result_p3 = partitions[2] if len(partitions) > 2 else ""
result_p4 = partitions[3] if len(partitions) > 3 else ""
"""
[>] .result_p1:pg.string >> .partition1
[>] .result_p2:pg.string >> .partition2
[>] .result_p3:pg.string >> .partition3
[>] .result_p4:pg.string >> .partition4

// Process partitions in parallel
[p] |ProcessPartition1
[r] |Run.Python
[<] .code:pg.string << """
import csv
from io import StringIO

csv_reader = csv.DictReader(StringIO(.partition1))
rows = list(csv_reader)

# Example: Calculate sum of 'amount' column
total = sum(int(row.get('amount', 0)) for row in rows)
result = total
"""
[>] .result:pg.int >> .sum1
[X]

[p] |ProcessPartition2
[r] |Run.Python
[<] .code:pg.string << """
import csv
from io import StringIO

csv_reader = csv.DictReader(StringIO(.partition2))
rows = list(csv_reader)
total = sum(int(row.get('amount', 0)) for row in rows)
result = total
"""
[>] .result:pg.int >> .sum2
[X]

[p] |ProcessPartition3
[r] |Run.Python
[<] .code:pg.string << """
import csv
from io import StringIO

csv_reader = csv.DictReader(StringIO(.partition3))
rows = list(csv_reader)
total = sum(int(row.get('amount', 0)) for row in rows)
result = total
"""
[>] .result:pg.int >> .sum3
[X]

[p] |ProcessPartition4
[r] |Run.Python
[<] .code:pg.string << """
import csv
from io import StringIO

csv_reader = csv.DictReader(StringIO(.partition4))
rows = list(csv_reader)
total = sum(int(row.get('amount', 0)) for row in rows)
result = total
"""
[>] .result:pg.int >> .sum4
[X]

// Join and aggregate results
[Y] |Y.JoinAll
[<] ... .sum1
[<] ... .sum2
[<] ... .sum3
[<] ... .sum4

[r] |Run.Python
[<] .code:pg.string << """
grand_total = .sum1 + .sum2 + .sum3 + .sum4
result = f"Grand total: {grand_total}"
"""
[>] .result:pg.string >> .final_result

[o] .final_result:pg.string
[X]
```

### Running

```bash
polyglot run PartitionAndProcess \
  --input_file "transactions.csv" \
  --partition_count 4
```

### Expected Output

```
Grand total: 1250000
```

**Performance:** 4x faster on multi-core systems with CPU-bound processing.

---

## Example 3: Concurrent API Calls

Make multiple API requests in parallel instead of sequentially.

### Use Case

Fetch data from 3 different API endpoints simultaneously.

### Complete Code

**File:** `parallel_api_calls.pg`

```polyglot
[@] Local@Parallel.ConcurrentAPIs:1.0.0
[#] 1
[X]





// Pipeline: Fetch from multiple APIs in parallel
[|] FetchAPIsParallel

// Inputs
[i] .api1_url:pg.string
[i] .api2_url:pg.string
[i] .api3_url:pg.string

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Parallel API calls
[p] |FetchAPI1
[r] |Run.Python
[<] .code:pg.string << """
import urllib.request
import json

try:
    with urllib.request.urlopen(.api1_url, timeout=10) as response:
        data = json.loads(response.read().decode('utf-8'))
        result = json.dumps(data)
except Exception as e:
    result = json.dumps({"error": str(e)})
"""
[>] .result:pg.string >> .api1_data
[X]

[p] |FetchAPI2
[r] |Run.Python
[<] .code:pg.string << """
import urllib.request
import json

try:
    with urllib.request.urlopen(.api2_url, timeout=10) as response:
        data = json.loads(response.read().decode('utf-8'))
        result = json.dumps(data)
except Exception as e:
    result = json.dumps({"error": str(e)})
"""
[>] .result:pg.string >> .api2_data
[X]

[p] |FetchAPI3
[r] |Run.Python
[<] .code:pg.string << """
import urllib.request
import json

try:
    with urllib.request.urlopen(.api3_url, timeout=10) as response:
        data = json.loads(response.read().decode('utf-8'))
        result = json.dumps(data)
except Exception as e:
    result = json.dumps({"error": str(e)})
"""
[>] .result:pg.string >> .api3_data
[X]

// Join all API responses
[Y] |Y.JoinAll
[<] ... .api1_data
[<] ... .api2_data
[<] ... .api3_data

// Combine results
[r] |Run.Python
[<] .code:pg.string << """
import json

combined = {
    "api1": json.loads(.api1_data),
    "api2": json.loads(.api2_data),
    "api3": json.loads(.api3_data)
}

result = json.dumps(combined, indent=2)
"""
[>] .result:pg.string >> .combined_data

[o] .combined_data:pg.string
[X]
```

### Running

```bash
polyglot run FetchAPIsParallel \
  --api1_url "https://api.example.com/users" \
  --api2_url "https://api.example.com/products" \
  --api3_url "https://api.example.com/orders"
```

### Expected Output

```json
{
  "api1": {"users": [...]},
  "api2": {"products": [...]},
  "api3": {"orders": [...]}
}
```

**Performance:** 3x faster than sequential API calls (network I/O bound).

---

## Example 4: Parallel Processing with Resource Limits

Control parallelism with resource constraints.

### Use Case

Process multiple files in parallel but limit CPU and memory usage.

### Complete Code

**File:** `parallel_with_limits.pg`

```polyglot
[@] Local@Parallel.ResourceLimits:1.0.0
[#] 1
[X]





// Pipeline: Parallel processing with resource constraints
[|] ParallelWithResourceLimits

// Inputs
[i] .files:pg.string  // Comma-separated file paths

// Trigger: CLI
[t] |T.Cli

// Queue: Limit concurrent tasks
[Q] |Q.RequireResource
[<] .cpu_cores:pg.int << 2
[<] .memory_mb:pg.int << 4096
[<] .max_concurrent:pg.int << 4

// Runtime Wrappers
[W] |W.Python3.11
[W] |W.Rust1.70

// Parse file list
[r] |Run.Python
[<] .code:pg.string << """
files = .files.split(',')
result_f1 = files[0].strip() if len(files) > 0 else ""
result_f2 = files[1].strip() if len(files) > 1 else ""
result_f3 = files[2].strip() if len(files) > 2 else ""
result_f4 = files[3].strip() if len(files) > 3 else ""
"""
[>] .result_f1:pg.path >> .file1
[>] .result_f2:pg.path >> .file2
[>] .result_f3:pg.path >> .file3
[>] .result_f4:pg.path >> .file4

// Process files in parallel (max 4 concurrent)
[p] |ProcessFile1
[r] |File.ReadText
[<] .path:pg.path << .file1
[>] .content:pg.string >> .content1

[r] |Run.Rust
[<] .code:pg.string << """
let text = .content1;
let word_count = text.split_whitespace().count();
format!("{}", word_count)
"""
[>] .result:pg.string >> .count1
[X]

[p] |ProcessFile2
[r] |File.ReadText
[<] .path:pg.path << .file2
[>] .content:pg.string >> .content2

[r] |Run.Rust
[<] .code:pg.string << """
let text = .content2;
let word_count = text.split_whitespace().count();
format!("{}", word_count)
"""
[>] .result:pg.string >> .count2
[X]

[p] |ProcessFile3
[r] |File.ReadText
[<] .path:pg.path << .file3
[>] .content:pg.string >> .content3

[r] |Run.Rust
[<] .code:pg.string << """
let text = .content3;
let word_count = text.split_whitespace().count();
format!("{}", word_count)
"""
[>] .result:pg.string >> .count3
[X]

[p] |ProcessFile4
[r] |File.ReadText
[<] .path:pg.path << .file4
[>] .content:pg.string >> .content4

[r] |Run.Rust
[<] .code:pg.string << """
let text = .content4;
let word_count = text.split_whitespace().count();
format!("{}", word_count)
"""
[>] .result:pg.string >> .count4
[X]

// Join results
[Y] |Y.JoinAll
[<] ... .count1
[<] ... .count2
[<] ... .count3
[<] ... .count4

[r] |Run.Python
[<] .code:pg.string << """
total = int(.count1) + int(.count2) + int(.count3) + int(.count4)
result = f"Total words: {total}"
"""
[>] .result:pg.string >> .summary

[o] .summary:pg.string
[X]
```

### Explanation

**Resource Constraints:**
```polyglot
[Q] |Q.RequireResource
[<] .cpu_cores:pg.int << 2
[<] .memory_mb:pg.int << 4096
[<] .max_concurrent:pg.int << 4
```
- Limits to 2 CPU cores
- Maximum 4096 MB memory
- Maximum 4 concurrent tasks

### Running

```bash
polyglot run ParallelWithResourceLimits \
  --files "doc1.txt,doc2.txt,doc3.txt,doc4.txt"
```

### Expected Output

```
Total words: 15432
```

---

## Example 5: Serial Load vs Parallel Blocks

Compare `[s]` Serial Load (automatic join) with `[p]` Parallel blocks (manual join).

### Use Case

Load multiple data files. Understand when to use automatic serial load vs manual parallel execution.

### Complete Code - Serial Load (Automatic Join)

**File:** `serial_load_auto_join.pg`

```polyglot
[@] Local@Parallel.SerialLoadAutoJoin:1.0.0
[#] 1
[X]




// Pipeline: Serial load with automatic join
[|] LoadWithSerialBlocks

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Serial Load: All loads execute in parallel, automatic join
[s] .users << JSON"\\Data\\users.json"
[s] .products << JSON"\\Data\\products.json"
[s] .orders << JSON"\\Data\\orders.json"

// Automatic join happens HERE
// All three files loaded before continuing

// Error handling (scope-level)
[s][!] !File.NotFound
[r] |U.Log.Error
[<] .msg:pg.string << "Data file not found"

[s][!] !JSON.ParseError
[r] |U.Log.Error
[<] .msg:pg.string << "Invalid JSON in data file"

// Process loaded data
[r] |Run.Python
[<] .code:pg.string << """
import json

users = .users if .users != "#None.ErrorState" else []
products = .products if .products != "#None.ErrorState" else []
orders = .orders if .orders != "#None.ErrorState" else []

result = json.dumps({
    "users_count": len(users),
    "products_count": len(products),
    "orders_count": len(orders),
    "total_records": len(users) + len(products) + len(orders)
}, indent=2)
"""
[>] .result:pg.string >> .summary

[o] .summary:pg.string
[X]
```

### Complete Code - Parallel Blocks (Manual Join)

**File:** `parallel_manual_join.pg`

```polyglot
[@] Local@Parallel.ParallelManualJoin:1.0.0
[#] 1
[X]




// Pipeline: Parallel execution with manual join
[|] LoadWithParallelBlocks

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Must declare variables BEFORE parallel blocks
[r] .users:pg.string << ""
[r] .products:pg.string << ""
[r] .orders:pg.string << ""

// Parallel execution: Load files concurrently
[p] |LoadUsers
[r] |File.ReadText
[<] .path:pg.path << "\\Data\\users.json"
[>] .content:pg.string >> users  // Assign to pre-declared variable
[X]

[p] |LoadProducts
[r] |File.ReadText
[<] .path:pg.path << "\\Data\\products.json"
[>] .content:pg.string >> products
[X]

[p] |LoadOrders
[r] |File.ReadText
[<] .path:pg.path << "\\Data\\orders.json"
[>] .content:pg.string >> orders
[X]

// MANUAL join required
[Y] |Y.JoinAll
[<] ... users
[<] ... products
[<] ... orders

// Parse JSON (sequential, after join)
[r] |Run.Python
[<] .code:pg.string << """
import json

try:
    users_data = json.loads(.users) if .users else []
except:
    users_data = []

try:
    products_data = json.loads(.products) if .products else []
except:
    products_data = []

try:
    orders_data = json.loads(.orders) if .orders else []
except:
    orders_data = []

result = json.dumps({
    "users_count": len(users_data),
    "products_count": len(products_data),
    "orders_count": len(orders_data),
    "total_records": len(users_data) + len(products_data) + len(orders_data)
}, indent=2)
"""
[>] .result:pg.string >> .summary

[o] .summary:pg.string
[X]
```

### Comparison

| Feature | `[s]` Serial Load | `[p]` Parallel Blocks |
|---------|-------------------|----------------------|
| **Join** | Automatic | Manual `[Y]` required |
| **Variable Declaration** | Inline with load | Must pre-declare |
| **Error Handling** | `[s][!]` scope-level | Manual per block |
| **Data Parsing** | Automatic (JSON/YAML/etc) | Manual parsing needed |
| **Error State** | `#None.ErrorState` | No built-in error state |
| **Wildcards** | Supported (`*.json`) | Not supported |
| **Filters** | Chained (`.ExcludeFileName`) | Not supported |
| **Combination** | Merge strategies | Manual aggregation |
| **Code Complexity** | Low (5-10 lines) | High (20-30 lines) |

### When to Use Each

**Use `[s]` Serial Load when:**
- ✅ Loading serialized data files (JSON, YAML, TOML, XML)
- ✅ Need automatic parsing and error handling
- ✅ Want simplest code with automatic join
- ✅ Loading multiple configs at startup
- ✅ Using wildcards or filters

**Use `[p]` Parallel Blocks when:**
- ✅ Need custom processing logic per task
- ✅ Tasks are not file loading (API calls, computations, etc.)
- ✅ Need fine-grained control over synchronization
- ✅ Complex fan-out/fan-in patterns
- ✅ Manual result aggregation required

### Example Output (Both)

```json
{
  "users_count": 150,
  "products_count": 200,
  "orders_count": 500,
  "total_records": 850
}
```

### Performance

**Both approaches have similar performance:**
- 3 files load in parallel
- ~3x faster than sequential
- Serial load slightly faster (automatic parsing, optimized runtime)

**Serial Load advantages:**
- Less code to write
- Automatic error handling
- Built-in parsing
- Wildcard support

---

## Example 6: Hybrid Approach - Serial Load + Parallel Processing

Combine serial load for data and parallel blocks for processing.

### Use Case

Load configuration files using serial blocks, then process data using parallel execution.

### Complete Code

**File:** `hybrid_serial_parallel.pg`

```polyglot
[@] Local@Parallel.HybridApproach:1.0.0
[#] 1
[X]




// Pipeline: Serial load data, then parallel process
[|] HybridDataProcessing

// Trigger: CLI
[t] |T.Cli

// Runtime Wrappers
[W] |W.Python3.11

// Phase 1: Serial Load (automatic join)
[s] .raw_data1 << JSON"\\Data\\dataset1.json"
[s] .raw_data2 << JSON"\\Data\\dataset2.json"
[s] .raw_data3 << JSON"\\Data\\dataset3.json"

// Automatic join after serial loads

// Error handling
[s][!] !File.NotFound
[r] |U.Log.Error
[<] .msg:pg.string << "Dataset file missing"
[o] !DataError

// Phase 2: Parallel Processing (manual join)
[p] |ProcessDataset1
[r] |Run.Python
[<] .code:pg.string << """
import json

data = .raw_data1 if .raw_data1 != "#None.ErrorState" else []
# Heavy computation
processed = [item for item in data if item.get('active', False)]
result = len(processed)
"""
[>] .result:pg.int >> .count1
[X]

[p] |ProcessDataset2
[r] |Run.Python
[<] .code:pg.string << """
import json

data = .raw_data2 if .raw_data2 != "#None.ErrorState" else []
processed = [item for item in data if item.get('active', False)]
result = len(processed)
"""
[>] .result:pg.int >> .count2
[X]

[p] |ProcessDataset3
[r] |Run.Python
[<] .code:pg.string << """
import json

data = .raw_data3 if .raw_data3 != "#None.ErrorState" else []
processed = [item for item in data if item.get('active', False)]
result = len(processed)
"""
[>] .result:pg.int >> .count3
[X]

// Manual join for processing results
[Y] |Y.JoinAll
[<] ... .count1
[<] ... .count2
[<] ... .count3

// Aggregate results
[r] |Run.Python
[<] .code:pg.string << """
total = .count1 + .count2 + .count3
result = f"Total active records: {total}"
"""
[>] .result:pg.string >> .final_result

[o] .final_result:pg.string
[X]
```

### Explanation

**Phase 1 - Serial Load:**
```polyglot
[s] .raw_data1 << JSON"\\Data\\dataset1.json"
[s] .raw_data2 << JSON"\\Data\\dataset2.json"
[s] .raw_data3 << JSON"\\Data\\dataset3.json"
// Automatic join
```
- Load all data files in parallel
- Automatic JSON parsing
- Automatic join before next phase
- Simple error handling with `[s][!]`

**Phase 2 - Parallel Processing:**
```polyglot
[p] |ProcessDataset1 → .count1
[p] |ProcessDataset2 → .count2
[p] |ProcessDataset3 → .count3
[Y] |Y.JoinAll
```
- Process loaded data in parallel
- Custom processing logic per dataset
- Manual join to collect results
- Maximum performance for CPU-bound work

**Hybrid Benefits:**
- **Best of both worlds**
- Serial load for I/O (automatic join, error handling)
- Parallel blocks for computation (custom logic, control)

### Expected Output

```
Total active records: 1250
```

---

## Key Concepts

### 1. Parallel Block Markers

**Parallel Execution:**
```polyglot
[p] |TaskName
  // Task code
[X]
```
- `[p]` marks a parallel task
- Tasks execute concurrently
- Each task is independent

### 2. Join Operations

**Wait for All:**
```polyglot
[Y] |Y.JoinAll
[<] ... .result1
[<] ... .result2
[<] ... .result3
```
- `[Y]` waits for all parallel tasks
- `...` expansion operator collects results
- Blocks until all tasks complete

### 3. Expansion Operator

**Collect Results:**
```polyglot
[<] ... .variable
```
- `...` prefix collects values from parallel tasks
- Used in join operations
- Aggregates parallel outputs

---

## Performance Patterns

### Pattern 1: I/O-Bound Tasks (File, Network)
**Best for:** File reading, API calls, database queries

**Speedup:** Near-linear with task count (3 tasks ≈ 3x faster)

```polyglot
[p] |ReadFile1
[p] |ReadFile2
[p] |ReadFile3
[Y] |Y.JoinAll
```

### Pattern 2: CPU-Bound Tasks (Computation)
**Best for:** Data processing, calculations, parsing

**Speedup:** Limited by CPU cores (4 cores ≈ 4x max)

```polyglot
[Q] |Q.RequireResource
[<] .cpu_cores:pg.int << 4

[p] |ProcessData1
[p] |ProcessData2
[p] |ProcessData3
[p] |ProcessData4
[Y] |Y.JoinAll
```

### Pattern 3: Mixed Workload
**Best for:** Some I/O, some CPU work

**Strategy:** Partition by bottleneck type

```polyglot
// I/O phase (parallel)
[p] |FetchData1
[p] |FetchData2
[Y] |Y.JoinAll

// CPU phase (parallel with limits)
[Q] |Q.RequireResource
[<] .cpu_cores:pg.int << 4

[p] |ProcessData1
[p] |ProcessData2
[Y] |Y.JoinAll
```

---

## Common Patterns

### Pattern 1: Map-Reduce
```polyglot
// Map: Process partitions in parallel
[p] |MapPartition1 → .result1
[p] |MapPartition2 → .result2
[p] |MapPartition3 → .result3

// Reduce: Aggregate results
[Y] |Y.JoinAll
[r] |CombineResults
```

### Pattern 2: Fan-out, Fan-in
```polyglot
// Fan-out: Broadcast to multiple workers
[p] |Worker1
[p] |Worker2
[p] |Worker3

// Fan-in: Collect and merge
[Y] |Y.JoinAll
[r] |MergeResults
```

### Pattern 3: Pipeline Parallelism
```polyglot
// Stage 1: Parallel preprocessing
[p] |Preprocess1
[p] |Preprocess2
[Y] |Y.JoinAll

// Stage 2: Sequential processing
[r] |MainProcess

// Stage 3: Parallel postprocessing
[p] |Postprocess1
[p] |Postprocess2
[Y] |Y.JoinAll
```

---

## Best Practices

1. **Use Parallel for Independent Tasks**
   - Tasks must not depend on each other
   - No shared state between parallel blocks

2. **Limit Parallelism for CPU-Bound Work**
   - Use `[Q] |Q.RequireResource` to set limits
   - Match parallelism to available CPU cores

3. **Always Use Join**
   - Never skip `[Y]` join operations
   - Pipeline needs results before continuing

4. **Handle Partial Failures**
   - Wrap parallel tasks in error handling
   - One task failure shouldn't crash all

5. **Optimize Task Granularity**
   - Too small: Overhead dominates
   - Too large: Poor parallelization
   - Sweet spot: 100ms - 10s per task

---

## Performance Metrics

### Sequential vs Parallel

**Sequential (3 files, 2 seconds each):**
```
File1: 2s
File2: 2s
File3: 2s
Total: 6s
```

**Parallel (3 files, 2 seconds each):**
```
File1: 2s ┐
File2: 2s ├─ All concurrent
File3: 2s ┘
Total: 2s
```

**Speedup:** 3x faster

---

## Next Steps

1. **Complete Workflows** - [Complete Workflows](complete-workflows.md)
   - Production parallel patterns
   - Complex orchestration

2. **Error Handling** - [Error Handling Examples](error-handling.md)
   - Error handling in parallel tasks
   - Partial failure recovery

3. **Queue Control** - [Queue Control](../standard-library/02-queue-control.md)
   - Advanced resource management
   - Priority queues

---

## See Also

- [Examples Index](README.md) - All examples
- [Serial Load Blocks](../language/06-block-markers.md#s---serial-load-block) - `[s]` automatic join
- [Runtime Execution](../architecture/05-runtime-execution.md) - Parallel vs serial execution
- [File Operations](file-operations.md) - Serial load examples
- [Pipeline Lifecycle](../language/10-pipeline-lifecycle.md) - Execution model
- [Expansion Operator](../language/09-expansion-operator.md) - ... operator details

---

**Last Updated:** 2025-11-19