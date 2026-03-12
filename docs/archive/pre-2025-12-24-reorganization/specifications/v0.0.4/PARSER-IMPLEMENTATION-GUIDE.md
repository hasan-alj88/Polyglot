---
# BMAD Agent Context Block
# Schema: bmad-context-v1

# --- Identity ---
id: parser-implementation-guide
shard: false

# --- Classification ---
type: guide
topic: Parser Implementation Quick Start
summary: Quick start guide for implementing Polyglot v0.0.4 parser
keywords:
  - parser
  - implementation
  - guide
  - v0.0.4

# --- BMAD Agent Routing ---
agents:
  - developer
phase: implementation
workflow: any
module: bmm
complexity: high

# --- Dependency Chain ---
prereqs:
  - language/syntax/README.md
  - reference/README.md
  - reference/token-patterns.md
  - reference/syntax-patterns.md
unlocks:
  - parser implementation

# --- Relationships ---
related:
  - language/README.md
  - reference/grammar.md

# --- Metadata ---
status: stable
updated: 2025-12-18
version: 0.0.4
tags:
  - "#parser"
  - "#implementation"
  - "#guide"
last-redoc-date: 2025-12-18
---

# Polyglot v0.0.4 Parser Implementation Quick Start

**Created:** 2025-12-18
**Documentation Status:** ✅ Complete
**Total Lines:** ~3,000 lines of implementation guidance

---

## 📋 Documentation Inventory

### Core References (5 files)

1. **[language/syntax/README.md](./User/language/syntax/README.md)** - Critical syntax patterns
   - Reserved vs custom hierarchies (`;` vs `.`)
   - Definition vs invocation dual-context
   - Indentation-based nesting rules
   - Inline pipeline syntax

2. **[reference/README.md](./User/reference/README.md)** - Formal specifications
   - Operator precedence table (11 levels)
   - Token sequence patterns
   - Context-sensitive parsing rules
   - Error recovery strategies

3. **[reference/token-patterns.md](./User/reference/token-patterns.md)** - Token inventory
   - All 121 tokens catalogued
   - Token sequence recognition
   - AST node mapping
   - Lexer integration patterns

4. **[reference/syntax-patterns.md](./User/reference/syntax-patterns.md)** - Pattern catalog
   - 20 single-line pattern groups
   - 12 multi-line pattern groups
   - Complete EBNF grammar
   - Implementation priority phases (1-4)

5. **[language/README.md](./User/language/README.md)** - Language overview
   - Core language concepts
   - Implementation status
   - Quick start for implementers
   - Breaking changes from v0.0.3

---

## 🚀 Quick Start for Parser Developers

### Step 1: Understand Core Patterns (15 minutes)
Read: [language/syntax/README.md](./User/language/syntax/README.md)

**Focus on:**
- Reserved indication (`;` hierarchies)
- Inline pipelines (formatted string requirement)
- Indentation mechanics (3-space rule)
- Definition blocks (`{X}...{x}`)

### Step 2: Review Token System (10 minutes)
Read: [reference/token-patterns.md](./User/reference/token-patterns.md)

**Focus on:**
- Token categories and counts
- Context-sensitive token interpretation
- Lookahead requirements

### Step 3: Study Syntax Patterns (20 minutes)
Read: [reference/syntax-patterns.md](./User/reference/syntax-patterns.md)

**Focus on:**
- Phase 1 patterns (MVP)
- EBNF grammar structure
- Pattern validation checklist

