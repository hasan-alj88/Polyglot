# Polyglot Documentation Master Index

**Last Updated:** 2025-12-23  
**Total Documents:** 318 active documents  
**Documentation Version:** v0.0.4

---

## 📖 Quick Navigation

- [Getting Started](#getting-started)
- [Language Specification (v0.0.4)](#language-specification-v004)
- [Standard Library](#standard-library)
- [Examples & Tutorials](#examples--tutorials)
- [Technical Documentation](#technical-documentation)
- [Project Documentation](#project-documentation)
- [Reference](#reference)

---


## Getting Started

Essential documentation for new users of the Polyglot programming language.

- [Core Principles](getting-started/core-principles.md) - Fundamental concepts of Polyglot
- [Documentation README](README.md) - Overview of documentation structure

## Language Specification (v0.0.4)

**Location:** `docs/specifications/v0.0.4/`

The authoritative specification for Polyglot v0.0.4.

### Core Language

- [Inline Pipelines](specifications/v0.0.4/language/advanced/inline-pipelines.md) - advanced/
- [Loop System](specifications/v0.0.4/language/advanced/loop-system.md) - advanced/
- [Metadata System](specifications/v0.0.4/language/advanced/metadata-system.md) - advanced/
- [Reserved Indication](specifications/v0.0.4/language/advanced/reserved-indication.md) - advanced/
- [Serial Load Block](specifications/v0.0.4/language/advanced/serial-load-block.md) - advanced/
- [Pipeline Structure](specifications/v0.0.4/language/control-flow/pipeline-structure.md) - control-flow/
- [README](specifications/v0.0.4/language/README.md) - docs/specifications/v0.0.4/language/
- [Io Operators](specifications/v0.0.4/language/syntax/io-operators.md) - syntax/
- [Markers](specifications/v0.0.4/language/syntax/markers.md) - syntax/
- [Operators](specifications/v0.0.4/language/syntax/operators.md) - syntax/
- [Prefix System](specifications/v0.0.4/language/syntax/prefix-system.md) - syntax/
- [README](specifications/v0.0.4/language/syntax/README.md) - syntax/
- [Enums Serial](specifications/v0.0.4/language/types/enums-serial.md) - types/
- [Type System](specifications/v0.0.4/language/types/type-system.md) - types/
- [Variables Lifecycle](specifications/v0.0.4/language/types/variables-lifecycle.md) - types/

### Standard Library

Complete reference for Polyglot's standard library functions.

**Note:** Authoritative source is `specifications/v0.0.4/stdlib/`. The `stdlib/` folder at root is a mirror.

**Categories:**
- [wrappers/](specifications/v0.0.4/stdlib/wrappers/) - 1 documents
- [utilities/](specifications/v0.0.4/stdlib/utilities/) - 52 documents
- [loops/](specifications/v0.0.4/stdlib/loops/) - 35 documents

## Examples & Tutorials

- [Example 1 Config Validator](examples/inline-pipeline-parser/example-1-config-validator.md)
- [Example 2 Email Validator](examples/inline-pipeline-parser/example-2-email-validator.md)
- [Example 3 Url Parser](examples/inline-pipeline-parser/example-3-url-parser.md)
- [Example 4 Key Value Config Parser](examples/inline-pipeline-parser/example-4-key-value-config-parser.md)
- [Example 5 Command Parser With Action Enum](examples/inline-pipeline-parser/example-5-command-parser-with-action-enum.md)
- [Index](examples/inline-pipeline-parser/index.md)
- [Overview](examples/inline-pipeline-parser/overview.md)
- [Pattern Summary](examples/inline-pipeline-parser/pattern-summary.md)
- [See Also](examples/inline-pipeline-parser/see-also.md)

## Technical Documentation

Implementation details, architecture decisions, and technical specifications.

### Architecture
- [01-executive-summary](technical/architecture/01-executive-summary.md)
- [02-philosophy-and-concepts](technical/architecture/02-philosophy-and-concepts.md)
- [03-project-initialization-and-decisions](technical/architecture/03-project-initialization-and-decisions.md)
- [04-project-structure](technical/architecture/04-project-structure.md)
- [05-technology-stack](technical/architecture/05-technology-stack.md)
- [06-patterns](technical/architecture/06-patterns.md)
- [07-data-architecture](technical/architecture/07-data-architecture.md)
- [08-security](technical/architecture/08-security.md)
- [09-performance](technical/architecture/09-performance.md)
- [10-deployment](technical/architecture/10-deployment.md)
- [11-development-environment](technical/architecture/11-development-environment.md)
- [12-adrs](technical/architecture/12-adrs.md)
- [index](technical/architecture/index.md)

### Technical Decisions
- [approved](technical/decisions/approved.md)
- [pending](technical/decisions/pending.md)

## Project Documentation

Project management, user stories, and development planning.

### User Stories
- [1-1-project-workspace-build-system-setup](project/stories/1-1-project-workspace-build-system-setup.md)
- [1-2-lexer-token-definitions](project/stories/1-2-lexer-token-definitions.md)
- [1-3-lexer-implementation](project/stories/1-3-lexer-implementation.md)
- [1-4-ast-gap-analysis](project/stories/1-4-ast-gap-analysis.md)
- [1-4-parser-ast-definitions](project/stories/1-4-parser-ast-definitions.md)
- [1-5-5-multi-file-compilation-same-package-resolution](project/stories/1-5-5-multi-file-compilation-same-package-resolution.md)
- [1-6-syntax-validator-standalone](project/stories/1-6-syntax-validator-standalone.md)
- [1-7-december-2025-syntax-updates](project/stories/1-7-december-2025-syntax-updates.md)
- [1-8-serial-error-handling-test-coverage](project/stories/1-8-serial-error-handling-test-coverage.md)
- [1-9-syntax-consistency-operator-prefixes](project/stories/1-9-syntax-consistency-operator-prefixes.md)

---

## 📋 Documentation Structure

```
docs/
├── specifications/v0.0.4/   ⭐ Authoritative language specification
│   ├── language/             Language features and syntax
│   ├── stdlib/               Standard library reference
│   └── reference/            Quick references and guides
├── examples/                 Code examples and tutorials
├── technical/                Technical implementation docs
├── project/                  Project management docs
├── Audit/                    Documentation quality audits
└── archive/                  Historical versions
```

## 🔗 Related Resources

- [PARSER-IMPLEMENTATION-GUIDE.md](specifications/v0.0.4/PARSER-IMPLEMENTATION-GUIDE.md)
- [PARSER-QUICK-REFERENCE.md](specifications/v0.0.4/PARSER-QUICK-REFERENCE.md)
- [CORRECT-PATTERNS-v0.0.4.md](specifications/v0.0.4/CORRECT-PATTERNS-v0.0.4.md)

---

*Generated by Scribe Documentation System - 2025-12-23*
