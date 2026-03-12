---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "yaml-dump"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Data.YAML.Dump"
summary: "API reference: |U.Data.YAML.Dump"
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
# |U.Data.YAML.Dump

**Serialize to YAML string**

**Category:** Utilities > Data > YAML
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Data.YAML.Dump <data >result
```

**Inline:**
```polyglot
\|U.Data.YAML.Dump"{$data}"
```

---

## Parameters

**Inputs:**
- `<data` :pg.serial - Data to serialize

**Outputs:**
- `>result` :pg.string - YAML-formatted string

---

## Description

Serializes a `:pg.serial` structure into a YAML-formatted string.

---

## Examples

### Basic Usage

```polyglot
[r] $data :pg.serial << #Serial
   name << "Alice"
   age << 30

[r] $yaml :pg.string << \|U.Data.YAML.Dump"{$data}"
```

**Output:**
```yaml
name: Alice
age: 30
```

---

### Nested Structure

```polyglot
[r] $config :pg.serial << #Serial
   server << #Serial
      host << "localhost"
      port << 8080
   database << #Serial
      url << "postgresql://localhost/mydb"

[r] $yaml :pg.string << \|U.Data.YAML.Dump"{$config}"
```

**Output:**
```yaml
server:
  host: localhost
  port: 8080
database:
  url: postgresql://localhost/mydb
```

---

### Serialize Array

```polyglot
[r] $users :pg.array.pg.serial << [
   #Serial (name << "Alice", age << 30),
   #Serial (name << "Bob", age << 25)
]

[r] $data :pg.serial << #Serial
   users << $users

[r] $yaml :pg.string << \|U.Data.YAML.Dump"{$data}"
```

**Output:**
```yaml
users:
  - name: Alice
    age: 30
  - name: Bob
    age: 25
```

---

### Generate Configuration File

```polyglot
[r] $config :pg.serial << #Serial
   app_name << "MyApp"
   version << "1.0.0"
   features << #Serial
      auth << true
      logging << true
      debug << false

[r] $yaml_output :pg.string << \|U.Data.YAML.Dump"{$config}"

// Write to file (using hypothetical file write pipeline)
[r] !WriteFile <path << "config.yaml" <content << $yaml_output
```

---

## Common Patterns

### Pattern 1: Convert JSON to YAML

```polyglot
[r] $json_str :pg.string << $api_response."body"
[r] $data :pg.serial << \|U.Data.JSON.Parse"{$json_str}"
[r] $yaml_str :pg.string << \|U.Data.YAML.Dump"{$data}"
```

### Pattern 2: Build and Export Configuration

```polyglot
[r] $config :pg.serial << #Serial
   environment << $env
   database << #Serial
      host << $db_host
      port << $db_port
      name << $db_name

[r] $yaml :pg.string << \|U.Data.YAML.Dump"{$config}"
```

### Pattern 3: Serialize Collected Data

```polyglot
[r] ~ForEach.Array
[~] <array << $records
[~] >item >> $record
   [r] $entry :pg.serial << #Serial
      id << $record."id"
      name << $record."name"

   [v] *Into.Array
   [*] <item << $entry
   [*] >array >> $entries

[r] $output :pg.serial << #Serial
   records << $entries

[r] $yaml :pg.string << \|U.Data.YAML.Dump"{$output}"
```

---

## Data Type Serialization

**From Polyglot to YAML:**
- `:pg.string` → String
- `:pg.int` → Integer
- `:pg.float` → Float
- `:pg.bool` → Boolean (`true`/`false`)
- `:pg.serial` → Mapping (object)
- `:pg.array.*` → Sequence (array)

**Formatting:**
- Nested structures are indented (2 spaces)
- Arrays use `- ` prefix
- Strings with special characters are quoted
- Multi-line strings use `|` or `>` indicators

---

## Output Format

**Clean, human-readable YAML:**
- Proper indentation
- No unnecessary quotes
- Readable structure
- Compatible with standard YAML parsers

---

## Related Pipelines

- [|U.Data.YAML.Parse](./yaml-parse.md) - Parse YAML string
- [|U.Data.YAML.Load](./yaml-load.md) - Load YAML from file
- [|U.Data.JSON.Dump](./json-dump.md) - Serialize to JSON string

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
