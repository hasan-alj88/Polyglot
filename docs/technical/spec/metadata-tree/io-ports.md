---
audience: design
type: spec
updated: 2026-04-03
---

# IO Port Nesting

<!-- @source:metadata-tree/INDEX -->

`.<` (inputs) and `.>` (outputs) are fixed typed data sections within each pipeline, expander, and collector instance:

```polyglot
%-:ProcessData:0
├── .<                      <- input ports
│   ├── .filepath#path
│   └── .options#serial
└── .>                      <- output ports
    └── .content#string
```

Parameter names within `.<` and `.>` are flexible — they follow the pipeline's `(-)` IO declarations.

Wrappers use `.<` (inputs) and `.>` (outputs) — same as pipelines:

```polyglot
%W:DB.Connection:0
├── .<                       <- wrapper inputs
│   └── .connectionString#string
└── .>                       <- wrapper outputs
    └── .dbConn
```

Parameter names within `.<` and `.>` are flexible — they follow the wrapper's `(-)` IO declarations.

See also: [[branches|Branch Specifications]], [[path-grammar|Path Grammar]]
