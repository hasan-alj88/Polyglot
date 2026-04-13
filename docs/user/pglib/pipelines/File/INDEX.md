---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
---

# -File.* File Pipelines

pglib pipelines for file system operations. All are native definitions -- no `[@]` import needed. Success is signalled by `!NoError`. Side-effect-only pipelines (Write, Append, Copy, Move, Delete) have no output -- `!NoError` confirms completion.

`-File.Serial.*` pipelines load and save structured data files (JSON, YAML, TOML). Format is auto-detected from file extension. Internally delegates to `-#.JSON.Parse`, `-#.YAML.Parse`, or `-#.TOML.Parse` base parsers (see [[pglib/pipelines/Schema/INDEX|pipelines/Schema]]).

## Permissions

All `-File.*` pipelines perform filesystem IO and require a `{_}` permission object granting the listed capabilities. See [[permissions]] for the permission system and [[errors#Built-in Error Namespaces]] for `!Permission.*` errors.

| Pipeline | Required Capability | Category |
|----------|-------------------|----------|
| `-File.Text.Read` | File.Read | File |
| `-File.Text.Write` | File.Write | File |
| `-File.Text.Append` | File.Write | File |
| `-File.Serial.Read` | File.Read | File |
| `-File.Serial.Write` | File.Write | File |
| `-File.Serial.Read.Field` | File.Read | File |
| `-File.Copy` | File.Read + File.Write | File |
| `-File.Move` | File.Read + File.Write | File |
| `-File.Delete` | File.Delete | File |
| `-File.Access` | File.Read | File |
| `-File.List` | File.Read | File |

## Pipeline Listing

### Text Operations

- [[pglib/pipelines/File/Text.Read|-File.Text.Read]] -- Read text file contents
- [[pglib/pipelines/File/Text.Write|-File.Text.Write]] -- Write text to file
- [[pglib/pipelines/File/Text.Append|-File.Text.Append]] -- Append text to file

### Serial Operations

- [[pglib/pipelines/File/Serial.Read|-File.Serial.Read]] -- Read and parse structured data file
- [[pglib/pipelines/File/Serial.Write|-File.Serial.Write]] -- Serialize data tree to file
- [[pglib/pipelines/File/Serial.Read.Field|-File.Serial.Read.Field]] -- One-step field extraction from structured file

### File Management

- [[pglib/pipelines/File/Copy|-File.Copy]] -- Copy file
- [[pglib/pipelines/File/Move|-File.Move]] -- Move/rename file
- [[pglib/pipelines/File/Delete|-File.Delete]] -- Delete file
- [[pglib/pipelines/File/Access|-File.Access]] -- Check file access permissions
- [[pglib/pipelines/File/List|-File.List]] -- List files in folder

## Related

- [[pglib/pipelines/INDEX|pglib Pipeline Index]]
- [[pglib/pipelines/Schema/INDEX|-# Base Parsers]]
- [[permissions|Permission System]]
- [[errors#Built-in Error Namespaces|Built-in Error Namespaces]]
