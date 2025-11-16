# Polyglot Grammar Rules (BNF)

**Version:** 0.0.2
**Last Updated:** 2025-11-14
**Status:** Complete

---

## Core Rules

### Line Structure
- All lines MUST begin with block element which is one square bracketed character
- Block elements can only be at the start of new line and not allowed mid-line
- Format: `[marker] content`

### Data Types
- Basic form: `{language\type}`
- Mutable form: `{language.mutable\type}`
- Examples:
  - `pg\int`, `pg\string`, `pg\bool`
  - `pg.mutable\int`, `py.mutable\dict`

### Variables
1. All variables must start with `.` and must NOT end with `.`
2. There can be any number of hierarchy of dots but `..` is not allowed
3. Allowed characters: `[a-z]`, `.`, and `_` only
4. Examples:
   - Valid: `.input`, `.file_path`, `.my.nested.var`
   - Invalid: `input`, `.path.`, `.bad..name`, `.CamelCase`

---

## File Structure

### Every `*.pg` file must have:
1. **One Package block** (first, mandatory)
2. **Any number of** (in any order):
   - Enumeration definition blocks
   - Custom Error definition blocks
   - Pipeline Definition blocks

---

## Package Block

### Structure
- **Start:** Package block element `[@]`
- **End:** Termination block element `[X]`
- **Allowed lines:** `[@]`, `[#]`, any number (or none) of `[<]`, terminating on `[X]`

### Package Declaration
**Syntax:** `[@] {Registry}@{package.submodule.submodule::Major.Minor.Patch}`

#### Registry Types
1. **Local Registry:**
   - `Local` - No network location
   - `Local.{ip|dns}:{port}` - With network location

2. **Community Registry:**
   - `Community.{Username}`

3. **Company Registry:**
   - `Company.{CompanyName}`

#### Examples
```polyglot
[@] Local@MyProject.DataProcessing::1.0.0
[@] Local.192.168.100.22:4444@MyProject.DataProcessing::1.0.0
[@] Community.user123@MyProject.DataProcessing::1.0.0
[@] Company.acme@InternalLib::2.1.3
```

### File Enumeration
**Syntax:** `[#] {file_number}`

Indicates this file's position in the package (e.g., `[#] 1` means this is the 1st file)

**Example:**
```polyglot
[@] Local@MyProject::1.0.0
[#] 1
[X]
```

### Import Declaration
**Syntax:** `[<] {import_alias} << {Registry}@{Package.Path::Major.Minor.Patch}`

**Examples:**
```polyglot
[<] DataUtils << Community.hasan@DataProcessing::1.2.0
[<] InternalAPI << Company.acme@APILib::3.0.1
[<] LocalHelpers << Local@Helpers::1.0.0
```

### Complete Package Block Example
```polyglot
[@] Local@MyProject.DataProcessing::1.0.0
[#] 1
[<] Utils << Community.hasan@StringUtils::2.1.0
[<] DbLib << Local.192.168.1.100:5000@DatabaseLib::1.0.0
[X]
```

---

## Enumeration Definition Block

### Structure
- **Start:** Enumeration block element `[#]` with name
- **End:** Termination block element `[X]`
- **Scope continues** until `[X]` or next top-level block

### Enumeration Declaration
**Syntax:** `[#] {enumeration.full.path}`

### Alias Declaration (Optional)
**Syntax:** `[A] {AliasName}`

Creates a package-scoped alias for the enumeration

### Field Assignment
**Syntax:** `[<] .{field}: {dtype} << {value}`

### Example
```polyglot
[#] MyApp.Configuration
[A] AppConfig
[<] .host: pg\string << "localhost"
[<] .port: pg\int << 8080
[<] .debug: pg\bool << False
[X]
```

### Nested Enumeration Example
```polyglot
[#] Path.Identifiers.MyApp.DataDirectory
[A] DataDir
[<] .unix: pg\path << \\UnixRoot\\opt\data\
[<] .windows: pg\path << \\C\\Data\
[X]
```

