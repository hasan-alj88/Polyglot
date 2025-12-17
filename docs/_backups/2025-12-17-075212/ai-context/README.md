# Polyglot AI Context Packages

**Purpose:** Version-specific, machine-parseable language specifications optimized for AI agent consumption

**Format:** Maximum-density structured data (EBNF, JSON, YAML, annotated code)

---

## 📋 Available Versions

### [v0.0.2](v0.0.2/) ⚠️ OUTDATED
**Status:** Historical - Language has evolved beyond this version
**Date:** 2025-11-25
**Current Stable:** v0.0.3 (see [user documentation](../user/))

**Contents:**
- Complete v0.0.2 language specification (~25KB across 13 files)
- EBNF grammar, type system, state machine, operators
- DateTime system, runtime wrappers, constraints
- Annotated examples and anti-patterns

**Note:** This version uses:
- `,` (comma) variable prefix (changed to `$` in v0.0.4)
- `\~\` backslash markers (changed to indentation in v0.0.4)
- Limited loop support (full loop system added in v0.0.4)

**Use Case:** Historical reference only - refer to current [user documentation](../user/) for accurate syntax

---

### v0.0.3 🚧 TO BE CREATED
**Status:** Current stable version - AI context package not yet created
**Current Syntax:** See [/docs/user/](../user/) for complete language reference

**Changes from v0.0.2:**
- Same basic syntax as v0.0.2
- Foundation for v0.0.4 features
- See [version roadmap](../specifications/version-roadmap.md) for details

---

### v0.0.4 💡 PLANNED
**Status:** Design phase - Specification in progress
**Target:** Q2 2026

**Major Changes:**
- Variable prefix: `,` → `$`
- Indentation-based nesting (3 spaces) replaces `\~\` markers
- Loop unpack/pack system with mini-pipeline iterations
- Reserved indication with `;` prefix
- 30 total new features

**Design Specs:** See [/docs/specifications/v0.0.4/](../specifications/v0.0.4/)

**AI Context Package:** Will be created after v0.0.4 implementation

---

## 📁 Package Structure

Each version folder contains:

```
v0.0.X/
├── README.md                        # Package overview and usage guide
├── grammar.ebnf                     # Complete syntactic grammar (EBNF)
├── type-system.json                 # Type rules and constraints
├── operators.json                   # Complete operator reference
├── state-machine.yaml               # Variable lifecycle states
├── reserved-enums.json              # Runtime-provided enumerations
├── constraints.yaml                 # Edge cases and validation rules
├── datetime-system.yaml             # DateTime type specification
├── runtime-wrappers.yaml            # Runtime wrapper specifications
├── anti-patterns.yaml               # Common mistakes and anti-patterns
├── examples-annotated.pg            # Canonical patterns with annotations
├── doc-index.yaml                   # Documentation index
├── AI-CONTEXT-CORRECTIONS.md        # Known issues and corrections
└── AI-CONTEXT-PACKAGE-FIXES.md      # Package maintenance notes
```

---

## 🎯 Purpose and Usage

### What AI Context Packages Provide

**Information Density:** 4.3x more compact than prose documentation
**Format:** 100% structured (queryable, parseable, unambiguous)
**Coverage:** Complete language specification in machine-readable formats

### Comparison with Other Documentation

| Documentation Type | Purpose | Format | Audience |
|-------------------|---------|--------|----------|
| **User Docs** ([/user](../user/)) | Language guide | Prose, examples | Developers |
| **AI Context** (this folder) | Language spec | Structured data | AI agents, tools |
| **Design Specs** ([/specifications](../specifications/)) | Evolution | Design documents | Language designers |
| **Technical Docs** ([/technical](../technical/)) | Architecture | Architecture docs | Implementers |

### When to Use AI Context Packages

**Use AI Context When:**
- Generating code programmatically
- Building parsers or compilers
- Implementing language tools (linters, formatters)
- Training AI models on Polyglot syntax
- Quick reference for specific syntax rules

**Use User Docs Instead When:**
- Learning Polyglot as a developer
- Understanding language concepts
- Following tutorials and examples
- Writing `.pg` files manually

---

## 🚀 Quick Start (for AI Agents)

### Reading Order (First Time)
1. `README.md` - Package overview
2. `grammar.ebnf` - Syntax structure
3. `constraints.yaml` - Critical rules
4. `examples-annotated.pg` - Common patterns
5. Reference other files as needed

### Reference Order (During Code Generation)
1. **Syntax check:** `grammar.ebnf` - Is this valid syntax?
2. **Rule check:** `constraints.yaml` - Am I violating a rule?
3. **Type check:** `type-system.json` - Is this type correct?
4. **Operator check:** `operators.json` - What does this operator do?
5. **Pattern check:** `examples-annotated.pg` - How do I use this idiom?

---

## ⚠️ Version Status Warning

**IMPORTANT:** Always verify which version you're using!

Current Polyglot stable version: **v0.0.3**
Current AI context package version: **v0.0.2 (OUTDATED)**

**For current syntax:** See [/docs/user/](../user/) documentation

**For upcoming syntax:** See [/docs/specifications/v0.0.4/](../specifications/v0.0.4/) design specs

---

## 🔧 Package Maintenance

### Creating New Version Packages

When a new Polyglot version is released:

1. **Create version folder:** `ai-context/vX.X.X/`
2. **Generate all specification files:**
   - Update grammar.ebnf with syntax changes
   - Update type-system.json with new types
   - Update constraints.yaml with new rules
   - Update all other files as needed
3. **Create version README.md** explaining changes
4. **Update this index** with version status
5. **Mark previous version** as historical if applicable

### File Format Guidelines

- **EBNF:** Syntax specifications (unambiguous, parseable)
- **JSON:** Structured data (types, operators, enums)
- **YAML:** Hierarchical specs (states, constraints, configs)
- **Annotated .pg:** Real code examples with inline annotations

---

## 🔗 Related Documentation

**Current Language Reference:** [/docs/user/](../user/)
**Design Specifications:** [/docs/specifications/](../specifications/)
**Technical Architecture:** [/docs/technical/](../technical/)
**Project Planning:** [/docs/project/](../project/)

---

## 📊 Version Timeline

| Version | Status | AI Context | Date | Notes |
|---------|--------|------------|------|-------|
| v0.0.2 | Historical | ✅ Available | 2025-11-25 | Initial AI package |
| v0.0.3 | ✅ Current | ❌ Not created | 2025-12-12 | See user docs |
| v0.0.4 | 🔧 Design | ⏳ Planned | Q2 2026 | Major syntax refinement |
| v0.0.5 | 💡 Concept | ⏳ Planned | Q4 2026 | Type system |

---

**Last Updated:** 2025-12-12
**Maintained By:** Polyglot Documentation Team
**Current Stable Version:** v0.0.3 (AI context pending)
