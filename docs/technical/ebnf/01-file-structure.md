---
audience: design
type: spec
updated: 2026-03-30
---

<!-- @ebnf/INDEX -->

## 1. File Structure

```ebnf
file                ::= package_block { definition } ;

definition          ::= data_def
                      | pipeline_def
                      | wrapper_def
                      | queue_def
                      | error_def
                      | array_def
                      | comment_block ;
```

---