---

## Custom Error Definition Block

### Structure
- **Start:** Error block element `[!]` with error name
- **End:** Termination block element `[X]`
- **Required fields:** `.message`, `.code`, `.trace` (always required)

### Error Declaration
**Syntax:** `[!] !{Error.Full.Path}`

Error names must start with `!`

### Alias Declaration (Optional)
**Syntax:** `[A] !{AliasName}`

### Field Assignment
**Syntax:** `[<] .{field}: {dtype} << {value}`

**Required fields:**
- `.message: pg\string`
- `.code: pg\int`
- `.trace: pg\string`

**Optional:** Additional custom fields

### Example
```polyglot
[!] !MyApp.ValidationError
[A] !ValidationErr
[<] .message: pg\string << "Validation failed"
[<] .code: pg\int << 4000
[<] .trace: pg\string << ""
[<] .field_name: pg\string << ""
[<] .invalid_value: pg\string << ""
[X]
```

---

## Pipeline Definition Block

### Structure
- **Start:** Pipeline block element `[|]` with name
- **End:** Termination block element `[X]`

### Pipeline Declaration
**Syntax:** `[|] {PipelineName}`

### Input Declaration
**Syntax:**
- Required: `[i] .{param}: {dtype}`
- Fixed: `[i] Fixed .{param}: {dtype} << {value}`
- Default: `[i] Default .{param}: {dtype} << {value}`
- If no Input: `[i] #None`

### Trigger Declaration
**Syntax:** `[t] |T.{TriggerType}`

Followed by trigger parameters using `[<]`if the trigger pipelnine have inputs.

### Wrapper Context (Optional)
**Syntax:** `[w] |W.{RuntimeType}`

Examples: `|W.Python3.11`, `|W.Node20`, `|W.Rust`

### Queue Control (Optional)
**Syntax:** `[Q] |Q.{QueueOperation}`

Followed by queue parameters using `[<]`

### Operations
**Sequential:** `[r] |{PipelineName}` or `[r] .{var}: {dtype} << {value}`

**Parallel:** `[p] |{PipelineName}`

### Conditional Blocks
**Syntax:** `[?] {condition}`

Followed by nested operations using `[~]`

### Input Passing
**Syntax:** `[<] .{param}: {dtype} << {value}`

### Output Extraction
**Syntax:** `[>] .{field}: {dtype} >> {variable}`

### Output Declaration (Optional)
**Syntax:**
- `[o] .{output}: {dtype}`
- `[o] #None` (no output)

### Error Handling
**Syntax:** `[!] !{ErrorType}`

Followed by error handling operations

### Expansion/Nesting
**Syntax:** `[~][marker] ...`

Used for operations within expanded contexts (parallel blocks, conditionals, etc.)

### Example: Complete Pipeline
```polyglot
[|] ProcessData
[i] .input_file: pg\path
[i] Default .timeout: pg\int << 30
[t] |T.File.Modified
[<] .path: pg\path << .input_file
[w] |W.Python3.11
[r] |ReadFile
[<] .path: pg\path << .input_file
[>] .content: pg\string >> file_data
[r] |ProcessContent
[<] .data: pg\string << file_data
[>] .result: pg\string >> output
[!] !pg.FileSystem.NotFound
[>] .message: pg\string >> err_msg
[r] |U.Log.Error
[<] .msg: pg\string << err_msg
[o] .result: pg\string
[X]
```

### Example: Parallel Pipeline
```polyglot
[|] ParallelProcessor
[i] .data: pg\string
[r] .result_a: pg\string << ""
[r] .result_b: pg\string << ""
[p] |ProcessPartA
[<] .input: pg\string << .data
[>] .output >> result_a
[p] |ProcessPartB
[<] .input: pg\string << .data
[>] .output >> result_b
[Y] |Y.Join
[>] result_a
[>] result_b
[r] |CombineResults
[<] .a: pg\string << result_a
[<] .b: pg\string << result_b
[X]
```

