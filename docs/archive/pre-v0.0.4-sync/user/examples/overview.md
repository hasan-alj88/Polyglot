---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/examples/overview.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Examples Overview

**Version:** 0.0.2  
**Last Updated:** 2025-12-02

---

## Introduction

This directory contains **complete, working examples** of Polyglot code demonstrating real-world use cases.

---

## Prerequisites

### 1. Polyglot Service Running

```bash
polyglot-service start
```

### 2. Language Runtimes Installed

- **Python 3.8+** - For Python wrappers
- **Rust 1.70+** - For Rust wrappers
- **Node.js 18+** - For Node wrappers
- **Go 1.20+** - For Go wrappers (optional

---

## Example Categories

### [Cross-Language Integration](cross-language-integration.md

Real-world examples of calling Python, Rust, Node, and Go from Polyglot

### [Automation Workflows](automation-workflows.md

Time-based and event-driven automation

### [Multi-Step Pipelines](multi-step-pipelines.md

Complex data processing pipelines

### [Error Handling Patterns](error-handling-patterns.md

Production-ready error handling

---

## Quick Start Template

```polyglot
[@] @Local::MyExample:1.0.0.0
[X]

[|] |MyFirstPipeline
[i] .input:pg.string
[t] |T.Call
[o] .result:pg.string

[r] .result:pg.string << "Hello, {.input!"

[o] .result:pg.string
[X]
```

---

**Next:** [Cross-Language Integration →](cross-language-integration.md
