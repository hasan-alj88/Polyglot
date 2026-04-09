---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# *First

Sugar for `*Nth` with n=1. Takes the first arriving value; all other inputs are cancelled. All `(*) <<` inputs must be the same type.

## Syntax

```polyglot
(*) *First
   (*) << $candidateA
   (*) << $candidateB
   (*) >> $winner
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<< $var` | same type | Race candidate (repeat for each) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>> $var` | same as inputs | First value to arrive |

## Errors

None.

## Permissions

None.

## Related

- [[pglib/collectors/Sync/INDEX|Collect-All & Race Collectors]]
- [[pglib/collectors/Sync/Nth|*Nth]] -- generic race form
- [[pglib/collectors/Sync/All|*All]] -- collect-all alternative