### Example: Conditional Pipeline
```polyglot
[|] ConditionalProcessor
[i] .value: pg\int
[?] .value ?> 10
[~][r] |ProcessLarge
[~][<] .input: pg\int << .value
[?] .value ?> 5
[~][r] |ProcessMedium
[~][<] .input: pg\int << .value
[?] .value ?> Default
[~][r] |ProcessSmall
[~][<] .input: pg\int << .value
[X]
```

---

## BNF Grammar

### File Structure
```bnf
<file> ::= <package_block> <definition_block>*

<definition_block> ::= <enumeration_block>
                     | <error_block>
                     | <pipeline_block>
```

### Package Block
```bnf
<package_block> ::= <package_decl> <file_enum> <import_decl>* "[X]"

<package_decl> ::= "[@]" <registry> "@" <package_path> "::" <version>

<registry> ::= "Local" ["." <network_location>]
             | "Community." <username>
             | "Company." <company_name>

<network_location> ::= <ip_or_dns> [":" <port>]

<package_path> ::= <identifier> ("." <identifier>)*

<version> ::= <number> "." <number> "." <number>

<file_enum> ::= "[#]" <number>

<import_decl> ::= "[<]" <alias> "<<" <registry> "@" <package_path> "::" <version>
```

### Enumeration Block
```bnf
<enumeration_block> ::= <enum_decl> [<alias_decl>] <field_assign>* "[X]"

<enum_decl> ::= "[#]" <enum_path>

<enum_path> ::= <identifier> ("." <identifier>)*

<alias_decl> ::= "[A]" <identifier>

<field_assign> ::= "[<]" <variable> ":" <dtype> "<<" <value>
```

### Error Block
```bnf
<error_block> ::= <error_decl> [<alias_decl>] <required_fields> <custom_fields>* "[X]"

<error_decl> ::= "[!]" "!" <error_path>

<error_path> ::= <identifier> ("." <identifier>)*

<required_fields> ::= <message_field> <code_field> <trace_field>

<message_field> ::= "[<]" ".message" ":" "pg\string" "<<" <string_value>

<code_field> ::= "[<]" ".code" ":" "pg\int" "<<" <int_value>

<trace_field> ::= "[<]" ".trace" ":" "pg\string" "<<" <string_value>

<custom_fields> ::= <field_assign>
```

### Pipeline Block
```bnf
<pipeline_block> ::= <pipeline_decl> <pipeline_content>* "[X]"

<pipeline_decl> ::= "[|]" <identifier>

<pipeline_content> ::= <input_decl>
                     | <output_decl>
                     | <trigger_decl>
                     | <wrapper_decl>
                     | <queue_decl>
                     | <operation>
                     | <parallel_block>
                     | <conditional_block>
                     | <error_handler>
                     | <join_block>

<input_decl> ::= "[i]" ("#None" | [<input_modifier>] <variable> ":" <dtype> ["<<" <value>])

<input_modifier> ::= "Fixed" | "Default"

<output_decl> ::= "[o]" (<variable> ":" <dtype> | "#None")

<trigger_decl> ::= "[t]" "|T." <trigger_type> [<param_assign>]*

<wrapper_decl> ::= "[w]" "|W." <runtime_type>

<queue_decl> ::= "[Q]" "|Q." <queue_operation> [<param_assign>]*

<operation> ::= "[r]" (<pipeline_call> | <assignment> | <unpack_operation>)

<pipeline_call> ::= "|" <pipeline_name> [<input_assign>]* [<output_extract>]*

<assignment> ::= <variable> ":" <dtype> "<<" <value>

<unpack_operation> ::= "~" (<variable> | <enum_ref> | <pipeline_call>)

<parallel_block> ::= "[p]" "|" <pipeline_name> [<input_assign>]* [<output_extract>]* [<nested_op>]*

<conditional_block> ::= "[?]" <condition> [<nested_op>]*

<join_block> ::= "[Y]" "|Y.Join" [<output_extract>]+

<nested_op> ::= "[~]" "[" <block_char> "]" <nested_content>

<nested_content> ::= (<pipeline_call> | <assignment> | <unpack_operation>) [<input_assign>]* [<output_extract>]*
                   | <conditional_block>
                   | <error_handler>

<error_handler> ::= "[!]" "!" <error_type> [<output_extract>]* [<nested_op>]*

<input_assign> ::= "[<]" <variable> ":" <dtype> "<<" <value>

<output_extract> ::= "[>]" <variable> [":" <dtype>] ">>" <target_var>

<param_assign> ::= "[<]" <variable> ":" <dtype> "<<" <value>

<condition> ::= <variable> "?>" (<number> | "Default")
```

