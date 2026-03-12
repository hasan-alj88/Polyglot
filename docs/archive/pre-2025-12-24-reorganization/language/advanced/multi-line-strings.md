# Multi-Line String System
**Feature Type:** Language Feature
**Category:** String Handling
**Marker:** `[+]`
**Since:** v0.0.4
**Status:** ✅ Verified in Training

---

## Overview

Polyglot provides a powerful multi-line string concatenation system using the `[+]` marker, with language-specific utilities that handle syntax and escaping automatically.

**Key Features:**
- `[+]` marker for building multi-line strings
- `+"` for same-line continuation
- `-"` for new-line continuation
- Language-specific utilities (`|U.String.Python`, `|U.String.Rust`, `|U.String.JS`)
- Double-brace variable interpolation `{{$var}}`
- Automatic handling of quotes, braces, and indentation

---

## Basic Syntax

### The `[+]` Marker

```polyglot
<param:string << |U.String.{Language}""
[+] +"first part" +"continues on same line"
[+] -"second line (newline created)"
[+] -"third line (another newline)"
```

### Line Continuation Operators

| Operator | Behavior | Result |
|----------|----------|--------|
| `[+] +"text"` | Append to current line | No newline added |
| `[+] -"text"` | Append as new line | Newline added before text |

---

## Language-Specific Utilities

### Python: `|U.String.Python`

**Purpose:** Build Python code strings with proper indentation handling

**Features:**
- Handles Python indentation correctly
- No need to escape quotes or braces
- Supports `{{$var}}` interpolation

**Example:**
```polyglot
(|) <code:string << |U.String.Python""
[+] +"def hello(name):"
[+] -"    print(f'Hello, {name}!')"
[+] -"    return True"
```

**Resulting Python Code:**
```python
def hello(name):
    print(f'Hello, {name}!')
    return True
```

---

### Rust: `|U.String.Rust`

**Purpose:** Build Rust code strings without escaping

**Features:**
- Handles `"` and `{` without need for `\"` `\{`
- Proper Rust syntax support
- Supports `{{$var}}` interpolation

**Example:**
```polyglot
(|) <code:string << |U.String.Rust""
[+] +"use std::fs::File;"
[+] -"use std::io::Write;"
[+] -"fn main() -> std::io::Result<()> {"
[+] -"    let mut file = File::create(\"{{$file_path}}\")?;"
[+] -"    file.write_all(b\"Hello from Rust\")?;"
[+] -"    Ok(())"
[+] -"}"
```

**Resulting Rust Code:**
```rust
use std::fs::File;
use std::io::Write;
fn main() -> std::io::Result<()> {
    let mut file = File::create("/path/to/file.txt")?;
    file.write_all(b"Hello from Rust")?;
    Ok(())
}
```

---

### JavaScript: `|U.String.JS`

**Purpose:** Build JavaScript code strings

**Features:**
- Handles template literals correctly
- No escaping needed
- Supports `{{$var}}` interpolation

**Example:**
```polyglot
(|) <code:string << |U.String.JS""
[+] +"const fs = require('fs');"
[+] -"const data = 'Hello from JavaScript';"
[+] -"fs.writeFileSync('{{$output_file}}', data);"
```

**Resulting JavaScript Code:**
```javascript
const fs = require('fs');
const data = 'Hello from JavaScript';
fs.writeFileSync('/path/to/output.txt', data);
```

---

## Variable Interpolation

### Double-Brace Syntax

Use `{{$variable}}` to interpolate Polyglot variables into multi-line strings.

**Syntax:**
```polyglot
{{$var_name}}
```

**Example:**
```polyglot
[r] $file_path << "/tmp/output.txt"
[r] $count << 42

(|) <code:string << |U.String.Python""
[+] +"file_path = '{{$file_path}}'"
[+] -"count = {{$count}}"
```

**Result:**
```python
file_path = '/tmp/output.txt'
count = 42
```

**Note:** Variables are interpolated at Polyglot compile/runtime, not by the target language.

---

## Complete Example: Multi-Language Code Generation

```polyglot
{|} |CodeGenerator
[<] <file_path:pg.path
[<] <message:string
[>] >python_code:string
[>] >rust_code:string
[>] >js_code:string

# Generate Python code
[r] |U.String.Python""
(|) >python_code:string >> >python_code
[+] +"with open('{{$file_path}}', 'w') as f:"
[+] -"    f.write('{{$message}}')"

# Generate Rust code
[r] |U.String.Rust""
(|) >rust_code:string >> >rust_code
[+] +"use std::fs;"
[+] -"fn write_file() {"
[+] -"    fs::write(\"{{$file_path}}\", \"{{$message}}\");"
[+] -"}"

# Generate JavaScript code
[r] |U.String.JS""
(|) >js_code:string >> >js_code
[+] +"const fs = require('fs');"
[+] -"fs.writeFileSync('{{$file_path}}', '{{$message}}');"
{x}
```

