---
audience: automation-builder
type: specification
updated: 2026-04-15
status: draft
---

# =*Into.Dataframe

Expands a collection and collects items into a dataframe (row-oriented table).

## Syntax

```polyglot
[-] =*Into.Dataframe
   (=) <Array << $records
   (*) >Dataframe >> $table
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<Array` / `<Map` / `<Serial` | collection | Source collection to expand |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>Dataframe` | `#Dataframe` | Collected dataframe of all rows |

## Equivalent Expand-Collect

```polyglot
[-] =ForEach.Array
   (=) <Array << $records
   (=) >item >> $record

   [-] *Into.Dataframe
      (*) <row << $record
      (*) >Dataframe >> $table
```

## Errors

None.

## Permissions

None.

## Aliases

None.

## Related

- [[pglib/reassemblers/Into/INDEX|=*Into Transformation Reassemblers]]
- [[pglib/collectors/Into/Dataframe|*Into.Dataframe]] -- standalone collector equivalent
- [[concepts/collections/reassemble|Reassemble Operators]]
