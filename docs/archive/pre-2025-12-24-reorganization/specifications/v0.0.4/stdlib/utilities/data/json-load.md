---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "json-load"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Data.JSON.Load"
summary: "API reference: |U.Data.JSON.Load"
keywords:
  - stdlib
  - api
  - reference

# --- BMAD Agent Routing ---
agents:
  - developer
phase: implementation
workflow: any
module: bmm
complexity: low

# --- Dependency Chain ---
prereqs:
  - type-system
unlocks:
  []

# --- Relationships ---
related:
  []

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#stdlib"
  - "#api"
---
# |U.Data.JSON.Load

**Load JSON from file**

**Category:** Utilities > Data > JSON
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Data.JSON.Load <path >result
```

**Inline:**
```polyglot
\|U.Data.JSON.Load"{$path}"
```

---

## Parameters

**Inputs:**
- `<path` :pg.string - File path (relative or absolute)

**Outputs:**
- `>result` :pg.serial - Parsed JSON data

---

## Description

Loads and parses a JSON file into a `:pg.serial` structure.

**Produces errors:**
- `!Data.JSON.FileNotFound` - File doesn't exist
- `!Data.JSON.ReadError` - Cannot read file (permissions, etc.)
- `!Data.JSON.ParseError` - Invalid JSON syntax

---

## Examples

### Basic Usage

```polyglot
[r] $data :pg.serial << \|U.Data.JSON.Load"{\"data.json\"}"
```

---

### Load and Access Fields

**File: `config.json`**
```json
{
  "server": {
    "host": "localhost",
    "port": 8080
  },
  "database": {
    "url": "postgresql://localhost/mydb"
  }
}
```

**Code:**
```polyglot
[r] $config :pg.serial << \|U.Data.JSON.Load"{\"config.json\"}"
[r] $host :pg.string << $config."server.host"
[r] $port :pg.int << $config."server.port"
[r] $db_url :pg.string << $config."database.url"
```

**Output:** `$host = "localhost"`, `$port = 8080`

---

### Handle Load Errors

```polyglot
[r] $data :pg.serial << \|U.Data.JSON.Load"{$file_path}"

[!] !Data.JSON.FileNotFound
   [r] !App.ConfigMissing << "Configuration file not found"
[!] !Data.JSON.ParseError
   [r] !App.ConfigInvalid << "Invalid JSON syntax in configuration"
```

---

### Load Array from File

**File: `users.json`**
```json
[
  {"name": "Alice", "age": 30},
  {"name": "Bob", "age": 25}
]
```

**Code:**
```polyglot
[r] $users_serial :pg.serial << \|U.Data.JSON.Load"{\"users.json\"}"
[r] $users :pg.array.pg.serial << $users_serial."$root"  // Root level array
```

---

## Common Patterns

### Pattern 1: Load API Response Cache

```polyglot
[r] $cached :pg.serial << \|U.Data.JSON.Load"{\"cache/api_response.json\"}"

[!] !Data.JSON.FileNotFound
   // Make fresh API call
   [r] $cached :pg.serial << !CallAPI
```

### Pattern 2: Load Multiple JSON Files

```polyglot
[r] $file_names :pg.array.pg.string << ["users.json", "products.json", "orders.json"]

[r] ~ForEach.Array
[~] <array << $file_names
[~] >item >> $filename
   [r] $path :pg.string << \|U.String.Concat"{\"data/\", $filename}"
   [r] $data :pg.serial << \|U.Data.JSON.Load"{$path}"
   [v] *Into.Array
   [*] <item << $data
   [*] >array >> $all_data
```

### Pattern 3: Load and Merge Configurations

```polyglot
[r] $default_config :pg.serial << \|U.Data.JSON.Load"{\"config.default.json\"}"
[r] $user_config :pg.serial << \|U.Data.JSON.Load"{\"config.user.json\"}"

[!] !Data.JSON.FileNotFound
   // User config is optional, use only defaults
   [r] $merged :pg.serial << $default_config
   [v] [r] [^]

// Merge configs (user overrides default)
[r] $merged :pg.serial << !MergeSerials <default << $default_config <override << $user_config
```

---

## File Path Resolution

**Relative paths:** Resolved from current working directory
```polyglot
[r] $data :pg.serial << \|U.Data.JSON.Load"{\"data.json\"}"  // ./data.json
[r] $data :pg.serial << \|U.Data.JSON.Load"{\"./api/response.json\"}"
```

**Absolute paths:**
```polyglot
[r] $data :pg.serial << \|U.Data.JSON.Load"{\"/var/data/records.json\"}"
```

---

## Related Pipelines

- [|U.Data.JSON.Parse](./json-parse.md) - Parse JSON string
- [|U.Data.JSON.Dump](./json-dump.md) - Serialize to JSON string
- [|U.Data.YAML.Load](./yaml-load.md) - Load YAML file

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
