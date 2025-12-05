# Polyglot Examples Index

**Version:** 0.0.2
**Purpose:** Comprehensive collection of validated Polyglot code examples

## Overview

This directory contains canonical code examples demonstrating all major features of the Polyglot language. All examples use correct v0.0.2 syntax and are designed to be educational, practical, and idiomatic.

### Example Philosophy

- **Correctness:** All examples follow v0.0.2 syntax specifications
- **Clarity:** Code is well-commented and explained
- **Completeness:** Examples show complete, runnable pipelines
- **Progressive:** Examples build from simple to complex
- **Practical:** Real-world use cases and patterns

## Learning Paths

### 🌱 Beginner Path

Start here if you're new to Polyglot:

1. [Hello World Examples](01-hello-world.md) - Your first Polyglot programs
2. [Data Processing Examples](02-data-processing.md) - Basic data operations
3. [Error Handling Examples](03-error-handling.md) - Handling errors gracefully

**Time Estimate:** 1-2 hours

### 🌿 Intermediate Path

After mastering the basics:

1. [Parallel Execution Examples](04-parallel-execution.md) - Concurrent programming
2. [File Operations Examples](05-file-operations.md) - Working with files and paths
3. [Complete Workflows](06-complete-workflows.md) - Combining multiple features

**Time Estimate:** 2-4 hours

### 🌳 Advanced Path

For complex systems and patterns:

1. [Complete Workflows](06-complete-workflows.md) - Multi-pipeline orchestration
2. Review all examples for advanced patterns
3. Explore real-world use cases

**Time Estimate:** 4+ hours

## Example Categories

### 1. Hello World Examples

**File:** [01-hello-world.md](01-hello-world.md)

The canonical starting point for any language. Includes:
- Minimal Hello World
- Hello World with input
- Hello World with triggers
- Hello World with error handling

**Key Concepts:** Pipeline structure, input/output, basic syntax

---

### 2. Data Processing Examples

**File:** [02-data-processing.md](02-data-processing.md)

Common data processing patterns and transformations:
- Reading and parsing data
- Data transformation pipelines
- Data validation
- Aggregation and filtering
- Working with collections (arrays, sets)

**Key Concepts:** Type system, operators, data flow, collections

---

### 3. Error Handling Examples

**File:** [03-error-handling.md](03-error-handling.md)

Comprehensive error handling patterns:
- Minimal error catching
- Detailed error field extraction
- Partial field extraction
- Custom error definitions
- Error recovery and retry
- Error aggregation

**Key Concepts:** !Error types, error catching, field extraction, custom errors

---

### 4. Parallel Execution Examples

**File:** [04-parallel-execution.md](04-parallel-execution.md)

Concurrent programming with parallel blocks:
- Simple parallel execution
- Parallel with join synchronization
- Selective synchronization
- Nested parallel blocks
- Parallel data processing
- Fan-out/fan-in patterns

**Key Concepts:** [p] blocks, [Y] join, copy semantics, synchronization

---

### 5. File Operations Examples

**File:** [05-file-operations.md](05-file-operations.md)

Working with files and the filesystem:
- Reading files
- Writing files
- Path identifiers and cross-platform paths
- Directory operations
- File watching and monitoring
- Batch file processing

**Key Concepts:** pg\path type, path identifiers, file I/O, cross-platform code

---

### 6. Complete Workflows

**File:** [06-complete-workflows.md](06-complete-workflows.md)

End-to-end examples combining multiple features:
- Multi-pipeline workflows
- ETL (Extract, Transform, Load) pipelines
- API integration workflows
- Data processing pipelines with error handling
- Orchestrated parallel workflows
- Real-world application patterns

**Key Concepts:** Pipeline orchestration, queue control, triggers, comprehensive patterns

---

## Example Format

Each example follows a consistent format:

```markdown
### Example N: Descriptive Title

**Purpose:** Brief description of what this example demonstrates

**Key Concepts:**
- Concept 1
- Concept 2
- Concept 3

**Code:**
\`\`\`polyglot
[|] ExamplePipeline
// Well-commented code here
[X]
\`\`\`

**Explanation:**
Detailed explanation of how the code works, key decisions, and patterns used.

**Output:**
Expected output or behavior when executed.

**See Also:**
- [Related Documentation](../language/topic.md)
- [Related Example](another-example.md)
```

## Quick Reference: By Feature

### By Language Feature

| Feature | Examples |
|---------|----------|
| Basic Pipelines | Hello World, Data Processing |
| Input/Output | All examples |
| Type System | Data Processing, File Operations |
| Error Handling | Error Handling examples |
| Parallel Execution | Parallel Execution examples |
| Join Operations | Parallel Execution, Complete Workflows |
| Triggers | Hello World (with triggers), Complete Workflows |
| Path Types | File Operations |
| Enumerations | Data Processing, File Operations |
| Queue Control | Complete Workflows |
| Runtime Wrappers | Complete Workflows |

### By Use Case