### Basic Elements
```bnf
<variable> ::= "." <var_name> ("." <var_name>)*

<var_name> ::= [a-z_] [a-z0-9_]*

<identifier> ::= [A-Za-z] [A-Za-z0-9_]*

<dtype> ::= <language> ["." "mutable"] "\" <type> ["{" <dtype> "}"]

<language> ::= "pg" | "py" | "js" | "rs" | "go" | ...

<type> ::= "int" | "uint" | "string" | "bool" | "path" | "dt" | "array" | "set" | "serial" | ...

<value> ::= <literal> | <variable> | <enum_ref> | <error_ref>

<literal> ::= <string_literal> | <number_literal> | <bool_literal> | <dt_literal> | <path_literal>

<bool_literal> ::= "True" | "False"

<enum_ref> ::= "#" <enum_path> ["." <field>]

<error_ref> ::= "!" <error_path>

<marker> ::= "[" <block_char> "]"

<block_char> ::= "|" | "X" | "i" | "o" | "r" | "p" | "<" | ">" | "Y" | "t" | "Q" | "w" | "#" | "!" | "A" | "~" | "?" | ...
```

---

## Block Marker Reference

| Marker | Purpose | Context |
|--------|---------|---------|
| `[@]` | Package declaration | Package block only |
| `[|]` | Pipeline definition | Top-level |
| `[X]` | End marker | Closes blocks |
| `[i]` | Input declaration | Pipeline |
| `[o]` | Output declaration | Pipeline |
| `[r]` | Run sequential | Pipeline operations |
| `[p]` | Parallel execution | Pipeline operations |
| `[<]` | Input/field assignment | Multiple contexts |
| `[>]` | Output extraction | Multiple contexts |
| `[Y]` | Join block | Synchronization |
| `[t]` | Trigger | Pipeline |
| `[Q]` | Queue control | Pipeline |
| `[w]` | Wrapper context | Pipeline |
| `[#]` | Enumeration definition | Top-level / File enum |
| `[!]` | Error definition/catching | Top-level / Error handling |
| `[A]` | Alias definition | Enumeration/Error blocks |
| `[~]` | Expansion/nesting | Nested contexts |
| `[?]` | Conditional | Pipeline operations |

---

## Comments

### Single-line
**Syntax:** `// comment text`

### Multi-line
**Syntax:** `/* comment text */`

**Note:** Comments use forward slash `/`, while type separators use backslash `\`

---

## Operators

| Operator | Purpose | Example |
|----------|---------|---------|
| `|` | Pipeline call | `|ProcessData` |
| `~` | Unpack/expand | `~myArray` |
| `@` | Package access | `@pkg|Pipeline` |
| `#` | Enumeration reference | `#MyEnum` |
| `!` | Error type | `!CustomError` |
| `<<` | Push assignment (INTO) | `.x << value` |
| `>>` | Pull assignment (FROM) | `.x >> output` |
| `\` | Type separator | `pg\int` |
| `\\` | Path/alias reference | `\\DataDir\\` |

---

## See Also

- [Complete Syntax Reference](../01-syntax-complete.md)
- [Block Markers](../06-block-markers.md)
- [Type System](../02-type-system.md)
- [Error Handling](../04-error-handling.md)
- [Enumerations](../03-enumerations.md)

---

**End of BNF Grammar**