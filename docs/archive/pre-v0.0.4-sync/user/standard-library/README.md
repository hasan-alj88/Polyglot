---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/standard-library/README.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Standard Library

**Built-in components, utilities, and system interfaces**

---

## Overview

The Polyglot Standard Library provides a comprehensive set of built-in components that are available in every Polyglot program. This includes runtime wrappers, queue controllers, triggers, utilities, join operations, and reserved enumerations.

**Note:** For complete standard library specifications, see the [v0.0.4 Standard Library Specification](../../specifications/v0.0.4/standard-library/).

---

## Standard Library Documentation

### 🔧 [00 - Library Overview](./00-overview.md)
High-level overview of the standard library organization.

**Topics:**
- Library structure
- Naming conventions
- Import paths
- Version compatibility
- Standard library philosophy

**Categories:**
- Runtime wrappers (`|W.*`)
- Queue controllers (`|Q.*`)
- Triggers (`|T.*`)
- Utilities (`|U.*`)
- Join operations
- Reserved enumerations

---

### 📦 [01 - Runtime Wrappers](./01-runtime-wrappers.md)
Wrappers for pipeline execution environments.

**Built-in Wrappers:**
- `|W.Polyglot.Scope` - Standard scope isolation
- `|W.Polyglot.Stateless` - No state persistence
- `|W.Polyglot.Transaction` - Transactional execution
- `|W.HTTP.Request` - HTTP request wrapper
- `|W.Database.Connection` - Database connection wrapper

**Topics:**
- Wrapper lifecycle
- State management
- Resource cleanup
- Custom wrappers

---

### 🎛 [02 - Queue Control](./02-queue-control.md)
Queue configurations for pipeline scheduling.

**Built-in Queues:**
- `|Q.Serial` - Sequential execution (FIFO)
- `|Q.Parallel` - Concurrent execution
- `|Q.Priority` - Priority-based scheduling
- `|Q.Throttle` - Rate-limited execution

**Topics:**
- Queue selection
- Backpressure handling
- Queue monitoring
- Performance tuning

---

### 🛠 [03 - Utilities](./03-utilities.md)
Standard utility pipelines.

**Utility Categories:**
- **Math:** `|U.Math.*` - Arithmetic operations
- **String:** `|U.String.*` - String manipulation
- **DateTime:** `|U.DateTime.*` - Date/time operations
- **Data:** `|U.Data.*` - Data format handling (JSON, YAML, XML, TOML)

**Common Utilities:**
- `|U.String.Concat` - String concatenation
- `|U.Math.Add` - Addition
- `|U.DateTime.Now` - Current timestamp
- `|U.Data.JSON.Parse` - Parse JSON

---

### 📚 [03 - Utilities Catalog](./03-utilities-catalog.md)
Complete catalog of all standard utilities with examples.

**Organized by:**
- Category and subcategory
- Input/output parameters
- Use cases
- Example code

---

### 📡 [04 - Triggers](./04-triggers.md)
Pipeline trigger types and configuration.

**Built-in Triggers:**
- `|T.Call` - Direct invocation
- `|T.Schedule` - Time-based (cron)
- `|T.Event` - Event-driven
- `|T.Stream` - Continuous stream processing
- `|T.HTTP` - HTTP endpoint

**Topics:**
- Trigger configuration
- Event sources
- Schedule syntax (cron)
- Stream connectors

---

### 📚 [04 - Triggers Catalog](./04-triggers-catalog.md)
Complete catalog of trigger types with configuration examples.

**For Each Trigger:**
- Configuration options
- Use cases
- Example pipelines
- Monitoring

---

### 🔀 [05 - Join Operations](./05-join-operations.md)
Join marker (`[v]`) patterns and advanced synchronization.

**Topics:**
- Basic join with `[v]`
- Collecting results from parallel branches
- Timeout patterns
- Error aggregation from multiple branches

**Patterns:**
- Fork-join parallelism
- Scatter-gather
- Map-reduce

---

### 🏷 [06 - Reserved Enumerations](./06-reserved-enumerations.md)
System-defined enumerations available globally.

**Reserved Enums:**
- `#;Boolean;True`, `#;Boolean;False` - Boolean values
- `#;Null` - Null value representation
- `#;Result.*` - Success/failure results
- `#;Option.*` - Optional values

**Note:** These enums are always available and follow special naming with semicolons.

---

## Quick Reference

### Utility Categories

**Mathematics (`|U.Math.*`):**
```polyglot
|U.Math.Add <a <b >result        // Addition
|U.Math.Multiply <a <b >result   // Multiplication
|U.Math.Divide <a <b >result     // Division
|U.Math.Round <value >result     // Rounding
```

**Strings (`|U.String.*`):**
```polyglot
|U.String.Concat <parts >result     // Concatenation
|U.String.Split <text <delimiter >parts  // Split
|U.String.Replace <text <old <new >result  // Replace
|U.String.Length <text >length      // Length
```

**DateTime (`|U.DateTime.*`):**
```polyglot
|U.DateTime.Now >timestamp           // Current time
|U.DateTime.Format <dt <format >str  // Format date
|U.DateTime.Parse <str >dt           // Parse date
|U.DateTime.Add <dt <duration >dt    // Add duration
```

**Data Formats (`|U.Data.*`):**
```polyglot
|U.Data.JSON.Parse <json >serial      // Parse JSON
|U.Data.JSON.Dump <serial >json       // Serialize JSON
|U.Data.YAML.Load <path >serial       // Load YAML file
|U.Data.XML.Parse <xml >serial        // Parse XML
```

---

## Usage Examples

### Using a Utility

```polyglot
[r] $sum :pg.int << |U.Math.Add"{10, 20}"
// $sum = 30

[r] $greeting :pg.string << |U.String.Concat"{\"Hello, \", $name, \"!\"}"
```

### Using a Wrapper

```polyglot
{|} |MyPipeline
[|] <input :pg.string
[|] >output :pg.string

[t] |T.Call
[W] |W.Polyglot.Scope  // Standard wrapper

   [r] $result << $input
   [|] >output >> $result
{x}
```

### Using a Queue

```polyglot
{|} |BatchProcessor
[|] <items :pg.array.pg.string

[t] |T.Call
[Q] |Q.Parallel  // Process items concurrently
[W] |W.Polyglot.Stateless

   // Process items in parallel...
{x}
```

### Using a Trigger

```polyglot
{|} |DailyReport
[|] >report :pg.string

[t] |T.Schedule
   [%] %cron << "0 9 * * *"  // Every day at 9 AM
[W] |W.Polyglot.Scope

   // Generate report...
{x}
```

---

## Versioning and Stability

### Stability Guarantees

**Stable:**
- Core wrappers (`|W.Polyglot.*`)
- Core queues (`|Q.Serial`, `|Q.Parallel`)
- Core triggers (`|T.Call`, `|T.Schedule`)
- Math utilities (`|U.Math.*`)
- String utilities (`|U.String.*`)

**Beta:**
- Advanced wrappers
- Specialized queues
- Some data utilities

**Experimental:**
- Cutting-edge features
- Subject to change
- Documented as experimental

---

## Related Documentation

**Specifications:**
- [Standard Library Specification](../../specifications/v0.0.4/standard-library/) - Complete technical specs

**User Guides:**
- [Language Documentation](../language/) - Using standard library
- [Examples](../examples/) - Real-world usage

**Advanced:**
- [Architecture](../architecture/) - Implementation details
- [CLI](../cli/) - Working with standard library

---

**Last Updated:** 2025-12-15
**Version:** v0.0.4
**Maintained by:** Polyglot Standard Library Team