| Use Case | Recommended Examples |
|----------|---------------------|
| Getting Started | Hello World |
| Data Transformation | Data Processing |
| Error Recovery | Error Handling |
| Performance/Concurrency | Parallel Execution |
| File I/O | File Operations |
| Complex Systems | Complete Workflows |
| Cross-Platform Code | File Operations |
| API Integration | Complete Workflows |
| ETL Pipelines | Data Processing, Complete Workflows |

## Running Examples

**Note:** The Polyglot compiler is not yet implemented (v0.0.2 is documentation only).

When the compiler becomes available:

```bash
# Compile a pipeline
polyglot compile example.pg

# Register in local registry
polyglot register example.pg

# Activate for execution
polyglot activate ExamplePipeline
```

See [CLI Documentation](../cli/00-workflow.md) for complete details.

## Example Validation

All examples in this directory are validated against:

1. **Syntax Correctness:** Follows v0.0.2 syntax specification
2. **Type Safety:** All types properly declared and used
3. **Completeness:** Examples are runnable (when compiler available)
4. **Clarity:** Well-commented and explained
5. **Best Practices:** Follows idiomatic Polyglot patterns

## Contributing Examples

When the language reaches implementation stage, community contributions will be welcomed. Guidelines for contributing:

1. **Follow v0.0.2 Syntax:** Ensure examples use correct, up-to-date syntax
2. **Document Thoroughly:** Include purpose, key concepts, and explanations
3. **Test Examples:** Verify examples compile and run correctly
4. **Use Comments:** Comment code clearly for educational purposes
5. **Show Best Practices:** Demonstrate idiomatic Polyglot patterns

## Common Patterns Demonstrated

### Pattern 1: Sequential Data Pipeline
Process data through multiple transformation stages.

**Examples:** Data Processing (Examples 1-3)

### Pattern 2: Fan-Out, Fan-In
Distribute work across parallel operations, then aggregate results.

**Examples:** Parallel Execution (Examples 2-4)

### Pattern 3: Error Recovery with Retry
Catch errors and retry operations with fallback strategies.

**Examples:** Error Handling (Example 5)

### Pattern 4: File Watching and Processing
Monitor files for changes and process them automatically.

**Examples:** File Operations (Example 5)

### Pattern 5: Multi-Pipeline Orchestration
Coordinate multiple pipelines with queue control and triggers.

**Examples:** Complete Workflows (Examples 1-4)

### Pattern 6: Parallel Array Processing
Process array elements concurrently using parallel blocks.

**Examples:** Parallel Execution (Example 5)

### Pattern 7: Cross-Platform File Operations
Handle files with platform-independent code.

**Examples:** File Operations (Examples 3-4)

### Pattern 8: Conditional Execution
Use triggers and conditionals for dynamic execution flow.

**Examples:** Hello World (Example 3), Complete Workflows

## Example Complexity Levels

### Level 1: Basic (★☆☆☆☆)
- Single pipeline
- No parallel execution
- Basic input/output
- Minimal error handling

**Examples:** Hello World (1-2)

### Level 2: Elementary (★★☆☆☆)
- Single pipeline with multiple operations
- Basic error handling
- Simple data processing

**Examples:** Hello World (3-4), Data Processing (1-2)

### Level 3: Intermediate (★★★☆☆)
- Parallel execution
- Detailed error handling
- Multiple data transformations
- File operations

**Examples:** Data Processing (3-5), Error Handling (1-4), Parallel Execution (1-3)

### Level 4: Advanced (★★★★☆)
- Nested parallel execution
- Custom error types
- Complex data flows
- Multi-file operations

**Examples:** Error Handling (5-6), Parallel Execution (4-6), File Operations (4-6)

### Level 5: Expert (★★★★★)
- Multi-pipeline orchestration
- Queue control
- Trigger coordination
- Complete workflows

**Examples:** Complete Workflows (all examples)

## Troubleshooting Examples

**Q: Example uses syntax I don't recognize**
- Check that you're reading v0.0.2 documentation
- Refer to [Complete Syntax Reference](../language/01-syntax-complete.md)

**Q: Not sure why example uses specific pattern**
- Read the "Explanation" section below each example
- Check "See Also" links for related documentation

**Q: Want to modify example for my use case**
- Understand the example pattern first
- Modify incrementally, testing as you go
- Refer to relevant language documentation

**Q: Example seems overly complex**
- Start with simpler examples in the same category
- Follow the learning path recommendations
- Review prerequisite concepts in language docs

## Next Steps

1. **Start with Hello World:** [01-hello-world.md](01-hello-world.md)
2. **Review Language Basics:** [Complete Syntax Reference](../language/01-syntax-complete.md)
3. **Explore Your Use Case:** Use the "By Use Case" table above
4. **Practice and Experiment:** Modify examples to deepen understanding

## See Also

- [Complete Syntax Reference](../language/01-syntax-complete.md) - Full language syntax
- [Type System](../language/02-type-system.md) - Understanding types
- [Error Handling](../language/04-error-handling.md) - Error handling details
- [Parallel Execution](../language/08-parallel-execution.md) - Concurrency patterns
- [Standard Library Overview](../standard-library/00-overview.md) - Available utilities

---

**Navigation:**
[Language Documentation](../language/01-syntax-complete.md) | [Standard Library](../standard-library/00-overview.md) | [Hello World →](01-hello-world.md)