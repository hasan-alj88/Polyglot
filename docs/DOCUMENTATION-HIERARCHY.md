# Polyglot Documentation Hierarchy

**Version:** 2.0
**Last Updated:** 2026-03-12
**Replaces:** Scribe Documentation Hierarchy v1.0

---

## Structure

```
docs/
├── User/                    # For Polyglot language users
│   ├── getting-started/     # Tutorials, first steps
│   ├── language/            # Language syntax and semantics
│   │   ├── syntax/          # Operators, markers, prefixes
│   │   ├── types/           # Type system, enums, variables
│   │   ├── control-flow/    # Pipelines, loops, conditionals
│   │   └── advanced/        # Advanced features
│   ├── examples/            # Working code examples
│   ├── reference/           # Quick reference, grammar
│   ├── specifications/      # Formal language specifications
│   │   └── v0.0.4/          # Current version spec
│   └── stdlib/              # Standard library reference
│
├── Tech/                    # For language developers
│   ├── Architecture/        # System-level design
│   ├── Polyglot/            # Language internals
│   │   ├── Syntax/          # Syntax design decisions
│   │   ├── Parser/          # Parser implementation
│   │   └── Compiler/        # Compiler design
│   ├── QueueManager/        # Job queue infrastructure
│   ├── Runner/              # Execution runtime
│   ├── ai-context/          # AI development context
│   ├── automation/          # Build and automation
│   ├── development/         # Contributing guides
│   └── implementation/      # Implementation details
│
├── Audit/                   # Documentation health
│   ├── documentation/       # Audit findings, violations, fixes
│   ├── decisions/           # Design decisions requiring doc sync
│   ├── history/             # Change tracking
│   └── checks/              # Validation reports
│
├── archive/                 # Historical content (read-only)
├── _templates/              # Document templates (7 templates)
├── _index.yaml              # Root index for AI navigation
├── _conventions.md          # Documentation ground rules
└── INDEX.md                 # Human-readable master index
```

---

## Audience Routing

| I am a... | Start here | Then explore |
|-----------|-----------|-------------|
| **New Polyglot user** | `User/getting-started/` | `User/language/`, `User/examples/` |
| **Experienced user** | `User/reference/` | `User/specifications/v0.0.4/`, `User/stdlib/` |
| **Language developer** | `Tech/Architecture/` | `Tech/Polyglot/`, `Tech/implementation/` |
| **Parser/compiler dev** | `Tech/Polyglot/Parser/` | `Tech/Polyglot/Compiler/`, `User/specifications/` |
| **Documentation maintainer** | `Audit/documentation/` | `_conventions.md`, `_templates/` |
| **AI tool** | `_index.yaml` | Follow subdirectory pointers in indexes |

---

## Authoritative Sources

| Topic | Authoritative Location | Notes |
|-------|----------------------|-------|
| Language syntax (formal) | `User/specifications/v0.0.4/` | The spec is the source of truth |
| Language syntax (guides) | `User/language/` | Complements the spec with examples |
| Standard library | `User/stdlib/` | Function reference |
| System architecture | `Tech/Architecture/` | Component design |
| Parser internals | `Tech/Polyglot/Parser/` | Implementation details |
| Compiler design | `Tech/Polyglot/Compiler/` | Compilation pipeline |
| Queue infrastructure | `Tech/QueueManager/` | Job scheduling |
| Runtime execution | `Tech/Runner/` | Execution engine |
| Documentation health | `Audit/documentation/` | Findings and fixes |
| Design decisions | `Audit/decisions/` | Records with rationale |

---

## Conventions

All documentation follows the ground rules defined in [_conventions.md](_conventions.md):

- **Audiences:** user, developer, AI (via `_index.yaml`)
- **File size limit:** < 50 KB (split by concept if larger)
- **Frontmatter:** 5 required fields on every `.md` file
- **Cross-referencing:** Section-level anchors, bidirectional linking
- **Indexes:** `_index.yaml` at every directory level
- **Templates:** 7 templates in `_templates/` for consistent structure
- **Syntax notation:** EBNF + railroad diagrams for language specs

---

## Navigation

### For AI Tools

1. Read `docs/_index.yaml` (root index)
2. Follow `subdirectories` pointers to relevant area
3. Scan `files[].keywords` and `files[].summary` for matches
4. Load files or sections using `line_range` for efficient context

### For Humans

1. Browse `INDEX.md` for a complete listing
2. Use directory `README.md` files for section overviews
3. Follow cross-references in "See Also" sections

---

*Created: 2026-03-12*
*Maintained by: Project documentation team*
