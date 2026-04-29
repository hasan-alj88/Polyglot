---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# *Into.Level

Gathers key-value outputs from mini-pipelines into serialized siblings at a specific level, accessible one level up from the expand scope.

## Syntax

```aljam3
[-] *Into.Level
   (*) <key << $key
   (*) <value << $value
   (*) >Serial >> $result
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<key` | `#KeyString` | Level key |
| `<value` | any | Value for key |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Serial` | `#Serial` | Collected serial at target level |

## Errors

None.

## Permissions

None.

## Related

- [[pglib/collectors/Into/INDEX|*Into Data Collectors]]
- [[concepts/collections/collect|Collect Operators]]
