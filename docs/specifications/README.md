# Polyglot Design Specifications

**Purpose:** Version specifications, design decisions, and language evolution documentation

**Audience:** Language designers, architects, contributors

---

## 📁 Folder Structure

```
specifications/
├── version-roadmap.md           # Version planning and timeline (v0.0.3 → v0.1.0)
├── v0.0.4/                      # v0.0.4 specifications (Major Syntax Refinement)
│   ├── loop-system/             # Loop unpack/pack system with mini-pipelines
│   └── syntax-refinement/       # 30 syntax features and improvements
├── v0.0.5/                      # v0.0.5 specifications (Type System)
└── brainstorming/               # Design exploration sessions
```

---

## 📋 Version Overview

### [Version Roadmap](version-roadmap.md)
Complete version planning from v0.0.3 (current stable) through v0.1.0 (first stable release), including feature matrices, migration guides, and implementation timeline.

### [v0.0.4 - Major Syntax Refinement](v0.0.4/) 🔧 DESIGN PHASE
**Status:** Design phase - Breaking changes + major new features
**Target:** Q2 2026

**Major Features:**
1. **Loop System** - Mini-pipeline iterations with unpack/pack operators
2. **Syntax Refinement** - Variable prefix `$`, indentation-based nesting, boolean markers, 33 total features

**Breaking Changes:**
- Variable prefix: `,` → `$`
- Nesting: `\~\` markers → 3-space indentation
- Reserved indication: Add `;` prefix to enum/error segments

### [v0.0.5 - Type System](v0.0.5/) 💡 CONCEPT PHASE
**Status:** Concept phase - Future improvements
**Target:** Q4 2026

**Planned Features:**
- Type definition blocks with `{:}` syntax
- Constrained types with validation
- Cross-language type mappings
- Type composition and conversions

### [Brainstorming Sessions](brainstorming/)
Design exploration and decision-making sessions capturing the evolution of language features.

---

## 🚀 Quick Navigation

**Understanding current design decisions?**
→ See [v0.0.4 specifications](v0.0.4/)

**Looking for loop system details?**
→ See [v0.0.4 Loop System](v0.0.4/loop-system/)

**Checking syntax refinement features?**
→ See [v0.0.4 Syntax Refinement](v0.0.4/syntax-refinement/)

**Following design evolution?**
→ See [Brainstorming Sessions](brainstorming/)

**Understanding version timeline?**
→ See [Version Roadmap](version-roadmap.md)

---

## 📊 Implementation Status

| Version | Status | Spec Complete | Target | Notes |
|---------|--------|---------------|--------|-------|
| v0.0.3 | ✅ Current | Yes | Stable | Current production version |
| v0.0.4 | 🔧 Design | 95% | Q2 2026 | Loop system + syntax refinement |
| v0.0.5 | 💡 Concept | 20% | Q4 2026 | Type system |
| v0.1.0 | 🎯 Target | - | Q1 2027 | First stable release |

---

## 🔗 Related Documentation

**Current Language Syntax:** [/docs/user/](../user/) - Complete language reference
**AI Context Packages:** [/docs/ai-context/](../ai-context/) - Machine-readable specs
**Technical Architecture:** [/docs/technical/](../technical/) - Implementation details
**Project Planning:** [/docs/project/](../project/) - PRD, epics, stories

---

## 🔧 Contributing to Specifications

### Creating New Specifications

When adding new design documents:

1. **Determine version category:**
   - v0.0.4.x → Add to [v0.0.4](v0.0.4/) appropriate subfolder
   - v0.0.5+ → Add to [v0.0.5](v0.0.5/) or create new version folder
   - Brainstorming → Add to [brainstorming](brainstorming/)

2. **Follow naming conventions:**
   - Version-specific: `v0.0.X-feature-name.md`
   - Feature-specific: `feature-name-specification.md`
   - Analysis: `feature-name-analysis.md`
   - Brainstorming: `brainstorming-session-YYYY-MM-DD.md`

3. **Include frontmatter:**
   ```markdown
   ---
   version: v0.0.X
   status: Design|Concept|Complete
   date: YYYY-MM-DD
   ---
   ```

4. **Update navigation:**
   - Update folder README.md
   - Update main specifications catalog
   - Cross-reference related documents

### Document Status Markers

- ✅ **Complete & Approved** - Finalized, ready for implementation
- 🔧 **Design Phase** - Active design work, subject to change
- 💡 **Concept Phase** - Early exploration, may change significantly
- ⏸️ **On Hold** - Paused pending other decisions
- ❌ **Rejected** - Considered and rejected with rationale

---

## 📝 Key Design Principles

From the brainstorming sessions and specifications:

1. **Async-First Architecture** - All variables have states, not just values
2. **Pipeline Orchestration** - Coordinate external runtimes, not inline code mixing
3. **Explicit Over Implicit** - Clear syntax even if slightly more verbose
4. **Functional Patterns** - Immutability, scope isolation, pure transformations
5. **Universal Interchange** - `pg\serial` as lingua franca across languages
6. **Mini-Pipeline Iterations** - Each loop iteration is an independent pipeline
7. **No Keywords** - All identifiers require operator prefixes (`.` `#` `|` `!`)

---

**Last Updated:** 2025-12-12
**Maintained By:** Polyglot Language Design Team
