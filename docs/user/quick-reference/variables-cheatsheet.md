# Variables Quick Reference

**One-page cheatsheet for Polyglot variables**

---

## Three Assignment Operators

```polyglot
# 1. Schema-Only (no default)
[<] .field: Type

# 2. Default (can override once)
[<] .field: Type <~ default_value
[>] .field: Type ~> .var

# 3. Constant/Async
[<] .field: Type << constant_value
[>] .field: Type >> .var
```

---

## When Are Variables Ready?

**At `[i]` blocks** - Polyglot waits automatically

```polyglot
[i] .user_name:pg.string   # Ready here
[i] .user_age:pg.int       # Ready here

[r] |Process
[<] .name << .user_name     # Can use immediately
```

---

## Error Handling

### Check State
```polyglot
[?] .var.state =? #Variables.States.Ready
[~][r] |ProcessSuccess

[?] .var.state =? #Variables.States.Faulted
[~][r] |HandleError
[~][<] .errors << .var.errors
```

### Use Error Blocks
```polyglot
[r] |RiskyOperation
[>] .result >> .data
[~][!] !pg.Network.Timeout
[~][~][r] |HandleTimeout
```

---

## Common Patterns

### Config with Defaults
```polyglot
[#] Config
[<] .timeout:pg.int <~ 30
[<] .retries:pg.int <~ 3
[X]
```

### Fallback on Error
```polyglot
[r] |FetchLive
[>] .data >> .live_data
[~][!] !pg.Network.*
[~][~][r] |GetCached
[~][~][>] .data >> .cached_data
[~][~][o] .cached_data
```

### Parallel Operations
```polyglot
[p] |Task1
[>] .result >> .data1

[p] |Task2
[>] .result >> .data2

[Y] |Y.Join
[<] .data1
[<] .data2
```

---

## Reserved Fields

- `.var.state` - Current state
- `.var.errors` - Error details

---

## States (For Reference)

**Core:** Declared, DefaultReady, Pending, Ready, Faulted
**Queue:** Retrying, Paused, Cached, Dirty

*Most of the time you don't need to know these - runtime handles it*

---

## Quick Tips

✅ **DO:**
- Use `<~` for config defaults
- Use `<<` for constants
- Check `.errors` on failure
- Let runtime wait automatically

❌ **DON'T:**
- Try to reassign Ready variables
- Override DefaultReady twice
- Write `await` (runtime handles it)
- Use Declared fields at `[i]`

---

**Full Docs:**
- [User Guide](../language/variables-user-guide.md)
- [Technical Spec](../../technical/variable-states-specification.md)
