---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "serial-load-block"
shard: false

# --- Classification ---
type: spec
topic: Serial Load Block
summary: "Advanced: Serial Load Block"
keywords:
  - advanced
  - features
  - specification

# --- BMAD Agent Routing ---
agents:
  - architect
phase: solutioning
workflow: greenfield
module: bmm
complexity: high

# --- Dependency Chain ---
prereqs:
  - language-syntax
  - type-system
  - control-flow
unlocks:
  - stdlib

# --- Relationships ---
related:
  []
parent: "language-advanced"

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#advanced"
  - "#features"
---
# Serial Load Block

**Feature Status:** ⭐ NEW in v0.0.4
**Marker:** `[s]`
**Purpose:** Load serial data (files, streams) with parallel execution and unified error handling

---

## Overview

The **Serial Load Block** (`[s]`) is a powerful feature for loading serial data (files, streams, serialized content) with:

1. **Parallel Execution** - Multiple loads run concurrently
2. **Unified Error Handling** - Single error handler for all loads
3. **Two Contexts** - Struct/enum definitions and pipeline execution
4. **Auto-Collection** - Results automatically aggregated

### What is Serial Data?

**Serial data** in Polyglot refers to data that must be **deserialized** from an external format:

- **Files:** YAML, JSON, TOML, XML, CSV
- **Streams:** Network streams, file streams
- **Serialized Content:** Binary formats, protocol buffers

**Type:** `:pg.serial`

---

## Two Contexts

### Context 1: Struct/Enum Field Mapping

Load serial files and map fields to struct/enum properties.

**Syntax:**
```polyglot
{#} #StructName
[s] .field_name :pg.file.format
   [.] .property1 :type
   [.] .property2 :type
[s][!] *! ? #StructName.error_variant
{#}
```

### Context 2: Pipeline Execution (Load Entire Content)

Load serial data in pipeline and access entire content.

**Syntax:**
```polyglot
[s] |LoaderPipeline
[s] <file << $file_path
[s] >content >> $variable :pg.serial
   [.] << *                        // Load entire content
[s][!] *! >> $error_variable :!
```

---

## Context 1: Struct/Enum Field Mapping

### Basic Example

```polyglot
{#} #AppConfig
[s] .yaml_file :pg.file.yaml       // Serial load block starts
   [.] .database :string           // Map YAML field to property
   [.] .port :int
   [.] .host :string
[s][!] *! ? #AppConfig.load_error  // Error handler for this block
{#}
```

**What happens:**
1. Polyglot loads the YAML file
2. Maps `database`, `port`, `host` fields from YAML to struct properties
3. If any error occurs (file not found, parse error, missing field), creates `#AppConfig.load_error` variant

### Multiple Serial Blocks (Parallel Loading)

**Critical Feature:** All `[s]` blocks at the **same level** run **in parallel**.

```polyglot
{#} #MultiSourceConfig
// These THREE blocks load IN PARALLEL:

[s] .database_config :pg.file.yaml
   [.] .db_host :string
   [.] .db_port :int

[s] .api_config :pg.file.json
   [.] .api_url :string
   [.] .api_key :string

[s] .feature_flags :pg.file.toml
   [.] .enable_beta :bool
   [.] .enable_debug :bool

// Single error handler for ALL three:
[s][!] *! ? #MultiSourceConfig.load_error

{#}
```

**Execution:**
1. All three files load **concurrently** (parallel I/O)
2. If **any** file fails, entire struct becomes `load_error` variant
3. Only succeeds if **all** files load successfully

### Field Mapping Rules

**YAML Example:**

```yaml
# config.yaml
database: postgresql
port: 5432
host: localhost
credentials:
  username: admin
  password: secret
```

**Struct Definition:**

```polyglot
{#} #DatabaseConfig
[s] .yaml_file :pg.file.yaml
   [.] .database :string           // Maps to "database"
   [.] .port :int                  // Maps to "port"
   [.] .host :string               // Maps to "host"
   [.] .credentials                // Nested object
      [.] .username :string
      [.] .password :string
[s][!] *! ? #DatabaseConfig.load_error
{#}
```

