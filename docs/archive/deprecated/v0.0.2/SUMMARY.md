---
status: deprecated
archived: 2026-03-22
source_branch: main
note: Superseded by docs/user/ and docs/technical/ in v0.0.4 documentation rewrite
replaced_by: docs/user/SPEC-INDEX.md
---
<!-- @d:user/SPEC-INDEX.md -->
> **Deprecated:** This document is superseded. See the current spec for up-to-date content.

# Polyglot v0.0.2 Documentation - Table of Contents

**Version:** 0.0.2
**Last Updated:** 2025-11-11

---

## Getting Started
- [Documentation Home](README.md)
- [Quick Start Guide](language/00-quick-start.md) *(future)*

---

## Language Specification

### Core Syntax
- [Complete Syntax Reference](language/01-syntax-complete.md)
- [Type System](language/02-type-system.md)
- [Operators](language/05-operators.md)
- [Block Markers](language/06-block-markers.md)
- [Comments](language/11-comments.md)
- [BNF Grammar](language/12-bnf-grammar.md)

### Data Structures
- [Enumerations](language/03-enumerations.md)
- [Error Handling](language/04-error-handling.md)
- [DateTime System](language/07-datetime-system.md)

### Advanced Features
- [Parallel Execution](language/08-parallel-execution.md)
- [Expansion Operator](language/09-expansion-operator.md)
- [Pipeline Lifecycle](language/10-pipeline-lifecycle.md)

---

## Standard Library

### Core Namespaces
- [Overview](standard-library/00-overview.md)
- [Runtime Wrappers (|W.*)](standard-library/01-runtime-wrappers.md)
- [Queue Control (|Q.*)](standard-library/02-queue-control.md)
- [Join Operations (|Y.*)](standard-library/05-join-operations.md)

### Catalogs (APIs TBD)
- [Utilities (|U.*)](standard-library/03-utilities.md)
- [Triggers (|T.*)](standard-library/04-triggers.md)

### Reserved Enumerations
- [Reserved Enumerations](standard-library/06-reserved-enumerations.md) *(deferred)*

---

## Examples

### Basic Examples
- [Examples Index](examples/README.md)
- [Hello World](examples/hello-world.md)
- [Data Processing](examples/data-processing.md)

### Pattern Examples
- [Error Handling](examples/error-handling.md)
- [Parallel Execution](examples/parallel-execution.md)
- [File Operations](examples/file-operations.md)

### Complete Examples
- [Complete Workflows](examples/complete-workflows.md)

---

## CLI & Tools

### Command Line Interface
- [Workflow Overview](cli/00-workflow.md)
- [polyglot compile](cli/01-compile.md)
- [polyglot register](cli/02-register.md)
- [polyglot activate](cli/03-activate.md)
- [polyglot test](cli/04-test.md) *(methodology TBD)*

---

## Package Management

### Package System
- [Package Overview](packages/00-overview.md)
- [Registries](packages/01-registries.md)
- [Creating Packages](packages/02-creating-packages.md)
- [Importing Packages](packages/03-importing-packages.md)

---

## Architecture

### Implementation Details
- [Architecture Overview](architecture/00-overview.md)
- [IR Representation](architecture/01-ir-representation.md)
- [Queue System](architecture/02-queue-system.md)
- [Trigger Monitoring](architecture/03-trigger-monitoring.md)
- [Runtime Execution](architecture/04-runtime-execution.md)

---

## Planning Documents

### Internal Documentation
- [Decision Log](decision-log.md)
- [Inconsistencies Log](inconsistencies-log.md)
- [Reserved Enumeration Schemas](reserved-enumeration-schema-decisions.md)
- [Documentation Plan](documentation-plan.md)

---

## Quick Reference

