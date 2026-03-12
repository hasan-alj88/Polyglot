# Overview

The `%Pipeline.Inline` metadata system allows pipelines to accept formatted string arguments and automatically parse them into structured inputs using a dedicated parser pipeline.

**Key Features:**
- Parse inline string arguments (e.g., `"Database:config.yml"`) into structured data
- Automatic invocation of parser pipeline
- Access to special `%Formatted_string` variable
- Direct wiring from parser outputs to main pipeline inputs

**Common Use Cases:**
- Config validators that accept `"type:value"` format
- Email parsers from `"user@domain"` format
- URL parsers from `"protocol://host:port/path"` format
- Any domain-specific inline DSL parsing

---
