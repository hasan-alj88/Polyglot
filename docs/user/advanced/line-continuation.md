# Polyglot Line Continuation

**Version:** 0.0.2  
**Last Updated:** 2025-12-02

---

## Overview

Use `[*]` marker for line continuation across multiple lines.

---

## Syntactic Continuation `[*]`

Continue logical line:

```polyglot
[r] .long_pipeline_call:pg.string <<
[*]   |VeryLongPipelineName
[*]   [<] .param1 << "value1"
[*]   [<] .param2 << "value2"
[*]   [>] .result >> .output
```

---

## String Concatenation `+"`

Use `+"` prefix for multi-line strings:

```polyglot
[r] .message:pg.string << "This is the first line"
[*] +"This is the second line"
[*] +"This is the third line"
```

**Result:** `"This is the first lineThis is the second lineThis is the third line"`

---

## Whitespace Handling

Whitespace **after** `+"` is preserved:

```polyglot
[r] .message:pg.string << "Line 1 "
[*] +"Line 2 "
[*] +"Line 3"
// Result: "Line 1 Line 2 Line 3"
```

Add explicit newlines if needed:

```polyglot
[r] .message:pg.string << "Line 1\n"
[*] +"Line 2\n"
[*] +"Line 3"
```

---

## Line Length Guidelines (PFG v1.0

| Content | Limit |
|---------|-------|
| Comments | 79 chars |
| Code | 99 chars |
| Strings | 120 chars |

Use `[*]` when exceeding limits.

---

## Multi-Line Readability

```polyglot
[r] .config:pg.serial <<
[*]   {
[*]     "host": "localhost",
[*]     "port": 8080,
[*]     "timeout": 30
[*]   
```

---

## Complete Example

```polyglot
[|] ProcessLongData
[i] .input_file:pg.path
[i] .output_file:pg.path
[i] .transformation_config:pg.serial
[t] |T.Call
[W] RT.Python"processor.py"

[r] .long_description:pg.string << "This pipeline processes input data "
[*] +"by applying a series of transformations "
[*] +"and writes the result to an output file."

[r] |ReadFile
[<] <path:pg.path << .input_file
[>] >content:pg.string >> .raw_data

[r] |Transform
[<] <data:pg.string << .raw_data
[<] <config:pg.serial << .transformation_config
[>] >result:pg.string >> .processed_data

[o] .processed_data:pg.string
[X]
```

---

**End of Advanced Reference**

**See Also:**
- [Syntax Overview](/docs/user/syntax/overview.md
- [Examples](/docs/user/examples/
