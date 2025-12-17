# Polyglot Version Index

**Last Updated:** 2025-12-14
**Current Stable:** v0.0.3
**Latest Finalized:** v0.0.4 (December 2025)
**Implementation Target:** v0.0.3 (Epic 1)

---

## Quick Reference Matrix

| Version | Status | Spec Status | Target Date | Documentation | AI Context |
|---------|--------|-------------|-------------|---------------|------------|
| v0.0.2 | 📦 Archived | Complete | Historical | [Archive](../../archive/old-user-docs/) | [Available](../../ai-context/v0.0.2/) |
| v0.0.3 | ✅ Current | Complete | Stable Now | [User Docs](../../user/) | 🚧 Pending |
| v0.0.4 | ✅ Finalized | **100%** | Q2 2026 | [Specifications](v0.0.4/COMPLETE-SPEC.md) | 💡 Planned |
| v0.0.5 | 💡 Concept | 20% | Q4 2026 | [Proposals](v0.0.5/) | - |
| v0.1.0 | 🎯 Target | - | Q1 2027 | - | - |

**Legend:**
- 📦 Archived - Historical reference only
- ✅ Current/Finalized - Active or completed
- 🚧 Pending - Planned for creation
- 💡 Concept - Early design phase
- 🎯 Target - Future milestone

---

## Version Details

### v0.0.2 - Original Design (Archived)

**Status:** 📦 Archived (Historical)
**Date:** 2025 (Historical)
**Type:** Initial implementation

