# Parser Pipeline Pattern

The parser pipeline is a **separate, complete pipeline** that extracts structured data from the formatted string.

## Parser Requirements

```polyglot
{|} |ParserPipeline
[t] |T.Call  // Must be callable

[<] <formatted_string:pg.string  // Input from %Formatted_string
[>] >field1:type                  // Outputs wire to main inputs
[>] >field2:type

// Parsing logic (often uses runtime wrappers)
{x}
```

## Parser Output → Main Input Wiring

```polyglot
[%] %Pipeline.Inline
   [%] |ParserPipeline
   (|) <formatted_string:pg.string << %Formatted_string
   (|) >parsed_field :type >> <main_input  // ← Direct wiring
```

**Key Pattern:** Parser outputs bind directly to main pipeline inputs using `>output >> <input` syntax.

---
