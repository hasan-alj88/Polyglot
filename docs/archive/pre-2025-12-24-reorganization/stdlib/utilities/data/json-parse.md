---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "json-parse"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.Data.JSON.Parse"
summary: "API reference: |U.Data.JSON.Parse"
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
# |U.Data.JSON.Parse

**Parse JSON string**

**Category:** Utilities > Data > JSON
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.Data.JSON.Parse <json >result
```

**Inline:**
```polyglot
\|U.Data.JSON.Parse"{$json}"
```

---

## Parameters

**Inputs:**
- `<json` :pg.string - JSON string to parse

**Outputs:**
- `>result` :pg.serial - Parsed JSON data

---

## Description

Parses a JSON-formatted string into a `:pg.serial` structure.

**Produces error:**
- `!Data.JSON.ParseError` - Invalid JSON syntax

---

## Examples

### Basic Usage

```polyglot
[r] $json_string :pg.string << "{\\\"name\\\": \\\"Alice\\\", \\\"age\\\": 30}"
[r] $data :pg.serial << \|U.Data.JSON.Parse"{$json_string}"
```

**Output:** Serial with `name="Alice"`, `age=30`

---

### Parse API Response

```polyglot
[r] $response :pg.serial << !HTTPGet <url << "https://api.example.com/user/123"
[r] $body :pg.string << $response."body"
[r] $user_data :pg.serial << \|U.Data.JSON.Parse"{$body}"
[r] $username :pg.string << $user_data."name"
[r] $email :pg.string << $user_data."email"
```

---

### Parse Nested JSON

```polyglot
[r] $json :pg.string << "{\\\"server\\\": {\\\"host\\\": \\\"localhost\\\", \\\"port\\\": 8080}}"
[r] $config :pg.serial << \|U.Data.JSON.Parse"{$json}"
[r] $host :pg.string << $config."server.host"
[r] $port :pg.int << $config."server.port"
```

---

### Handle Parse Errors

```polyglot
[r] $data :pg.serial << \|U.Data.JSON.Parse"{$untrusted_input}"

[!] !Data.JSON.ParseError
   [r] !Validation.InvalidJSON << "Invalid JSON format in input"
```

---

### Parse Array

```polyglot
[r] $json :pg.string << "[\\\"apple\\\", \\\"banana\\\", \\\"cherry\\\"]"
[r] $data :pg.serial << \|U.Data.JSON.Parse"{$json}"
[r] $fruits :pg.array.pg.string << $data."$root"
```

---

## Common Patterns

### Pattern 1: Parse and Validate API Response

```polyglot
[r] $response_body :pg.string << $api_response."body"
[r] $data :pg.serial << \|U.Data.JSON.Parse"{$response_body}"

[!] !Data.JSON.ParseError
   [r] !API.InvalidResponse << "API returned invalid JSON"
   [v] [r] [^]

[r] $status :pg.string << $data."status"

[f] $status != "success"
   [r] !API.RequestFailed << $data."error"
```

### Pattern 2: Parse WebSocket Message

```polyglot
[r] $ws_message :pg.string << $websocket."message"
[r] $msg_data :pg.serial << \|U.Data.JSON.Parse"{$ws_message}"

[!] !Data.JSON.ParseError
   // Invalid message format, ignore
   [v] [r] [^]

[r] $msg_type :pg.string << $msg_data."type"
[r] $payload :pg.serial << $msg_data."payload"

[f] $msg_type == "update"
   // Handle update
[&] $msg_type == "notification"
   // Handle notification
```

### Pattern 3: Parse Multiple JSON Objects

```polyglot
[r] $json_lines :pg.array.pg.string << \|U.String.Split"{$ndjson, \"\\n\"}"

[r] ~ForEach.Array
[~] <array << $json_lines
[~] >item >> $json_line
   [r] $trimmed :pg.string << \|U.String.Trim"{$json_line}"

   [f] \|U.String.Length"{$trimmed}" == 0
      // Skip empty lines
      [v] [r] [^]

   [r] $data :pg.serial << \|U.Data.JSON.Parse"{$trimmed}"

   [!] !Data.JSON.ParseError
      // Skip invalid lines
      [v] [r] [^]

   [v] *Into.Array
   [*] <item << $data
   [*] >array >> $parsed_objects
```

---

## JSON Features Supported

**Primitives:**
- Strings → `:pg.string`
- Numbers → `:pg.int` or `:pg.float`
- Booleans → `:pg.bool`
- Null → Undefined/empty

**Collections:**
- Objects → `:pg.serial`
- Arrays → `:pg.array.*`

**Structure:**
- Nested objects and arrays
- Unicode escape sequences
- Whitespace (ignored)

**Strict JSON compliance:**
- Keys must be quoted strings
- No trailing commas
- No comments

---

## Related Pipelines

- [|U.Data.JSON.Load](./json-load.md) - Load JSON from file
- [|U.Data.JSON.Dump](./json-dump.md) - Serialize to JSON string
- [|U.Data.YAML.Parse](./yaml-parse.md) - Parse YAML string

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
