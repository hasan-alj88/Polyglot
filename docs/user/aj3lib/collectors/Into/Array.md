---
audience: automation-builder
type: specification
updated: 2026-04-07
status: stable
---

# *Into.Array

Gathers outputs from mini-pipelines back into a single array, accessible one level up from the expand scope.

## Syntax

```aljam3
[-] *Into.Array
   (*) <item << $doubled
   (*) >Array >> $results
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<item` | any | Item to append to result array |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Array` | `#Array` | Collected array of all items |

## Errors

None.

## Permissions

None.

## Related

- [[jm3lib/collectors/Into/INDEX|*Into Data Collectors]]
- [[concepts/collections/collect|Collect Operators]]
