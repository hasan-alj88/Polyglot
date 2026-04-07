---
audience: [architect, designer]
type: spec
updated: 2026-04-03
---

# String Subtype Nesting

<!-- @source:metadata-tree/INDEX -->

String subtypes live under `%#:String:*` at a flexible level:

```polyglot
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

**Depth consistency:** `#String` composes `##Scalar` (`Depth.Max=1`). The subtypes `:int`, `:float`, etc. are one level of flexible nesting — consistent with `Depth.Max=1`. Each subtype's leaf fields (`.string`, `.regex`) are fixed and do not add further depth.

## Alias Resolution

User code `#int` is an alias for `#Int`. The `%##Alias` schema property enables this — each subtype declares `[#] %##Alias << "int"` (lowercase shorthand). The compiler resolves:

| User writes | Alias resolves to | Schema (validation) | Tree path (data) |
|-------------|-------------------|---------------------|------------------|
| `#int` | `#Int` | `##Int` | `%#:String:int` |
| `#uint` | `#UnsignedInt` | `##UnsignedInt` | `%#:String:uint` |
| `#float` | `#Float` | `##Float` | `%#:String:float` |
| `#string` | `#String` | `#String` | `%#:String` |
| `#emailAddress` | `#String.emailAddress` | `#String.emailAddress` | `%#:String:emailAddress` |

**Key distinction (canonical reference):** The single-`#` names (`#Int`, `#Float`, etc.) are **user-facing aliases** — convenient shorthand you write in type annotations. The double-`##` names (`##Int`, `##Float`, etc.) are **schema descriptors** — metadata the compiler enforces on `#String`. `##` describes `#` the way `###` describes leaf fields; a `#` struct can compose multiple `##` schemas as long as they don't contradict. The colon-prefixed names (`:int`, `:float`, etc.) are **metadata tree paths** — flexible-level keys under `%#:String`. Data instances validated by these schemas live at `%#:String:*` on the data tree.

All subtypes share the `#String` schema (`.string#RawString` + `.regex#RawString`) with `.regex` pre-filled per subtype.

See also: [[field-expansion|Field Expansion]], [[definition-templates|Definition Templates]]
