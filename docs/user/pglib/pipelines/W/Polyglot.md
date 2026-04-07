---
audience: pg-coder
type: specification
updated: 2026-04-07
status: complete
---

# =W.Polyglot

Pure Polyglot Code — no external runtime, no setup/cleanup.

## Definition

```polyglot
{N} =W.Polyglot
   [%] .Kind << #NativeKind.Wrapper
   [%] .Rust << "WPolyglot"
   [%] .description << "Pure Polyglot Code — no external runtime, no setup/cleanup."
   [ ] Calls =DoNothing for both [\] and [/].
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

## Related

- [[pglib/pipelines/W/INDEX|=W.* Wrappers]]
