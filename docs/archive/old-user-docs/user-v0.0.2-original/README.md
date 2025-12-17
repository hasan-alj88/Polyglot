# Polyglot Documentation

**Version:** 0.0.2 (Current)
**Status:** In Development
**Language Type:** Asynchronous Automation Language

---

## ⚠️ READ THIS FIRST: Polyglot is NOT a Traditional Programming Language

**If you're coming from Python, JavaScript, Rust, Java, or any traditional language, you need to unlearn those patterns.**

Polyglot is an **automation programming language** - the first language designed specifically for writing **automated jobs** (called pipelines) that run in response to **triggers** (events, schedules, conditions).

### What Polyglot Is:
✅ **Automation-first:** Write jobs that respond to triggers (cron-like schedules, file changes, webhooks)
✅ **Boxes with I/O:** Pipelines are boxes connected by inputs/outputs, not functions
✅ **Async-centric:** Everything is asynchronous by default, no explicit await
✅ **Event-driven:** Jobs run when conditions are met, not when you "call" them
✅ **State-aware:** Variables transition through states (Pending → Ready), not just values

### What Polyglot Is NOT:
❌ **NOT a scripting language** - You don't run pipelines with `polyglot run myscript.pg`
❌ **NOT imperative execution** - You don't write `if/else/for/while` loops
❌ **NOT synchronous** - Functions don't return immediately, they trigger async operations
❌ **NOT like other languages** - Zero keywords, operator prefixes required for ALL identifiers

### The Mental Shift You Need:

| Traditional Thinking | Polyglot Thinking |
|---------------------|-------------------|
| "I'm writing a script" | **"I'm writing an automated job"** |
| "Functions I call" | **"Boxes triggered by events"** |
| "Variables have values now" | **"Variables transition through states"** |
| "Run my code" | **"Register job, service runs it when triggered"** |