**Mapping:**
- Top-level YAML keys → struct fields (`.database`, `.port`)
- Nested YAML objects → nested struct fields (`.credentials.username`)
- Field names **must match** YAML keys (case-sensitive)
- Field types **must be compatible** with YAML values

### Supported File Formats

| Format | Type Notation | Extension |
|--------|---------------|-----------|
| **YAML** | `:pg.file.yaml` | `.yaml`, `.yml` |
| **JSON** | `:pg.file.json` | `.json` |
| **TOML** | `:pg.file.toml` | `.toml` |
| **XML** | `:pg.file.xml` | `.xml` |
| **CSV** | `:pg.file.csv` | `.csv` |

### Error Handling (forall/each)

**Key Concept:** Single error handler `[s][!] *!` applies to **all** `[s]` blocks (forall/each semantics).

```polyglot
{#} #Config
[s] .file1 :pg.file.yaml
   [.] .field1 :string

[s] .file2 :pg.file.json
   [.] .field2 :int

[s] .file3 :pg.file.toml
   [.] .field3 :bool

// This handler catches errors from ANY of the three files:
[s][!] *! ? #Config.load_error

{#}
```

**Behavior:**
- If `file1` fails → `#Config.load_error`
- If `file2` fails → `#Config.load_error`
- If `file3` fails → `#Config.load_error`
- If **multiple** fail → still `#Config.load_error` (one error variant)

**Cannot distinguish which file failed** (by design - unified error handling).

---

## Context 2: Pipeline Execution (Load Entire Content)

### Basic Example

Load a file's entire content into a variable:

```polyglot
[r] $file_path :string << "/etc/config.yaml"

[s] |YAML.Load                     // Serial load pipeline
[s] <file << $file_path            // Input: file path
[s] >content >> $yaml_content :pg.serial  // Output: content
   [.] << *                        // Load entire content (special syntax)
[s][!] *! >> $load_error :!        // Error handler
```

**Key Syntax:** `[.] << *`
- The `*` means "entire content"
- Loads the complete deserialized structure into `$yaml_content`

### Accessing Loaded Content

After loading with `[.] << *`, access fields:

```polyglot
[s] |YAML.Load
[s] <file << "/config.yaml"
[s] >content >> $config :pg.serial
   [.] << *

// Access fields from loaded content:
[r] $database :string << $config.database
[r] $port :int << $config.port
[r] $host :string << $config.host
```

### Multiple Parallel Loads in Pipeline

```polyglot
[r] $config_path :string << "/config.yaml"
[r] $secrets_path :string << "/secrets.yaml"
[r] $features_path :string << "/features.toml"

// All THREE loads run IN PARALLEL:

[s] |YAML.Load
[s] <file << $config_path
[s] >content >> $config :pg.serial
   [.] << *

[s] |YAML.Load
[s] <file << $secrets_path
[s] >content >> $secrets :pg.serial
   [.] << *

[s] |TOML.Load
[s] <file << $features_path
[s] >content >> $features :pg.serial
   [.] << *

// Single error handler for ALL three:
[s][!] *! >> $load_error :!

// If all succeed, continue:
[r] $db_host :string << $config.database.host
[r] $api_key :string << $secrets.api.key
[r] $beta_enabled :bool << $features.beta.enabled
```

**Execution:**
1. All three `|YAML.Load`/`|TOML.Load` pipelines start **concurrently**
2. Loads happen **in parallel** (I/O optimization)
3. If **any** load fails, `$load_error` receives the error
4. Only proceeds if **all** loads succeed

### Field Mapping vs Entire Content

**Difference between Context 1 and Context 2:**

| Feature | Context 1: Struct/Enum | Context 2: Pipeline |
|---------|------------------------|---------------------|
| **When** | Definition time | Execution time |
| **Syntax** | `[s] .field :pg.file.yaml` | `[s] \|YAML.Load` |
| **Mapping** | Explicit field mapping | `[.] << *` (entire content) |
| **Result** | Struct with mapped fields | Variable with full content |
| **Access** | Via struct fields | Via variable path |

