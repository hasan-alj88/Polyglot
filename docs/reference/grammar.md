---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: grammar
shard: false

# --- Classification ---
type: reference
topic: Polyglot EBNF Grammar (v0.0.4)
summary: "Reference: Polyglot EBNF Grammar (v0.0.4)"
keywords:
  - reference
  - documentation

# --- BMAD Agent Routing ---
agents:
  - architect
  - developer
phase: any
workflow: any
module: any
complexity: medium

# --- Dependency Chain ---
prereqs:
  []
unlocks:
  []

# --- Relationships ---
related:
  []

# --- Metadata ---
status: stable
updated: 2025-12-16
version: 0.0.4
tags:
  - "#reference"
---
# Polyglot EBNF Grammar (v0.0.4)

**Extended Backus-Naur Form grammar for the Polyglot programming language**

**Version:** v0.0.4
**Last Updated:** 2025-12-15
**Purpose:** Formal syntax definition for AI models and parser implementation

---

## Notation

- `::=` - Definition
- `|` - Alternation (or)
- `[ ]` - Optional
- `{ }` - Zero or more repetitions
- `( )` - Grouping
- `" "` - Literal string
- `' '` - Literal character

---

## Program Structure

```ebnf
program ::= { statement }

statement ::= block_definition
           | pipeline_call
           | variable_declaration
           | conditional
           | loop
           | match_expression
           | error_handling
           | comment
           | import_statement

comment ::= "//" { any_character_except_newline }
```

---

## Block Definitions

```ebnf
block_definition ::= app_block
                  | pipeline_block
                  | enum_block
                  | error_block
                  | alias_block

app_block ::= "{@}" app_name { metadata } { statement } "{x}"
app_name ::= "@" identifier

pipeline_block ::= "{|}" pipeline_name
                   io_parameters
                   trigger_marker
                   [ queue_marker ]
                   wrapper_marker
                   { statement }
                   "{x}"

enum_block ::= "{#}" enum_name
               [ alias_definition ]
               { enum_field | serial_load_block }
               "{x}"

error_block ::= "{!}" error_name
                [ alias_definition ]
                { error_field }
                "{x}"

alias_block ::= "{A}" alias_name
                { alias_mapping }
                "{x}"
```

---

## Markers

```ebnf
marker ::= execution_marker
        | io_marker
        | control_flow_marker
        | structure_marker
        | boolean_marker

execution_marker ::= "[r]"    (* Sequential *)
                  | "[p]"    (* Parallel *)
                  | "[b]"    (* Background *)
                  | "[y]"    (* Fork *)
                  | "[v]"    (* Join *)

io_marker ::= "[|]"           (* Pipeline I/O *)
           | "[~]"           (* Unpack *)
           | "[*]"           (* Pack *)

control_flow_marker ::= "[m]"  (* Match *)
                     | "[?]"  (* Match case *)
                     | "[z]"  (* Try block *)
                     | "[!]"  (* Error handler *)

structure_marker ::= "[.]"     (* Subfield *)
                  | "[s]"     (* Serial load *)
                  | "[t]"     (* Trigger *)
                  | "[Q]"     (* Queue *)
                  | "[W]"     (* Wrapper *)
                  | "[A]"     (* Alias *)
                  | "[<]"     (* Import *)
                  | "[%]"     (* Metadata *)
                  | "[+]"     (* Multi-line continuation *)

boolean_marker ::= "[&]"       (* AND *)
                | "[^]"       (* XOR *)
```

---

## Identifiers and Names

```ebnf
identifier ::= ( letter | "_" ) { letter | digit | "_" | "." }

pipeline_name ::= "|" identifier
enum_name ::= "#" identifier
error_name ::= "!" identifier
unpack_operator ::= "~" identifier
pack_operator ::= "*" identifier
variable_name ::= "$" identifier

fully_qualified_name ::= identifier { "." identifier }
```

---

## Data Types

```ebnf
type_annotation ::= ":" type_spec

type_spec ::= primitive_type
           | array_type
           | serial_type
           | enum_type
           | wildcard_type

primitive_type ::= ":pg.string"
                | ":pg.int"
                | ":pg.float"
                | ":pg.bool"
                | ":pg.datetime"

array_type ::= ":pg.array." type_spec

serial_type ::= ":pg.serial"

enum_type ::= ":" enum_name

wildcard_type ::= ":*"
```

