---
archived: true
archived_date: 2025-12-17
archived_reason: "Superseded by v0.0.4 sync"
original_path: user/quick-reference/README.md
---

<!-- ARCHIVED: 2025-12-17 | Reason: Superseded by v0.0.4 sync -->

# Polyglot Quick Reference

**Cheat sheets and quick lookup guides**

---

## Overview

This directory contains quick reference materials, cheat sheets, and condensed guides for rapid lookup. Perfect for when you need a quick reminder without reading full documentation.

---

## Quick Reference Materials

### 📋 [Variables Cheatsheet](./variables-cheatsheet.md)
Quick reference for variable syntax and usage.

**Covers:**
- Variable declaration: `[r] $name :type << value`
- Scope rules
- Variable operators (`<<`, `>>`, `<~`)
- Common patterns

**Quick Examples:**
```polyglot
[r] $name :pg.string << "Alice"
[r] $age :pg.int << 30
[r] $active :pg.bool << true
[r] $data :pg.serial << #Serial
   name << $name
   age << $age
```

---

## Coming Soon

### Syntax Cheatsheets
- **Markers Cheatsheet** - All markers on one page
- **Operators Cheatsheet** - Operator quick reference
- **Types Cheatsheet** - Type system summary
- **Pipeline Cheatsheet** - Pipeline patterns

### Command Reference
- **CLI Cheatsheet** - Common commands
- **Keyboard Shortcuts** - IDE shortcuts
- **REPL Commands** - Interactive shell

### Common Patterns
- **Error Handling Patterns** - Quick error patterns
- **Loop Patterns** - Iteration shortcuts
- **Conditional Patterns** - Common conditionals

---

## Cheatsheet Format

All cheatsheets follow this format:

### Topic Section

**Syntax:**
```polyglot
// Syntax pattern
```

**Example:**
```polyglot
// Working example
```

**Notes:**
- Key points
- Common gotchas
- Best practices

---

## How to Use Quick References

**For Learning:**
- Start with full docs in [Language](../language/)
- Use cheatsheets as reinforcement
- Practice with [Examples](../examples/)

**For Reference:**
- Keep cheatsheets handy while coding
- Quick syntax lookup
- Pattern reminders

**For Teaching:**
- Printable format
- One-page summaries
- Workshop handouts

---

## Printable Versions

Many cheatsheets are optimized for printing:

- **Single page** - One sheet reference
- **Readable fonts** - Clear even when printed
- **Organized sections** - Easy scanning

To print:
1. Open cheatsheet in browser
2. Print to PDF or paper
3. Use landscape orientation for tables

---

## Related Documentation

**Full Documentation:**
- [Language Documentation](../language/) - Complete guides
- [Syntax Reference](../syntax/) - Detailed syntax docs

**Learning:**
- [Examples](../examples/) - Code examples
- [Guides](../guides/) - Step-by-step tutorials

**Quick Help:**
- [CLI Help](../cli/) - Command-line reference
- [Standard Library](../standard-library/) - Library quick reference

---

## Contributing Cheatsheets

When creating new cheatsheets:

1. **Focus on essentials** - Only the most important info
2. **Use tables** - Easy to scan
3. **Include examples** - Show, don't just tell
4. **Keep it short** - Aim for 1-2 pages
5. **Test with users** - Is it actually useful?

---

**Last Updated:** 2025-12-15
**Version:** Current
**Maintained by:** Polyglot Documentation Team
