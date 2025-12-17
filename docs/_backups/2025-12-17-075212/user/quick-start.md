# Quick Start Guide - Polyglot

**Build your first automated job in 15 minutes - from zero to a daily report pipeline.**

---

> **⚠️ DEVELOPMENT STATUS WARNING**
>
> **This guide describes the PLANNED user experience for Polyglot, which is currently in active development.**
>
> - **Status:** Planning & Design Phase
> - **Code Availability:** Not yet available - implementation in progress
> - **Purpose:** This document serves as a UX specification and design validation
> - **Timeline:** Check [GitHub](https://github.com/polyglot-lang/polyglot) for development progress
>
> All commands, APIs, and workflows shown below represent the **intended** developer experience once Polyglot is released.

---

## ⚠️ CRITICAL: Polyglot is an Automation Language, Not a Traditional Language

**If you're expecting Python/JavaScript/Rust syntax, STOP. Read this first.**

### What Polyglot Is:
Polyglot is the **first language designed specifically for automation workflows**. You write **automated jobs** (called **pipelines**) that run when **triggered** by events (schedules, file changes, webhooks, conditions).

**Think:**
- **Cron + full programming power** - Schedule jobs with conditional logic
- **Event-driven orchestration** - Respond to file changes, API events, database updates
- **Boxes with I/O** - Pipelines are async operations connected by inputs/outputs, not functions
- **Three-service architecture** - Trigger Monitor (watches events) + Queue Manager (prioritizes) + Runner (executes)

### What You're NOT Building:
❌ Scripts you run with `polyglot run myscript.pg` (Polyglot doesn't work this way!)
❌ Traditional programs with IF/ELSE/FOR/WHILE
❌ Functions that return values immediately

### What You ARE Building:
✅ Automated jobs that run when triggers fire
✅ Pipelines with async boxes connected by I/O
✅ Event-driven workflows with exhaustive conditional triggers

**Mental Model:** You're writing automation jobs that the Polyglot service orchestrates, NOT scripts you execute directly.

---

## What You'll Build

By the end of this guide, you'll have built TWO things:

### **Example 1: Cross-Language Integration (Support Feature)**
✅ Python script calling Rust functions via Polyglot
✅ Understanding how Polyglot bridges languages (one of its features, not its purpose)

### **Example 2: Automation Pipeline (Core Purpose)**
✅ Automated daily report job (scheduled trigger)
✅ Multi-step workflow: Rust → Python → LLM
✅ Runs automatically every day at 2 AM (hands-free)
✅ **THIS is what Polyglot is designed for**

**The "Aha!" Moment:**
- **Part 1:** See Rust's compile-time types work with Python's dynamic types (cross-language integration)
- **Part 2:** Build a job that runs itself every day without you touching it (automation programming)

---

## Prerequisites

Before starting, ensure you have:

- **Rust toolchain** (1.70+) - [Install here](https://rustup.rs/)
- **Python 3.11+** - [Install here](https://www.python.org/downloads/)
- **PostgreSQL 14+** - [Install here](https://www.postgresql.org/download/)
- **Docker** (optional but recommended for easy service setup)
- **VS Code** (recommended for IDE support)

**System Requirements:**
- OS: Linux, macOS, or Windows (WSL2 recommended)
- RAM: 4GB minimum, 8GB recommended
- Disk: 2GB free space

---

## ⚠️ BEFORE YOU START: Critical Syntax Warning

**If you've programmed in Python, JavaScript, C, Java, or Rust, Polyglot will feel unfamiliar.**

Polyglot's syntax is **fundamentally different** from mainstream languages:

❌ **NO curly braces `{}`** - Scope is defined by block markers only
❌ **NO function signatures** - Pipelines use `[|]...[X]`, not `func()`
❌ **NO keywords** - Zero keywords (`if`, `for`, `while`, `return`, etc.)
✅ **Block markers define everything** - Every line starts with `[marker]`

**📖 REQUIRED READING:** [Common Mistakes & Anti-Patterns](common-mistakes-antipatterns.md)

**Read this before writing ANY Polyglot code** - it will save you hours of debugging incorrect syntax.

---

## Step 1: Install Polyglot Service

The Polyglot service consists of 3 backend components that orchestrate your pipelines.

### Option A: Docker (Recommended for Quick Start)

```bash
# Pull Polyglot service image
docker pull polyglot/service:latest

# Run with docker-compose (includes PostgreSQL)
curl -O https://polyglot-lang.org/install/docker-compose.yml
docker-compose up -d

# Verify services are running
docker-compose ps
# Expected: trigger-monitor, queue-manager, runner, postgres (all "Up")
```

### Option B: Manual Installation

```bash
# Clone the Polyglot repository
git clone https://github.com/polyglot-lang/polyglot.git
cd polyglot

# Build all services
cargo build --release

# Set up PostgreSQL database
createdb polyglot_db
psql polyglot_db < schema/init.sql

# Start services (in separate terminals or use systemd)
./target/release/trigger-monitor &
./target/release/queue-manager &
./target/release/runner &
```

**Verify Installation:**
```bash
# Check service health
# Note: Port number (8080) is not finalized - subject to change
curl http://localhost:8080/health
# Expected: {"status": "healthy", "services": ["trigger-monitor", "queue-manager", "runner"]}
```

---

## Step 2: Install Polyglot CLI

The CLI is your interface to compile, register, and manage pipelines.

```bash
# Install via cargo
cargo install polyglot-cli

# Or download pre-built binary
curl -L https://polyglot-lang.org/install/cli.sh | sh

# Verify installation
polyglot --version
# Expected: polyglot-cli 0.1.0

# Connect CLI to service
# Note: Port number (8080) is placeholder - final port TBD
polyglot connect --url http://localhost:8080
# Expected: ✓ Connected to Polyglot service
```

---

## Step 3: Install VS Code Extension (Optional)

Get syntax highlighting, IntelliSense, and debugging support.

```bash
# Open VS Code
code --install-extension polyglot-lang.polyglot-vscode

# Or install from VS Code Marketplace:
# 1. Open VS Code
# 2. Go to Extensions (Ctrl+Shift+X)
# 3. Search "Polyglot Language"
# 4. Click Install
```

---

## Step 4: Create Your First Cross-Language Pipeline

> **Note:** Cross-language integration is **one of Polyglot's features**, not its primary purpose.
>
> **Polyglot is fundamentally an automation programming language** - the first language designed specifically for automation workflows. It can:
> - Schedule tasks (cron-like triggers)
> - Respond to file system events
> - Handle API webhooks
> - Manage priority queues and resource allocation
> - Orchestrate multi-step workflows
>
> Cross-language integration enhances these automation capabilities by letting you use the best tool for each step in your pipeline. We'll show automation-focused examples later in this guide.

Let's start with a compelling cross-language example: **Python calling Rust for high-performance matrix operations.**

### The Scenario

You have a Rust library with optimized matrix operations using **compile-time sized types**, and you want to use it from Python for data analysis.

**The Traditional Approach:**
This CAN be done with FFI tools (PyO3, pybind), but it requires:
- Setting up PyO3 or pybind build configuration
- Writing manual FFI wrapper code that handles const generics
- Creating monomorphized versions for each matrix size
- Configuring complex build systems
- Hours of debugging type conversions and lifetime issues
- Writing boilerplate for every function signature variant

**With Polyglot:** All that complexity is abstracted away. You focus on your code, Polyglot handles the integration (likely using FFI tools under the hood, but you never see it).

---

### 4.1: Prepare the Rust Module

Create a simple Rust project with a matrix operation:

```bash
# Create Rust library
cargo new --lib matrix_ops
cd matrix_ops
```

**Edit `src/lib.rs`:**
```rust
/// Extract diagonal elements from a 2D matrix
/// Uses compile-time SIZED constraint - requires manual FFI wrapper code!
pub fn get_diagonal<const N: usize>(matrix: [[f64; N]; N]) -> [f64; N] {
    let mut diagonal = [0.0; N];
    for i in 0..N {
        diagonal[i] = matrix[i][i];
    }
    diagonal
}

/// Transpose a matrix with SIZED compile-time dimensions
/// This is the complex scenario that requires tedious FFI work:
/// - Rust REQUIRES knowing N at compile time (const generic parameter)
/// - Python doesn't know N until runtime (CSV file loaded)
/// - Traditional approach: Write manual wrapper for each N value (3x3, 4x4, etc.)
/// - Polyglot approach: Abstracts this complexity - detects size and handles it automatically
pub fn transpose<const N: usize>(matrix: [[f64; N]; N]) -> [[f64; N]; N] {
    let mut result = [[0.0; N]; N];
    for i in 0..N {
        for j in 0..N {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

/// Alternative: Dynamic version for comparison
/// Traditional FFI can handle this more easily (no const generics)
pub fn get_diagonal_dynamic(matrix: Vec<Vec<f64>>) -> Vec<f64> {
    let size = matrix.len().min(matrix[0].len());
    (0..size).map(|i| matrix[i][i]).collect()
}
```

**Key Insight:** The `<const N: usize>` parameter means Rust **requires** knowing the matrix size at compile time. Python reads a CSV file - the size is only known at **runtime**.

**Traditional FFI approach (PyO3/pybind):**
- Requires writing monomorphized wrappers for each N: `get_diagonal_3x3()`, `get_diagonal_4x4()`, etc.
- Or writing generic wrapper code with runtime dispatch
- Manual boilerplate for every function with const generics
- This works, but it's tedious and error-prone

**Polyglot's automated approach:**
1. Python loads CSV → size known at runtime (e.g., 3x3)
2. Polyglot detects dimensions automatically
3. Polyglot generates or dispatches to appropriate monomorphized version
4. Rust executes with compile-time guarantees intact
5. **You wrote zero FFI boilerplate** - Polyglot handled it (likely using FFI tools under the hood)

```bash
# Build the Rust library
cargo build --release
```

---

### 4.2: Write the Python Script

Create `analyze_data.py`:

```python
import polyglot as pg
import pandas as pd
from pathlib import Path

# Initialize Rust module
# This will scan the repo for all its Rust functions
# and prepare scripts to run them (post-MVP might even use FFI directly)
rust_module = pg.init.rust(
    repo=Path('path/to/rust/project/repo'),  # Path to your Rust project
    module='matrix_ops'                       # Module name from Cargo.toml
)

try:
    # Load data from CSV - size unknown until runtime!
    data = pd.read_csv('data.csv').values  # NumPy array (dynamic dimensions)

    # Call Rust function - Polyglot handles all type conversion automatically
    # Format: 'module_name::Full::path::Function'
    data_diagonal = rust_module('matrix_ops::get_diagonal')(data)

    print(f"Diagonal elements: {data_diagonal}")

    # Call another Rust function - same seamless experience
    transposed = rust_module('matrix_ops::transpose')(data)
    print(f"Transposed matrix:\n{transposed}")

except pg.Errors.PolyglotDisconnected as e:
    print(f"Error: Polyglot service not connected - {e}")
    print("Solution: Run 'polyglot connect --url http://localhost:8080'")
    print("         Or start services: 'docker-compose up -d'")

except pg.Errors.Rust.PanicError as e:
    print(f"Rust panic occurred: {e}")
    print("The Rust function encountered an unrecoverable error")

except pg.Errors.Rust.TypeError as e:
    print(f"Type conversion error: {e}")
    print("Ensure your data matches Rust function signature:")
    print("  Expected: Vec<Vec<f64>+")
    print("  Received: Check data.shape and data.dtype")

except pg.Errors.ModuleNotFound as e:
    print(f"Rust module not found: {e}")
    print("Ensure the Rust project is built: 'cd path/to/rust/project && cargo build --release'")

except Exception as e:
    print(f"Unexpected error: {e}")
    raise
```

**Key Points About This Code:**

1. **Dynamic Type Resolution:** `data` size is unknown until CSV is loaded - Polyglot resolves this at runtime
2. **Async Communication:** Under the hood, data is serialized → sent to Polyglot service → Rust executes → result returned
3. **Error Handling:** Specific exceptions for different failure modes (connection, type mismatch, Rust panics)
4. **Path Handling:** Uses `pathlib.Path` for cross-platform compatibility
5. **Implementation Strategy:** MVP uses script generation; post-MVP may optimize with direct FFI for performance-critical paths

**Create sample data (`data.csv`):**
```csv
1.0,2.0,3.0
4.0,5.0,6.0
7.0,8.0,9.0
```

---

### 4.3: Run It!

```bash
# Install Polyglot Python package
pip install polyglot-lang

# Run your Python script
python analyze_data.py
```

**Expected Output:**
```
Diagonal elements: [1.0, 5.0, 9.0]
Transposed matrix shape: (3, 3)
```

---

### 4.4: What About Polyglot Pipelines (.pg files)?

**Wait - What Did We Just Do?**

You might be wondering: "I thought Polyglot was a language, but we just used Python?"

**Great observation!** There are **two ways** to use Polyglot:

#### **Option 1: Polyglot FROM Your Code** (What We Just Did)
```python
import polyglot as pg
rust_module = pg.init.rust(repo='./matrix_ops', module='matrix_ops')
result = rust_module('matrix_ops::get_diagonal')(data)
```

- **Use case:** Enhance existing Python/Node/etc. projects with cross-language calls
- **You write:** Regular Python code, Polyglot handles FFI
- **Best for:** Adding Rust/Go performance to existing apps

#### **Option 2: Polyglot AS Your Automation Language** (Next Step!)
```polyglot
// DailyReport.pg
[|] DailyReportGenerator

[t] |T.Daily
[<] .time:pg.dt << |DT"02:00:"

[W] |W.Rust
[r] |GatherCodebaseStats

[W] |W.Python3.11
[r] |SendToLLM

[X]
```

- **Use case:** Build automated workflows, scheduled tasks, event-driven pipelines
- **You write:** `.pg` files, Polyglot orchestrates everything
- **Best for:** Automation, data pipelines, multi-step workflows

**The Relationship:**
- **Option 1** is for enhancing your apps
- **Option 2** is Polyglot's **primary purpose** - automation programming
- Cross-language integration serves both!

**Next, we'll build a real automation pipeline...**

---

## What Just Happened? (The Magic Explained)

When you ran `rust_module('matrix_ops::get_diagonal')(data)`, here's what Polyglot did:

### 1. **Runtime Type Detection** (Python side)
```python
data = pd.read_csv('data.csv').values  # Shape unknown until runtime!
```
- Python reads CSV file (size determined at runtime)
- NumPy array created with dynamic dimensions
- **At compile-time, we don't know:** How many rows? How many columns?

### 2. **Serialization & Service Call** (Polyglot bridge)
```
Python → Serialize data → Polyglot Service
```
- Polyglot inspects the NumPy array
- Detects: 3x3 matrix of float64 values
- Serializes to intermediate representation (JSON/binary)
- Sends to Polyglot service with function signature

### 3. **Type Resolution** (Polyglot service)
```
Polyglot analyzes:
- Input: Dynamic 2D array (Python) - 3x3 matrix detected
- Target: [[f64; N]; N] with const generic (Rust)
- Determines: N=3 for this specific call
- Conversion: Map Python array → Rust fixed-size array [[f64; 3]; 3]
```

### 4. **Rust Execution** (Runner service)
```
Polyglot Service → Spawn Rust process → Execute function
```
- Creates Rust process with your compiled library
- Converts serialized data to `[[f64; 3]; 3]` (monomorphized for N=3)
- Calls `get_diagonal::<3>(matrix)`
- Returns `[f64; 3]` result

### 5. **Result Deserialization** (Back to Python)
```
Rust result → Serialize → Python list
```
- Rust `[f64; 3]` serialized by Polyglot
- Converted back to Python list `[1.0, 5.0, 9.0]`
- Returned to your Python script

---

## The Breakthrough: Polyglot Abstracts FFI Complexity

**Traditional FFI Approach (PyO3/pybind):**
- **Rust needs:** Array size known at compile-time (`<const N: usize>`)
- **Python provides:** Dynamic data, size only known at runtime
- **Manual solution:** Write wrapper functions for each size, or complex generic dispatch
- **Your work:** Hours of boilerplate, build configuration, type conversion code

**Polyglot's Abstraction:**
- Rust operation is **asynchronous** from Python's perspective
- Polyglot service acts as runtime intermediary
- **At runtime:** Polyglot detects Python array size → dispatches to correct monomorphized Rust function
- **Under the hood:** Polyglot likely uses FFI tools (PyO3/pybind/etc.), but **you never write the integration code**
- **Your work:** Zero FFI boilerplate - just write your Rust and Python code

**The Value Proposition:**
It's not that this is *impossible* with traditional FFI - it's that Polyglot **eliminates the tedious integration work**. Focus on your algorithm, let Polyglot handle the plumbing.

---

## Step 5: Create Your First Automation Pipeline

Now let's build a **real automation pipeline** using Polyglot's `.pg` language!

**The Goal:** Automated daily report that analyzes your codebase, processes data, and sends insights to an LLM - all hands-free.

---

### 5.1: Create the .pg Pipeline File

Create `DailyCodeReport.pg`:

```polyglot
// Package declaration (required)
[@] Local@QuickStartExample:1.0.0
[#] 1
[X]



// Pipeline: Daily Codebase Analysis Report
[|] DailyCodeReport

// === INPUTS ===
[i] .repo_path:pg.path
[i] .stat_rust_file:pg.path << \\FileDir\\rustrepo\\statgen.rs
[i] .pyfile:pg.path << \\FileDir\\pythonrepo\\formatter.py
[i] .py_function_name:pg.string << "report_formatter"
[i] .python_depen:pg.path << \\FileDir\\pythonrepo\\lock.toml
[i] .llm_config:pg.path << \\FileDir\\llm.yaml

// === TRIGGER: Every day at 2 AM ===
[t] |T.Daily
[<] .time:pg.dt << |DT"02:00:"

// === QUEUE PRIORITY: Medium-high ===
[Q] |Q.Priority
[<] .level:pg.int << 7

// === STEP 1: Setup Runtimes ===
[W] |W.Rust1.8
[W] |W.Python3.11
[<] .dependencies:pg.path << .python_depen

// === STEP 2: Gather codebase statistics with Rust ===
[r] |U.Rust.Run
[<] .file:pg.path << .stat_rust_file
[>] .results: rs\HashMap >> .rust_stats:pg.serial
// Rust returns HashMap<String, i32>, Polyglot auto-converts to pg\serial
// Note: rs\HashMap is the foreign type reference for Rust's HashMap

// === STEP 3: Format report with Python ===
[r] |U.Python.Run
[<] .python_file:pg.path << .pyfile
[<] .function:pg.string << .py_function_name
[<] .raw_data: py\str << .rust_stats
[>] .formatted: py\str >> .python_report:pg.string
// Python creates markdown report from data

// === STEP 4: Send to LLM for insights ===
[r] |U.LLM.prompt
[<] .config:pg.path << .llm_config
[<] .report:pg.string << .python_report
[>] .analysis:pg.string >> .llm_insights
// Calls OpenAI/Claude API for code health analysis

// === OUTPUT ===
[o] .final_report:pg.string

[X]  // End pipeline definition
```

---

### 5.2: Create the Rust Code

**File:** `rustrepo/statgen.rs`

```rust
use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub fn main() -> HashMap<String, i32> {
    // This is the entry point Polyglot calls
    // Returns: HashMap<String, i32> → Polyglot type: rs\HashMap

    let repo_path = std::env::var("REPO_PATH").unwrap_or_else(|_| ".".to_string());
    let path = Path::new(&repo_path);

    let mut stats: HashMap<String, i32> = HashMap::new();
    let mut file_count = 0;
    let mut line_count = 0;

    // Walk directory tree
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    file_count += 1;

                    // Count lines
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        line_count += content.lines().count() as i32;
                    }
                }
            }
        }
    }

    // Return HashMap - Polyglot auto-converts to pg\serial
    stats.insert("files".to_string(), file_count);
    stats.insert("lines".to_string(), line_count);
    stats.insert("avg_lines".to_string(), if file_count > 0 { line_count / file_count } else { 0 });

    stats
}
```

**Key Points:**
- Rust returns `HashMap<String, i32>` (Polyglot type: `rs\HashMap`)
- Polyglot **automatically converts** `rs\HashMap` → `:pg.serial` (cross-language serializable format)
- No manual JSON serialization needed!
- Function signature: `pub fn main() -> HashMap<String, i32>`

**Type System Note:**
- ❌ WRONG: `rust\map<string&, i32>` (maps removed in v0.0.2, invalid syntax)
- ✅ CORRECT: `rs\HashMap` (foreign type reference to Rust's HashMap)
- ✅ CORRECT: `:pg.serial` (Polyglot's dynamic key-value structure)

---

### 5.3: Create the Python Code

**File:** `pythonrepo/formatter.py`

```python
from datetime import datetime

def report_formatter(raw_data: dict) -> str:
    """
    Format stats dictionary into markdown report.

    This is the function called by Polyglot via |U.Python.Run
    Function name matches .py_function_name input: "report_formatter"

    Args:
        raw_data: Deserialized pg\serial data (originally from Rust HashMap)

    Returns:
        Formatted markdown report as string
    """
    report = f"""# Daily Codebase Report
Generated: {datetime.now().strftime('%Y-%m-%d %H:%M')}

## Statistics
- **Total Files:** {raw_data.get('files', 0):,}
- **Total Lines:** {raw_data.get('lines', 0):,}
- **Average Lines per File:** {raw_data.get('avg_lines', 0)}

## Health Indicators
- Code volume: {"Large" if raw_data.get('lines', 0) > 50000 else "Medium" if raw_data.get('lines', 0) > 10000 else "Small"}
- Modularity: {"Good" if raw_data.get('avg_lines', 0) < 300 else "Needs improvement"}
"""
    return report
```

**File:** `pythonrepo/lock.toml` (Python dependencies)

```toml
# Python dependencies for this pipeline
# Polyglot installs these via uv before running

[dependencies]
python = "^3.11"
```

**Key Points:**
- Function name `report_formatter` matches the `.pg` file input
- Polyglot passes deserialized data directly (no manual JSON parsing!)
- Polyglot installs dependencies from `lock.toml` automatically
- Returns plain `str` - Polyglot converts `py\str` → `:pg.string`

---

### 5.4: Create the LLM Configuration

**File:** `llm.yaml`

```yaml
# LLM configuration for |U.LLM.prompt
provider: openai
model: gpt-4
api_key_env: OPENAI_API_KEY  # Read from environment variable
temperature: 0.7
max_tokens: 500

system_prompt: |
  You are a code health analyst. Analyze codebase reports and provide:
  1. Overall assessment
  2. Strengths
  3. Areas for improvement
  4. Recommendations
```

**Key Points:**
- `|U.LLM.prompt` is a built-in Polyglot utility
- Reads config from YAML file
- Supports OpenAI, Anthropic Claude, local models
- API key from environment variable (secure!)

---

### 5.5: Compile and Register the Pipeline

```bash
# Compile the .pg file to IR (validates syntax, generates 3-IR)
polyglot compile DailyCodeReport.pg

# Register with Polyglot service (uploads to database)
polyglot register DailyCodeReport

# Activate the pipeline (starts monitoring triggers)
# Note: Only need to provide [i] inputs without defaults
# Constants ([i] with <<) are already in the .pg file
polyglot activate DailyCodeReport \
  --input repo_path=/home/user/my-project

# Verify it's activated
polyglot list --filter activated=true
```

**Expected Output:**
```
✓ Compiled DailyCodeReport.pg → trigger_ir, queue_ir, runner_ir
✓ Registered pipeline: DailyCodeReport (ID: abc123)
✓ Activated DailyCodeReport
  - Inputs resolved:
    • .repo_path = /home/user/my-project
    • .stat_rust_file = \\FileDir\\rustrepo\\statgen.rs
    • .pyfile = \\FileDir\\pythonrepo\\formatter.py
    • .py_function_name = "report_formatter"
    • .python_depen = \\FileDir\\pythonrepo\\lock.toml
    • .llm_config = \\FileDir\\llm.yaml
  - Trigger: Daily at 02:00:00 UTC
  - Next run: 2025-11-21 02:00:00
```

**What Just Happened:**
1. **Compile:** Polyglot generated 3 separate IRs (trigger_ir, queue_ir, runner_ir) from your `.pg` file
2. **Register:** IRs uploaded to PostgreSQL database
3. **Activate:** Trigger Monitor now watching for 2 AM daily trigger
4. **Constants resolved:** All `[i]` constant inputs (with `<<`) were resolved from file paths

---

### 5.6: Test It Manually (Don't Wait Until 2 AM!)

```bash
# Manually trigger the pipeline for testing
polyglot trigger DailyCodeReport

# Watch the execution
polyglot logs DailyCodeReport --follow
```

**Expected Log Output:**
```
[14:35:01] DailyCodeReport: Started (manual trigger)
[14:35:01] Queue: Priority=7, dispatching to runner pool
[14:35:01] Setup: Preparing Rust1.8 runtime
[14:35:02] Setup: Preparing Python3.11 runtime (installing dependencies from lock.toml)
[14:35:05] Setup: Complete
[14:35:05] Step 1/3: |U.Rust.Run (statgen.rs)
[14:35:06] Step 1/3: Complete → rs\HashMap{files: 152, lines: 45203, avg_lines: 297}
[14:35:06] Type conversion: rs\HashMap → pg\serial
[14:35:06] Step 2/3: |U.Python.Run (formatter.py::report_formatter)
[14:35:07] Step 2/3: Complete → py\str (526 chars)
[14:35:07] Type conversion: py\str → pg\string
[14:35:07] Step 3/3: |U.LLM.prompt (OpenAI GPT-4)
[14:35:12] Step 3/3: Complete → pg\string (LLM analysis: 342 chars)
[14:35:12] DailyCodeReport: Completed successfully
[14:35:12] Output saved: /var/polyglot/outputs/DailyCodeReport-2025-11-20-14-35-01.txt
```

**Key Observations:**
- **Runtime setup:** Polyglot prepared both Rust and Python environments
- **Automatic type conversions:** `rs\HashMap` → `:pg.serial` → `py\dict` → `py\str` → `:pg.string`
- **Built-in utilities:** `|U.Rust.Run`, `|U.Python.Run`, `|U.LLM.prompt` handled execution
- **No manual integration:** Zero FFI code, zero API wrapper code!

---

### 5.7: View the Results

```bash
# Get the latest execution result
polyglot output DailyCodeReport --latest
```

**Example Output:**
```markdown
# Daily Codebase Report
Generated: 2025-11-20 14:35

## Statistics
- **Total Files:** 152
- **Total Lines:** 45,203
- **Average Lines per File:** 297

## Health Indicators
- Code volume: Medium
- Modularity: Good

---

## LLM Analysis

### Overall Assessment
This codebase demonstrates healthy growth characteristics with well-balanced modularity.
The 297 average lines per file indicates thoughtful component sizing.

### Strengths
1. **Good Modularity:** Files are sized appropriately (not too large, not fragmented)
2. **Active Development:** 45K+ lines suggests substantial, production-ready code
3. **Maintainability:** File count and structure support team collaboration

### Areas for Improvement
- Consider documenting architectural decisions for files exceeding 500 lines
- Monitor complexity metrics alongside line counts

### Recommendations
- Continue current modularization practices
- Implement automated complexity tracking in CI/CD
- Review largest files quarterly for refactoring opportunities
```

---

## What You Just Built

🎉 **Congratulations!** You just created a production-ready automated pipeline!

### **Key Accomplishments:**

**1. Used Polyglot Two Ways:**
- ✅ **Option 1:** Called Rust from Python using `import polyglot` (cross-language integration)
- ✅ **Option 2:** Built automation pipeline with `.pg` language (scheduled workflows)

**2. Learned Core `.pg` Syntax:**
- `[@]` Package declaration
- `[i]` Inputs (required, defaults with `<~`, or constants with `<<`)
- `[t]` Triggers (`|T.Daily`)
- `[W]` Runtime wrappers (`|W.Rust1.8`, `|W.Python3.11`)
- `[r]` Run operations (`|U.Rust.Run`, `|U.Python.Run`, `|U.LLM.prompt`)
- `[<]` Input passing / `[>]` Output extraction
- `[Q]` Queue priority control
- `[o]` Output declarations

**3. Orchestrated Multi-Language Workflow:**
- ✅ Rust gathered codebase stats (`HashMap<String, i32>` = `rs\HashMap`)
- ✅ Polyglot auto-converted types (`rs\HashMap` → `:pg.serial` → `py\dict` → `py\str`)
- ✅ Python formatted report
- ✅ LLM analyzed with AI insights
- ✅ **Zero manual FFI or API integration code!**

**4. Automated Everything:**
- ✅ Trigger: Daily at 2 AM (hands-free)
- ✅ Priority: Medium-high (queue control)
- ✅ Dependencies: Auto-installed from `lock.toml`
- ✅ Type conversions: Automatic across language boundaries
- ✅ Results: Saved and accessible via CLI

---

## Next Steps

### Learn More:

1. **[Language Guide](./language/01-syntax-complete.md)** - Learn Polyglot's `.pg` syntax
2. **[Pipeline Tutorial](./tutorials/pipelines.md)** - Build automated workflows
3. **[Architecture Overview](./architecture/00-overview.md)** - Understand the 3-service design
4. **[Examples Gallery](./examples/)** - More automation and cross-language recipes

### Try These Next:

**More Automation Pipeline Ideas:**

1. **File Watch Automation**
   - Monitor directories for new files (CSV, images, logs)
   - Trigger processing pipeline when files appear
   - Use Rust for validation, Python for transformation
   - See: [File Operations Examples](./examples/file-operations.md)

2. **API Webhook Handler**
   - Receive GitHub/GitLab webhooks
   - Run CI/CD tests with appropriate runtime
   - Send notifications to Slack/Discord
   - See: [Complete Workflows](./examples/complete-workflows.md)

3. **Resource-Aware Batch Processing**
   - Schedule low-priority jobs (reports, backups)
   - Automatically pause when high-priority work arrives
   - Resume when resources available
   - See: [Queue System Guide](./architecture/03-queue-system.md)

4. **Multi-Language Data Pipeline**
   - Extract data with Python
   - Transform with Rust (performance)
   - Load with Go (concurrency)
   - See: [Data Processing Examples](./examples/data-processing.md)

5. **AI/ML Workflows**
   - Preprocess data with Rust
   - Run inference with Python (TensorFlow/PyTorch)
   - Post-process with JavaScript
   - See: [Complete Workflows - AI/ML Section](./examples/complete-workflows.md#aiml-workflows)

**Full `.pg` Language Tutorial:**
- [Complete Syntax Guide](./language/01-syntax-complete.md) - All block markers, operators, and patterns
- [Pipeline Tutorial](./tutorials/pipelines.md) - Step-by-step pipeline building
- [Trigger Catalog](./standard-library/04-triggers.md) - All available trigger types

---

## Troubleshooting

### Polyglot service not responding
```bash
# Check service status
docker-compose ps  # (if using Docker)
# or
ps aux | grep polyglot

# Check logs
docker-compose logs  # (Docker)
# or
journalctl -u polyglot-*  # (systemd)
```

### CLI connection failed
```bash
# Verify service is running
curl http://localhost:8080/health

# Reconnect CLI
polyglot connect --url http://localhost:8080
```

### Rust module not found
```bash
# Ensure Rust project is built
cd matrix_ops
cargo build --release

# Verify path in pg.init.rust() is correct (absolute or relative to script)
```

### Type conversion errors
- Ensure Python data types match Rust function signature
- Check Rust function accepts `Vec<Vec<f64>>` (not fixed-size arrays `[[f64; N]; M]`)
- Use Polyglot's type introspection: `pg.inspect.rust('matrix_ops::get_diagonal')`

---

## Get Help

- **Documentation:** [https://polyglot-lang.org/docs](https://polyglot-lang.org/docs)
- **Discord Community:** [https://discord.gg/polyglot](https://discord.gg/polyglot)
- **GitHub Issues:** [https://github.com/polyglot-lang/polyglot/issues](https://github.com/polyglot-lang/polyglot/issues)
- **Stack Overflow:** Tag `polyglot-lang`

---

**Now go build something amazing with the right tool for every job!** 🚀
