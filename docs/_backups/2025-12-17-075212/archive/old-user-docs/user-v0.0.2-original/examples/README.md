# Polyglot Examples Index

**Version:** 0.0.2
**Status:** Active Development - Design Phase

---

## Overview

Welcome to the Polyglot examples collection! These examples demonstrate real-world workflows that leverage multiple programming languages in a single pipeline.

**Featured Use Case:** AI/ML workflows where Python handles model inference while Rust processes large datasets at high performance.

---

## Quick Start

**New to Polyglot?** Start here:
1. [Hello World](hello-world.md) - Your first multi-language pipeline
2. [Data Processing Basics](data-processing.md) - Working with data across languages
3. [Error Handling](error-handling.md) - Robust error management patterns

---

## Examples by Difficulty

### Beginner
| Example | Description | Languages | Lines |
|---------|-------------|-----------|-------|
| [Hello World](hello-world.md) | Basic pipeline with Python and Rust | Python, Rust | ~20 |
| [File Operations](file-operations.md) | Read, process, and write files | Python | ~30 |

### Intermediate
| Example | Description | Languages | Lines |
|---------|-------------|-----------|-------|
| [Data Processing](data-processing.md) | ETL pipeline with type conversion | Python, Rust | ~50 |
| [Parallel Execution](parallel-execution.md) | Fork/join pattern for concurrent processing | Python | ~60 |
| [Error Handling](error-handling.md) | Error types and recovery strategies | Python, Rust | ~70 |

