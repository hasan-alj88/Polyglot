---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "toml-parse"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Data.TOML.Parse"
summary: "API reference: |U.Data.TOML.Parse"
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
# |U.Data.TOML.Parse

**Parse TOML string**

**Category:** Utilities > Data > TOML
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Data.TOML.Parse <toml >result
```

**Inline:**
```polyglot
\|U.Data.TOML.Parse"{$toml}"
```

---

## Parameters

**Inputs:**
- `<toml` :pg.string - TOML string to parse

**Outputs:**
- `>result` :pg.serial - Parsed TOML data

---

## Description

Parses a TOML-formatted string into a `:pg.serial` structure.

**Produces error:**
- `!Data.TOML.ParseError` - Invalid TOML syntax

---

## Examples

### Basic Usage

```polyglot
[r] $toml_string :pg.string << "name = \\\"Alice\\\"\\nage = 30"
[r] $data :pg.serial << \|U.Data.TOML.Parse"{$toml_string}"
```

**Output:** Serial with `name="Alice"`, `age=30`

---

### Parse with Table

```polyglot
[r] $toml :pg.string << "[server]\\nhost = \\\"localhost\\\"\\nport = 8080"
[r] $config :pg.serial << \|U.Data.TOML.Parse"{$toml}"
[r] $host :pg.string << $config."server.host"
[r] $port :pg.int << $config."server.port"
```

---

### Parse Nested Tables

```polyglot
[r] $toml :pg.string << "[database.connection]\\nhost = \\\"localhost\\\"\\nport = 5432"
[r] $config :pg.serial << \|U.Data.TOML.Parse"{$toml}"
[r] $db_host :pg.string << $config."database.connection.host"
```

---

### Handle Parse Errors

```polyglot
[r] $data :pg.serial << \|U.Data.TOML.Parse"{$untrusted_input}"

[!] !Data.TOML.ParseError
   [r] !Validation.InvalidFormat << "Invalid TOML format in input"
```

---

### Parse Array of Tables

```polyglot
[r] $toml :pg.string << "[[users]]\\nname = \\\"Alice\\\"\\n\\n[[users]]\\nname = \\\"Bob\\\""
[r] $data :pg.serial << \|U.Data.TOML.Parse"{$toml}"
[r] $users :pg.array.pg.serial << $data."users"
```

---

## Common Patterns

### Pattern 1: Parse Configuration String

```polyglot
[r] $config_str :pg.string << $environment_variable."APP_CONFIG_TOML"
[r] $config :pg.serial << \|U.Data.TOML.Parse"{$config_str}"

[!] !Data.TOML.ParseError
   // Use default config
   [r] $config :pg.serial << #Serial
      debug << false
```

### Pattern 2: Parse and Validate

```polyglot
[r] $data :pg.serial << \|U.Data.TOML.Parse"{$toml_input}"

[!] !Data.TOML.ParseError
   [r] !Validation.ParseFailed << "Invalid TOML"
   [v] [r] [^]

[r] $required_field :pg.string << $data."server.host"

[!] !Serial.FieldNotFound
   [r] !Validation.MissingField << "Missing 'server.host' field"
```

### Pattern 3: Convert TOML to JSON

```polyglot
[r] $toml_str :pg.string << $input_toml
[r] $data :pg.serial << \|U.Data.TOML.Parse"{$toml_str}"
[r] $json_str :pg.string << \|U.Data.JSON.Dump"{$data}"
```

---

## TOML Features Supported

**Scalars:**
- Strings (basic and literal)
- Integers
- Floats
- Booleans (`true`, `false`)
- Dates and times → `:pg.datetime`

**Collections:**
- Tables (sections) → Serial with fields
- Arrays → `:pg.array.*`
- Inline tables
- Arrays of tables

**Structure:**
- Nested tables using dot notation `[a.b.c]`
- Table arrays using double brackets `[[array]]`
- Comments (stripped during parsing)

**String types:**
- Basic strings: `"string"`
- Literal strings: `'string'`
- Multi-line basic: `"""string"""`
- Multi-line literal: `'''string'''`

---

## TOML Syntax

**Tables:**
```toml
[server]
host = "localhost"
port = 8080
```

**Nested tables:**
```toml
[database.connection]
host = "localhost"
```

**Inline tables:**
```toml
server = { host = "localhost", port = 8080 }
```

**Arrays:**
```toml
ports = [8080, 8081, 8082]
```

**Arrays of tables:**
```toml
[[users]]
name = "Alice"
role = "admin"

[[users]]
name = "Bob"
role = "user"
```

---

## Related Pipelines

- [|U.Data.TOML.Load](./toml-load.md) - Load TOML from file
- [|U.Data.YAML.Parse](./yaml-parse.md) - Parse YAML string
- [|U.Data.JSON.Parse](./json-parse.md) - Parse JSON string

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
