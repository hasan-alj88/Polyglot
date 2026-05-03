# Aljam3 Syntax Highlighting Reference

This document maps all the Aljam3 syntactical constructs to their corresponding TextMate grammar scopes. This acts as a reference for theme developers or anyone looking to customize the visual representation of Aljam3 code in VS Code or other TextMate-compatible editors.

| Construct | Example | TextMate Scope | Prefix | Literal | Regex Matches | Description |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Pipelines** | `-ProcessData` | `entity.name.function.pipeline` | `-` | `[-]`, `{-}`, `(-)` | `\\[-\\]`&#124;`\\{-\\}`&#124;`\\(-\\)`, `\\-[A-Za-z0-9_.]+` | |
| **Parallel** | `=ForEach.Array` | `entity.name.function.parallel` | `=ForEach.` | `[=]`, `(=)` | `\\[=\\]`&#124;`\\{=\\}`&#124;`\\(=\\)`, `\\=[A-Za-z0-9_.]+` | |
| **Triggers** | `T.Manual` | `entity.name.type.trigger` | `-T.` | `[T]` | `\\[T\\]`&#124;`\\{T\\}`&#124;`\\(T\\)`, `\\-T\\.[A-Za-z0-9_.]+` | |
| **Queues** | `Q.Default` | `entity.name.type.queue` | `-Q.` | `[Q]` | `\\[Q\\]`&#124;`\\{Q\\}`&#124;`\\(Q\\)`, `\\-Q\\.[A-Za-z0-9_.]+` | |
| **Wrapper** | `W.Aljam3` | `entity.name.type.wrapper` | `-W.` | `[W]` | `\\[W\\]`&#124;`\\{W\\}`&#124;`\\(W\\)`, `\\-W\\.[A-Za-z0-9_.]+` | |
| **Environment** | `;PythonEnv` | `entity.name.environment` | `;` | `{;}` | `\\{;\\}`&#124;`\\[;\\]`&#124;`\\(;\\)`, `;[A-Za-z0-9_.]+` | |
| **Variables** | `$sanitized`, `$*` | `variable.name` | `$` | `{$}`, `[$]`, `($)` | `\\[\\$\\]`&#124;`\\{\\$\\}`&#124;`\\(\\$\\)`, `\\$[A-Za-z0-9_.]+`&#124;`\\$\\*` | |
| **Packages** | `@examples` | `entity.name.namespace` | `@` | `[@]`, `{@}` | `\\[@\\]`&#124;`\\{@\\}`&#124;`\\(@\\)`, `@[A-Za-z0-9_:.]+` | Ends with the beginning of another prefix |
| **Input** | `<input` | `entity.name.tag.io` | `<` | `(<)` | `\\(>\\)`&#124;`\\(<\\)`, `[<>][A-Za-z0-9_.]+` | |
| **Output** | `>output` | `entity.name.tag.io` | `>` | `(>)` | `\\(>\\)`&#124;`\\(<\\)`, `[<>][A-Za-z0-9_.]+` | |
| **Datatypes** | `#string` | `entity.name.type` | `#`, `##`, `###` | `[#]`, `{#}`, `(#)` | `\\[##\\]`&#124;`\\{##\\}`&#124;`\\(##\\)`&#124;`\\[#\\]`&#124;`\\{#\\}`&#124;`\\(#\\)`, `###[A-Za-z0-9_]+`&#124;`##[A-Za-z0-9_]+`&#124;`#[A-Za-z0-9_:.]+` | |
| **Schema** | `##Array` | `entity.name.type` | `##` | N/A | `\\[##\\]`&#124;`\\{##\\}`&#124;`\\(##\\)`&#124;`\\[#\\]`&#124;`\\{#\\}`&#124;`\\(#\\)`, `###[A-Za-z0-9_]+`&#124;`##[A-Za-z0-9_]+`&#124;`#[A-Za-z0-9_:.]+` | |
| **Properties** | `.message`, `.code` | `variable.name.property` | `.` | `[.]`, `(.)`, `[:]` , `(:)` | `\\[\\.\\]`&#124;`\\(\\.\\)`&#124;`\\[:\\]`&#124;`\\(:\\)`, `\\.[A-Za-z0-9_]+` | |
| **Constants** | `%#Active`, `%` | `constant.language` | `%` | N/A | `%[A-Za-z0-9_.]+` | |
| **Exceptions** | `!ValidationError` | `invalid.illegal` | `!` | `[!]`, `{!}`, `<!`, `>!` | `\\[!\\]`&#124;`\\{!\\}`&#124;`<!`&#124;`>!`, `![A-Za-z0-9_:.]+` | |
| **Logical Nodes** | `?=` | `keyword.operator.logical` | `?` | `[?]`,`[+]`, `[&]`, `[^]` | `\\[\\?\\]`&#124;`\\[\\+\\]`&#124;`\\[&\\]`&#124;`\\[\\^\\]`, `\\\?(?:=\|!=\|>\|<\|>=\|<=\|!>\|!<=\|!>=\|\\*\|\\[\|\\(\\|\\]\|\\)\|in\\b\|has\\b\|#\|##\|_\|@\|!\|-)` | query operators (`?=`,`?>`, etc) |
| **Collectors** | `*Into.Array` | `entity.name.collector` | `*` | `[*]`,`{*}`, `(*)` | `\\[\\*\\]`&#124;`\\{\\*\\}`&#124;`\\(\\*\\)`, `\\*[A-Za-z0-9_.]+` | |
| **Comments** | `{ }` | `comment.line.bracket` | N/A | `{ }`, `( )`, `[ ]` | `(\\{\\s*\\}`&#124;`\\[\\s*\\]`&#124;`\\(\\s*\\)).*$`, `//.*$` | from literal to the END of the line |
| **Assignment Operators** | `<<`, `<~`, `<-`, `>>` | `keyword.operator.assignment` | N/A | `<<`, `<~`, `<-`, `>>` | `<<`&#124;`<~`&#124;`<-`&#124;`>>` | Push, Default, Set, and Pull operations. |
| **String Literals** | `"Hello"` | `string.quoted.double` | `"` | `"` | `begin: "\"", end: "\""` | Standard text. |
| **Floats** | `3.14` | `constant.numeric.float` | N/A | N/A | `\\b\\d+\\.\\d+\\b` | Floating point numbers. |
| **Integers** | `42` | `constant.numeric.integer` | N/A | N/A | `\\b\\d+\\b` | Integer numbers. |
| **Permissions** | `_FileAccess` | `constant.other.permission` | `_`, `__`, `___` | `{_}`, `[_]`, `(_)` | `\\[_\\]`&#124;`\\{_\\}`&#124;`\\(_\\)`, `_[A-Za-z0-9_.]+` | |
| **Error** | `!ValidationError` | `invalid.illegal` | `!` | `[!]`, `{!}`, `<!`, `>!` | `\\[!\\]`&#124;`\\{!\\}`&#124;`<!`&#124;`>!`, `![A-Za-z0-9_:.]+` | |