# Standard Library Documentation

## ⚠️ Important: This is a Mirror

**Authoritative Source:** `docs/User/specifications/v0.0.4/stdlib/`

This folder mirrors the standard library documentation from the v0.0.4 specification for convenience. However, **always use the specification version as the canonical reference**.

---

## 📚 Complete Standard Library Reference

👉 **[Go to Official Standard Library Documentation](../User/specifications/v0.0.4/stdlib/)**

---

## Quick Links

### Loop Operations
- **[Unpack Operations](../User/specifications/v0.0.4/stdlib/loops/unpack/)** - foreach, iter, zip
- **[Pack Operations](../User/specifications/v0.0.4/stdlib/loops/pack/)** - collect, math, into

### Utilities
- **[String Functions](../User/specifications/v0.0.4/stdlib/utilities/string/)** - length, upper, lower, trim
- **[DateTime Functions](../User/specifications/v0.0.4/stdlib/utilities/datetime/)** - now, format, calendar operations
- **[Data Parsing](../User/specifications/v0.0.4/stdlib/utilities/data/)** - json-parse, xml-parse
- **[Math Functions](../User/specifications/v0.0.4/stdlib/utilities/math/)** - modulo

### Wrappers
- **[Runtime Wrappers](../User/specifications/v0.0.4/stdlib/wrappers/)** - Platform integrations

---

## Why This Structure?

- **Canonical Source**: `specifications/v0.0.4/stdlib/` is the single source of truth
- **Mirror**: This folder provides convenient top-level access
- **Sync**: Changes should always be made in the specification folder first

---

## Documentation Organization

```
docs/
├── specifications/v0.0.4/
│   └── stdlib/              ⭐ EDIT HERE - Authoritative source
│       ├── loops/
│       ├── utilities/
│       └── wrappers/
│
└── stdlib/                  🔗 MIRROR - Do not edit directly
    ├── loops/               (mirrors spec/stdlib/loops/)
    ├── utilities/           (mirrors spec/stdlib/utilities/)
    └── wrappers/            (mirrors spec/stdlib/wrappers/)
```

---

**Maintained by:** Scribe Documentation System  
**Last Updated:** 2025-12-23

For the complete, authoritative standard library documentation:
👉 **[specifications/v0.0.4/stdlib/](../User/specifications/v0.0.4/stdlib/)**
