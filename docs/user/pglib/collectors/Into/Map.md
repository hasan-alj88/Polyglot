---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# *Into.Map

Gathers key-value outputs from mini-pipelines back into a single `#Map`, accessible one level up from the expand scope.

**Duplicate keys:** If two mini-pipelines produce the same key, this is a runtime error. Each key in the resulting Map must be unique.

## Syntax

```polyglot
[-] *Into.Map
   (*) <key << $ticker
   (*) <value << $adjusted
   (*) >Map >> $adjustedPrices
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<key` | `#KeyString` | Map key |
| `<value` | any | Value for key |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Map` | `#Map` | Collected map of all key-value pairs |

## Errors

None. Duplicate keys are a runtime error.

## Permissions

None.

## Related

- [[pglib/collectors/Into/INDEX|*Into Data Collectors]]
- [[pglib/collectors/Into/Serial|*Into.Serial]] -- similar key-value collection
- [[concepts/collections/collect|Collect Operators]]
