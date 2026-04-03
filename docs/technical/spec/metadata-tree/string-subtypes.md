---
audience: [architect, designer]
type: spec
updated: 2026-04-03
---

# String Subtype Nesting

<!-- @source:metadata-tree/INDEX -->

String subtypes live under `%#:String:*` at a flexible level:

```
%#:String
├── :int          <- .string#RawString + .regex = "^-?[0-9]+$"
├── :uint         <- .string#RawString + .regex = "^[0-9]+$"
├── :float        <- .string#RawString + .regex = "^-?[0-9]+\.[0-9]+$"
├── :sci          <- .string#RawString + .regex = scientific notation
├── :eng          <- .string#RawString + .regex = engineering notation
├── :dim          <- .string#RawString + .regex = "^[1-9][0-9]*$"
├── :emailAddress <- user-defined: .regex = custom pattern
└── :(any)        <- extensible — users define new subtypes
```

## Alias Resolution

User code `#int` is an alias for `#Int`. The `%##Alias` schema property enables this — each subtype declares `[#] %##Alias << "int"` (lowercase shorthand). The compiler resolves:

| User writes | Compiler resolves to | Tree path |
|-------------|---------------------|-----------|
| `#int` | `#Int` | `%#:String:int` |
| `#uint` | `#UnsignedInt` | `%#:String:uint` |
| `#float` | `#Float` | `%#:String:float` |
| `#string` | `#String` | `%#:String` |
| `#emailAddress` | `#String.emailAddress` | `%#:String:emailAddress` |

All subtypes share the `#String` schema (`.string#RawString` + `.regex#RawString`) with `.regex` pre-filled per subtype.

See also: [[field-expansion|Field Expansion]], [[definition-templates|Definition Templates]]
