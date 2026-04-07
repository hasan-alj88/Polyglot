---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# *All

Waits for ALL listed variables to reach Final state. Uses `[*] <<` only -- no `[*] >>` output. All variables stay accessible after the barrier completes.

No type constraint on inputs.

## Syntax

```polyglot
[*] *All
   [*] << $profile
   [*] << $history
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<< $var` | any | Variable to wait for (repeat for each) |

## Outputs

None. All waited variables remain accessible in their original bindings.

## Errors

None.

## Permissions

None.

## Related

- [[pglib/collectors/Sync/INDEX|Collect-All & Race Collectors]]
- [[pglib/collectors/Sync/First|*First]] -- race alternative
- [[concepts/collections/collect|Collect Operators]]