---

## Variables and Assignment

```ebnf
variable_declaration ::= execution_marker variable_name type_annotation operator expression

operator ::= input_operator
          | output_operator
          | assignment_operator
          | default_operator

input_operator ::= "<input_param" "<<" expression
output_operator ::= ">output_param" ">>" variable_name
assignment_operator ::= "<<" expression
default_operator ::= "<~" expression
```

---

## Expressions

```ebnf
expression ::= literal
            | variable_reference
            | pipeline_call
            | serial_construction
            | array_construction
            | enum_construction
            | field_access
            | comparison
            | arithmetic_expression

literal ::= string_literal
         | integer_literal
         | float_literal
         | boolean_literal

string_literal ::= '"' { string_character } '"'
integer_literal ::= [ "-" ] digit { digit }
float_literal ::= [ "-" ] digit { digit } "." digit { digit }
boolean_literal ::= "true" | "false"

variable_reference ::= variable_name [ field_access ]

field_access ::= "." field_path
              | "." '"' field_path '"'

field_path ::= identifier { "." identifier }
```

---

## Serial Construction

```ebnf
serial_construction ::= "#Serial" [ serial_fields ]

serial_fields ::= subfield_definition { subfield_definition }

subfield_definition ::= "[.]" "." field_name "<<" expression
                     | "[.]" "." field_name type_annotation "<<" expression
                     | "[.]" "." field_name ":" enum_type "<<" enum_value
```

---

## Enum Definition and Construction

```ebnf
enum_field ::= "[.]" "." variant_name [ enum_subfields ]

variant_name ::= identifier

enum_subfields ::= { value_field }

value_field ::= "[.]" "." field_name type_annotation [ "<~" default_value ]

enum_construction ::= enum_name "." variant_name [ enum_field_assignments ]

enum_field_assignments ::= { "[.]" "." field_name "<<" expression }
```

---

## Pipeline Definition and Calls

```ebnf
pipeline_parameters ::= { io_parameter }

io_parameter ::= "[|]" input_parameter
              | "[|]" output_parameter

input_parameter ::= "<" parameter_name type_annotation

output_parameter ::= ">" parameter_name type_annotation

trigger_marker ::= "[t]" trigger_type
trigger_type ::= "|T.Call" | "|T.Schedule" | "|T.Event" | "|T.Stream"

queue_marker ::= "[Q]" queue_type
queue_type ::= "|Q.Serial" | "|Q.Parallel" | "|Q.Priority"

wrapper_marker ::= "[W]" wrapper_type
wrapper_type ::= "|W.Polyglot.Scope" | "|W.Polyglot.Stateless" | custom_wrapper

pipeline_call ::= execution_marker pipeline_name [ inline_args ]
               | pipeline_name [ inline_args ]

(* Inline arguments are formatted string templates *)
inline_args ::= formatted_string_template

formatted_string_template ::= '"' template_content '"'

template_content ::= { literal_part | substitution }

literal_part ::= any_character_except ( "{" | "}" | '"' )
              | escaped_brace
              | escaped_quote

escaped_brace ::= "\{" | "\}"
escaped_quote ::= '\"'

substitution ::= "{" substitution_content "}"

substitution_content ::= variable_ref [ ":" format_specifier ]
                      | literal
                      | expression
                      | nested_template

nested_template ::= formatted_string_template  (* Nested templates allowed *)

variable_ref ::= "$" identifier { "." identifier }

format_specifier ::= "hex"           (* Hexadecimal integer *)
                  | "json"           (* JSON serialization *)
                  | "iso8601"        (* ISO 8601 timestamp *)
                  | "default"        (* Explicit default format *)
                  | custom_format    (* User-defined formatter *)

custom_format ::= identifier  (* Maps to |U.String.Polyglot.{Type}.{Format} *)

(* Examples - Basic Templates:
   ""                                    - Empty (no parameters)
   "{$value}"                            - Single variable, default format
   "{$x}, {$y}"                          - Multiple variables with separator
   "production"                          - Literal-only template
*)

(* Examples - Format Specifiers:
   "{$id:hex}"                           - Variable with format specifier
   "{$data:json}"                        - JSON serialization
   "{$timestamp:iso8601}"                - ISO 8601 timestamp
   "{$value:default}"                    - Explicit default format
*)

(* Examples - Path-style Templates:
   "users/{$user_id}/posts/{$post_id}"  - REST-style paths
   "/api/v1/{$resource}/{$id:hex}"      - Mixed literals and formats
*)

(* Examples - Complex Templates:
   "{$base_url}/users/{$id}/profile?active={$is_active}"
   "Error: {$error.message} (Code: {$error.code:hex})"
   "Processing {$count} items at {$timestamp:iso8601}"
*)

(* Examples - Escaped Characters:
   "Value: \{not a variable\}"          - Escaped braces (literal)
   "JSON: \"{$data:json}\""             - Escaped quotes
*)

(* Note: This is NOT simple argument passing. Behind the scenes:
   1. Each {$var:fmt} triggers |U.String.Polyglot.{Type}.{Format}
   2. Results substituted into template
   3. Formatted string passed to formatter pipeline defined in %Inline metadata
   4. Formatter parses string and outputs main pipeline's input parameters

   Format Specifier Mapping:
   - {$x}           → |U.String.Polyglot.Int.Default
   - {$x:hex}       → |U.String.Polyglot.Int.Hex
   - {$data:json}   → |U.String.Polyglot.Serial.Json
   - {$time:iso8601} → |U.String.Polyglot.DateTime.ISO8601
*)
```

