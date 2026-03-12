---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "json-dump"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Data.JSON.Dump"
summary: "API reference: |U.Data.JSON.Dump"
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
# |U.Data.JSON.Dump

**Serialize to JSON string**

**Category:** Utilities > Data > JSON
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Data.JSON.Dump <data >result
```

**Inline:**
```polyglot
\|U.Data.JSON.Dump"{$data}"
```

---

## Parameters

**Inputs:**
- `<data` :pg.serial - Data to serialize

**Outputs:**
- `>result` :pg.string - JSON-formatted string

---

## Description

Serializes a `:pg.serial` structure into a JSON-formatted string.

**Output is minified (no extra whitespace) by default.**

---

## Examples

### Basic Usage

```polyglot
[r] $data :pg.serial << #Serial
   name << "Alice"
   age << 30

[r] $json :pg.string << \|U.Data.JSON.Dump"{$data}"
```

**Output:** `"{\"name\":\"Alice\",\"age\":30}"`

---

### Nested Structure

```polyglot
[r] $config :pg.serial << #Serial
   server << #Serial
      host << "localhost"
      port << 8080
   database << #Serial
      url << "postgresql://localhost/mydb"

[r] $json :pg.string << \|U.Data.JSON.Dump"{$config}"
```

**Output:** `"{\"server\":{\"host\":\"localhost\",\"port\":8080},\"database\":{\"url\":\"postgresql://localhost/mydb\"}}"`

---

### Serialize Array

```polyglot
[r] $users :pg.array.pg.serial << [
   #Serial (name << "Alice", age << 30),
   #Serial (name << "Bob", age << 25)
]

[r] $data :pg.serial << #Serial
   users << $users

[r] $json :pg.string << \|U.Data.JSON.Dump"{$data}"
```

**Output:** `"{\"users\":[{\"name\":\"Alice\",\"age\":30},{\"name\":\"Bob\",\"age\":25}]}"`

---

### Send as API Request Body

```polyglot
[r] $payload :pg.serial << #Serial
   username << $username
   email << $email
   preferences << #Serial
      theme << "dark"
      notifications << true

[r] $json_body :pg.string << \|U.Data.JSON.Dump"{$payload}"

[r] $response :pg.serial << !HTTPPost
   <url << "https://api.example.com/users"
   <body << $json_body
   <content_type << "application/json"
```

---

## Common Patterns

### Pattern 1: Build API Request

```polyglot
[r] $request_body :pg.serial << #Serial
   action << "update"
   resource_id << $resource_id
   data << $update_data

[r] $json :pg.string << \|U.Data.JSON.Dump"{$request_body}"

[r] $response :pg.serial << !APICall <body << $json
```

### Pattern 2: Convert YAML to JSON

```polyglot
[r] $yaml_data :pg.serial << \|U.Data.YAML.Load"{\"config.yaml\"}"
[r] $json_output :pg.string << \|U.Data.JSON.Dump"{$yaml_data}"
```

### Pattern 3: Serialize Collected Results

```polyglot
[r] ~ForEach.Array
[~] <array << $items
[~] >item >> $item
   [r] $entry :pg.serial << #Serial
      id << $item."id"
      name << $item."name"
      status << $item."status"

   [v] *Into.Array
   [*] <item << $entry
   [*] >array >> $entries

[r] $result :pg.serial << #Serial
   items << $entries
   total << \|U.Math.Count <items << $entries

[r] $json_response :pg.string << \|U.Data.JSON.Dump"{$result}"
```

---

## Data Type Serialization

**From Polyglot to JSON:**
- `:pg.string` → String (escaped)
- `:pg.int` → Number
- `:pg.float` → Number
- `:pg.bool` → Boolean (`true`/`false`)
- `:pg.serial` → Object
- `:pg.array.*` → Array

**String escaping:**
- `"` → `\"`
- `\` → `\\`
- Newline → `\n`
- Tab → `\t`
- Unicode → `\uXXXX`

---

## Output Format

**Minified JSON:**
- No extra whitespace
- Compact representation
- Fully compliant JSON

**Benefits:**
- Smaller payload size
- Faster transmission
- Standard JSON parser compatible

**For pretty-printed JSON, use external formatting tools.**

---

## Related Pipelines

- [|U.Data.JSON.Parse](./json-parse.md) - Parse JSON string
- [|U.Data.JSON.Load](./json-load.md) - Load JSON from file
- [|U.Data.YAML.Dump](./yaml-dump.md) - Serialize to YAML string

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
