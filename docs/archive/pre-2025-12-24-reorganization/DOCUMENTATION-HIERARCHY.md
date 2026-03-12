# Polyglot Documentation Hierarchy Guide

**Version:** 1.0  
**Last Updated:** 2025-12-23  
**Maintained by:** Scribe Documentation System

---

## 🎯 Purpose

This guide establishes the canonical documentation structure for the Polyglot programming language project, resolving ambiguities about where documentation should live and which sources are authoritative.

---

## 📋 Documentation Principles

1. **Single Source of Truth** - Each piece of information has one authoritative location
2. **Clear Hierarchy** - Readers know where to find what they need
3. **No Duplication** - Mirrors are clearly marked; edits happen in one place
4. **User-Centric** - Organization serves both learners and implementers
5. **Discoverable** - Clear navigation and comprehensive indexing

---

## 🏛️ Authoritative Structure

### Level 1: Primary Documentation (Authoritative Sources)

```
docs/
├── specifications/v0.0.4/   ⭐ AUTHORITATIVE - Language specification
│   ├── language/             Complete language reference
│   ├── stdlib/               Standard library functions
│   ├── reference/            Quick references
│   ├── getting-started/      Official tutorials
│   └── examples/             Reference implementations
│
├── technical/                ⭐ AUTHORITATIVE - Implementation docs
│   ├── architecture/         System architecture
│   └── decisions/            Technical decisions
│
├── project/                  ⭐ AUTHORITATIVE - Project management
│   ├── stories/              User stories
│   ├── examples/             Development examples
│   └── tickets/              Issue tracking
│
└── examples/                 ⭐ AUTHORITATIVE - Working examples
    └── inline-pipeline-parser/  Example collection
```

### Level 2: User-Friendly Guides (Complementary)

```
docs/
├── language/                 💡 USER GUIDES - Practical explanations
│   ├── syntax/               User-friendly syntax guides
│   ├── types/                Type system tutorials
│   ├── control-flow/         Flow control patterns
│   └── advanced/             Advanced topics with examples
│
├── getting-started/          💡 USER GUIDES - First steps
│   └── core-principles.md    Fundamental concepts
│
└── reference/                💡 QUICK REFERENCE - Lookup tables
```

### Level 3: Convenience Mirrors (Read-Only)

```
docs/
└── stdlib/                   🔗 MIRROR of specifications/v0.0.4/stdlib/
    └── README.md             (redirects to authoritative source)
```

### Level 4: Reserved for Future Use

```
docs/
├── Users/                    (Empty - future user documentation)
├── Tech/                     (Empty - alternative tech docs location)
├── Architecture/             (Empty - alternative architecture location)
└── Agile/                    (Empty - alternative agile location)
```

### Level 5: Infrastructure

```
docs/
├── INDEX.md                  📊 Master index of all documentation
├── Audit/                    📊 Quality tracking
│   ├── history/              Document change history
│   └── checks/               Quality audit reports
│
├── archive/                  🗄️ Historical documentation
├── _backups/                 🗄️ Timestamped backups
└── _patches/                 🗄️ Training session patches
```

---

## 📖 Documentation Types & Locations

### Where Does Each Type of Documentation Go?

| Documentation Type | Primary Location | Secondary Location | Notes |
|--------------------|------------------|-------------------|-------|
| **Language Syntax** | `specifications/v0.0.4/language/syntax/` | `language/syntax/` | Spec is authoritative; language/ has examples |
| **Type System** | `specifications/v0.0.4/language/types/` | `language/types/` | Spec is authoritative |
| **Standard Library** | `specifications/v0.0.4/stdlib/` | `stdlib/` (mirror) | stdlib/ is read-only mirror |
| **Control Flow** | `specifications/v0.0.4/language/control-flow/` | `language/control-flow/` | Spec is authoritative |
| **Advanced Features** | `specifications/v0.0.4/language/advanced/` | `language/advanced/` | Both have value |
| **Getting Started** | `specifications/v0.0.4/getting-started/` | `getting-started/` | Spec preferred |
| **Code Examples** | `examples/` | `specifications/v0.0.4/examples/` | examples/ for working code |
| **Architecture** | `technical/architecture/` | - | Single location |
| **Tech Decisions** | `technical/decisions/` | - | Single location |
| **User Stories** | `project/stories/` | - | Single location |
| **Project Planning** | `project/` | - | Single location |

---

## 🎯 Decision Matrix: Where to Put New Documentation

### Use This Flowchart:

```
New Documentation?
│
├─ Is it formal language specification?
│  └─ YES → specifications/v0.0.4/language/
│
├─ Is it standard library function reference?
│  └─ YES → specifications/v0.0.4/stdlib/
│
├─ Is it working code example?
│  └─ YES → examples/
│
├─ Is it technical architecture/decision?
│  └─ YES → technical/
│
├─ Is it project management (stories/tickets)?
│  └─ YES → project/
│
├─ Is it user-friendly tutorial/guide?
│  └─ YES → language/ (if about language features)
│         or getting-started/ (if introductory)
│
└─ Still not sure?
   └─ Use Scribe's add-doc workflow - it will auto-detect!
```

---

## ✏️ Editing Guidelines

