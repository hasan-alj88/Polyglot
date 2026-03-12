---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: readme
shard: false

# --- Classification ---
type: api
topic: Data Utilities
summary: API reference: Data Utilities
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
# Data Utilities

**Package:** `|U.Data.*`
**Category:** Utilities
**Since:** v0.0.1

---

## Overview

Data utilities provide parsing, loading, and dumping capabilities for common data formats including YAML, JSON, TOML, and XML.

---

## Package Structure

**YAML**
- [**|U.Data.YAML.Load**](./yaml-load.md) - Load YAML from file
- [**|U.Data.YAML.Parse**](./yaml-parse.md) - Parse YAML string
- [**|U.Data.YAML.Dump**](./yaml-dump.md) - Serialize to YAML string

**JSON**
- [**|U.Data.JSON.Load**](./json-load.md) - Load JSON from file
- [**|U.Data.JSON.Parse**](./json-parse.md) - Parse JSON string
- [**|U.Data.JSON.Dump**](./json-dump.md) - Serialize to JSON string

**TOML**
- [**|U.Data.TOML.Load**](./toml-load.md) - Load TOML from file
- [**|U.Data.TOML.Parse**](./toml-parse.md) - Parse TOML string

**XML**
- [**|U.Data.XML.Parse**](./xml-parse.md) - Parse XML string

---

## Quick Reference

### YAML Operations

| Pipeline | Inputs | Outputs | Description |
|----------|--------|---------|-------------|
| `\|U.Data.YAML.Load` | `<path :pg.string` | `>result :pg.serial` | Load YAML file |
| `\|U.Data.YAML.Parse` | `<yaml :pg.string` | `>result :pg.serial` | Parse YAML string |
| `\|U.Data.YAML.Dump` | `<data :pg.serial` | `>result :pg.string` | Serialize to YAML |

### JSON Operations

| Pipeline | Inputs | Outputs | Description |
|----------|--------|---------|-------------|
| `\|U.Data.JSON.Load` | `<path :pg.string` | `>result :pg.serial` | Load JSON file |
| `\|U.Data.JSON.Parse` | `<json :pg.string` | `>result :pg.serial` | Parse JSON string |
| `\|U.Data.JSON.Dump` | `<data :pg.serial` | `>result :pg.string` | Serialize to JSON |

### TOML Operations

| Pipeline | Inputs | Outputs | Description |
|----------|--------|---------|-------------|
| `\|U.Data.TOML.Load` | `<path :pg.string` | `>result :pg.serial` | Load TOML file |
| `\|U.Data.TOML.Parse` | `<toml :pg.string` | `>result :pg.serial` | Parse TOML string |

### XML Operations

| Pipeline | Inputs | Outputs | Description |
|----------|--------|---------|-------------|
| `\|U.Data.XML.Parse` | `<xml :pg.string` | `>result :pg.serial` | Parse XML string |

---

## Common Patterns

### Pattern 1: Load Configuration File

```polyglot
[r] $config :pg.serial << \|U.Data.YAML.Load"{\"config.yaml\"}"
[r] $db_host :pg.string << $config."database.host"
[r] $db_port :pg.int << $config."database.port"
```

### Pattern 2: Parse API Response

```polyglot
[r] $json_string :pg.string << $api_response."body"
[r] $data :pg.serial << \|U.Data.JSON.Parse"{$json_string}"
[r] $user_name :pg.string << $data."user.name"
```

### Pattern 3: Convert Between Formats

```polyglot
[r] $yaml_data :pg.serial << \|U.Data.YAML.Load"{\"input.yaml\"}"
[r] $json_string :pg.string << \|U.Data.JSON.Dump"{$yaml_data}"
```

### Pattern 4: Generate Configuration File

```polyglot
[r] $config :pg.serial << #Serial
   server << #Serial
      host << "localhost"
      port << 8080
   database << #Serial
      url << "postgresql://localhost/mydb"

[r] $yaml_output :pg.string << \|U.Data.YAML.Dump"{$config}"
```

---

## Error Handling

All data utilities produce format-specific errors:

**YAML:**
- `!Data.YAML.ParseError` - Invalid YAML syntax
- `!Data.YAML.FileNotFound` - File doesn't exist
- `!Data.YAML.ReadError` - Cannot read file

**JSON:**
- `!Data.JSON.ParseError` - Invalid JSON syntax
- `!Data.JSON.FileNotFound` - File doesn't exist
- `!Data.JSON.ReadError` - Cannot read file

**TOML:**
- `!Data.TOML.ParseError` - Invalid TOML syntax
- `!Data.TOML.FileNotFound` - File doesn't exist
- `!Data.TOML.ReadError` - Cannot read file

**XML:**
- `!Data.XML.ParseError` - Invalid XML syntax

---

## Data Type Mapping

**From format to :pg.serial:**
- Objects/Maps → Serial with fields
- Arrays/Lists → `:pg.array.*`
- Strings → `:pg.string`
- Numbers → `:pg.int` or `:pg.float`
- Booleans → `:pg.bool`
- Null/None → Empty/undefined behavior

**From :pg.serial to format:**
- Serial fields → Objects/Maps
- `:pg.array.*` → Arrays/Lists
- `:pg.string` → Strings
- `:pg.int`, `:pg.float` → Numbers
- `:pg.bool` → Booleans

---

## File Path Handling

**Load operations accept:**
- Relative paths (from current working directory)
- Absolute paths
- File must be readable

**Example paths:**
```polyglot
[r] $data1 :pg.serial << \|U.Data.YAML.Load"{\"config.yaml\"}"
[r] $data2 :pg.serial << \|U.Data.YAML.Load"{\"./configs/app.yaml\"}"
[r] $data3 :pg.serial << \|U.Data.YAML.Load"{\"/etc/myapp/settings.yaml\"}"
```

---

## Related Packages

- [String Utilities](../string/README.md) - For string manipulation before/after parsing
- [Serial Load](../../../core-syntax/serial-load.md) - For inline serial construction

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
