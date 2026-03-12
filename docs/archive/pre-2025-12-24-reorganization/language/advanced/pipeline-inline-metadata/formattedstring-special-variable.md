# %Formatted_string Special Variable

The `%Formatted_string` is a **built-in special variable** available only within `%Pipeline.Inline` metadata blocks.

**Properties:**
- **Type:** `pg.string`
- **Scope:** Only accessible in %Pipeline.Inline metadata
- **Purpose:** Captures the string argument passed to the inline pipeline call
- **Binding:** `<formatted_string:pg.string << %Formatted_string`

**Example:**
```polyglot
[%] %Pipeline.Inline
   [%] |ParseEmail
   (|) <formatted_string:pg.string << %Formatted_string  // ← Captures "user@domain.com"
   (|) >user :pg.string >> <user
   (|) >domain :pg.string >> <domain
```

---