**👉 See the complete "Unlearning Traditional Programming" guide in [Architecture Overview](../technical/architecture.md#what-is-polyglot-unlearning-traditional-programming)**

---

## 📌 Documentation Overview

**This documentation is the authoritative language specification for Polyglot.** It defines the complete syntax, semantics, and features of the Polyglot language.

**Relationship with BMAD Implementation:**
- **This documentation** = What the language IS (complete syntax reference)
- **BMAD project docs** (`/docs/project/prd.md`, `/docs/technical/architecture.md`, `/docs/project/epics.md`) = What gets implemented and WHEN (MVP phases, architecture decisions)

**Key Points:**
- Not all features in this specification will be implemented in MVP
- BMAD documentation defines implementation scope and priorities
- This specification serves as the reference for developers writing `.pg` files

👉 **For implementation guidance**, see [BMAD Alignment Document](../project/v0.0.2-bmad-alignment.md)

---

## Welcome to Polyglot

Polyglot is a precision automation language designed for orchestrating complex workflows across multiple runtime environments. This documentation covers the complete language specification, standard library, examples, and tooling.

---

## Quick Navigation

### 🚀 Getting Started
- **New to Polyglot?** Start with [Quick Start Guide](language/00-quick-start.md) *(coming in future version)*
- **Language Overview:** See [Complete Syntax Reference](language/01-syntax-complete.md)
- **Hello World:** Check [Hello World Examples](examples/hello-world.md)

### 📚 Core Documentation

#### Language Specification
Complete language reference covering syntax, types, and semantics.

- [Complete Syntax Reference](language/01-syntax-complete.md) - All syntax in one place
- [Type System](language/02-type-system.md) - Types, literals, and type safety
- [Enumerations](language/03-enumerations.md) - Enumeration system and reserved enumerations
- [Error Handling](language/04-error-handling.md) - !Error types and error handling patterns
- [Operators](language/05-operators.md) - All operators: |, ~, @, #, !, <<, >>
- [Block Markers](language/06-block-markers.md) - Block elements: [|], [i], [r], [p], etc.
- [DateTime System](language/07-datetime-system.md) - DT literals and calendar support
- [Parallel Execution](language/08-parallel-execution.md) - Parallel blocks and join operations
- [Expansion Operator](language/09-expansion-operator.md) - [~] nesting and scope rules
- [Pipeline Lifecycle](language/10-pipeline-lifecycle.md) - Instances, execution, and queues
- [Comments](language/11-comments.md) - Comment syntax and conventions

#### Standard Library Reference
Built-in pipelines, utilities, and system enumerations.

- [Overview](standard-library/00-overview.md) - Standard library organization
- [Runtime Wrappers](standard-library/01-runtime-wrappers.md) - |W.* for Python, Node, Rust, etc.
- [Queue Control](standard-library/02-queue-control.md) - |Q.* queue operations
- [Utilities](standard-library/03-utilities.md) - |U.* utilities catalog
- [Triggers](standard-library/04-triggers.md) - |T.* trigger patterns catalog
- [Join Operations](standard-library/05-join-operations.md) - |Y.* join operations
- [Reserved Enumerations](standard-library/06-reserved-enumerations.md) - #Path, #Queues, #DT, #Status *(deferred)*

#### Code Examples
Canonical examples demonstrating Polyglot features.

- [Examples Index](examples/README.md) - All examples organized by topic
- [Hello World](examples/hello-world.md) - Basic pipeline examples
- [Data Processing](examples/data-processing.md) - Data transformation patterns
- [Error Handling](examples/error-handling.md) - Error handling patterns
- [Parallel Execution](examples/parallel-execution.md) - Parallel processing examples
- [File Operations](examples/file-operations.md) - File I/O and path handling
- [Complete Workflows](examples/complete-workflows.md) - Full workflow examples

#### Command-Line Tools
CLI reference for compiling, registering, and activating pipelines.

- [Workflow Overview](cli/00-workflow.md) - Compile → Register → Activate
- [polyglot compile](cli/01-compile.md) - Compile Polyglot code to IR
- [polyglot register](cli/02-register.md) - Register packages
- [polyglot activate](cli/03-activate.md) - Activate pipelines for monitoring
- [polyglot test](cli/04-test.md) - Test pipelines *(methodology TBD)*

#### Package Management
Package system and registry reference.

- [Package Overview](packages/00-overview.md) - Package system overview
- [Registries](packages/01-registries.md) - Local, Community, Company registries
- [Creating Packages](packages/02-creating-packages.md) - How to create packages
- [Importing Packages](packages/03-importing-packages.md) - How to use packages

#### Architecture
Implementation details for compiler and runtime developers.

- [Architecture Overview](architecture/00-overview.md) - System architecture
- [Database Schema](architecture/01-database-schema.md) - IR storage design
- [IR Representation](architecture/02-ir-representation.md) - Intermediate Representation
- [Queue System](architecture/03-queue-system.md) - Queue architecture
- [Trigger Monitoring](architecture/04-trigger-monitoring.md) - Trigger system
- [Runtime Execution](architecture/05-runtime-execution.md) - Execution model

---

## Documentation Organization

### By Learning Path

**For Beginners:**
1. Start with [Complete Syntax Reference](language/01-syntax-complete.md)
2. Review [Hello World Examples](examples/hello-world.md)
3. Explore [Type System](language/02-type-system.md)
4. Learn [Error Handling](language/04-error-handling.md)
5. Try [Data Processing Examples](examples/data-processing.md)

**For Intermediate Users:**
1. Master [Parallel Execution](language/08-parallel-execution.md)
2. Understand [Pipeline Lifecycle](language/10-pipeline-lifecycle.md)
3. Learn [Queue Control](standard-library/02-queue-control.md)
4. Explore [Runtime Wrappers](standard-library/01-runtime-wrappers.md)
5. Study [Complete Workflows](examples/complete-workflows.md)

**For Advanced Users:**
1. Deep dive into [Expansion Operator](language/09-expansion-operator.md)
2. Study [Architecture Documentation](architecture/00-overview.md)
3. Review [IR Representation](architecture/02-ir-representation.md)
4. Understand [Queue System Architecture](architecture/03-queue-system.md)

### By Use Case

**Automation Workflows:**
- [Triggers](standard-library/04-triggers.md) - Time and event triggers
- [Queue Control](standard-library/02-queue-control.md) - Execution management
- [Complete Workflows](examples/complete-workflows.md) - Full examples

**Multi-Language Integration:**
- [Runtime Wrappers](standard-library/01-runtime-wrappers.md) - Python, Node, Rust, etc.
- [Package System](packages/00-overview.md) - Code organization

**Data Processing:**
- [Type System](language/02-type-system.md) - Data types
- [Parallel Execution](language/08-parallel-execution.md) - Concurrent processing
- [Data Processing Examples](examples/data-processing.md) - Patterns

**Error Handling & Reliability:**
- [Error Handling](language/04-error-handling.md) - !Error types
- [Error Handling Examples](examples/error-handling.md) - Patterns
- [Queue Control](standard-library/02-queue-control.md) - Pause/resume

---

## Document Conventions

### Code Blocks
All Polyglot code examples use the `polyglot` language tag:

```polyglot
[|] ExamplePipeline
[i] .input: pg\string
[r] |ProcessData
[<] .data: pg\string << .input
[X]
```

### File References
File and code references use markdown links:
- Files: [filename.md](path/to/filename.md)
- Specific sections: [Section Title](file.md#section-anchor)

### Cross-References
Related documentation is linked via "See Also" sections at the end of each document.

### Terminology
- **Pipeline** - Individual unit of execution (like a function)
- **Workflow** - Collection of interconnected pipelines
- **Pipeline Instance** - Runtime instantiation of a pipeline (like an object)
- See [Pipeline Lifecycle](language/10-pipeline-lifecycle.md) for complete terminology

---

## Planning & Decision Documents

### Internal Documentation
These documents track design decisions and planning:

- [Decision Log](audit/decision-log.md) - All v0.0.2 syntax decisions
- [Inconsistencies Log](audit/inconsistencies-log.md) - v0.0.1 issues resolved
- [Reserved Enumeration Schemas](audit/reserved-enumeration-schema-decisions.md) - Enum schema decisions *(pending)*
- [Documentation Plan](documentation-plan.md) - Documentation progress tracking

### Decision Reference
All documentation is based on 30 syntax decisions documented in [audit/decision-log.md](audit/decision-log.md). Key decisions:

- **Decision #1:** Maps removed, enumerations only
- **Decision #2:** Type separator is `\` (backslash)
- **Decision #3:** DateTime system uses `DT` prefix
- **Decision #4:** Assignment uses `<<` and `>>` operators
- **Decision #7:** Block markers are case-sensitive
- **Decision #13:** Errors use `!Error` types with reserved fields
- See [complete decision log](audit/decision-log.md) for all 30 decisions

---

## Version Information

### Current Status
**Version:** 0.0.2
**Phase:** Phase 1 - Foundation Documentation
**Status:** In Progress
**Last Updated:** 2025-11-20

### Language Evolution
The current specification (v0.0.2) represents a refined version that resolved 30 inconsistencies from initial planning:
- Removed maps, use enumerations exclusively
- Standardized type separator to `\` (backslash)
- Unified DateTime system with `DT` prefix
- Clarified error handling with `!Error` syntax
- Documented complete operator semantics
- See [audit/decision-log.md](audit/decision-log.md) for complete list

### Future Enhancements
Features planned for future documentation versions:
- Interactive examples
- Video tutorials
- Standard library complete API reference (|U.*, |T.*)
- Reserved enumeration complete schemas

---

## Contributing to Documentation

### Documentation Standards
- Follow [kebab-case](audit/decision-log.md#issue-17-file-naming-inconsistencies) for file names
- Use `polyglot` language tag for code blocks
- Include "See Also" sections for cross-references
- Validate all examples against language specification
- Maintain single source of truth per topic

### Reporting Issues
Found an error or inconsistency? Please report:
- Location (file name and section)
- Description of issue
- Suggested correction (if applicable)

---

## Additional Resources

### External Links
- Polyglot Repository: *(TBD)*
- Community Forum: *(TBD)*
- Package Registry: *(TBD)*

### Support
- Documentation Issues: See [audit/decision-log.md](audit/decision-log.md)
- Language Questions: Refer to [Complete Syntax Reference](language/01-syntax-complete.md)
- Examples: Browse [Examples Index](examples/README.md)

---

## License

*(License information TBD)*

---

**Note:** This documentation represents the current specification for the Polyglot language (v0.0.2). The language is in active development, and this documentation reflects the authoritative language design based on resolved design decisions.