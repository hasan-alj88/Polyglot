<!-- ARCHIVED: 2025-12-16 | Reason: Historical context, not current specification | Superseded by: Current specifications in /language/ and /features/ -->

# v0.0.4 Syntax Refinement Specifications

**Status:** 🔧 Design Phase - 33 Features
**Target Release:** Q2 2026

---

## 📋 Overview

The syntax refinement component of v0.0.4 introduces 33 features designed to make Polyglot cleaner, more expressive, and more powerful. These features range from breaking changes (indentation, variable prefix) to entirely new capabilities (metadata system, pipeline composition, boolean logic markers).

**Character Reduction:** 44% fewer characters with indentation-based nesting
**New Capabilities:** Metadata, composition, inline evaluation, advanced operators
**Breaking Changes:** Variable prefix, nesting syntax, reserved indication

---

## 📁 Core Specifications

### [v0.0.4 Final Decisions](v0.0.4-final-decisions.md) ⭐
**Purpose:** Comprehensive decisions document - primary reference
**Status:** ✅ Finalized

**Contents:**
- All 30 features with rationale
- Breaking changes justified
- Integration with loop system
- Complete feature overview

### [v0.0.4 Complete Syntax](v0.0.4-complete-syntax.md)
**Purpose:** Full syntax specification with all features
**Status:** ✅ Complete

**Contents:**
- Syntax for all 30 features
- Grammar rules and patterns
- Usage examples for each feature
- Edge cases and constraints

### [v0.0.4 Final Syntax Decisions](v0.0.4-final-syntax-decisions.md)
**Purpose:** Finalized syntax choices and rationale
**Contents:**
- Syntax decision history
- Alternatives considered
- Final choices with justification

### [v0.0.4 Design Decisions Final](v0.0.4-design-decisions-final.md)
**Purpose:** Design philosophy and principles
**Contents:**
- Design principles guiding v0.0.4
- Trade-offs and compromises
- Long-term vision alignment

---

## 📁 Feature-Specific Documents

### Reserved Enumeration System

#### [Reserved Enumeration Indication](reserved-enum-indication.md)
**Purpose:** Complete evaluation of all proposals for reserved enum marking
**Contents:**
- All 8 proposals evaluated
- Pros/cons analysis
- Final recommendation

#### [Reserved Enum Semicolon Prefix](reserved-enum-semicolon-prefix.md) ⭐
**Purpose:** Finalized `;` prefix approach for reserved segments
**Status:** ✅ Approved

**Syntax:**
```polyglot
#;Boolean;True              // Reserved enum
#;DT;Business.MyCompany     // Mix reserved + user-defined
!;Error;Network;Timeout     // Reserved error
```

**Rationale:**
- Visually distinct without ceremony
- Allows mixing reserved/user segments
- No syntax conflicts
- Clear at-a-glance identification

### Syntax Patterns and References

#### [v0.0.4 Hierarchy Pattern Reference](v0.0.4-hierarchy-pattern-reference.md)
**Purpose:** Hierarchy notation patterns across all constructs
**Contents:**
- Variable hierarchies (`.variable.field.nested`)
- Pipeline namespaces (`DT.Gregorian.November`)
- Enumeration fields (`#Enum.variant.nested`)
- Error hierarchies (`!Error.Category.Specific`)
- ASCII tree notation standards

#### [v0.0.4 Syntax Comparison](v0.0.4-syntax-comparison.md)
**Purpose:** Before/after syntax comparisons
**Contents:**
- v0.0.3 vs v0.0.4 side-by-side examples
- Visual impact of changes
- Migration examples

### Type System Enhancements

#### [v0.0.4 Alias System](v0.0.4-alias-system.md)
**Purpose:** Type and path aliasing system
**Contents:**
- Type aliasing syntax
- Path aliasing for imports
- Namespace shortcuts
- Resolution rules

---

## 📁 Enhancement Proposals

### [Additional Syntax Improvements](additional-syntax-improvements.md)
**Purpose:** Proposed syntax enhancements beyond core 30 features
**Status:** 💡 Future consideration

**Contents:**
- Additional operator proposals
- Syntax sugar candidates
- Future language enhancements