### Advanced
| Example | Description | Languages | Lines |
|---------|-------------|-----------|-------|
| [AI Model Inference Pipeline](complete-workflows.md#ai-inference) | ML model with Rust data preprocessing | Python, Rust | ~150 |
| [Complete Workflows](complete-workflows.md) | Production-ready multi-stage pipelines | Python, Rust, JS | ~200+ |

---

## Examples by Topic

### AI/ML Workflows

**Featured: Large Dataset Processing with AI Models**

Polyglot excels at AI/ML workflows where:
- **Python** handles model inference (scikit-learn, TensorFlow, PyTorch)
- **Rust** preprocesses massive datasets at native speed
- **Automatic type conversion** handles data passing between languages

**Examples:**
- [AI Model Inference Pipeline](complete-workflows.md#ai-inference) - Image classification with Rust preprocessing
- [Batch Prediction Workflow](complete-workflows.md#batch-prediction) - Process millions of records efficiently
- [Real-time ML Pipeline](complete-workflows.md#realtime-ml) - Streaming data with model scoring

**Typical Pattern:**
```polyglot
[|] MLInferencePipeline

// Rust: Fast data preprocessing (millions of rows/sec)
[r] |Rust.PreprocessData
[<] .input_csv: pg\path << \\Data\\raw_dataset.csv
[>] .clean_data: rs\Vec<Record> >> .preprocessed

// Python: ML model inference
[r] |Python.RunModel
[<] .data: py\list << .preprocessed        // Auto type conversion
[<] .model_path: pg\path << \\Models\\trained_model.pkl
[>] .predictions: py\ndarray >> .results

// Rust: High-performance result aggregation
[r] |Rust.AggregateResults
[<] .predictions: rs\Vec<f64> << .results  // Auto type conversion
[>] .summary: pg\string >> .final_output

[o] .final_output: pg\string
[X]
```

### Data Processing & ETL

**Examples:**
- [Data Processing Basics](data-processing.md) - CSV transformation pipeline
- [File Operations](file-operations.md) - Reading and writing data files
- [Complete Workflows](complete-workflows.md#etl) - Production ETL pipeline

**Use Cases:**
- CSV/JSON data transformation
- Database import/export
- Data validation and cleansing
- Format conversion (CSV → Parquet, JSON → Avro)

### Parallel & Concurrent Processing

**Examples:**
- [Parallel Execution](parallel-execution.md) - Fork/join patterns
- [Complete Workflows](complete-workflows.md#parallel) - Advanced parallelism

**Patterns:**
- Fork/join for independent tasks
- Parallel data partitioning
- Concurrent API calls
- Multi-threaded processing

### Error Handling & Resilience

**Examples:**
- [Error Handling](error-handling.md) - Error types and recovery
- [Complete Workflows](complete-workflows.md#error-recovery) - Production error patterns

**Topics:**
- `!Error` types and custom errors
- Retry strategies
- Fallback mechanisms
- Graceful degradation

### Automation & Scheduling

**Examples:**
- [File Operations](file-operations.md#file-watch) - File system triggers
- [Complete Workflows](complete-workflows.md#scheduled) - Scheduled pipelines

**Triggers:**
- Schedule (cron-like)
- File watch (inotify)
- Webhooks (HTTP)
- Resource-based (CPU/RAM thresholds)

---

## Examples by Use Case

### Machine Learning & AI
| Use Case | Example | Key Features |
|----------|---------|--------------|
| Batch Inference | [AI Model Inference](complete-workflows.md#ai-inference) | Rust preprocessing, Python ML, parallel processing |
| Data Preprocessing | [Data Processing](data-processing.md) | Fast Rust transforms, type conversion |
| Model Training Pipeline | [Complete Workflows](complete-workflows.md#training) | Multi-stage training, checkpointing |
| Real-time Scoring | [Complete Workflows](complete-workflows.md#realtime-ml) | Streaming data, low latency |

### Data Engineering
| Use Case | Example | Key Features |
|----------|---------|--------------|
| ETL Pipeline | [Complete Workflows](complete-workflows.md#etl) | Extract, transform, load with error handling |
| Data Validation | [Error Handling](error-handling.md) | Schema validation, quality checks |
| File Processing | [File Operations](file-operations.md) | Automated file ingestion, transformation |
| Database Migration | [Complete Workflows](complete-workflows.md#migration) | Cross-database data movement |

### DevOps & Automation
| Use Case | Example | Key Features |
|----------|---------|--------------|
| CI/CD Integration | [Complete Workflows](complete-workflows.md#cicd) | Multi-language build & test |
| Log Processing | [Data Processing](data-processing.md) | Parse, filter, aggregate logs |
| System Monitoring | [Complete Workflows](complete-workflows.md#monitoring) | Resource triggers, alerting |
| Backup Automation | [File Operations](file-operations.md) | Scheduled backups, rotation |

### Web Services & APIs
| Use Case | Example | Key Features |
|----------|---------|--------------|
| API Integration | [Complete Workflows](complete-workflows.md#api) | Multi-service orchestration |
| Webhook Processing | [Complete Workflows](complete-workflows.md#webhook) | Event-driven workflows |
| Data Aggregation | [Parallel Execution](parallel-execution.md) | Concurrent API calls |

---

## Learning Paths

### Path 1: Getting Started (Beginner)
**Time:** 1-2 hours
**Goal:** Write your first multi-language pipeline

1. [Hello World](hello-world.md) - Basic syntax and structure
2. [File Operations](file-operations.md) - Working with files
3. [Data Processing](data-processing.md) - Type conversion basics
4. **Next:** [Quick Start Guide](../language/00-quick-start.md)

### Path 2: AI/ML Developer
**Time:** 4-6 hours
**Goal:** Build production ML pipelines with Polyglot

1. [Hello World](hello-world.md) - Polyglot basics
2. [Data Processing](data-processing.md) - Data transformation
3. [Parallel Execution](parallel-execution.md) - Concurrent processing
4. [AI Model Inference Pipeline](complete-workflows.md#ai-inference) - Full ML workflow
5. **Next:** [Complete Workflows](complete-workflows.md)

**Key Skills:**
- Rust for high-speed data preprocessing
- Python for model inference
- Type conversion between languages
- Parallel processing for batch predictions
- Error handling in ML pipelines

### Path 3: Data Engineer
**Time:** 3-5 hours
**Goal:** Build robust ETL pipelines

1. [Data Processing](data-processing.md) - ETL basics
2. [Error Handling](error-handling.md) - Resilient pipelines
3. [File Operations](file-operations.md) - File automation
4. [Complete Workflows](complete-workflows.md#etl) - Production ETL
5. **Next:** [Standard Library](../standard-library/00-overview.md)

### Path 4: DevOps Engineer
**Time:** 2-4 hours
**Goal:** Automate infrastructure workflows

1. [Hello World](hello-world.md) - Pipeline basics
2. [File Operations](file-operations.md) - File triggers
3. [Parallel Execution](parallel-execution.md) - Concurrent tasks
4. [Complete Workflows](complete-workflows.md#cicd) - CI/CD integration
5. **Next:** [Trigger Catalog](../standard-library/04-triggers.md)

---

## Example Template

All examples follow this structure:

```markdown
# Example Title

**Difficulty:** Beginner/Intermediate/Advanced
**Languages:** Python, Rust, JavaScript
**Topics:** Topic1, Topic2
**Time:** ~X minutes

## Overview
Brief description of what this example demonstrates.

## Use Case
Real-world scenario this example solves.

## Complete Code
Full .pg file with comments.

## Explanation
Step-by-step breakdown of how it works.

## Running the Example
How to compile, register, and execute.

## Expected Output
What you should see when it runs.

## Variations
Alternative approaches and modifications.

## See Also
Related examples and documentation.
```

---

## AI/ML Example Highlight

### Image Classification Pipeline with Rust Preprocessing

**Scenario:** Process 10 million images through a CNN model
- **Challenge:** Python is too slow for image preprocessing
- **Solution:** Rust handles image loading, resizing, normalization at native speed

```polyglot
[@] Local@ML.ImageClassification:1.0.0
[#] 1
[X]





// Pipeline: High-performance image classification
[|] BatchImageClassification

[i] .image_dir: pg\path
[i] .batch_size: pg\int << 1000

[t] |T.Call

// Queue: High priority, require GPU
[Q] |Q.Priority
[<] .level: pg\int << 9

[Q] |Q.RequireResource
[<] .cpu_cores: pg\int << 8
[<] .memory_mb: pg\int << 16384
[<] .gpu_count: pg\int << 1

// Rust: Fast image preprocessing (10,000 images/sec)
[r] |Rust.PreprocessImages
[<] .input_dir: pg\path << .image_dir
[<] .target_size: rs\(u32,u32) << (224, 224)
[<] .normalize: rs\bool << true
[>] .image_tensors: rs\Vec<f32> >> .preprocessed_data

// Python: CNN model inference (PyTorch/TensorFlow)
[r] |Python.RunCNNModel
[<] .tensors: py\ndarray << .preprocessed_data     // Auto type conversion
[<] .model_path: pg\path << \\Models\\resnet50.pth
[<] .batch_size: py\int << .batch_size
[>] .predictions: py\ndarray >> .class_predictions
[>] .confidence: py\ndarray >> .confidence_scores

// Rust: Aggregate results and generate report
[r] |Rust.GenerateReport
[<] .predictions: rs\Vec<i32> << .class_predictions  // Auto type conversion
[<] .confidence: rs\Vec<f64> << .confidence_scores
[>] .report: pg\string >> .final_report

[o] .final_report: pg\string
[X]
```

**Performance:**
- **Rust preprocessing:** 10,000 images/sec (vs. 100 images/sec in pure Python)
- **Total throughput:** 100x faster than pure Python pipeline
- **Memory efficiency:** Rust's zero-cost abstractions minimize memory overhead

**See:** [Complete AI/ML Workflow](complete-workflows.md#ai-inference)

---

## Contributing Examples

Have a great Polyglot example? We'd love to include it!

**Guidelines:**
1. Follow the example template above
2. Use v0.0.2 syntax (check [BNF Grammar](../language/bnf/polyglot grammer.md))
3. Include clear comments and explanations
4. Test the example thoroughly
5. Document expected inputs/outputs

**Submit via:** Project issue tracker or pull request

---

## Available Examples

### Core Examples
- [01: Hello World](01-hello-world.md) - Approved beginner examples
- [Hello World Collection](hello-world.md) - Multiple hello world patterns
- [Data Processing](data-processing.md) - ETL and data transformation
- [Error Handling](error-handling.md) - Error types and recovery
- [Parallel Execution](parallel-execution.md) - Concurrent processing patterns
- [File Operations](file-operations.md) - File I/O and automation
- [Complete Workflows](complete-workflows.md) - Production-ready pipelines

### Specialized Examples
- [07: Approved Examples](07-approved-examples.md) - Curated collection

---

## See Also

### Language Documentation
- [Quick Start Guide](../language/00-quick-start.md) - Get started quickly
- [Complete Syntax](../language/01-syntax-complete.md) - Full syntax reference
- [Type System](../language/02-type-system.md) - Cross-language types
- [BNF Grammar](../language/bnf/polyglot grammer.md) - Authoritative grammar

### Reference Documentation
- [Standard Library](../standard-library/00-overview.md) - Built-in utilities
- [Trigger Catalog](../standard-library/04-triggers.md) - All trigger types
- [Queue Control](../standard-library/02-queue-control.md) - Queue operations

### Architecture
- [System Architecture](../architecture/00-overview.md) - How Polyglot works
- [IR Representation](../architecture/02-ir-representation.md) - Type system internals

---

**Last Updated:** 2025-11-15