---

## Use Cases

### 1. Code Generation
Generate code in other languages dynamically

```polyglot
[r] $template_path << "/templates/handler.py"

(|) <code:string << |U.String.Python""
[+] +"def handle_request(data):"
[+] -"    # Generated handler"
[+] -"    return process(data)"
```

### 2. SQL Query Building
Build complex SQL queries

```polyglot
(|) <query:string << |U.String.SQL""
[+] +"SELECT * FROM users"
[+] -"WHERE status = 'active'"
[+] -"  AND created_at > '{{$start_date}}'"
[+] -"ORDER BY created_at DESC"
```

### 3. Configuration File Generation
Generate config files in various formats

```polyglot
(|) <nginx_config:string << |U.String.Nginx""
[+] +"server {"
[+] -"    listen {{$port}};"
[+] -"    server_name {{$domain}};"
[+] -"    root {{$webroot}};"
[+] -"}"
```

### 4. Template Expansion
Multi-line template processing

```polyglot
(|) <html:string << |U.String.HTML""
[+] +"<html>"
[+] +"<body>"
[+] -"    <h1>{{$title}}</h1>"
[+] -"    <p>{{$content}}</p>"
[+] +"</body>"
[+] +"</html>"
```

---

## Best Practices

### 1. Choose the Right Utility

Match the utility to your target language:
- `|U.String.Python` for Python code
- `|U.String.Rust` for Rust code
- `|U.String.JS` for JavaScript/TypeScript
- `|U.String.SQL` for SQL queries
- `|U.String.JSON` for JSON documents
- `|U.String.HTML` for HTML markup
- `|U.String.{Language}` for other languages

### 2. Indentation Consistency

Use consistent indentation in your multi-line strings:

```polyglot
✅ Good:
[+] +"def func():"
[+] -"    return True"  # Consistent 4-space indent

❌ Bad:
[+] +"def func():"
[+] -"  return True"    # Inconsistent 2-space indent
```

### 3. Variable Interpolation Placement

Place interpolation where values are needed:

```polyglot
✅ Good:
[+] +"file_path = '{{$path}}'"

❌ Avoid:
[+] +"file_path = '" +"{{$path}}'"  # Unnecessary split
```

### 4. Use `-"` for Multi-Line Readability

Use `-"` to create actual line breaks in generated code:

```polyglot
✅ Good (readable output):
[+] +"import sys"
[+] -"import os"
[+] -"import json"

❌ Bad (single line):
[+] +"import sys" +"import os" +"import json"
```

---

## Common Patterns

### Pattern 1: Function Template

```polyglot
(|) <func:string << |U.String.Python""
[+] +"def {{$func_name}}({{$params}}):"
[+] -"    \"\"\"{{$docstring}}\"\"\""
[+] -"    {{$body}}"
[+] -"    return {{$return_value}}"
```

### Pattern 2: Class Template

```polyglot
(|) <class:string << |U.String.Python""
[+] +"class {{$class_name}}:"
[+] -"    def __init__(self, {{$init_params}}):"
[+] -"        {{$init_body}}"
[+] -""
[+] -"    def {{$method_name}}(self):"
[+] -"        {{$method_body}}"
```

### Pattern 3: Configuration Template

```polyglot
(|) <config:string << |U.String.YAML""
[+] +"version: '{{$version}}'"
[+] -"services:"
[+] -"  app:"
[+] -"    image: {{$image}}"
[+] -"    ports:"
[+] -"      - '{{$port}}:80'"
```

---

## Technical Notes

### Escape Handling

The language-specific utilities handle escaping automatically:

| Language | Auto-Escaped | Example |
|----------|--------------|---------|
| Python | `"`, `'`, `\` | `"Hello \"World\""` → no manual escaping needed |
| Rust | `"`, `{`, `}` | `File::create("{}")` → no manual escaping needed |
| JavaScript | `"`, `'`, `` ` `` | Template literals handled automatically |

### Performance

- Multi-line strings are concatenated at compile time when possible
- Variable interpolation occurs at runtime
- No performance penalty for using `[+]` vs single-line strings

### Limitations

- Maximum line length: No inherent limit
- Nesting: Can nest interpolated variables, but not string builders
- Cannot use `[+]` outside of string building context

---

## See Also

- [String Interpolation](/docs/User/language/syntax/string-interpolation.md)
- [Inline Pipelines](/docs/User/language/advanced/inline-pipelines.md)
- [Standard Library: String Utilities](/docs/User/stdlib/utilities/string-utilities.md)
- [Runtime Wrappers](/docs/User/stdlib/wrappers/runtime-wrappers.md)

---

**Added:** 2025-12-21 (Training Session 2)
**Verified:** Human review in training session
**Source:** L-2025-12-21-006
