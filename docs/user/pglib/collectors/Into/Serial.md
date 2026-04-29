---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# *Into.Serial

Gathers key-value outputs from mini-pipelines back into a single serial, accessible one level up from the expand scope.

## Syntax

```aljam3
[-] *Into.Serial
   (*) <key << $key
   (*) <value << $value
   (*) >Serial >> $result
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<key` | `#KeyString` | Serial key |
| `<value` | any | Value for key |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Serial` | `#Serial` | Collected serial of all key-value pairs |

## Errors

None.

## Permissions

None.

## Related

- [[pglib/collectors/Into/INDEX|*Into Data Collectors]]
- [[pglib/collectors/Into/Map|*Into.Map]] -- similar key-value collection
- [[concepts/collections/collect|Collect Operators]]
