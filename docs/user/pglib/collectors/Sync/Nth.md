---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# *Nth

Generic race collector -- waits for the Nth arriving value and cancels all remaining inputs. All `(*) <<` inputs must be the same type.

`*First` is sugar for `*Nth` with n=1. `*Second` is sugar for `*Nth` with n=2.

## Syntax

```polyglot
(*) *Nth
   (*) <n << 2
   (*) << $candidateA
   (*) << $candidateB
   (*) << $candidateC
   (*) >> $winner
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<n` | `#int` | Which arrival to capture (1-based) |
| `<< $var` | same type | Race candidate (repeat for each) |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>> $var` | same as inputs | The Nth value to arrive |

## Aliases

| Alias | Equivalent |
|-------|-----------|
| `*First` | `*Nth` with n=1 |
| `*Second` | `*Nth` with n=2 |

## Errors

None.

## Permissions

None.

## Related

- [[pglib/collectors/Sync/INDEX|Collect-All & Race Collectors]]
- [[pglib/collectors/Sync/First|*First]] -- n=1 sugar
- [[pglib/collectors/Sync/All|*All]] -- collect-all alternative
