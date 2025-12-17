---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: "string-lines"
shard: true
shard_of: stdlib

# --- Classification ---
type: api
topic: "*String.Lines"
summary: "API reference: *String.Lines"
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
complexity: medium

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
# *String.Lines

**Join strings with newlines**

**Category:** Collection Building > String
**Since:** v0.0.1

---

## Signature

```polyglot
[v] *String.Lines
[*] <line :pg.string
[*] >lines :pg.string
```

---

## Parameters

**Inputs:**
- `<line` :pg.string - Line from iteration scope

**Outputs:**
- `>lines` :pg.string - All lines joined with newlines (`\n`) in main scope

---

## Description

Joins all iteration strings with newline characters (`\n`) into a single multi-line string. This is equivalent to [*String.Concat](./string-concat.md) but with automatic newline separators.

**Order:**
- Sequential `[r]` loops: Lines in input order
- Parallel `[p]` loops: Order non-deterministic

---

## Examples

### Basic Usage

```polyglot
[r] ~ForEach.Array
[~] <array << $lines_array
[~] >item >> $line

   [v] *String.Lines
   [*] <line << $line
   [*] >lines >> $multi_line_text
```

**Input:** `$lines_array = ["Line 1", "Line 2", "Line 3"]`
**Output:**
```
$multi_line_text = "Line 1
Line 2
Line 3"
```

---

### Generate Multi-line File

```polyglot
[r] ~ForEach.Array
[~] <array << $log_entries
[~] >item >> $entry

   [r] $timestamp :pg.string << \|DT.Now""
   [r] $log_line :pg.string << \|U.String.Concat"{$timestamp, \" - \", $entry}"

   [v] *String.Lines
   [*] <line << $log_line
   [*] >lines >> $log_content
```

**Output:**
```
2025-12-15T10:00:00Z - Entry 1
2025-12-15T10:00:01Z - Entry 2
2025-12-15T10:00:02Z - Entry 3
```

---

### Build Configuration File

```polyglot
[r] ~ForEach.Array
[~] <array << $config_entries
[~] >item >> $entry

   [r] $line :pg.string << \|U.String.Concat"{$entry.key, \"=\", $entry.value}"

   [v] *String.Lines
   [*] <line << $line
   [*] >lines >> $config_file
```

**Input:** Config entries: `{key: "host", value: "localhost"}`, `{key: "port", value: "5432"}`
**Output:**
```
host=localhost
port=5432
```

---

### Generate CSV

```polyglot
[r] ~ForEach.Array
[~] <array << $rows
[~] >item >> $row

   [r] $csv_row :pg.string << \|U.String.Concat"{$row.col1, \",\", $row.col2, \",\", $row.col3}"

   [v] *String.Lines
   [*] <line << $csv_row
   [*] >lines >> $csv_content
```

**Output:**
```
Alice,30,Engineer
Bob,25,Designer
Charlie,35,Manager
```

---

### Build SQL Script

```polyglot
[r] ~ForEach.Array
[~] <array << $users
[~] >item >> $user

   [r] $sql :pg.string << \|U.String.Concat"{
      \"INSERT INTO users (name, email) VALUES ('\",
      $user.name,
      \"', '\",
      $user.email,
      \"');\"
   }"

   [v] *String.Lines
   [*] <line << $sql
   [*] >lines >> $sql_script
```

**Output:**
```sql
INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com');
INSERT INTO users (name, email) VALUES ('Bob', 'bob@example.com');
INSERT INTO users (name, email) VALUES ('Charlie', 'charlie@example.com');
```

---

## Trailing Newline

**Behavior:** No trailing newline is added after the last line.

```polyglot
[r] ~ForEach.Array
[~] <array << ["A", "B", "C"]
[~] >item >> $line

   [v] *String.Lines
   [*] <line << $line
   [*] >lines >> $result
```

**Output:** `"A\nB\nC"` (no trailing `\n`)

**To add trailing newline:**
```polyglot
[r] $with_trailing :pg.string << \|U.String.Concat"{$result, \"\n\"}"
```

---

## Comparison with Other Operators

| Operator | Separator | Use Case |
|----------|-----------|----------|
| `*String.Lines` | Newline (`\n`) | Multi-line text, files |
| `*tring.Concat` | None | Direct concatenation |
| `\|U.String.Join` | Custom | Any separator |

---

## Common Patterns

### Pattern 1: Generate File Content

```polyglot
[r] ~ForEach.Array
[~] <array << $content_lines
[~] >item >> $line

   [v] *String.Lines
   [*] <line << $line
   [*] >lines >> $file_content

[r] \|File.Write <path << "/output.txt" <content << $file_content
```

### Pattern 2: Build Report

```polyglot
[r] ~ForEach.Array
[~] <array << $report_data
[~] >item >> $data

   [r] $report_line :pg.string << \|FormatReportLine <data << $data

   [v] *String.Lines
   [*] <line << $report_line
   [*] >lines >> $report
```

### Pattern 3: Log Aggregation

```polyglot
[r] ~ForEach.Array
[~] <array << $events
[~] >item >> $event

   [r] $log :pg.string << \|FormatLogEntry <event << $event

   [v] *String.Lines
   [*] <line << $log
   [*] >lines >> $log_file
```

---

## File Writing Pattern

**Common pattern for writing multi-line files:**

```polyglot
{|} \|GenerateConfigFile
[|] <entries :pg.array.pg.serial
[|] >success :pg.bool

[t] \|T.Call
[W] \|W.Polyglot.Scope

   [r] ~ForEach.Array
   [~] <array << $entries
   [~] >item >> $entry

      [r] $line :pg.string << \|FormatConfigLine <entry << $entry

      [v] *String.Lines
      [*] <line << $line
      [*] >lines >> $config_content

   [r] \|File.Write
   [|] <path << "/etc/config.conf"
   [|] <content << $config_content

   [|] >success << #True

{x}
```

---

## Performance

**Time Complexity:** O(n * m) where:
- n = number of lines
- m = average line length

**Space Complexity:** O(total_length + n) where:
- total_length = sum of all line lengths
- n = newline characters

---

## Related Operators

- [*String.Concat](./string-concat.md) - Concatenate without separator
- [*Into.Array](../into/into-array.md) - Collect lines into array

---

## See Also

- [Loop System](../../../../language/advanced/loop-system.md)
- [Pack Operators Overview](../../README.md)
- [String Utilities](../../../utilities/string/README.md)

---

**Last Updated:** 2025-12-15
**Part of:** [v0.0.4 Specification](../../../../README.md)
