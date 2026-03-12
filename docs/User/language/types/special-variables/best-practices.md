# Best Practices

## 1. Always Bind to Typed Parameter

```polyglot
✅ Good:
<formatted_string:pg.string << %Formatted_string

❌ Bad:
$str << %Formatted_string  // Direct to variable may not work
```

## 2. Check Scope Availability

Before using a special variable, verify it's available in your context:

```polyglot
✅ %Formatted_string in %Pipeline.Inline:
[%] %Pipeline.Inline
   (|) <str << %Formatted_string

❌ %Formatted_string elsewhere:
[<] <input << %Formatted_string  // Error
```

## 3. Document Special Variable Usage

When using special variables, add comments:

```polyglot
[%] %Pipeline.Inline
   [%] |Parser
   (|) <formatted_string:pg.string << %Formatted_string  // ← Input from inline string
   (|) >output >> <input
```

---
