---
audience: automation-builder
type: specification
updated: 2026-04-10
status: draft
---

# IO Inputs as Variables

IO inputs declared with `(-)` become `$`-prefixed variables in the execution body once filled. There is no need to redeclare them:

```polyglot
(-) <incoming#Alert
[ ] ...execution...
[ ] Use directly as $incoming — it's already Final
[?] $incoming.level >? 5
```

IO inputs with no assignment must be filled externally and are in Final state when the pipeline fires. See [[concepts/pipelines/io-triggers#IO as Implicit Triggers]], [[variable-lifecycle]].