**Context 1 Example:**
```polyglot
{#} #Config
[s] .yaml_file :pg.file.yaml
   [.] .database :string           // Explicit mapping
   [.] .port :int
{#}

[r] $config << #Config
// Access via: $config.database, $config.port
```

**Context 2 Example:**
```polyglot
[s] |YAML.Load
[s] <file << "/config.yaml"
[s] >content >> $config :pg.serial
   [.] << *                        // Load all

// Access via: $config.database, $config.port (same result, different approach)
```

**When to use which:**
- **Context 1:** Configuration loaded at definition time (static config)
- **Context 2:** Dynamic loading at runtime (user-provided files, dynamic paths)

---

## Parallel Execution Semantics

### Implicit Parallelism

**All `[s]` blocks at the same level execute in parallel** - you don't need to mark them with `[p]`:

```polyglot
// These run IN PARALLEL automatically:
[s] |YAML.Load <file << "file1.yaml" >content >> $config1 [.] << *
[s] |JSON.Load <file << "file2.json" >content >> $config2 [.] << *
[s] |TOML.Load <file << "file3.toml" >content >> $config3 [.] << *
```

**Why automatic parallelism?**
- Serial loads are **I/O bound**
- Loading files sequentially is inefficient
- Parallel loading is almost always desired
- Polyglot optimizes by default

### Synchronization Point

Execution **blocks** until all `[s]` blocks complete:

```polyglot
[r] $start << |Timer.Now

[s] |YAML.Load <file << "file1.yaml" >content >> $config1 [.] << *
[s] |YAML.Load <file << "file2.yaml" >content >> $config2 [.] << *
[s] |YAML.Load <file << "file3.yaml" >content >> $config3 [.] << *

// Execution WAITS here until all three loads complete

[r] $end << |Timer.Now
[r] $duration << $end - $start     // Total time ≈ slowest load (not sum of all)
```

### Auto-Collection

Results from parallel `[s]` blocks are **automatically collected**:

```polyglot
[s] |YAML.Load <file << "file1.yaml" >content >> $config1 [.] << *
[s] |YAML.Load <file << "file2.yaml" >content >> $config2 [.] << *
[s] |YAML.Load <file << "file3.yaml" >content >> $config3 [.] << *

// $config1, $config2, $config3 are all available here
// No explicit collection needed
```

---

## Error Handling

### Unified Error Handler

**Pattern:** `[s][!] *!`

```polyglot
[s] |YAML.Load <file << "config.yaml" >content >> $config [.] << *
[s][!] *! >> $error :!
```

**The `*!` means:**
- `*` - Any error (wildcard)
- `!` - Error type

**Applies to all `[s]` blocks:**

```polyglot
[s] |YAML.Load <file << "file1.yaml" >content >> $config1 [.] << *
[s] |JSON.Load <file << "file2.json" >content >> $config2 [.] << *
[s] |TOML.Load <file << "file3.toml" >content >> $config3 [.] << *

// Catches errors from ANY of the three:
[s][!] *! >> $error :!
```

### Common Errors

**File Not Found:**
```polyglot
[s] |YAML.Load <file << "/nonexistent.yaml" >content >> $config [.] << *
[s][!] !IO.File.NotFound >> $error :!
```

**Parse Error:**
```polyglot
[s] |YAML.Load <file << "/invalid.yaml" >content >> $config [.] << *
[s][!] !Parse.YAML.Invalid >> $error :!
```

**Permission Denied:**
```polyglot
[s] |YAML.Load <file << "/root/secret.yaml" >content >> $config [.] << *
[s][!] !IO.File.PermissionDenied >> $error :!
```

**Any Error (Catch-All):**
```polyglot
[s] |YAML.Load <file << $file_path >content >> $config [.] << *
[s][!] *! >> $error :!             // Catches any error
```

### Error Handling Pattern