### Where to Make Changes

1. **Specifications** (specifications/v0.0.4/)
   - Edit directly in this location
   - These are the authoritative sources
   - Run `doc-validate` after changes

2. **User Guides** (language/, getting-started/)
   - Edit directly for user-facing content
   - Add more examples and explanations
   - These complement, don't duplicate, the spec

3. **Mirrors** (stdlib/)
   - **DO NOT EDIT DIRECTLY**
   - Edit the source in specifications/v0.0.4/stdlib/
   - Mirror is automatically synced

4. **Empty Folders** (Users/, Tech/, Architecture/, Agile/)
   - Reserved for future use
   - Don't create content here yet
   - Awaiting structural decision

---

## 🔄 Synchronization Rules

### stdlib/ Mirror

**Rule:** `docs/stdlib/` mirrors `docs/specifications/v0.0.4/stdlib/`

**Process:**
1. All edits happen in `specifications/v0.0.4/stdlib/`
2. Mirror is updated via sync process
3. `stdlib/README.md` clearly marks this as a mirror

**Why?** Convenient top-level access while maintaining single source of truth

### language/ Complement

**Rule:** `docs/language/` complements `docs/specifications/v0.0.4/language/`

**Relationship:**
- Specification: Formal, complete, authoritative
- Language guides: Practical, examples, user-friendly
- Some overlap is acceptable for user experience

**Process:**
1. Specification changes take precedence
2. Language guides can add examples and explanations
3. Both are actively maintained

---

## 📊 Maintenance Responsibilities

| Location | Maintainer | Update Frequency | Sync Required |
|----------|-----------|------------------|---------------|
| `specifications/v0.0.4/` | Language Team | On language changes | No |
| `language/` | Docs Team | On major features | With spec |
| `stdlib/` (mirror) | Auto-sync | Auto | Yes (from spec) |
| `examples/` | Docs Team | As needed | No |
| `technical/` | Dev Team | On architectural changes | No |
| `project/` | PM Team | Ongoing | No |
| `INDEX.md` | Scribe | On any doc changes | Yes (metadata workflow) |
| `Audit/` | Scribe | Automated | Yes (audit workflow) |

---

## 🔍 Discovery & Navigation

### For New Users
1. Start: `getting-started/core-principles.md`
2. Then: `examples/inline-pipeline-parser/`
3. Then: `language/` for user-friendly explanations
4. Reference: `specifications/v0.0.4/` when needed

### For Language Implementers
1. Start: `specifications/v0.0.4/PARSER-IMPLEMENTATION-GUIDE.md`
2. Reference: `specifications/v0.0.4/language/`
3. Examples: `specifications/v0.0.4/examples/`
4. Architecture: `technical/architecture/`

### For Contributors
1. Start: `project/` for current stories
2. Reference: `technical/decisions/` for context
3. Code: `examples/` for patterns
4. Spec: `specifications/v0.0.4/` for accuracy

---

## 🚨 Common Pitfalls

### ❌ Don't Do This

1. **Don't edit stdlib/ directly** - Edit specifications/v0.0.4/stdlib/ instead
2. **Don't duplicate specifications** - Add to language/ only if adding user value
3. **Don't create content in empty folders yet** - Use established structure
4. **Don't skip metadata** - Every document needs YAML frontmatter
5. **Don't bypass INDEX.md** - Use Scribe's metadata workflow to update

### ✅ Do This Instead

1. **Use Scribe workflows** - `add-doc`, `doc-edit`, etc.
2. **Check INDEX.md first** - See if content already exists
3. **Add metadata** - YAML frontmatter with title, type, tags
4. **Run validation** - `doc-validate` after changes
5. **Update INDEX.md** - Via `metadata` workflow

---

## 📈 Evolution & Future Plans

### Planned Improvements

1. **Populate Users/** - Comprehensive user-facing documentation
2. **Expand examples/** - More working code examples
3. **Add tutorials** - Step-by-step learning path
4. **Create glossary** - Term definitions
5. **Add diagrams** - Visual explanations

### Under Consideration

- Versioned documentation (v0.0.5+)
- API documentation generation
- Interactive examples
- Video tutorials
- Community contributions

---

## 📝 Quick Reference

### Key Locations

```
⭐ EDIT HERE (Authoritative):
   specifications/v0.0.4/language/
   specifications/v0.0.4/stdlib/
   technical/
   project/
   examples/

💡 EDIT HERE (User Guides):
   language/
   getting-started/

🔗 DO NOT EDIT (Mirrors):
   stdlib/

📊 AUTO-MAINTAINED:
   INDEX.md
   Audit/
```

---

## 🆘 Need Help?

1. **Finding docs?** Check `INDEX.md`
2. **Adding docs?** Use Scribe's `add-doc` workflow
3. **Unsure where it goes?** Scribe auto-detects location
4. **Validation errors?** Run `doc-validate`
5. **Health check?** Run `doc-audit`

---

**Maintained by:** Scribe Documentation System  
**Questions?** Check the master INDEX.md or run Scribe workflows  
**Last Audit:** 2025-12-23 | Health Score: 68/100

*This hierarchy is living documentation. Suggest improvements through project issues.*
