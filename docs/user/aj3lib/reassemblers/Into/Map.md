---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# =*Into.Map

Expands a collection and collects items into a map (key-value pairs).

## Syntax

```aljam3
[-] =*Into.Map
   (=) <Array << $records
   (*) >Map >> $index
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<Array` / `<Serial` / `<Dataframe` | collection | Source collection to expand |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Map` | `#Map` | Collected map of key-value pairs |

## Equivalent Expand-Collect

```aljam3
[-] =ForEach.Array
   (=) <Array << $records
   (=) >item >> $record

   [-] *Into.Map
      (*) <key << $record.id
      (*) <value << $record
      (*) >Map >> $index
```

## Errors

None.

## Permissions

None.

## Aliases

None.

## Related

- [[aj3lib/reassemblers/Into/INDEX|=*Into Transformation Reassemblers]]
- [[aj3lib/collectors/Into/Map|*Into.Map]] -- standalone collector equivalent
- [[concepts/collections/reassemble|Reassemble Operators]]
