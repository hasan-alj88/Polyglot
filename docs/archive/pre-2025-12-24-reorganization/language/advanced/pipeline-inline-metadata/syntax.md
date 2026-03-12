# Syntax

## Basic Structure

```polyglot
{|} |MainPipeline
[%] %Pipeline.Inline
   [%] |ParserPipeline
   (|) <formatted_string:pg.string << %Formatted_string
   (|) >parsed_output:type >> <input_param

[<] <input_param:type
[>] >output:type

// Main pipeline logic using parsed inputs
{x}
```

## Components

1. **`[%] %Pipeline.Inline`** - Metadata block marker
2. **`[%] |ParserPipeline`** - Invokes the parser pipeline
3. **`%Formatted_string`** - Special variable containing the inline string
4. **`>output >> <input`** - Wires parser outputs to main inputs

---