```polyglot
[s] |YAML.Load <file << $config_path >content >> $config [.] << *
[s][!] !IO.File.* >> $file_error :!
[s][!] !Parse.* >> $parse_error :!
[s][!] *! >> $unknown_error :!

// Check which error occurred:
[f] $file_error.state =? :pg.state.faulted
   [r] $message << "File error: cannot read config"

[f] $parse_error.state =? :pg.state.faulted
   [r] $message << "Parse error: invalid YAML syntax"

[f] $unknown_error.state =? :pg.state.faulted
   [r] $message << "Unknown error occurred"
```

---

## Complete Examples

### Example 1: Multi-File Application Config

```polyglot
{#} #ApplicationConfig
// Load three config files in parallel:

[s] .database :pg.file.yaml
   [.] .host :string
   [.] .port :int
   [.] .username :string

[s] .api :pg.file.json
   [.] .base_url :string
   [.] .api_key :string
   [.] .timeout :int

[s] .features :pg.file.toml
   [.] .enable_beta :bool
   [.] .enable_analytics :bool
   [.] .max_upload_size :int

// Unified error handling:
[s][!] *! ? #ApplicationConfig.load_error

{#}

// Usage:
[r] $config << #ApplicationConfig

// Access loaded config:
[r] $db_host :string << $config.database.host
[r] $api_url :string << $config.api.base_url
[r] $beta_enabled :bool << $config.features.enable_beta
```

### Example 2: Dynamic File Loading in Pipeline

```polyglot
{|} |LoadUserConfig
[|] <user_id :string
[|] >config :pg.serial

[t] |T.Call
[W] |W.Polyglot.Scope

   // Build file paths:
   [r] $config_path :string << |String.Concat
   [|] <parts << {"/users/", $user_id, "/config.yaml"}

   [r] $preferences_path :string << |String.Concat
   [|] <parts << {"/users/", $user_id, "/preferences.json"}

   // Load both files in parallel:
   [s] |YAML.Load
   [s] <file << $config_path
   [s] >content >> $config_data :pg.serial
      [.] << *

   [s] |JSON.Load
   [s] <file << $preferences_path
   [s] >content >> $pref_data :pg.serial
      [.] << *

   // Error handling:
   [s][!] !IO.File.NotFound >> $not_found :!
   [s][!] *! >> $other_error :!

   // Merge configs:
   [r] $merged << |MergeConfigs
   [|] <config << $config_data
   [|] <preferences << $pref_data

   [|] >config << $merged

{x}
```

### Example 3: Batch File Processing

```polyglot
{|} |ProcessBatchFiles
[|] <file_paths :array.string
[|] >results :array.pg.serial

[t] |T.Call
[W] |W.Polyglot.Scope

   // Loop through files:
   [p] ~ForEach.Array
   [~] <array << $file_paths
   [~] >item >> $file_path

      // Load each file (parallel at iteration level):
      [s] |YAML.Load
      [s] <file << $file_path
      [s] >content >> $file_content :pg.serial
         [.] << *

      [s][!] *! >> $load_error :!

      // Handle errors:
      [f] $load_error.state =? :pg.state.faulted
         [r] $file_content << #EmptyConfig  // Fallback

      // Aggregate results:
      [v] *Into.Array
      [*] <item << $file_content
      [*] >array >> $all_contents

   [|] >results << $all_contents

{x}
```

---

## Best Practices

### 1. Use Parallel Loading for Independent Files

**Do:**
```polyglot
[s] |YAML.Load <file << "config.yaml" >content >> $config [.] << *
[s] |JSON.Load <file << "data.json" >content >> $data [.] << *
// Both load in parallel ✅
```

**Don't:**
```polyglot
[r] $config << |YAML.Load <file << "config.yaml"
[r] $data << |JSON.Load <file << "data.json"
// Sequential loading (slower) ❌
```

### 2. Always Provide Error Handlers

**Do:**
```polyglot
[s] |YAML.Load <file << $path >content >> $config [.] << *
[s][!] *! >> $error :!             // Handle errors ✅
```

**Don't:**
```polyglot
[s] |YAML.Load <file << $path >content >> $config [.] << *
// No error handler - unhandled errors! ❌
```

