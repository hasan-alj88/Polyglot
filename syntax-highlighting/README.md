# Polyglot Syntax Highlighting

This directory contains syntax highlighting definitions for the Polyglot automation language.

## Files

- **polyglot.tmLanguage.json**: TextMate grammar definition for Polyglot syntax
- **vscode-extension/**: VSCode extension for Polyglot syntax highlighting (optional)

## Using the TextMate Grammar

### For VSCode

1. Copy `polyglot.tmLanguage.json` to your VSCode extensions directory:
   - Linux: `~/.vscode/extensions/polyglot-syntax/syntaxes/`
   - macOS: `~/.vscode/extensions/polyglot-syntax/syntaxes/`
   - Windows: `%USERPROFILE%\.vscode\extensions\polyglot-syntax\syntaxes\`

2. Create a `package.json` in the extension root with the following content:

```json
{
  "name": "polyglot-syntax",
  "displayName": "Polyglot Language Support",
  "description": "Syntax highlighting for Polyglot automation language",
  "version": "0.1.0",
  "engines": {
    "vscode": "^1.60.0"
  },
  "categories": ["Programming Languages"],
  "contributes": {
    "languages": [{
      "id": "polyglot",
      "aliases": ["Polyglot", "polyglot"],
      "extensions": [".pg"],
      "configuration": "./language-configuration.json"
    }],
    "grammars": [{
      "language": "polyglot",
      "scopeName": "source.polyglot",
      "path": "./syntaxes/polyglot.tmLanguage.json"
    }]
  }
}
```

3. Reload VSCode (`Developer: Reload Window`)

### For Markdown Code Blocks

To enable syntax highlighting in markdown fenced code blocks, you need to register the grammar as an embedded language:

#### Option 1: VSCode Extension with Markdown Injection

Create a second grammar file `polyglot.markdown.injection.json`:

```json
{
  "scopeName": "markdown.polyglot.codeblock",
  "injectionSelector": "L:markup.fenced_code.block.markdown",
  "patterns": [
    {
      "include": "#fenced_code_block_polyglot"
    }
  ],
  "repository": {
    "fenced_code_block_polyglot": {
      "begin": "(^|\\G)(\\s*)(`{3,}|~{3,})\\s*(?i:(polyglot|pg)(\\s+[^`~]*)?$)",
      "name": "markup.fenced_code.block.markdown",
      "end": "(^|\\G)(\\2|\\s{0,3})(\\3)\\s*$",
      "beginCaptures": {
        "3": {
          "name": "punctuation.definition.markdown"
        },
        "4": {
          "name": "fenced_code.block.language.markdown"
        }
      },
      "endCaptures": {
        "3": {
          "name": "punctuation.definition.markdown"
        }
      },
      "patterns": [
        {
          "begin": "(^|\\G)(\\s*)(.*)",
          "while": "(^|\\G)(?!\\s*([`~]{3,})\\s*$)",
          "contentName": "meta.embedded.block.polyglot",
          "patterns": [
            {
              "include": "source.polyglot"
            }
          ]
        }
      ]
    }
  }
}
```

Add to `package.json`:

```json
"grammars": [
  {
    "language": "polyglot",
    "scopeName": "source.polyglot",
    "path": "./syntaxes/polyglot.tmLanguage.json"
  },
  {
    "scopeName": "markdown.polyglot.codeblock",
    "path": "./syntaxes/polyglot.markdown.injection.json",
    "injectTo": ["text.html.markdown"],
    "embeddedLanguages": {
      "meta.embedded.block.polyglot": "polyglot"
    }
  }
]
```

#### Option 2: GitHub/GitLab Linguist

For GitHub/GitLab syntax highlighting, add to repository's `.gitattributes`:

```
*.pg linguist-language=Polyglot
```

And create `.github/linguist.yml` (if using a custom grammar repository):

```yaml
Polyglot:
  type: programming
  color: "#7e57c2"
  extensions:
    - ".pg"
  tm_scope: source.polyglot
  ace_mode: text
```

### For Other Editors

- **Sublime Text**: Copy `polyglot.tmLanguage.json` to `Packages/User/`
- **Atom**: Create a package with the grammar in `grammars/polyglot.json`
- **TextMate**: Convert JSON to plist format and install in `~/Library/Application Support/TextMate/Bundles/`

## Syntax Highlighting Scopes

The grammar defines the following scope categories:

### Block Markers
- **Registry**: `[@]`, `[|]`, `[#]`, `[!]`, `[M]` - `keyword.control.registry.polyglot`
- **Data Flow**: `[i]`, `[o]`, `[<]`, `[>]` - `keyword.control.dataflow.polyglot`
- **Execution**: `[r]`, `[p]`, `[b]`, `[s]`, `[Y]` - `keyword.control.execution.polyglot`
- **Control Flow**: `[?]`, `[t]`, `[Q]`, `[W]` - `keyword.control.flow.polyglot`
- **Scope**: `[~]`, `[\]`, `[/]`, `[{]`, `[}]` - `keyword.control.scope.polyglot`
- **Logical**: `[&]`, `[+]`, `[-]`, `[^]`, `[.]` - `keyword.operator.logical.polyglot`
- **Special**: `[X]`, `[A]`, `[*]` - `keyword.control.special.polyglot`

### Operators
- **Push**: `<<` - `keyword.operator.dataflow.push.polyglot`
- **Pull**: `>>` - `keyword.operator.dataflow.pull.polyglot`
- **Default**: `<~` - `keyword.operator.dataflow.default.polyglot`
- **Comparison**: `=?`, `>?`, `<?`, `>=?`, `<=?`, `!?` - `keyword.operator.comparison.polyglot`
- **Range**: `?[`, `?(` - `keyword.operator.range.polyglot`
- **Collection**: `~*`, `~Y.*` - `keyword.operator.collection.polyglot`

### Identifiers
- **Pipeline**: `|PipelineName` - `entity.name.function.pipeline.polyglot`
- **Enumeration**: `#EnumName` - `entity.name.type.enumeration.polyglot`
- **Error**: `!ErrorName` - `entity.name.type.error.polyglot`
- **Variable**: `.variableName` - `variable.other.polyglot`
- **Package**: `@package/name` - `entity.name.package.polyglot`

### Types
- **Namespace**: `pg\`, `rs\`, `py\`, `js\`, `go\` - `storage.type.namespace.polyglot`
- **Primitive**: `int`, `float`, `string`, `bool`, `path`, `url`, `datetime` - `storage.type.primitive.polyglot`
- **Collection**: `array`, `map`, `set` - `storage.type.collection.polyglot`
- **Runtime Wrapper**: `RT.Python`, `RT.Rust`, etc. - `storage.type.wrapper.polyglot`

### Literals
- **String**: `"text"` - `string.quoted.double.polyglot`
- **DateTime String**: `DT"2025-12-03"` - `string.quoted.datetime.polyglot`
- **Number**: `123`, `45.67` - `constant.numeric.*.polyglot`
- **Reserved Enums**: `#Boolean.True`, `#None`, `#PgVar.States.Ready` - `constant.language.*.polyglot`
- **Error Markers**: `!No.Input`, `!No.Output` - `constant.language.error.polyglot`

### Comments
- **Line Comment**: `// comment` - `comment.line.double-slash.polyglot`

## Customizing Colors

To customize syntax highlighting colors, add theme overrides to your VSCode `settings.json`:

```json
"editor.tokenColorCustomizations": {
  "textMateRules": [
    {
      "scope": "keyword.control.registry.polyglot",
      "settings": {
        "foreground": "#C792EA",
        "fontStyle": "bold"
      }
    },
    {
      "scope": "keyword.operator.dataflow.polyglot",
      "settings": {
        "foreground": "#89DDFF"
      }
    },
    {
      "scope": "entity.name.function.pipeline.polyglot",
      "settings": {
        "foreground": "#82AAFF",
        "fontStyle": "italic"
      }
    }
  ]
}
```

## Testing

Test the grammar with sample Polyglot code:

```polyglot
[@] @example/hello-world

[|] |HelloWorld
  [i] !No.Input
  [t] |T.Manual

  [r] .message: pg\string << "Hello, World!"
  [r] .timestamp: pg\datetime << DT"2025-12-03T00:00:00Z"

  [o] .message: pg\string
  [o] .timestamp: pg\datetime
[X]
```

## Contribution

To extend the grammar:

1. Add new patterns to the appropriate repository section in `polyglot.tmLanguage.json`
2. Assign appropriate scope names following TextMate conventions
3. Test with sample code
4. Update this README with new scope documentation

## Resources

- [TextMate Language Grammars](https://manual.macromates.com/en/language_grammars)
- [VSCode Syntax Highlight Guide](https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide)
- [Scope Naming Conventions](https://www.sublimetext.com/docs/scope_naming.html)
