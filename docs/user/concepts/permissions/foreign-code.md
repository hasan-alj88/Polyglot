---
audience: automation-builder
type: specification
updated: 2026-04-05
status: complete
---

# Foreign Code Permissions

<!-- @u:blocks#Foreign Code -->
Pipelines using `[C]` foreign code blocks ([[blocks#Foreign Code]]) interact with permissions as follows:

- The pipeline must reference `[_]` permission objects for the IO the foreign code will perform
- The **compiler issues a warning** (not an error) that foreign code cannot be statically verified against declared permissions
- The **programmer takes responsibility** for ensuring the foreign code stays within declared permissions
- The **foreign runtime** (Python, Node, etc.) handles its own enforcement mechanisms if any

```polyglot
{_} _AnalyzeGrant
   [.] .intent << #Grant
   [.] .File.Read "data/report.csv"

{-} -AnalyzeData
   [_] _AnalyzeGrant
   [ ] compiler warning: [C] block cannot be statically verified
   [T] -T.Manual
   [Q] -Q.Default
   [W] -W.Polyglot
   [-] -RT.Python.Script
      (-) <env << $env
      (-) <script <<
         [C] import pandas as pd
         [C] df = pd.read_csv("data/report.csv")
         [C] result = df.describe()
      (-) >stdout >> $output
```
