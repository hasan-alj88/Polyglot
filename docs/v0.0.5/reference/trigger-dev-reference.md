# Trigger System Developer Reference

**Version:** v0.0.5
**Audience:** Polyglot Language Developers
**Last Updated:** 2026-01-05
**Confidence:** Verified (V)

---

## Table of Contents

1. [Overview](#overview)
2. [Formal Syntax Specification](#formal-syntax-specification)
3. [AST Structure](#ast-structure)
4. [Type System Integration](#type-system-integration)
5. [Compilation Rules](#compilation-rules)
6. [Runtime Interface](#runtime-interface)
7. [Testing Requirements](#testing-requirements)

---

## Overview

This document specifies the formal syntax, semantics, and implementation requirements for the Polyglot v0.0.5 trigger system. It is intended for developers implementing the Polyglot compiler, runtime, and standard library.

### Document Conventions

- **MUST:** Mandatory requirement
- **SHOULD:** Recommended but not mandatory
- **MAY:** Optional
- `code`: Polyglot syntax
- `TypeName`: Type system reference
- `AST_Node`: AST node type

---

## Formal Syntax Specification

### Trigger Block

```ebnf
TriggerBlock ::= '[t]' WHITESPACE TriggerType
                 ( NEWLINE TriggerIO )*

TriggerType ::= '|T.' TriggerVariant ( StringLiteral )?

TriggerVariant ::= 'Cli'
                 | 'Cron'
                 | 'DT.Daily' StringLiteral
                 | 'Interval'
                 | 'Folder.NewFiles'
                 | 'HTTP.Endpoint'
                 | 'Calendar.BusinessWeek'
                 | 'Calendar.Date'
                 | 'Calendar.Range'

TriggerIO ::= ' |' WHITESPACE IODirection Identifier ':' TypeAnnotation
              WHITESPACE IOOperator WHITESPACE IOTarget

IODirection ::= '<'  /* Input to trigger (configuration) */
              | '>'  /* Output from trigger (to pipeline) */

IOOperator ::= '<<'  /* Pull final */
             | '>>'  /* Push final */

IOTarget ::= StringLiteral           /* For configuration inputs */
           | '<' Identifier           /* For pipeline input wiring */
           | EnumValue                /* For reserved enum inputs */
```

### Block Ordering Constraint

```ebnf
Pipeline ::= PackageDecl
             ( PackageImport )*
             ( SchemaDecl )*
             PipelineDecl
             ( DocBlock )?
             TriggerBlock?           /* Optional trigger */
             ( InputDecl )*
             ( OutputDecl )*
             ( WrapperDecl )*
             ( QueueDecl )?          /* Optional queue */
             ( Statement )*
             PipelineClose

/* Ordering MUST be enforced */
Ordering ::= PipelineDecl → TriggerBlock → InputDecl/OutputDecl → WrapperDecl → QueueDecl → Statement → PipelineClose
```

**Compilation Error:**
If blocks appear out of order, compiler MUST emit:
```
Error: Invalid block ordering. Expected [t] before [<]/[>], but found [w] before [t]
```

### Trigger I/O Syntax

#### Configuration Input

```ebnf
TriggerConfigInput ::= ' |' WHITESPACE '<' ConfigName ':' TypeAnnotation
                       WHITESPACE '<<' WHITESPACE Literal

ConfigName ::= Identifier                    /* Simple config */
             | Identifier '.' SubField       /* Nested config */

SubField ::= Identifier

Examples:
 |  <cmd:string << "greet"
 |  <schedule:string << "0 0 * * *"
 |  <settings-DB-Settings.host:string << "localhost"
 |  <server-HTTP-Server << #APIServer
```

#### Kwargs Wiring

```ebnf
TriggerKwargsWiring ::= ' |' WHITESPACE '<kwargs.' ParamName ':' TypeAnnotation
                        WHITESPACE '<<' WHITESPACE '<' PipelineInput

ParamName ::= Identifier

Examples:
 |  <kwargs.name:string << <name
 |  <kwargs.action:string << <action
```

**Semantics:**
- `<kwargs.{param}` extracts CLI argument named `param`
- Wires to pipeline input `<{input}`
- Type MUST match between kwarg and pipeline input

#### Output Wiring

```ebnf
TriggerOutputWiring ::= ' |' WHITESPACE '>' OutputName ':' TypeAnnotation
                        WHITESPACE '>>' WHITESPACE '<' PipelineInput

OutputName ::= Identifier

Examples:
 |  >timestamp:dt >> <backup_time
 |  >files:array.path >> <new_files
 |  >request:serial >> <req
```

**Semantics:**
- Trigger produces output data
- Wires to pipeline input `<{input}`
- Type MUST match between output and pipeline input

---

## AST Structure

### Trigger AST Node

```rust
pub struct TriggerNode {
    pub trigger_type: TriggerType,
    pub inline_param: Option<StringLiteral>,  // For |T.DT.Daily"12AM"
    pub config_inputs: Vec<TriggerConfigInput>,
    pub kwargs_wiring: Vec<TriggerKwargsWiring>,
    pub outputs: Vec<TriggerOutput>,
    pub span: Span,
}

pub enum TriggerType {
    Cli,
    Cron,
    DTDaily,
    Interval,
    FolderNewFiles,
    HTTPEndpoint,
    CalendarBusinessWeek,
    CalendarDate,
    CalendarRange,
}

pub struct TriggerConfigInput {
    pub name: String,               // "cmd", "schedule", etc.
    pub subfield: Option<String>,   // For "settings.host"
    pub type_annotation: TypeAnnotation,
    pub value: Literal,             // String, Path, Enum, etc.
    pub span: Span,
}

pub struct TriggerKwargsWiring {
    pub param_name: String,         // "name", "action", etc.
    pub type_annotation: TypeAnnotation,
    pub pipeline_input: String,     // Name of <pipeline_input>
    pub span: Span,
}

pub struct TriggerOutput {
    pub name: String,               // "timestamp", "files", "request"
    pub type_annotation: TypeAnnotation,
    pub pipeline_input: String,     // Name of <pipeline_input>
    pub span: Span,
}
```

### Pipeline AST Node (with Trigger)

```rust
pub struct PipelineNode {
    pub package_decl: PackageDecl,
    pub imports: Vec<PackageImport>,
    pub schemas: Vec<SchemaDecl>,
    pub pipeline_decl: PipelineDecl,
    pub doc: Option<DocBlock>,
    pub trigger: Option<TriggerNode>,    // ← Trigger node
    pub inputs: Vec<InputDecl>,
    pub outputs: Vec<OutputDecl>,
    pub wrappers: Vec<WrapperDecl>,
    pub queue: Option<QueueDecl>,
    pub statements: Vec<Statement>,
    pub span: Span,
}
```

### Example AST Construction

```polyglot
[t] |T.Cli
 |  <cmd:string << "greet"
 |  <kwargs.name:string << <name
 |  >args:serial >> <all_args
```

**Produces:**
```rust
TriggerNode {
    trigger_type: TriggerType::Cli,
    inline_param: None,
    config_inputs: vec![
        TriggerConfigInput {
            name: "cmd".to_string(),
            subfield: None,
            type_annotation: TypeAnnotation::String,
            value: Literal::String("greet".to_string()),
            span: ...,
        }
    ],
    kwargs_wiring: vec![
        TriggerKwargsWiring {
            param_name: "name".to_string(),
            type_annotation: TypeAnnotation::String,
            pipeline_input: "name".to_string(),
            span: ...,
        }
    ],
    outputs: vec![
        TriggerOutput {
            name: "args".to_string(),
            type_annotation: TypeAnnotation::Serial,
            pipeline_input: "all_args".to_string(),
            span: ...,
        }
    ],
    span: ...,
}
```

---

## Type System Integration

### Trigger Output Types

Each trigger variant produces specific output types:

```rust
pub fn trigger_output_schema(trigger_type: &TriggerType) -> HashMap<String, Type> {
    match trigger_type {
        TriggerType::Cli => hashmap! {
            "args" => Type::Serial,
            // kwargs are dynamic, based on wiring
        },
        TriggerType::Cron | TriggerType::DTDaily => hashmap! {
            "timestamp" => Type::DateTime,
        },
        TriggerType::Interval => hashmap! {
            "timestamp" => Type::DateTime,
        },
        TriggerType::FolderNewFiles => hashmap! {
            "files" => Type::Array(Box::new(Type::Path)),
        },
        TriggerType::HTTPEndpoint => hashmap! {
            "request" => Type::Serial,  // Contains {method, headers, body, params}
        },
        TriggerType::CalendarBusinessWeek => hashmap! {
            "date" => Type::DateTime,
            "day_of_week" => Type::String,
        },
        TriggerType::CalendarDate => hashmap! {
            "timestamp" => Type::DateTime,
        },
        TriggerType::CalendarRange => hashmap! {
            "current_date" => Type::DateTime,
        },
    }
}
```

### Type Checking Rules

**Rule 1: Output-to-Input Type Matching**

For each trigger output wiring:
```polyglot
 |  >output:TypeA >> <pipeline_input
[<] <pipeline_input:TypeB
```

Compiler MUST verify: `TypeA == TypeB`

**Error if mismatch:**
```
Type error: Trigger output 'timestamp:dt' wired to input 'backup_time:string'
Expected: dt
Found: string
```

**Rule 2: Kwargs Type Consistency**

For each kwargs wiring:
```polyglot
 |  <kwargs.param:TypeA << <pipeline_input
[<] <pipeline_input:TypeB
```

Compiler MUST verify: `TypeA == TypeB`

**Rule 3: Configuration Type Validation**

For each config input:
```polyglot
 |  <config:TypeA << value
```

Compiler MUST verify: `typeof(value) == TypeA`

**Example:**
```polyglot
 |  <cmd:string << "greet"     // ✓ String literal matches :string
 |  <port:uint << "8080"       // ✗ String literal does not match :uint
 |  <port:uint << 8080         // ✓ Integer literal matches :uint
```

### Reserved Enum Handling

Reserved enums in trigger configuration:

```polyglot
{#} -HTTP-Server#APIServer
[A] #APIServer
[.] .host:string << "0.0.0.0"
[.] .port:uint << 8080
{x}

[t] |T.HTTP.Endpoint
 |  <server-HTTP-Server << #APIServer
```

**Type Checking:**
- `<server-HTTP-Server` expects reserved enum type `-HTTP-Server`
- `#APIServer` is alias for extended `-HTTP-Server` schema
- Compiler verifies alias activation `[A]` and type compatibility

---

## Compilation Rules

### Compilation Phases

**Phase 1: Parsing**
- Parse trigger block syntax
- Build TriggerNode AST
- Validate block ordering

**Phase 2: Type Checking**
- Verify trigger output types match pipeline input types
- Verify kwargs wiring types
- Verify configuration input types
- Check reserved enum usage

**Phase 3: I/O Wiring Analysis**
- Build data flow graph from trigger outputs to pipeline inputs
- Verify all wired pipeline inputs are declared
- Verify no dangling trigger outputs

**Phase 4: Code Generation**
- Generate trigger registration code
- Generate I/O extraction code
- Generate session creation code
- Generate pipeline invocation code

### Code Generation Example

**Source:**
```polyglot
[t] |T.Cli
 |  <cmd:string << "greet"
 |  <kwargs.name:string << <name

[<] <name:string
[>] >message:string

[w] |W.Console
 |  >session >> $console_session

[r] $greeting:string << "Hello, {$name}!"
[r] |U.Console"Hello, {$name}!"
[>] >message << $greeting
{x}
```

**Generated (Pseudocode):**
```rust
// Register CLI command
cli_router.register_command("greet", |args| {
    // Extract kwargs
    let name: String = args.get("name")?;

    // Create console session
    let console_session = ConsoleWrapper::create_session();

    // Prepare pipeline inputs
    let inputs = PipelineInputs {
        name: name,
    };

    // Execute pipeline
    let outputs = execute_pipeline_Greet(inputs, console_session)?;

    // Return output
    outputs.message
});
```

### Validation Rules

**V1: Single Trigger Per Pipeline**

Compiler MUST reject pipelines with multiple `[t]` blocks:
```
Error: Multiple trigger blocks found. Each pipeline may have at most one trigger.
```

**V2: Required Wrappers**

Compiler SHOULD warn if trigger type lacks expected wrapper:

| Trigger Type | Expected Wrapper |
|--------------|------------------|
| Cli          | W.Console        |
| FolderNewFiles | W.File         |
| HTTPEndpoint | W.HTTP           |
| Cron/Interval/Calendar | (none required) |

**V3: Reserved Enum Validation**

Compiler MUST verify reserved enum usage:
- `-Session-{Type}` used only in wrapper outputs
- `-HTTP-Method-{Variant}` used only for HTTP method comparison
- `-Regex-{Pattern}` used only with `re?` operator
- `-HTTP-Server` used only in HTTP trigger configuration

---

## Runtime Interface

### Trigger Engine Interface

```rust
pub trait TriggerEngine: Send + Sync {
    /// Register a triggered pipeline
    fn register_trigger(
        &self,
        trigger_config: TriggerConfig,
        pipeline: Arc<dyn Pipeline>,
    ) -> Result<TriggerHandle, TriggerError>;

    /// Unregister a trigger
    fn unregister_trigger(&self, handle: TriggerHandle) -> Result<(), TriggerError>;

    /// Start listening for events
    fn start(&self) -> Result<(), TriggerError>;

    /// Stop listening for events
    fn stop(&self) -> Result<(), TriggerError>;
}

pub struct TriggerConfig {
    pub trigger_type: TriggerType,
    pub config: HashMap<String, Value>,    // Configuration inputs
    pub io_wiring: Vec<IOWiring>,          // Kwargs and output wiring
}

pub struct IOWiring {
    pub source: IOSource,                  // kwargs.name, output.timestamp, etc.
    pub target: String,                    // Pipeline input name
    pub type_annotation: Type,
}

pub enum IOSource {
    Kwarg(String),                         // CLI kwarg
    Output(String),                        // Trigger output
}

pub struct TriggerHandle {
    pub id: Uuid,
    pub trigger_type: TriggerType,
}
```

### Pipeline Invocation

```rust
pub trait Pipeline: Send + Sync {
    /// Execute pipeline with inputs
    fn execute(
        &self,
        inputs: PipelineInputs,
        session: Session,
    ) -> Result<PipelineOutputs, PipelineError>;
}

pub struct PipelineInputs {
    pub values: HashMap<String, Value>,
}

pub struct PipelineOutputs {
    pub values: HashMap<String, Value>,
}
```

### Trigger Event Flow

```rust
// Pseudocode for CLI trigger engine
impl TriggerEngine for CliTriggerEngine {
    fn register_trigger(
        &self,
        trigger_config: TriggerConfig,
        pipeline: Arc<dyn Pipeline>,
    ) -> Result<TriggerHandle, TriggerError> {
        // Extract command name from config
        let cmd = trigger_config.config.get("cmd")?.as_str()?;

        // Create handler
        let handler = move |args: HashMap<String, String>| {
            // Extract inputs via I/O wiring
            let mut inputs = PipelineInputs::new();

            for wiring in &trigger_config.io_wiring {
                match &wiring.source {
                    IOSource::Kwarg(kwarg_name) => {
                        let value = args.get(kwarg_name)?;
                        inputs.set(&wiring.target, value);
                    }
                    IOSource::Output(output_name) => {
                        // CLI trigger outputs 'args' serial
                        if output_name == "args" {
                            inputs.set(&wiring.target, &args);
                        }
                    }
                }
            }

            // Create session
            let session = ConsoleWrapper::create_session();

            // Execute pipeline
            let outputs = pipeline.execute(inputs, session)?;

            Ok(outputs)
        };

        // Register with CLI router
        self.cli_router.register(cmd, handler);

        Ok(TriggerHandle { id: Uuid::new(), trigger_type: TriggerType::Cli })
    }
}
```

---

## Testing Requirements

### Unit Tests

**T1: Trigger Parsing**

Test that trigger blocks parse correctly:
```rust
#[test]
fn test_parse_cli_trigger() {
    let source = r#"
[t] |T.Cli
 |  <cmd:string << "greet"
 |  <kwargs.name:string << <name
    "#;

    let ast = parse_trigger_block(source).expect("Parse failed");

    assert_eq!(ast.trigger_type, TriggerType::Cli);
    assert_eq!(ast.config_inputs.len(), 1);
    assert_eq!(ast.config_inputs[0].name, "cmd");
    assert_eq!(ast.kwargs_wiring.len(), 1);
    assert_eq!(ast.kwargs_wiring[0].param_name, "name");
}
```

**T2: Block Ordering Validation**

Test that out-of-order blocks are rejected:
```rust
#[test]
fn test_invalid_block_order() {
    let source = r#"
{|} |Pipeline
[w] |W.Console
[t] |T.Cli
 |  <cmd:string << "test"
    "#;

    let result = parse_pipeline(source);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::InvalidBlockOrder);
}
```

**T3: Type Checking**

Test that type mismatches are detected:
```rust
#[test]
fn test_type_mismatch_trigger_output() {
    let source = r#"
[t] |T.Cron
 |  <schedule:string << "0 0 * * *"
 |  >timestamp:dt >> <backup_time

[<] <backup_time:string  // Type mismatch! dt vs string
    "#;

    let result = type_check_pipeline(source);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::TypeMismatch);
}
```

### Integration Tests

**T4: CLI Trigger Execution**

Test CLI trigger end-to-end:
```rust
#[test]
fn test_cli_trigger_execution() {
    let pipeline = compile_and_deploy(r#"
{|} |Greet
[t] |T.Cli
 |  <cmd:string << "greet"
 |  <kwargs.name:string << <name

[<] <name:string
[>] >message:string

[r] >message << "Hello, {$name}!"
{x}
    "#).expect("Deploy failed");

    // Simulate CLI invocation
    let output = invoke_cli("greet", &[("name", "Alice")]).expect("Execution failed");

    assert_eq!(output.get("message"), Some(&Value::String("Hello, Alice!".to_string())));
}
```

**T5: HTTP Trigger Execution**

Test HTTP trigger with request handling:
```rust
#[test]
fn test_http_trigger_execution() {
    let pipeline = compile_and_deploy(r#"
{#} -HTTP-Server#TestServer
[A] #TestServer
[.] .host:string << "127.0.0.1"
[.] .port:uint << 9999
{x}

{|} |Echo
[t] |T.HTTP.Endpoint
 |  <server-HTTP-Server << #TestServer
 |  <route:string << "/echo"
 |  >request:serial >> <req

[<] <req:serial
[>] >response:serial

[r] >response << {
[+]  .status:uint << 200,
[+]  .body:string << $req.body
[+] }
{x}
    "#).expect("Deploy failed");

    // Send HTTP request
    let response = http_post("http://127.0.0.1:9999/echo", "test body")
        .expect("Request failed");

    assert_eq!(response.status, 200);
    assert_eq!(response.body, "test body");
}
```

**T6: Folder Trigger Execution**

Test folder trigger with file watching:
```rust
#[test]
fn test_folder_trigger_execution() {
    let temp_dir = create_temp_dir();

    let pipeline = compile_and_deploy(&format!(r#"
{{|}} |ProcessFiles
[t] |T.Folder.NewFiles
 |  <folder:path << \\FileDir\\{}\
 |  >files:array.path >> <new_files

[<] <new_files:array.path
[>] >count:uint

[r] $count:uint << |Array.Length
 |  <array << $new_files
 |  >length >> $count

[>] >count << $count
{{x}}
    "#, temp_dir.display())).expect("Deploy failed");

    // Create test files
    create_file(&temp_dir.join("file1.txt"), "test1");
    create_file(&temp_dir.join("file2.txt"), "test2");

    // Wait for trigger execution
    let output = wait_for_pipeline_output(Duration::from_secs(5))
        .expect("No execution within timeout");

    assert_eq!(output.get("count"), Some(&Value::UInt(2)));
}
```

### Regression Tests

**T7: Trigger-Specific Tests**

Maintain regression test suite for each trigger type:
- CLI trigger with various kwargs combinations
- Cron trigger with different cron expressions
- Interval trigger with various durations
- Folder trigger with large file batches
- HTTP trigger with different request methods and content types
- Calendar trigger with business days, specific dates, date ranges

**T8: Error Handling Tests**

Test error propagation:
- Pipeline errors are caught by `[!]` blocks
- Trigger configuration errors are reported at deployment
- Runtime errors don't crash trigger engine

---

## See Also

### Related Documentation

- **[Trigger System User Guide](trigger-system.md)** - Usage patterns and examples
- **[Trigger Technical Guide](trigger-technical.md)** - Implementation details
- **[Standard Triggers YAML](../stdlib/standard-triggers.yaml)** - Trigger specifications

### Implementation References

- AST Design Document (internal)
- Type System Specification (internal)
- Runtime API Reference (internal)

---

**Document Status:** ✅ Complete
**Training Session:** 2026-01-05
**Lines:** 727

**Generated by:** Polly Language Expert
**For:** Scribe Documentation Architect