**Key Features:**
- Basic syntax with backslash markers (`\~\`)
- Variable prefix: `,` (comma)
- Block delimiters: `[|]...[X]`
- Pipeline definitions
- Enum/struct system
- Error handling basics

**Documentation:**
- 📂 [Archived User Docs - Original](../../archive/old-user-docs/user-v0.0.2-original/) - Earlier iteration (66 files)
- 📂 [Archived User Docs - Revised](../../archive/old-user-docs/user-v0.0.2-revised/) - Later iteration (67 files)
- 🤖 [AI Context Package](../../ai-context/v0.0.2/) - 13 machine-readable files

**Notable Differences Between Iterations:**
- **Original:** Uses `Fixed`/`Default` keywords
- **Revised:** Uses operators `<<` for fixed, `<~` for default

**Current Use:** Historical reference only

---

### v0.0.3 - Current Stable

**Status:** ✅ Current Stable
**Date:** 2025 (Current)
**Type:** Production version
**Epic 1 Target:** This is the implementation target for Epic 1

**Key Features:**
- Refined syntax with backslash markers (`\~\`)
- Variable prefix: `,` (comma)
- Block delimiters: `{|}...{x}` for pipelines
- Block delimiters: `{#}...{#}` for enums/structs
- Block delimiters: `{!}...{!}` for errors
- Block delimiters: `{@}...{x}` for registries
- Conditional markers `[y]`
- Pattern markers (ForEach, etc.)
- Basic I/O operators

**Documentation:**
- 📖 [User Documentation](../../user/) - Complete language reference
- 📋 [PRD](../../project/prd.md) - Product requirements
- 📋 [Epic Breakdown](../../project/epics.md) - Implementation epics
- 📂 [Stories](../../project/stories/) - User stories for Epic 1
- 🔧 [Architecture](../../technical/architecture.md) - Technical design

**AI Context:**
- 🚧 To be created (pending Epic 1 completion)

**Implementation Status:**
- Epic 1: Lexer & Parser Foundation (IN PROGRESS)
- See [Sprint Status](../../project/stories/sprint-status.yaml)

**Next Steps:**
- Complete Epic 1 implementation
- Generate v0.0.3 AI context package
- Validate with real-world examples

---

### v0.0.4 - Major Syntax Refinement ✅ FINALIZED

**Status:** ✅ Finalized (December 2025)
**Date:** Finalized 2025-12-14
**Type:** Major breaking changes + new features
**Target Implementation:** Q2 2026

**Key Changes:**
1. **Variable Prefix:** `,` → `$` (breaking)
2. **Indentation:** `\~\` markers → 3-space indentation (breaking)
3. **Reserved Indication:** Semicolon `;` for reserved segments (new)
4. **Loop System:** Unpack `[~]` / Pack `[*]` operators (new)
5. **Metadata System:** `%` prefix annotations (new)

**Documentation Structure:**

**📌 Master Navigation:**
- [COMPLETE-SPEC.md](v0.0.4/COMPLETE-SPEC.md) - **Start here!** Navigation hub for all v0.0.4 documentation

**🎯 Core Features:**
- [Loop System](v0.0.4/features/loop-system.md) - Unpack/pack with mini-pipeline iterations
- [Reserved Indication System](v0.0.4/features/reserved-indication-system.md) - Semicolon for reserved segments
- [Metadata System](v0.0.4/features/metadata-system.md) - Complete metadata tree structure

**📂 Design Evolution:**
- [loop-system/](v0.0.4/loop-system/) - Loop system design history (7 files)
- [syntax-refinement/](v0.0.4/syntax-refinement/) - Syntax improvement proposals (8+ files)
- [core-syntax/](v0.0.4/core-syntax/) - Core syntax elements (to be created)

**📦 Archive:**
- [syntax-refinement/archive/](v0.0.4/syntax-refinement/archive/) - 4 historical specification files

**AI Context:**
- 💡 Planned after implementation begins

**Breaking Changes:**

| Change | v0.0.3 | v0.0.4 | Migration |
|--------|--------|--------|-----------|
| Variables | `,name` | `$name` | Automated find-replace |
| Nesting | `\~\` markers | 3-space indent | Automated parser |
| Reserved Enums | `#Boolean.True` | `#;Boolean;True` | Automated for stdlib |
| IO Definition | `[i]`/`[o]` | `[|] <param`/`[|] >param` | Manual review |
| Types | `pg\string` | `:pg.string` | Automated |

**New Features:**

**Loop System:**
```polyglot
[p] ~ForEach.Array
[~] <array << $items               // Unpack: main → iteration
[~] >item >> $element              // Output to iteration
   [r] $processed << |Transform <input << $element
   [v] *Into.Array                 // Pack: iteration → main
   [*] <item << $processed
   [*] >array >> $results
```

**Reserved Indication:**
```polyglot
#;Boolean;True                     // Fully reserved
#OrderStatus.Processing            // Fully user-defined
#;DT;Business.FiscalYear.Q1        // Mixed: reserved;user
```

**Metadata:**
```polyglot
{|} |ProcessOrder
[%] %Doc << "Processes order with validation"
[%] %Author
   [.] .name << "Alice <alice@example.com>"
   [.] .since << "1.0.0"
{x}
```

**Additional Syntax Features:**
- Inline pipelines: `|DT.Now""`
- Multi-line strings: `[+]` marker
- Collection literals: `{1, 2, 3}`
- Match expressions with exhaustiveness
- Range operators: `?[min, max]`
- Operator negation: `!=?`, `!>?`, etc.
- Collection membership: `in?`, `!in?`
- Early return pattern
- Struct shorthand
- Enum value fields
- Variadic input via `<<<` operator
- Pipeline composition: `|>`
- Block comments: `/* */`

**Implementation Priority:**
1. Parser updates for new markers
2. Variable prefix migration tooling
3. Indentation-based nesting
4. Loop system implementation
5. Metadata system
6. All additional features

**Migration Tools:** Provided by compiler team

**Timeline:**
- Specification: ✅ Complete (December 2025)
- Implementation: Q2 2026
- Release: Q2 2026

---

### v0.0.5 - Type System

**Status:** 💡 Concept Phase
**Date:** Concept ongoing
**Type:** Type system enhancements
**Target Implementation:** Q4 2026

**Spec Completion:** ~20%

**Planned Features:**

1. **Type Definition Blocks: `{:}`**
   - Define constrained types
   - Example: `:data.age` with min/max constraints
   - Violation handlers (clip, raise, transform, default)

2. **Cross-Language Types:**
   - `:py.str`, `:rust.i32`, `:js.number`
   - Backend-specific type mappings

3. **Type Composition:**
   - Constrained collections
   - Optional types
   - Type conversions

4. **Metadata-Driven Type Features:**
   - `%Constraint` - Validation rules
   - `%Backend` - Target language
   - `%Native` - Native type mapping

**Documentation:**
- 📂 [v0.0.5 Proposals](v0.0.5/) - Early design documents

**Current Status:**
- Early concept phase
- Design discussions ongoing
- No implementation timeline yet

**Dependencies:**
- v0.0.4 implementation complete
- Type system validation from v0.0.4 usage

---

### v0.1.0 - First Stable Release

**Status:** 🎯 Target Milestone
**Date:** Target Q1 2027
**Type:** Stabilization and release

**Goals:**
- All core syntax finalized
- Complete standard library
- Full compiler implementation
- Comprehensive test suite
- Documentation complete
- Real-world usage validation
- Performance optimization
- Backward compatibility guarantees

**Documentation:**
- To be created as milestone approaches

**Success Criteria:**
- Production-ready compiler
- Full language specification
- Complete test coverage (>90%)
- Documentation coverage (100%)
- Real-world adoption examples
- Performance benchmarks met

---

## Feature Comparison Matrix

| Feature | v0.0.2 | v0.0.3 | v0.0.4 | v0.0.5 | v0.1.0 |
|---------|--------|--------|--------|--------|--------|
| **Syntax** |
| Backslash markers (`\~\`) | ✅ | ✅ | ❌ | ❌ | ❌ |
| Indentation nesting | ❌ | ❌ | ✅ | ✅ | ✅ |
| Variable prefix `,` | ✅ | ✅ | ❌ | ❌ | ❌ |
| Variable prefix `$` | ❌ | ❌ | ✅ | ✅ | ✅ |
| Block delimiters | `[|]...[X]` | `{|}...{x}` | `{|}...{x}` | `{|}...{x}` | `{|}...{x}` |
| **Loop System** |
| Basic loops | Pattern | Pattern | Unpack/Pack | Unpack/Pack | Unpack/Pack |
| Unpack operator `[~]` | ❌ | ❌ | ✅ | ✅ | ✅ |
| Pack operator `[*]` | ❌ | ❌ | ✅ | ✅ | ✅ |
| Vacuum operator `[v]` | ❌ | ❌ | ✅ | ✅ | ✅ |
| Sequential loops | ❌ | ❌ | ✅ | ✅ | ✅ |
| Parallel loops | ❌ | ❌ | ✅ | ✅ | ✅ |
| **Enums & Errors** |
| Reserved indication `;` | ❌ | ❌ | ✅ | ✅ | ✅ |
| Enum value fields | ❌ | ❌ | ✅ | ✅ | ✅ |
| Mixed reserved/user | ❌ | ❌ | ✅ | ✅ | ✅ |
| **Types** |
| Basic types | ✅ | ✅ | ✅ | ✅ | ✅ |
| Type notation | `pg\type` | `pg\type` | `:pg.type` | `:pg.type` | `:pg.type` |
| Type definitions `{:}` | ❌ | ❌ | ❌ | ✅ | ✅ |
| Cross-language types | ❌ | ❌ | ❌ | ✅ | ✅ |
| **Metadata** |
| Metadata system `%` | ❌ | ❌ | ✅ | ✅ | ✅ |
| Documentation metadata | ❌ | ❌ | ✅ | ✅ | ✅ |
| Deprecation support | ❌ | ❌ | ✅ | ✅ | ✅ |
| **Advanced Features** |
| Inline pipelines | ❌ | ❌ | ✅ | ✅ | ✅ |
| Multi-line strings `[+]` | ❌ | ❌ | ✅ | ✅ | ✅ |
| Range operators | ❌ | ❌ | ✅ | ✅ | ✅ |
| Collection literals | ❌ | ❌ | ✅ | ✅ | ✅ |
| Match exhaustiveness | ❌ | ❌ | ✅ | ✅ | ✅ |
| Struct shorthand | ❌ | ❌ | ✅ | ✅ | ✅ |
| Pipeline composition `|>` | ❌ | ❌ | ✅ | ✅ | ✅ |
| Variadic input `<<<` | ❌ | ❌ | ✅ | ✅ | ✅ |
| Block comments `/* */` | ❌ | ❌ | ✅ | ✅ | ✅ |

**Legend:**
- ✅ Available
- ❌ Not available
- Pattern = Pattern-based loops (ForEach marker)
- Unpack/Pack = Explicit unpack/pack operators

---

## Migration Paths

### v0.0.2 → v0.0.3

**Type:** Evolution (moderate changes)

**Changes:**
- Block delimiters standardized
- Syntax refinements
- No major breaking changes

**Migration:** Manual review recommended

---

### v0.0.3 → v0.0.4

**Type:** Breaking (major changes)

**Impact:** HIGH - Requires migration

**Automated Migrations:**
1. Variable prefix: `,` → `$`
2. Nesting: `\~\` → 3-space indentation
3. Types: `pg\type` → `:pg.type`
4. Reserved enums: Add `;` where applicable

**Manual Review Required:**
1. Loop patterns → Unpack/pack system
2. I/O definitions: `[i]`/`[o]` → `[|] <param`/`[|] >param`
3. Metadata additions
4. New feature adoption

**Migration Tools:** Provided by compiler team

**Timeline:** Q2 2026

**Documentation:**
- Migration guide to be created in `v0.0.4/migrations/`

---

### v0.0.4 → v0.0.5

**Type:** Additive (new features)

**Impact:** LOW - Backward compatible additions

**Changes:**
- Type definition blocks added
- Cross-language types added
- Existing code continues to work

**Migration:** Optional adoption of new features

**Timeline:** Q4 2026

---

## Documentation Index by Version

### v0.0.2 Documentation
- [Archive: Original Iteration](../../archive/old-user-docs/user-v0.0.2-original/)
- [Archive: Revised Iteration](../../archive/old-user-docs/user-v0.0.2-revised/)
- [Archive README](../../archive/old-user-docs/README.md) - Explains differences
- [AI Context v0.0.2](../../ai-context/v0.0.2/)

### v0.0.3 Documentation
- [User Documentation](../../user/) - Complete language reference
- [Getting Started](../../user/getting-started.md)
- [Async-Centric Paradigm](../../user/async-centric-paradigm.md)
- [Variable State System](../../user/variable-state-system.md)
- [Language Reference](../../user/language/)
- [Standard Library](../../user/standard-library/)
- [Examples](../../user/examples/)

### v0.0.4 Documentation
- **[COMPLETE-SPEC.md](v0.0.4/COMPLETE-SPEC.md)** - ⭐ **Start here**
- [Loop System](v0.0.4/features/loop-system.md)
- [Reserved Indication](v0.0.4/features/reserved-indication-system.md)
- [Metadata System](v0.0.4/features/metadata-system.md)
- [Loop System Evolution](v0.0.4/loop-system/)
- [Syntax Refinement](v0.0.4/syntax-refinement/)

### v0.0.5 Documentation
- [Proposals](v0.0.5/) - Early concept documents

---

## AI Context Packages by Version

### Available

**v0.0.2** - ✅ Complete (13 files)
- Location: [ai-context/v0.0.2/](../../ai-context/v0.0.2/)
- Format: EBNF, JSON, YAML
- Density: 4.3x more compact than prose
- Files:
  - grammar.ebnf - Complete syntactic grammar
  - type-system.json - Type rules
  - operators.json - Operator reference
  - state-machine.yaml - Variable lifecycle
  - datetime-system.yaml - DateTime spec
  - And 8 more specialized files

**Status:** Historical reference (language has evolved)

### Pending

**v0.0.3** - 🚧 To be created
- Target: After Epic 1 completion
- Format: EBNF, JSON, YAML
- Scope: Current stable syntax

**v0.0.4** - 💡 Planned
- Target: After implementation begins
- Format: EBNF, JSON, YAML
- Scope: Finalized v0.0.4 syntax

---

## Timeline Overview

```
2025 (Historical)
├── v0.0.2 - Original design
│   └── 📦 Archived

2025 (Current)
├── v0.0.3 - Current stable
│   └── ✅ Epic 1 in progress

2025-12-14
└── v0.0.4 - Specification finalized
    └── ✅ Design complete

2026 Q2
└── v0.0.4 - Implementation target
    └── 🔧 Development planned

2026 Q4
└── v0.0.5 - Type system
    └── 💡 Concept phase

2027 Q1
└── v0.1.0 - First stable
    └── 🎯 Target milestone
```

---

## Quick Navigation

**I want to learn the current syntax:**
→ [User Documentation (v0.0.3)](../../user/)

**I want to see the future syntax:**
→ [v0.0.4 Complete Spec](v0.0.4/COMPLETE-SPEC.md)

**I want to understand version differences:**
→ [Feature Comparison Matrix](#feature-comparison-matrix)

**I want to plan for migration:**
→ [Migration Paths](#migration-paths)

**I want machine-readable specs:**
→ [AI Context Packages](../../ai-context/)

**I want to see version timeline:**
→ [Version Roadmap](version-roadmap.md)

**I want to understand design decisions:**
→ [Brainstorming Sessions](brainstorming/)

---

## Related Documentation

- [Master Documentation Index](../MASTER-INDEX.md) - Complete docs navigation
- [Version Roadmap](version-roadmap.md) - Detailed version planning
- [Design Specifications Catalog](../DESIGN-SPECIFICATIONS-CATALOG.md) - All specifications
- [PRD](../../project/prd.md) - Product requirements
- [Architecture](../../technical/architecture.md) - Technical design

---

**Version Index Created:** 2025-12-14
**Maintained By:** Polyglot Language Design Team
**Last Sync:** All versions current as of 2025-12-14

For version-specific questions, see the respective documentation folders or create an issue.