### By Topic
**Syntax Basics:**
- Block markers: `[|]`, `[i]`, `[r]`, `[p]`, `[t]`, `[Q]`, `[<]`, `[>]`, `[#]`, `[!]`, `[A]`, `[X]`, `[Y]`, `[w]`, `[~]`
- Operators: `|` (pipeline), `~` (unpack), `@` (package), `#` (enumeration), `!` (error), `<<` (push), `>>` (pull)
- Types: `pg\int`, `pg\string`, `pg\bool`, `pg\path`, `pg\dt`, `pg\array{}`, `pg\set{}`, `pg\serial`
- Comments: `//` (single-line), `/* */` (multi-line)

**Standard Library:**
- Runtime wrappers: `|W.Python3.10`, `|W.Node20`, `|W.Rust`
- Queue control: `|Q.Pause`, `|Q.Resume`, `|Q.Kill`, `|Q.PriorityBump`
- Join: `|Y.Join`

**Reserved Enumerations:**
- Path identifiers: `#Path.Identifiers.*`
- Queues: `#Queues.*`, `#Queues.Pending`, `#Queues.Dispatch`, `#Queues.Pause`
- Status: `#Status.*`
- None: `#None`

### By Use Case
**Create a Pipeline:**
```polyglot
[|] PipelineName
[i] .input: pg\string
[r] |ProcessData
[<] .data: pg\string << .input
[X]
```

**Handle Errors:**
```polyglot
[r] |MightFail
[!] !ErrorType
[>] .message: pg\string >> err_msg
[r] |HandleError
[<] .msg: pg\string << err_msg
```

**Run in Parallel:**
```polyglot
[p] |ProcessPartA
[<] .data: pg\string << input_data
[>] .output >> result1

[p] |ProcessPartB
[<] .data: pg\string << input_data
[>] .output >> result2

[Y] |Y.Join
[>] result1
[>] result2
```

**Use Runtime Wrapper:**
```polyglot
[w] |W.Python3.11
[r] |RunPythonScript
[<] .script: pg\path << "analyze.py"
```

---

## Glossary

**Pipeline** - Individual unit of execution defined with `[|]...[X]`, like a function or blackbox

**Workflow** - Collection of interconnected pipelines with triggers forming complete automation

**Pipeline Instance** - Runtime instantiation of a pipeline definition, like an object from a class

**Execution** - The act of running code within a pipeline instance

**Running** - Broad state including both execution and paused states, from dispatch until exit

**Queue** - System for managing pipeline execution: Pending, Dispatch, Pause

**Block Marker** - Syntax element that starts a code block: `[|]`, `[r]`, `[p]`, etc.

**Operator** - Symbol with specific semantic meaning: `|`, `~`, `@`, `#`, `!`, `<<`, `>>`

**Enumeration** - Immutable data structure with fixed schema defined with `[#]...[X]`

**Reserved Enumeration** - System-defined enumeration that users can extend with fixed schema

**Error Type** - Special enumeration marked with `!` having reserved fields: `.message`, `.code`, `.trace`

**Literal Syntax Sugar** - Syntax that compiles to underlying pipeline operations (DT"...", "...", array{}, etc.)

**IR (Intermediate Representation)** - Compiled form of Polyglot code stored in database

**Trigger** - Event or condition that activates a pipeline: time-based, file-based, event-based

**Registry** - Package repository: Local (development), Community (open-source), Company (enterprise)

---

## Navigation Tips

### Finding Information Quickly
- **Language syntax questions:** Start with [Complete Syntax Reference](language/01-syntax-complete.md)
- **How to do X:** Check [Examples Index](examples/README.md) first
- **Standard library function:** See [Overview](standard-library/00-overview.md) for namespace guide
- **CLI command:** Go to [Workflow Overview](cli/00-workflow.md)
- **Design decisions:** Review [Decision Log](decision-log.md)

### Document Status Indicators
- ✓ Complete and validated
- ⚠ Pending user input
- *(TBD)* - To be determined in future
- *(future)* - Planned for future version
- *(deferred)* - Postponed to later phase

---

## Contributing

See [README.md](README.md#contributing-to-documentation) for documentation standards and contribution guidelines.

---

**End of Table of Contents**