### [Metadata-Driven Improvements](metadata-driven-improvements.md)
**Purpose:** Metadata system proposals and patterns
**Contents:**
- Metadata types and uses
- Standard metadata tags
- Extension mechanisms
- Best practices

### [Pipeline Composition Examples](pipeline-composition-examples.md)
**Purpose:** Pipeline chaining patterns with `|>` operator
**Contents:**
- Composition patterns
- Real-world examples
- Best practices
- Performance considerations

---

## 🎯 The 33 Features

### Breaking Changes (3)

1. **Indentation-Based Nesting** - 3 spaces replace `\~\` markers (44% reduction)
2. **Variable Prefix Change** - `,` → `$` (eliminates ambiguity)
3. **Reserved Indication** - `;` prefix for reserved enum/error segments

### Pipeline Features (4)

4. **Pipeline I/O Distinction** - `[|] <param` and `[|] >param` for definition
5. **Inline Pipelines** - `|Pipeline""` evaluate in-place
6. **Pipeline Composition** - `|P1 |> |P2 |> |P3` chaining
7. **Trigger OR** - `[|]` under `[t]` for multiple trigger types

### String & Collection Features (3)

8. **Multi-line Strings** - `[+] +` explicit concatenation with prefix
9. **Collection Literals** - `{1, 2, 3}` array/set syntax
10. **String Interpolation** - Enhanced formatting

### Operator Features (4)

11. **Range Operators** - 4 variants: `?[,]`, `?(,]`, `?[,)`, `?(,)`
12. **Operator Negation** - `!?` universal negation (`!=?`, `>!?`, `!in?`)
13. **Collection Membership** - `in?` / `!in?` operators
14. **Comparison Operators** - Enhanced set

### Control Flow Features (5)

15. **Boolean Markers** - `[&]` AND, `[|]` OR, `[^]` XOR for complex conditions
16. **Wildcard Condition** - `[f] *` for exhaustive matching (else case)
17. **Match Exhaustiveness** - Compiler enforces wildcard case
18. **Early Return Pattern** - `[>] o>error << !Error` inside conditionals
19. **Indentation Grouping** - Logical grouping for boolean expressions

### Enum & Struct Features (3)

20. **Enum Value Fields** - Enums can contain value fields
21. **Struct Shorthand** - Field name inference: `[.] .name` assumes `<< $name`
22. **Struct Auto-fill** - Pipeline outputs auto-fill matching struct fields

### Metadata & Documentation (3)

23. **Metadata System** - `%Doc`, `%Author`, `%Deprecated`, etc.
24. **Variadic Input** - `<<<` shorthand enabled by `%InStream`
25. **Documentation Tags** - Standard metadata for docs generation

### Syntax Sugar (3)

26. **Block Comments** - `/* ... */` syntax
27. **ForEach.Range** - String literal ranges: `~ForEach.Range"1..100"`
28. **Operator Shortcuts** - Convenient syntax for common patterns

### Type System (2)

29. **Type Inference** - Enhanced type inference rules
30. **Type Compatibility** - Expanded compatibility matrix

### Error Handling (2)

31. **Error Chaining** - Link related errors
32. **Error Context** - Rich error information

### I/O Features (1)

33. **Streaming Support** - Foundation for streaming operations

---

## 🔑 Key Design Principles

### 1. Explicit Over Implicit
Features prioritize clarity even if slightly more verbose
- `[+]` for multi-line strings prevents silent bugs
- `;` prefix makes reserved segments obvious
- `|>` makes composition explicit

### 2. Consistency Across Constructs
Patterns work uniformly across language
- `!?` negation works for all operators
- Hierarchy notation works for variables, pipelines, enums, errors
- Metadata works for all declaration types

### 3. No Syntax Ambiguity
Unambiguous parsing is mandatory
- `$` eliminates comma/range confusion
- Indentation rules are strict (3 spaces)
- Reserved indication prevents namespace collisions

### 4. Extensibility Without Bloat
Features enable capabilities without syntax proliferation
- Metadata system adds features via `%` tags
- Inline pipelines reuse existing pipeline syntax
- Composition uses single operator `|>`

### 5. Migration-Friendly Breaking Changes
Breaking changes have clear migration paths
- Variable prefix: automated find-replace
- Indentation: automated conversion tool
- Reserved indication: automated addition

---

## 📖 Reading Order

### First Time (Understanding v0.0.4):
1. [v0.0.4 Final Decisions](v0.0.4-final-decisions.md) - Overview of all features
2. [v0.0.4 Complete Syntax](v0.0.4-complete-syntax.md) - Detailed syntax
3. [v0.0.4 Syntax Comparison](v0.0.4-syntax-comparison.md) - Before/after examples

### Breaking Changes Deep Dive:
1. [Reserved Enum Semicolon Prefix](reserved-enum-semicolon-prefix.md) - `;` prefix details
2. [v0.0.4 Final Syntax Decisions](v0.0.4-final-syntax-decisions.md) - Syntax choices
3. [v0.0.4 Design Decisions Final](v0.0.4-design-decisions-final.md) - Philosophy

### Specific Features:
1. [v0.0.4 Alias System](v0.0.4-alias-system.md) - Type aliasing
2. [Metadata-Driven Improvements](metadata-driven-improvements.md) - Metadata system
3. [Pipeline Composition Examples](pipeline-composition-examples.md) - Pipeline chaining
4. [v0.0.4 Hierarchy Pattern Reference](v0.0.4-hierarchy-pattern-reference.md) - Hierarchies

### Future Exploration:
1. [Additional Syntax Improvements](additional-syntax-improvements.md) - Future features
2. [Reserved Enumeration Indication](reserved-enum-indication.md) - All proposals evaluated

---

## 🔄 Migration Impact

### High Impact (Breaking Changes)

**Variable Prefix: `,` → `$`**
- **Affected:** All variable references
- **Migration:** Automated find-replace with context awareness
- **Effort:** Low (automated)

**Indentation: `\~\` → 3 spaces**
- **Affected:** All nested blocks
- **Migration:** Automated conversion tool
- **Effort:** Low (automated)

**Reserved Indication: Add `;`**
- **Affected:** Reserved enum/error references
- **Migration:** Automated addition to reserved segments
- **Effort:** Low (automated)

### Medium Impact (New Patterns Available)

- Loop system integration
- Inline pipelines for simple cases
- Early return patterns in conditionals
- Pipeline composition for readability

### Low Impact (Optional Enhancements)

- Metadata for documentation
- Struct shorthand for convenience
- Multi-line strings where needed
- Collection membership operators

---

## 🔗 Related Documentation

**v0.0.4 Overview:** [../README.md](../README.md) - Complete v0.0.4 features
**Loop System:** [../loop-system/](../loop-system/) - Loop specifications
**Version Roadmap:** [../../version-roadmap.md](../../version-roadmap.md) - Version timeline
**v0.0.5 Concepts:** [../../v0.0.5/](../../v0.0.5/) - Future type system
**Current Implementation:** [/docs/user/](../../../user/) - v0.0.3 reference

---

## ⏱️ Implementation Timeline

### Phase 1: Breaking Changes (Q1 2026)
- Indentation-based nesting parser
- Variable prefix migration
- Reserved indication support

### Phase 2: Core Features (Q2 2026)
- Inline pipelines
- Multi-line strings
- Range operators
- Operator negation
- Early return patterns

### Phase 3: Advanced Features (Q2 2026)
- Pipeline composition
- Metadata system
- Struct shorthand/auto-fill
- Enum value fields
- Type enhancements

### Phase 4: Refinement (Q3 2026)
- Documentation generation
- Migration tools
- Standard library updates
- Performance optimization

---

## 📊 Feature Adoption Strategy

### MVP Features (Must Have)
1. Indentation-based nesting
2. Variable prefix change
3. Reserved indication
4. Pipeline I/O distinction
5. Inline pipelines

### Phase 2 Features (High Value)
6. Multi-line strings
7. Range operators
8. Operator negation
9. Early return
10. Collection literals

### Phase 3 Features (Enhanced Capability)
11. Pipeline composition
12. Metadata system
13. Struct shorthand
14. Match exhaustiveness
15. Error chaining

### Future Features (Post-v0.0.4)
16. Advanced metadata tags
17. Additional operator shortcuts
18. Enhanced type inference
19. Streaming support
20. Variadic patterns

---

**Last Updated:** 2025-12-12
**Specification Status:** 🔧 Design Phase (95% complete)
**Implementation Target:** Q1-Q2 2026
