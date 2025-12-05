# Polyglot Expansion Operator

**Version:** 0.0.2  
**Last Updated:** 2025-12-02

---

## Overview

The `[~]` marker serves **two purposes**:
1. **Nesting indicator** - Marks scope depth
2. **Expansion marker** - Explicit expansion control

---

## Automatic (Implicit Expansion

**Parent-child relationships expand automatically:**

```polyglot
[?] .condition =? #Boolean.True
[~][r] .nested:pg.int << 1       // Implicit expansion
[~][r] |Pipeline                   // Implicit expansion
[~]
```

Direct children under `[?]` are automatically in nested scope.

---

## Explicit Expansion `[~]`

Use `[~]` to **explicitly mark expansion** when needed:

```polyglot
[#] #Config
[<] .database.host:pg.string << "localhost"
[<] .database.port:pg.int << 5432
[~]// Explicit nesting
[~][<] .database.credentials.user:pg.string << "admin"
[~][<] .database.credentials.pass:pg.string << "secret"
[X]
```

**Hierarchy:**
```
Config
├─ .database.host
├─ .database.port
└─ .database.credentials.*       // Nested deeper
    ├─ .database.credentials.user
    └─ .database.credentials.pass
```

---

## Nesting Rules

### Single Level

```polyglot
[?] .outer =? #Boolean.True
[~][r] .inner:pg.int << 1        // One level deep
[~]
```

### Multiple Levels

```polyglot
[?] .level1 =? #Boolean.True
[~][?] .level2 =? #Boolean.True
[~][~][r] .level3:pg.int << 1    // Two levels deep
[~][~]
[~]
```

---

## When to Use Explicit `[~]`

Use explicit expansion for:

1. **Deep nesting** in enumerations/serials
2. **Clarity** when nesting is complex
3. **Scope control** in parallel blocks

**Generally automatic expansion is sufficient** for most cases.

---

## Scope Control

`[~]` controls which variables are in scope:

```polyglot
[p] ~ForEach
[<] .items
[>] .item
[~][r] .processed:pg.string << .item  // In parallel scope
[~]
// .processed NOT accessible here (out of scope
```

---

## Complete Example

```polyglot
[#] #AppConfig
[<] .server.host:pg.string << "0.0.0.0"
[<] .server.port:pg.int << 8080
[~]
[~][<] .server.ssl.enabled:pg.bool << #Boolean.True
[~][<] .server.ssl.cert_path:pg.path << "/etc/certs/cert.pem"
[~]
[<] .database.url:pg.string << "postgresql://..."
[X]
```

**Structure:**
```
AppConfig
├─ .server.host
├─ .server.port
├─ .server.ssl.*              // Nested
│   ├─ .server.ssl.enabled
│   └─ .server.ssl.cert_path
└─ .database.url
```

---

**Next:** [Line Continuation →](line-continuation.md
