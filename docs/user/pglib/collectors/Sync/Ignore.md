---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# *Ignore

Explicit collector discard. The variable exists but is released -- used when a parallel pipeline produces output that is not needed.

For inline discard without creating a variable, use `$*` instead.

## Syntax

```polyglot
[*] *Ignore
   [*] << $unneededVar
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<< $var` | any | Variable to discard |

## Outputs

None.

## Errors

None.

## Permissions

None.

## Related

- [[pglib/collectors/Sync/INDEX|Collect-All & Race Collectors]]
- [[concepts/collections/collect|Collect Operators]]
