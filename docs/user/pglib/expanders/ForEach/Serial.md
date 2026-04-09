---
audience: pg-coder
type: specification
updated: 2026-04-07
status: stable
---

# =ForEach.Serial

Iterates over all key-item pairs in a serial at all levels. The execution marker on the expand line controls parallelism: `[=]` for parallel, `[-]` for sequential.

No `.Enumerate` variant -- Serial is unordered.

## Syntax

```polyglot
[-] =ForEach.Serial
   (=) <Serial << $data
   (=) >key >> $key
   (=) >item >> $item
   ...
```

## Inputs

| Name | Type | Description |
|------|------|-------------|
| `<Serial` | `#Serial` | Serial to iterate |

## Outputs

| Name | Type | Description |
|------|------|-------------|
| `>key` | `#KeyString` | Current key |
| `>item` | any | Current value |

## Errors

None.

## Permissions

None.

## Related

- [[pglib/expanders/ForEach/INDEX|=ForEach Expanders]]
- [[concepts/collections/expand|Expand Operators]]
