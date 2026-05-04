---
audience: automation-builder
type: specification
updated: 2026-04-22
---

# jm3lib Constructors

<!-- @u:syntax/constructors -->

`{$}` constructors are compile-time value producers that guarantee a valid Final value with no error surface. They replace inline pipeline notation (`-Pipeline"..."`) in execution body contexts, where the compiler can prove infallibility at definition time. See [[syntax/constructors]] for the full `{$}` block specification and the Three-Context Rule.

jm3lib defines constructors for core types where string-to-value conversion is common and provably safe.

## Registry

| Constructor | Target Type | Overloads | Doc |
|---|---|---|---|
| `$DT` | `#DateTime`, `#Date`, `#Time` | 7 (3 string-parsing + 4 keyword) | [[constructors/DT\|$DT]] |
| `$Path` | `#path` | 3 (1 string-parsing + 2 keyword) | [[constructors/Path\|$Path]] |
| `$Re` | `#Re` | 1 (native validation) | [[constructors/Re\|$Re]] |
| `$MIME` | `#MIME` | 1 (string-parsing) | [[constructors/MIME\|$MIME]] |
| `$Dur` | `#Duration` | 6 (string-parsing with native conversion) | [[constructors/Dur\|$Dur]] |
| `$Ver` | `#Ver` | 4 (string-parsing) | [[constructors/Ver\|$Ver]] |
| `$URL` | `#URL` | 1 (string-parsing with native conversion) | [[constructors/URL\|$URL]] |
| `$IP` | `#IP` | 2 (native validation) | [[constructors/IP\|$IP]] |
| `$Color` | `#Color` | 3 (hex conversion + native validation) | [[constructors/Color\|$Color]] |

## Related

- [[syntax/constructors]] -- `{$}` block syntax and semantics
- [[jm3lib/pipelines/DT/INDEX|-DT.* pipelines]] -- DateTime pipelines including `-DT.Parse`
- [[jm3lib/pipelines/Path|-Path pipeline]] -- Path pipeline including `-Path.Parse`
- [[jm3lib/pipelines/Re.Parse|-Re.Parse]] -- runtime regex string parsing
- [[jm3lib/pipelines/MIME.Parse|-MIME.Parse]] -- runtime MIME string parsing
- [[jm3lib/pipelines/Dur.Parse|-Dur.Parse]] -- runtime duration string parsing
- [[jm3lib/pipelines/Ver.Parse|-Ver.Parse]] -- runtime version string parsing
- [[jm3lib/pipelines/URL.Parse|-URL.Parse]] -- runtime URL string parsing
- [[jm3lib/pipelines/IP.Parse|-IP.Parse]] -- runtime IP address string parsing
- [[jm3lib/pipelines/Color.Parse|-Color.Parse]] -- runtime color string parsing
