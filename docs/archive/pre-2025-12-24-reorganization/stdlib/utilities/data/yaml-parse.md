---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "yaml-parse"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Data.YAML.Parse"
summary: "API reference: |U.Data.YAML.Parse"
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
# |U.Data.YAML.Parse

**Parse YAML string**

**Category:** Utilities > Data > YAML
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Data.YAML.Parse <yaml >result
```

**Inline:**
```polyglot
\|U.Data.YAML.Parse"{$yaml}"
```

---

## Parameters

**Inputs:**
- `<yaml` :pg.string - YAML string to parse

**Outputs:**
- `>result` :pg.serial - Parsed YAML data

---

## Description

Parses a YAML-formatted string into a `:pg.serial` structure.

**Produces error:**
- `!Data.YAML.ParseError` - Invalid YAML syntax

---

## Examples

### Basic Usage

```polyglot
[r] $yaml_string :pg.string << "name: Alice\\nage: 30"
[r] $data :pg.serial << \|U.Data.YAML.Parse"{$yaml_string}"
```

**Output:** Serial with `name="Alice"`, `age=30`

---

### Parse API Response

```polyglot
[r] $response_body :pg.string << $api_response."body"
[r] $data :pg.serial << \|U.Data.YAML.Parse"{$response_body}"
[r] $status :pg.string << $data."status"
```

---

### Parse Nested Structure

```polyglot
[r] $yaml :pg.string << "server:\\n  host: localhost\\n  port: 8080"
[r] $config :pg.serial << \|U.Data.YAML.Parse"{$yaml}"
[r] $host :pg.string << $config."server.host"
[r] $port :pg.int << $config."server.port"
```

---

### Handle Parse Errors

```polyglot
[r] $data :pg.serial << \|U.Data.YAML.Parse"{$untrusted_input}"

[!] !Data.YAML.ParseError
   [r] !Validation.InvalidFormat << "Invalid YAML format in input"
```

---

### Parse Array

```polyglot
[r] $yaml :pg.string << "- apple\\n- banana\\n- cherry"
[r] $data :pg.serial << \|U.Data.YAML.Parse"{$yaml}"
[r] $fruits :pg.array.pg.string << $data."$root"
```

---

## Common Patterns

### Pattern 1: Parse Configuration String

```polyglot
[r] $config_str :pg.string << $environment_variable."APP_CONFIG"
[r] $config :pg.serial << \|U.Data.YAML.Parse"{$config_str}"

[!] !Data.YAML.ParseError
   // Use default config
   [r] $config :pg.serial << #Serial
      debug << false
```

### Pattern 2: Parse and Validate

```polyglot
[r] $data :pg.serial << \|U.Data.YAML.Parse"{$yaml_input}"

[!] !Data.YAML.ParseError
   [r] !Validation.ParseFailed << "Invalid YAML"
   [v] [r] [^]

[r] $required_field :pg.string << $data."name"

[!] !Serial.FieldNotFound
   [r] !Validation.MissingField << "Missing 'name' field"
```

### Pattern 3: Parse Multiple YAML Documents

```polyglot
[r] $yaml_docs :pg.array.pg.string << \|U.String.Split"{$multi_doc_yaml, \"---\"}"

[r] ~ForEach.Array
[~] <array << $yaml_docs
[~] >item >> $yaml_str
   [r] $trimmed :pg.string << \|U.String.Trim"{$yaml_str}"
   [r] $data :pg.serial << \|U.Data.YAML.Parse"{$trimmed}"
   [v] *Into.Array
   [*] <item << $data
   [*] >array >> $parsed_docs
```

---

## YAML Features Supported

**Scalars:**
- Strings (quoted and unquoted)
- Numbers (integers and floats)
- Booleans (`true`, `false`)
- Null (`null`, `~`)

**Collections:**
- Mappings (objects) → `:pg.serial`
- Sequences (arrays) → `:pg.array.*`

**Structure:**
- Nested objects and arrays
- Multi-line strings
- Comments (stripped during parsing)

---

## Related Pipelines

- [|U.Data.YAML.Load](./yaml-load.md) - Load YAML from file
- [|U.Data.YAML.Dump](./yaml-dump.md) - Serialize to YAML string
- [|U.Data.JSON.Parse](./json-parse.md) - Parse JSON string

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