### 3. Use Context 1 for Static Config

**For configuration that doesn't change:**
```polyglot
{#} #StaticConfig
[s] .app_config :pg.file.yaml      // Loaded at definition ✅
   [.] .setting1 :string
{#}
```

### 4. Use Context 2 for Dynamic Paths

**For runtime-determined files:**
```polyglot
[r] $file_path :string << $user_input
[s] |YAML.Load <file << $file_path >content >> $config [.] << *  // ✅
```

### 5. Group Related Loads

```polyglot
// Load all related configs together:
[s] |YAML.Load <file << "database.yaml" >content >> $db_config [.] << *
[s] |YAML.Load <file << "cache.yaml" >content >> $cache_config [.] << *
[s] |YAML.Load <file << "queue.yaml" >content >> $queue_config [.] << *
[s][!] *! >> $error :!
// All infrastructure configs loaded in parallel ✅
```

---

## Limitations

### 1. Cannot Distinguish Which File Failed (Unified Error Handler)

```polyglot
[s] |YAML.Load <file << "file1.yaml" >content >> $config1 [.] << *
[s] |YAML.Load <file << "file2.yaml" >content >> $config2 [.] << *
[s][!] *! >> $error :!

// Cannot determine if file1 or file2 failed
```

**Workaround:** Use separate error handlers per file (verbose):

```polyglot
[s] |YAML.Load <file << "file1.yaml" >content >> $config1 [.] << *
[s][!] *! >> $error1 :!

[s] |YAML.Load <file << "file2.yaml" >content >> $config2 [.] << *
[s][!] *! >> $error2 :!
```

### 2. Field Names Must Match Exactly

**YAML:**
```yaml
userName: alice         # camelCase
```

**Struct:**
```polyglot
{#} #Config
[s] .yaml :pg.file.yaml
   [.] .user_name :string          # ❌ Doesn't match (snake_case)
{#}
```

**Fix:** Use same casing:
```polyglot
{#} #Config
[s] .yaml :pg.file.yaml
   [.] .userName :string           # ✅ Matches
{#}
```

### 3. No Partial Success

If loading 5 files and 1 fails, **all fail**:

```polyglot
[s] |YAML.Load <file << "file1.yaml" >content >> $c1 [.] << *
[s] |YAML.Load <file << "file2.yaml" >content >> $c2 [.] << *
[s] |YAML.Load <file << "file3.yaml" >content >> $c3 [.] << *
[s] |YAML.Load <file << "bad_file.yaml" >content >> $c4 [.] << *  // FAILS
[s] |YAML.Load <file << "file5.yaml" >content >> $c5 [.] << *
[s][!] *! >> $error :!

// $c1, $c2, $c3, $c5 are NOT available (entire operation faulted)
```

**Workaround:** Use loop with individual error handling (see Example 3).

---

## Summary

### Key Features
- **Marker:** `[s]`
- **Purpose:** Load serial data (files, streams)
- **Parallel Execution:** All `[s]` blocks at same level run concurrently
- **Unified Error Handling:** `[s][!] *!` catches all errors
- **Two Contexts:** Struct/enum definitions, pipeline execution

### Two Contexts
1. **Struct/Enum:** Field mapping at definition time
2. **Pipeline:** Entire content loading with `[.] << *`

### Advantages
- ✅ Automatic parallelism (I/O optimization)
- ✅ Clean syntax for multi-file loading
- ✅ Unified error handling
- ✅ Type-safe deserialization

### Limitations
- ❌ Cannot distinguish which file failed (unified errors)
- ❌ Field names must match exactly
- ❌ No partial success (all-or-nothing)

---

## Related Documentation

- [Markers Reference](../../User/language/syntax/markers.md) - `[s]` marker details
- [Variables & Lifecycle](../../User/language/types/variables-lifecycle.md) - Variable states
- [Error Handling](../error-handling/error-handling.md) - Error patterns
- [Standard Library - Loaders](../../User/stdlib/utilities/data/README.md) - `|YAML.Load`, `|JSON.Load`

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../README.md)
