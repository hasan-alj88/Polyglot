---
audience: pg-coder
type: specification
updated: 2026-03-30
status: complete
---

# =File — File Operations

<!-- @errors -->
Stdlib pipelines for file system operations. No `[@]` import needed. See [[errors#Pipeline Error Associations]] for error contracts.

Success is signalled by `!NoError`. Side-effect-only pipelines (Write, Append, Copy, Move, Delete) have no output — `!NoError` confirms completion.

```polyglot
=File
   .Text
      .Read
         <path#path
         >content#string
      .Write
         <path#path
         <content#string
      .Append
         <path#path
         <content#string
   .Serial
      .Read
         <path#path
         >data#serial
      .Write
         <path#path
         <data#serial
      .Read.Field
         <path#path
         <field#RawString
         >value#serial
   .Copy
      <source#path
      <destination#path
   .Move
      <source#path
      <destination#path
   .Delete
      <path#path
   .Access
      <path#path
      >access#FileAccess
   .List
      <folder#path
      >files#array:path
```

## Serial File IO

`=File.Serial.*` pipelines load and save structured data files (JSON, YAML, TOML). Format is auto-detected from file extension. Internally delegates to `=#.JSON.Parse`, `=#.YAML.Parse`, or `=#.TOML.Parse` base parsers (see [[#|pipelines/#]]).

### `=File.Serial.Read`

Reads a file, detects format from extension (.json/.yaml/.toml), parses content, returns a `#serial` data tree.

| IO | Type | Description |
|----|------|-------------|
| `<path` | `#path` | File path to read |
| `>data` | `#serial` | Parsed data tree |

| Error | When |
|-------|------|
| `!File.NotFound` | File doesn't exist at path |
| `!File.ReadError` | File exists but can't be read (permissions, locked) |
| `!File.ParseError` | File content isn't valid JSON/YAML/TOML |

### `=File.Serial.Write`

Serializes a `#serial` data tree to file. Detects target format from extension.

| IO | Type | Description |
|----|------|-------------|
| `<path` | `#path` | File path to write |
| `<data` | `#serial` | Data tree to serialize |

| Error | When |
|-------|------|
| `!File.NotFound` | Parent directory doesn't exist |
| `!File.WriteError` | Can't write to path (permissions, disk full) |

### `=File.Serial.Read.Field`

One-step field extraction: reads file, parses, and extracts a single field by tree path. Combines `=File.Serial.Read` + `=#.Field`.

| IO | Type | Description |
|----|------|-------------|
| `<path` | `#path` | File path to read |
| `<field` | `#RawString` | Tree path using `<` separator (e.g. `"database<host"`) |
| `>value` | `#serial` | Extracted field value |

| Error | When |
|-------|------|
| `!File.NotFound` | File doesn't exist |
| `!File.ReadError` | Can't read file |
| `!File.ParseError` | Invalid format |
| `!Field.NotFound` | Field path doesn't exist in parsed data |

## Permissions

<!-- @permissions -->
All `=File.*` pipelines perform filesystem IO and require a `{_}` permission object granting the listed capabilities. See [[permissions]] for the permission system and [[errors#Built-in Error Namespaces]] for `!Permission.*` errors.

| Pipeline | Required Capability | Category |
|----------|-------------------|----------|
| `=File.Text.Read` | File.Read | File |
| `=File.Text.Write` | File.Write | File |
| `=File.Text.Append` | File.Write | File |
| `=File.Serial.Read` | File.Read | File |
| `=File.Serial.Write` | File.Write | File |
| `=File.Serial.Read.Field` | File.Read | File |
| `=File.Copy` | File.Read + File.Write | File |
| `=File.Move` | File.Read + File.Write | File |
| `=File.Delete` | File.Delete | File |
| `=File.Access` | File.Read | File |
| `=File.List` | File.Read | File |

## Errors

```polyglot
=File.Text.Read
   !NoError
   !File.NotFound
   !File.ReadError

=File.Text.Write
   !NoError
   !File.NotFound
   !File.WriteError

=File.Text.Append
   !NoError
   !File.NotFound
   !File.WriteError

=File.Serial.Read
   !NoError
   !File.NotFound
   !File.ReadError
   !File.ParseError

=File.Serial.Write
   !NoError
   !File.NotFound
   !File.WriteError

=File.Serial.Read.Field
   !NoError
   !File.NotFound
   !File.ReadError
   !File.ParseError
   !Field.NotFound

=File.Copy
   !NoError
   !File.NotFound
   !File.CopyError

=File.Move
   !NoError
   !File.NotFound
   !File.MoveError

=File.Delete
   !NoError
   !File.NotFound
   !File.DeleteError

=File.List
   !NoError
   !Folder.NotFound
   !Folder.ReadError
```

## Implementation Status

| Pipeline | Status |
|---|---|
| `=File.Text.Read` | Deferred |
| `=File.Text.Write` | Deferred |
| `=File.Text.Append` | Deferred |
| `=File.Serial.Read` | Deferred |
| `=File.Serial.Write` | Deferred |
| `=File.Serial.Read.Field` | Deferred |
| `=File.Copy` | Deferred |
| `=File.Move` | Deferred |
| `=File.Delete` | Deferred |
| `=File.Access` | Deferred |
| `=File.List` | Deferred |
