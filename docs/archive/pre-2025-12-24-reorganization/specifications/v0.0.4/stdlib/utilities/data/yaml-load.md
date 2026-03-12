---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "yaml-load"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Data.YAML.Load"
summary: "API reference: |U.Data.YAML.Load"
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
# |U.Data.YAML.Load

**Load YAML from file**

**Category:** Utilities > Data > YAML
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Data.YAML.Load <path >result
```

**Inline:**
```polyglot
\|U.Data.YAML.Load"{$path}"
```

---

## Parameters

**Inputs:**
- `<path` :pg.string - File path (relative or absolute)

**Outputs:**
- `>result` :pg.serial - Parsed YAML data

---

## Description

Loads and parses a YAML file into a `:pg.serial` structure.

**Produces errors:**
- `!Data.YAML.FileNotFound` - File doesn't exist
- `!Data.YAML.ReadError` - Cannot read file (permissions, etc.)
- `!Data.YAML.ParseError` - Invalid YAML syntax

---

## Examples

### Basic Usage

```polyglot
[r] $config :pg.serial << \|U.Data.YAML.Load"{\"config.yaml\"}"
```

---

### Load and Access Fields

**File: `config.yaml`**
```yaml
server:
  host: localhost
  port: 8080
database:
  url: postgresql://localhost/mydb
```

**Code:**
```polyglot
[r] $config :pg.serial << \|U.Data.YAML.Load"{\"config.yaml\"}"
[r] $host :pg.string << $config."server.host"
[r] $port :pg.int << $config."server.port"
[r] $db_url :pg.string << $config."database.url"
```

**Output:** `$host = "localhost"`, `$port = 8080`

---

### Handle Load Errors

```polyglot
[r] $config :pg.serial << \|U.Data.YAML.Load"{$config_path}"

[!] !Data.YAML.FileNotFound
   [r] !Config.Missing << "Configuration file not found"
[!] !Data.YAML.ParseError
   [r] !Config.Invalid << "Invalid YAML syntax in configuration"
```

---

### Load Array from File

**File: `users.yaml`**
```yaml
- name: Alice
  age: 30
- name: Bob
  age: 25
```

**Code:**
```polyglot
[r] $users_data :pg.serial << \|U.Data.YAML.Load"{\"users.yaml\"}"
[r] $users :pg.array.pg.serial << $users_data."$root"  // Root level array
```

---

## Common Patterns

### Pattern 1: Load Application Configuration

```polyglot
[r] $config :pg.serial << \|U.Data.YAML.Load"{\"config.yaml\"}"

[!] !Data.YAML.FileNotFound
   // Use default configuration
   [r] $config :pg.serial << #Serial
      server << #Serial
         host << "localhost"
         port << 8080
```

### Pattern 2: Load Multiple Config Files

```polyglot
[r] $app_config :pg.serial << \|U.Data.YAML.Load"{\"app.yaml\"}"
[r] $db_config :pg.serial << \|U.Data.YAML.Load"{\"database.yaml\"}"

[r] $merged :pg.serial << #Serial
   app << $app_config
   database << $db_config
```

### Pattern 3: Load and Validate

```polyglot
[r] $config :pg.serial << \|U.Data.YAML.Load"{\"config.yaml\"}"

[r] $required_keys :pg.array.pg.string << ["server.host", "server.port", "database.url"]

[r] ~ForEach.Array
[~] <array << $required_keys
[~] >item >> $key
   [r] $value :* << $config.$key

   [!] !Serial.FieldNotFound
      [r] !Config.MissingField << \|U.String.Concat"{\"Missing required field: \", $key}"
```

---

## File Path Resolution

**Relative paths:** Resolved from current working directory
```polyglot
[r] $config :pg.serial << \|U.Data.YAML.Load"{\"config.yaml\"}"  // ./config.yaml
[r] $config :pg.serial << \|U.Data.YAML.Load"{\"./configs/app.yaml\"}"
```

**Absolute paths:**
```polyglot
[r] $config :pg.serial << \|U.Data.YAML.Load"{\"/etc/myapp/config.yaml\"}"
```

---

## Related Pipelines

- [|U.Data.YAML.Parse](./yaml-parse.md) - Parse YAML string
- [|U.Data.YAML.Dump](./yaml-dump.md) - Serialize to YAML string
- [|U.Data.JSON.Load](./json-load.md) - Load JSON file

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