---

## Conditionals

```ebnf
conditional ::= fork_branch { continuation_branch } [ join ]

fork_branch ::= "[y]" condition
                { indented_statement }

continuation_branch ::= boolean_marker condition
                       { indented_statement }

condition ::= comparison
           | boolean_expression
           | wildcard_condition

comparison ::= expression comparison_operator expression

comparison_operator ::= "=?"    (* Equal *)
                     | ">?"    (* Greater than *)
                     | "<?"    (* Less than *)
                     | ">=?"   (* Greater or equal *)
                     | "<=?"   (* Less or equal *)
                     | "!=?"   (* Not equal *)

wildcard_condition ::= "*?"

join ::= "[v]" { indented_statement }
```

---

## Match Expressions

```ebnf
match_expression ::= "[m]" variable_name "<<" match_value
                     { match_case }

match_case ::= "[?]" match_pattern "?" result_expression

match_pattern ::= enum_value
               | literal
               | wildcard

wildcard ::= "*"
```

---

## Loops (Unpack Operators)

```ebnf
loop ::= unpack_call { loop_body }

unpack_call ::= execution_marker unpack_operator
                { unpack_parameter }

unpack_parameter ::= "[~]" "<" param_name "<<" source
                  | "[~]" ">" param_name ">>" target

loop_body ::= indented_statement

pack_operation ::= "[v]" pack_operator
                   { pack_parameter }

pack_parameter ::= "[*]" "<" param_name "<<" expression
                | "[*]" ">" param_name ">>" target
```

---

## Error Handling

```ebnf
error_handling ::= error_handler { error_handler }

error_handler ::= "[!]" error_pattern
                  { indented_statement }

error_pattern ::= error_name
               | wildcard_error

wildcard_error ::= "*!"

error_raise ::= execution_marker error_name [ error_fields ]

error_fields ::= { "[.]" "." field_name "<<" expression }
```

---

## Serial Load Blocks

```ebnf
serial_load_block ::= "[s]" pipeline_call
                      { serial_field_mapping }
                      [ serial_error_handler ]

serial_field_mapping ::= "[.]" "." target_field type_annotation "<<" "." source_path

serial_error_handler ::= "[s][!]" "*!"
```

---

## Import Statements

```ebnf
import_statement ::= "[<]" import_path [ "as" alias ]

import_path ::= string_literal | identifier
```

---

## Metadata

```ebnf
metadata ::= "[%]" "%" metadata_key "<<" metadata_value

metadata_key ::= identifier
metadata_value ::= literal | expression
```

---

## Array Construction

```ebnf
array_construction ::= "[" [ array_elements ] "]"

array_elements ::= expression { "," expression }
```

---

## Indentation Rules

```ebnf
indented_statement ::= INDENT statement { statement } DEDENT

INDENT ::= (* Increase indentation level *)
DEDENT ::= (* Decrease indentation level *)
```

**Note:** Polyglot uses significant whitespace. Indentation determines code block scope.

---

