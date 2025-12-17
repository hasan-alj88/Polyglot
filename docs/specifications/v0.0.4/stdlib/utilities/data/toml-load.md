---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "toml-load"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Data.TOML.Load"
summary: "API reference: |U.Data.TOML.Load"
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
# |U.Data.TOML.Load

**Load TOML from file**

**Category:** Utilities > Data > TOML
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Data.TOML.Load <path >result
```

**Inline:**
```polyglot
\|U.Data.TOML.Load"{$path}"
```

---

## Parameters

**Inputs:**
- `<path` :pg.string - File path (relative or absolute)

**Outputs:**
- `>result` :pg.serial - Parsed TOML data

---

## Description

Loads and parses a TOML file into a `:pg.serial` structure.

**Produces errors:**
- `!Data.TOML.FileNotFound` - File doesn't exist
- `!Data.TOML.ReadError` - Cannot read file (permissions, etc.)
- `!Data.TOML.ParseError` - Invalid TOML syntax

---

## Examples

### Basic Usage

```polyglot
[r] $config :pg.serial << \|U.Data.TOML.Load"{\"config.toml\"}"
```

---

### Load and Access Fields

**File: `app.toml`**
```toml
[server]
host = "localhost"
port = 8080

[database]
url = "postgresql://localhost/mydb"
max_connections = 20
```

**Code:**
```polyglot
[r] $config :pg.serial << \|U.Data.TOML.Load"{\"app.toml\"}"
[r] $host :pg.string << $config."server.host"
[r] $port :pg.int << $config."server.port"
[r] $db_url :pg.string << $config."database.url"
[r] $max_conn :pg.int << $config."database.max_connections"
```

**Output:** `$host = "localhost"`, `$port = 8080`

---

### Handle Load Errors

```polyglot
[r] $config :pg.serial << \|U.Data.TOML.Load"{\"config.toml\"}"

[!] !Data.TOML.FileNotFound
   [r] !Config.Missing << "Configuration file not found"
[!] !Data.TOML.ParseError
   [r] !Config.Invalid << "Invalid TOML syntax in configuration"
```

---

### Load Array from TOML

**File: `servers.toml`**
```toml
[[servers]]
name = "web1"
ip = "192.168.1.1"

[[servers]]
name = "web2"
ip = "192.168.1.2"
```

**Code:**
```polyglot
[r] $config :pg.serial << \|U.Data.TOML.Load"{\"servers.toml\"}"
[r] $servers :pg.array.pg.serial << $config."servers"
```

---

## Common Patterns

### Pattern 1: Load Application Configuration

```polyglot
[r] $config :pg.serial << \|U.Data.TOML.Load"{\"config.toml\"}"

[!] !Data.TOML.FileNotFound
   // Use default configuration
   [r] $config :pg.serial << #Serial
      server << #Serial
         host << "localhost"
         port << 8080
```

### Pattern 2: Load Rust-Style Cargo Configuration

```polyglot
[r] $cargo :pg.serial << \|U.Data.TOML.Load"{\"Cargo.toml\"}"
[r] $package_name :pg.string << $cargo."package.name"
[r] $version :pg.string << $cargo."package.version"
[r] $dependencies :pg.serial << $cargo."dependencies"
```

### Pattern 3: Load and Validate Required Fields

```polyglot
[r] $config :pg.serial << \|U.Data.TOML.Load"{\"app.toml\"}"

[r] $required_keys :pg.array.pg.string << ["server.host", "server.port", "database.url"]

[r] ~ForEach.Array
[~] <array << $required_keys
[~] >item >> $key
   [r] $value :* << $config.$key

   [!] !Serial.FieldNotFound
      [r] !Config.MissingField << \|U.String.Concat"{\"Missing required field: \", $key}"
```

---

## TOML Features

**Tables (sections):**
```toml
[server]
host = "localhost"
```
Maps to: `$config."server.host"`

**Nested tables:**
```toml
[database.connection]
host = "localhost"
```
Maps to: `$config."database.connection.host"`

**Arrays of tables:**
```toml
[[users]]
name = "Alice"

[[users]]
name = "Bob"
```
Maps to: `$config."users"` (array)

**Inline tables:**
```toml
server = { host = "localhost", port = 8080 }
```

**Data types:**
- Strings
- Integers
- Floats
- Booleans
- Dates/Times → `:pg.datetime`
- Arrays
- Tables (objects)

---

## File Path Resolution

**Relative paths:** Resolved from current working directory
```polyglot
[r] $config :pg.serial << \|U.Data.TOML.Load"{\"config.toml\"}"
[r] $config :pg.serial << \|U.Data.TOML.Load"{\"./configs/app.toml\"}"
```

**Absolute paths:**
```polyglot
[r] $config :pg.serial << \|U.Data.TOML.Load"{\"/etc/myapp/config.toml\"}"
```

---

## Common Use Cases

**Configuration files:**
- Application settings
- Build configurations (Cargo.toml, pyproject.toml)
- Tool configurations

**Benefits of TOML:**
- Human-readable
- Easy to edit
- Explicit structure with sections
- Strong typing

---

## Related Pipelines

- [|U.Data.TOML.Parse](./toml-parse.md) - Parse TOML string
- [|U.Data.YAML.Load](./yaml-load.md) - Load YAML file
- [|U.Data.JSON.Load](./json-load.md) - Load JSON file

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
