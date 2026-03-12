---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: concat
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "|U.String.Concat"
summary: "API reference: |U.String.Concat"
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
# |U.String.Concat

**Concatenate multiple strings**

**Category:** Utilities > String
**Since:** v0.0.1

---

## Signature

```polyglot
\|U.String.Concat <strings... >result
```

**Inline (variadic):**
```polyglot
\|U.String.Concat"{$str1, $str2, $str3, ...}"
```

---

## Parameters

**Inputs:**
- `<strings...` - Variable number of strings to concatenate

**Outputs:**
- `>result` :pg.string - Concatenated string

---

## Description

Joins multiple strings together into a single string. Accepts variable number of arguments.

**Operation:** `result = str1 + str2 + str3 + ...`

---

## Examples

### Basic Usage

```polyglot
[r] $greeting :pg.string << \|U.String.Concat"{\"Hello\", \" \", \"World\"}"
```

**Output:** `$greeting = "Hello World"`

---

### Build Full Name

```polyglot
[r] $first :pg.string << "John"
[r] $last :pg.string << "Doe"
[r] $full_name :pg.string << \|U.String.Concat"{$first, \" \", $last}"
```

**Output:** `$full_name = "John Doe"`

---

### Build Path

```polyglot
[r] $base_path :pg.string << "/home/user"
[r] $filename :pg.string << "config.yaml"
[r] $full_path :pg.string << \|U.String.Concat"{$base_path, \"/\", $filename}"
```

**Output:** `$full_path = "/home/user/config.yaml"`

---

### Multiple Parts

```polyglot
[r] $message :pg.string << \|U.String.Concat"{
   \"User: \",
   $username,
   \" logged in at \",
   $timestamp
}"
```

---

### With Numbers

```polyglot
[r] $age :pg.int << 25
[r] $message :pg.string << \|U.String.Concat"{\"Age: \", $age, \" years\"}"
```

**Output:** `$message = "Age: 25 years"`

**Note:** Numbers are automatically converted to strings.

---

## Common Patterns

### Pattern 1: Format Message
```polyglot
[r] $msg :pg.string << \|U.String.Concat"{\"Error: \", $error_code, \" - \", $error_message}"
```

### Pattern 2: Build URL
```polyglot
[r] $url :pg.string << \|U.String.Concat"{\"https://api.example.com/users/\", $user_id}"
```

### Pattern 3: Multi-line Text
```polyglot
[r] $report :pg.string << \|U.String.Concat"{
   \"Name: \", $name, \"\\n\",
   \"Age: \", $age, \"\\n\",
   \"City: \", $city
}"
```

---

## Related Pipelines

- [*String.Concat](../../pack-operators/collection-building/string/string-concat.md) - Concatenate from iterations
- [|U.String.Split](./split.md) - Split strings

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../README.md)