### Step 4: Check Precedence & Rules (10 minutes)
Read: [reference/README.md](./User/reference/README.md#operator-precedence-table)

**Focus on:**
- Operator precedence (11 levels)
- Associativity rules
- Context disambiguation

---

## 🎯 Implementation Priorities

### Phase 1: Core Patterns (MVP)
✅ Already have lexer (~85% v0.0.4 complete)

**Next Steps:**
1. Variable assignment (`$x << 5`)
2. Literals (int, float, string)
3. Basic binary operators (`+`, `-`, `*`, `/`)
4. Simple conditionals (`[?]`)
5. Pipeline calls (basic, no I/O)

### Phase 2: Control Flow
6. Indentation-based nesting (3-space rule)
7. Match expressions (`[m]`)
8. Error handling (`[!]`)
9. Boolean logic markers (`[+]`, `[&]`, `[-]`)

### Phase 3: Advanced Features
10. Pipeline definitions (`{|}...{x}`)
11. I/O binding (`[|] <input << $val`)
12. Enum/error definitions (`{#}`, `{!}`)
13. Loop constructs (`[~]`, `[*]`)

### Phase 4: Complex Patterns
14. Reserved indication (`#Boolean.True`)
15. Inline pipelines (`|Pipe"{$arg}"`)
16. Dual-context markers
17. Pipeline composition (`|>`)
18. Type inference

---

## 🔍 Key Parsing Challenges

### Challenge 1: Reserved Indication
**Pattern:** `#DT.Business;Week.CustomWeek;RestDays`

**Rule:** Each delimiter (`;` or `.`) determines the nature of the **next** segment.

**Implementation:**
```rust
// Track current delimiter
// After ';' → mark next segment as reserved
// After '.' → mark next segment as custom
```

**See:** [language/syntax/README.md](./User/language/syntax/README.md#1-reserved-vs-custom-hierarchy)

### Challenge 2: Inline Pipelines
**Pattern:** `|FormatName"{$first} {$last}"`

**Rule:** PipelineIdentifier immediately followed by StringStart (no whitespace)

**Implementation:**
```rust
// Lookahead: if next token is StringStart
// Parse formatted string as ONLY input
// Extract arguments from interpolations
```

**See:** [reference/README.md](./User/reference/README.md#inline-pipeline-pattern)

### Challenge 3: Indentation Nesting
**Pattern:**
```polyglot
[m] $status
   [?] 1 ? #Active
   [?] 2 ? #Inactive
      [r] $log << "Message"
```

**Rule:** 3 spaces = 1 nesting level (sub-marker relationship)

**Implementation:**
```rust
// Track indentation stack
// Each 3 spaces increases nesting level
// Create parent-child AST relationships
```

**See:** [reference/README.md](./User/reference/README.md#indentation-pattern)

### Challenge 4: Dual-Context Markers
**Pattern:** `[|]` has different meanings in definition vs invocation

**Rule:** Track parser state (DEFINITION_BLOCK vs INVOCATION)

**Implementation:**
```rust
// State machine approach
// In definition: markers declare components
// In invocation: markers bind arguments
```

**See:** [reference/README.md](./User/reference/README.md#marker-x-disambiguation)

---

## 📊 Token Summary

**Total:** 121 tokens across 12 categories

**Categories:**
- Block Markers: 30 tokens
- Operators (Push): 6 tokens
- Operators (Compare): 18 tokens
- Operators (String): 1 token
- Operators (Other): 6 tokens
- Identifiers: 11 types
- Delimiters: 15 tokens
- Keywords: 8 tokens
- Literals: 10 types
- Comments: 2 types
- Metadata: 4 types
- Whitespace: 3 types

**See:** [reference/token-patterns.md](./User/reference/token-patterns.md#token-inventory-by-category)

---

## 🔗 Cross-References

**Main Spec:** [README.md](./README.md)
**Grammar:** [reference/grammar.md](./User/reference/grammar.md)
**AI Context:** [reference/ai-context.md](./User/reference/ai-context.md)
**Lexer Code:** `../../polyglot-lexer/src/`
**Parser Code:** `../../polyglot-parser/src/`

---

## ✅ Checklist for Parser Implementation

**Before Starting:**
- [ ] Read all 5 core reference documents
- [ ] Understand token system (121 tokens)
- [ ] Review EBNF grammar
- [ ] Check operator precedence table

**Phase 1 (MVP):**
- [ ] Parse variable assignments
- [ ] Parse literals (int, float, string)
- [ ] Parse binary expressions with precedence
- [ ] Parse simple conditionals
- [ ] Parse basic pipeline calls

**Phase 2 (Control Flow):**
- [ ] Implement indentation tracking
- [ ] Parse match expressions
- [ ] Parse error handling blocks
- [ ] Parse boolean logic markers

**Phase 3 (Advanced):**
- [ ] Parse pipeline definitions
- [ ] Parse I/O binding
- [ ] Parse enum/error definitions
- [ ] Parse loop constructs

**Phase 4 (Complex):**
- [ ] Parse reserved indication
- [ ] Parse inline pipelines
- [ ] Implement dual-context markers
- [ ] Parse pipeline composition

---

**Last Updated:** 2025-12-18
**Next Step:** Begin Phase 1 parser implementation
**Questions?** See individual documentation files for detailed explanations