## Lexical Elements

```ebnf
letter ::= "A".."Z" | "a".."z"
digit ::= "0".."9"
whitespace ::= " " | "\t"
newline ::= "\n" | "\r\n"
```

---

## Operator Precedence

**From highest to lowest:**

1. Field access (`.`)
2. Function/pipeline calls
3. Arithmetic operators (future)
4. Comparison operators (`=?`, `>?`, etc.)
5. Boolean markers (`[&]`, `[^]`)
6. Assignment operators (`<<`, `>>`, `<~`)

---

## Complete Example

```polyglot
{@} @MyApp
   [%] %version << "1.0.0"
   [%] %author << "Developer"
{x}

{#} #Status
[A] #S
[.] .Pending
[.] .Active
   [.] .started_at :pg.datetime
[.] .Completed
   [.] .finished_at :pg.datetime
{x}

{!} !ValidationError
[.] .field :pg.string
[.] .message :pg.string
{x}

{|} |ProcessOrder
[|] <order_id :pg.string
[|] <amount :pg.float
[|] >status :pg.string

[t] |T.Call
[W] |W.Polyglot.Scope

   [r] $order :pg.serial << |Database.GetOrder"{$order_id}"

   [!] !Database.NotFound
      [r] !ValidationError
         [.] .field << "order_id"
         [.] .message << "Order not found"
      [v] [r] [^]

   [y] $order."status" =? #Status.Pending
      [r] $new_status :#Status << #Status.Active
         [.] .started_at << |U.DateTime.Now

   [y] $amount >? 1000.0
   [&] $order."priority" =? "high"
      [r] |NotifyManager <order_id << $order_id

   [r] $result :pg.string << |Database.UpdateOrder
      <order_id << $order_id
      <status << $new_status

   [|] >status >> $result
{x}
```

---

## Notes for AI Models

### Key Syntax Features

1. **Markers are enclosed in square brackets**: `[r]`, `[y]`, `[|]`
2. **Block delimiters use curly braces**: `{@}`, `{|}`, `{#}`, `{x}`
3. **Operators are prefix symbols**: `|`, `#`, `!`, `~`, `*`, `$`
4. **IO operators**: `<` (input), `>` (output), `<<` (assign input), `>>` (capture output), `<~` (default)
5. **Comparison operators end with `?`**: `=?`, `>?`, `<?`, `>=?`, `<=?`, `!=?`
6. **Subfields use dot prefix**: `.field`, `.nested.path`
7. **Type annotations use colon prefix**: `:pg.string`, `:pg.int`

### Indentation Semantics

- Indentation determines scope (like Python)
- Code indented under markers like `[y]`, `[!]`, `[m]` belongs to that block
- Consistent indentation is required (tabs or spaces, not mixed)

### Context Switching

- `[y]` creates a new conditional branch
- `[&]` continues current branch with AND logic
- `[^]` continues current branch with XOR logic
- `[v]` joins/merges branches

### Pipeline Composition

- Pipelines are first-class citizens
- Can be called inline: `|Pipeline"{$arg1, $arg2}"`
- Can be called with explicit I/O: `[|] <input << $value` then `[|] >output >> $result`

### Error Handling

- Errors are typed and structured
- Use `[!]` to catch specific error types
- Use `[!] *!` to catch all errors
- Errors can carry data in fields

### Reserved Boolean Aliases

**Built-in shorthand aliases:**
```polyglot
{A} #Boolean
[A] #;Boolean;True >> #True   // Shorthand for reserved Boolean true
[A] #;Boolean;False >> #False // Shorthand for reserved Boolean false
{x}
```

**Usage:**
- Both `#True` and `#;Boolean;True` are valid
- Both `#False` and `#;Boolean;False` are valid
- No import required - built into language core
- Use short form for brevity in examples
- Use full form to emphasize reserved status

**Example:**
```polyglot
[r] $flag :pg.bool << #True                    // Short form
[y] $validated =? #;Boolean;True                // Full form
   [r] $status << "ok"
```

---

## Formal Grammar Validation

This grammar has been validated against:
- ✓ Core syntax examples
- ✓ Standard library specifications
- ✓ Complex real-world patterns
- ✓ Error handling scenarios
- ✓ Serial and enum constructions

---

**Part of:** [v0.0.4 Specification](../README.md)
