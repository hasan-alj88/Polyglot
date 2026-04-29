---
name: pg:help
description: Show Aljam3 agent command reference
---

<objective>
Display the complete Aljam3 agent command reference.

Output ONLY the reference content below. Do NOT add:

- Project-specific analysis
- Git status or file context
- Next-step suggestions
- Any commentary beyond the reference
</objective>

<reference>
# Aljam3 Agent Commands

The `/pg:*` commands help you generate, validate, and iteratively improve Aljam3 Code.

## Commands

| Command | Purpose |
|---------|---------|
| `/pg:train [prompt]` | Training loop: generate → correct → learn |
| `/pg:generate <desc>` | Generate Aljam3 code from description |
| `/pg:docs [section]` | Generate/update documentation PDF |
| `/pg:help` | This reference |

---

## `/pg:train` — Training Loop

The primary command. Runs an iterative cycle:

```
┌──────────────────────────────────────────────┐
│  1. GENERATE  →  Agent produces code         │
│  2. CORRECT   →  You fix it in draft.md      │
│  3. INFER     →  Agent identifies mistakes   │
│  4. UPDATE    →  Agent fixes spec docs       │
│  5. MEMORIZE  →  Agent saves lesson          │
│  6. LOOP      →  Another round or done       │
└──────────────────────────────────────────────┘
```

**Usage:**
- `/pg:train` — agent asks what to generate
- `/pg:train "a pipeline that processes CSV rows in parallel"` — generate specific code

**Correction workflow:**
1. Agent writes generated code to `docs/draft.md`
2. You edit corrections in your editor
3. Tell Claude "done" to continue
4. Say "correct" if the code needs no changes

---

## `/pg:generate` — Standalone Generation

Generate Aljam3 Code without the correction loop. Reads full spec + all accumulated training lessons.

**Usage:**
- `/pg:generate "hello world pipeline"`
- `/pg:generate "data pipeline with error handling and parallel processing"`

---

## How the Agent Learns

Three layers, from most to least authoritative:

| Layer | Location | Updated by |
|-------|----------|------------|
| **Spec files** | `docs/user/` | `/pg:train` (with your approval) |
| **Memory lessons** | `memory/pg_lesson_*.md` | `/pg:train` (automatic) |
| **CARL rules** | `~/.carl/aljam3` | Manual or `/pg:train` suggestion |

**Spec files** are the canonical source of truth. When training reveals a gap or ambiguity, the agent proposes edits for your approval.

**Memory lessons** persist across sessions. Each lesson records what went wrong, why, and how to avoid it. The agent loads all lessons before generating code.

**CARL rules** activate automatically when Aljam3 topics come up, even outside `/pg:*` commands. The agent suggests new rules when it sees recurring mistake patterns (3+).

---

## Adding New Spec Files

When you create a new spec file (e.g., `docs/user/syntax/error-handling.md`):

1. Add it to `docs/user/SPEC-INDEX.md`
2. That's it — `/pg:*` commands pick it up automatically

---

## `/pg:docs` — Documentation PDF

Generate the combined Aljam3 documentation book (343 docs → single PDF).

**Usage:**
- `/pg:docs` — full book from all docs/
- `/pg:docs user` — only user-facing documentation
- `/pg:docs technical` — only technical documentation

**Output:** `Aljam3-Documentation.pdf` in repo root.

**Auto-generation:** A PostToolUse hook automatically regenerates the PDF after any `git commit` that touches `docs/`.

---

*Aljam3 Agent v0.2 | 4 commands*
</reference>
