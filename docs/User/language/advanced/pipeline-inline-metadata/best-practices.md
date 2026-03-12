# Best Practices

## 1. Parser Pipeline Separation

Always define the parser as a **separate pipeline**, not inline:

```polyglot
✅ Good - Separate parser:
{|} |Parser
[t] |T.Call
// ...
{x}

{|} |Main
[%] %Pipeline.Inline
   [%] |Parser
   // ...
{x}

❌ Bad - Trying to inline parser logic:
{|} |Main
[%] %Pipeline.Inline
   // Cannot define pipeline logic here
{x}
```

## 2. Use Runtime Wrappers for Parsing

For complex parsing, use Python/JavaScript runtime wrappers:

```polyglot
✅ Recommended:
[w] |W.Runtime.Python3.11
[r] |RT.Python.Run.Code
(|) <code << |U.String.Python""
[+] +"def parse(s): ..."

❌ Avoid: Manual string manipulation in Polyglot
```

## 3. Always Bind %Formatted_string

The parser must receive the formatted string:

```polyglot
✅ Required:
[%] %Pipeline.Inline
   [%] |Parser
   (|) <formatted_string:pg.string << %Formatted_string

❌ Missing binding - parser won't receive input:
[%] %Pipeline.Inline
   [%] |Parser
```

## 4. Wire All Parser Outputs

Ensure all parser outputs wire to main inputs:

```polyglot
✅ Complete wiring:
(|) >field1 :type >> <field1
(|) >field2 :type >> <field2

❌ Missing wiring - data lost:
(|) >field1 :type  // Not wired to main input
```

---
