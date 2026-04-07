---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =File.Access

Check file access permissions at the given path. Returns a `#FileAccess` value describing what access is available.

## Definition

```polyglot
{N} =File.Access
   [%] .Kind << #NativeKind.Execution
   [%] .Rust << "FileAccess"
   [%] .description << "Check file access permissions"
   [=] <path#path
   [=] >access#FileAccess
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<path` | `#path` | Path to check access for |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>access` | `#FileAccess` | Access permissions available at path |

## Errors

None.

## Permissions

Requires `File.Read` capability.

## Related

- [[pglib/pipelines/File/INDEX|=File.* File Pipelines]]
- [[pglib/pipelines/File/List|=File.List]]
