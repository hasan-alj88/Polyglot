---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# =*Into.Array

Expands a collection and collects all items into an array.

## Syntax

```polyglot
[-] =*Into.Array
   (=) <Map << $lookup
   (*) >Array >> $values
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<Map` / `<Serial` / `<Dataframe` | collection | Source collection to expand |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Array` | `#Array` | Collected array of all items |

## Equivalent Expand-Collect

```polyglot
[-] =ForEach.Map
   (=) <Map << $lookup
   (=) >key >> $key
   (=) >item >> $value

   [-] *Into.Array
      (*) <item << $value
      (*) >Array >> $values
```

## Errors

None.

## Permissions

None.

## Aliases

None.

## Related

- [[pglib/reassemblers/Into/INDEX|=*Into Transformation Reassemblers]]
- [[pglib/collectors/Into/Array|*Into.Array]] -- standalone collector equivalent
- [[concepts/collections/reassemble|Reassemble Operators]]
