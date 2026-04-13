---
audience: automation-builder
type: specification
updated: 2026-04-07
status: complete
metadata_definition: "%definition.W:Polyglot"
metadata_instance: "%W:Polyglot:N"
---

# -W.Polyglot

Pure Polyglot Code — no external runtime, no setup/cleanup.

## Definition

```polyglot
{N} -W.Polyglot
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WPolyglot"
   [%] .description << "Pure Polyglot Code — no external runtime, no setup/cleanup."
   [ ] Calls -DoNothing for both [\] and [/].
   [ ] Every pipeline requires [W]; this no-op confirms intent.
```

## Inputs

None.

## Outputs

None.

## Errors

None.

## Permissions

None — pure computation.

## Metadata

| Path | Pattern | Description |
|------|---------|-------------|
| Definition | `%definition.W:Polyglot` | Compile-time pipeline template |
| Instance | `%W:Polyglot:N` | Runtime pipeline instance (N = instance number) |

## Related

- [[pglib/pipelines/W/INDEX|-W.* Wrappers]]
