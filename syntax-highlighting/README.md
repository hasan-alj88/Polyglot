# Aljam3 Syntax Highlighting

This directory contains syntax highlighting definitions for the Aljam3 automation language.

## Files

- **aljam3.tmLanguage.json**: TextMate grammar definition for Aljam3 syntax
- **vscode-extension/**: VSCode extension for Aljam3 syntax highlighting (optional)

## Using the TextMate Grammar

### For VSCode

1. Copy `aljam3.tmLanguage.json` to your VSCode extensions directory:
   - Linux: `~/.vscode/extensions/aljam3-syntax/syntaxes/`
   - macOS: `~/.vscode/extensions/aljam3-syntax/syntaxes/`
   - Windows: `%USERPROFILE%\.vscode\extensions\aljam3-syntax\syntaxes\`

2. Create a `package.json` in the extension root with the following content:

```json
{
  "name": "aljam3-syntax",
  "displayName": "Aljam3 Language Support",
  "description": "Syntax highlighting for Aljam3 automation language",
  "version": "0.1.0",
  "engines": {
    "vscode": "^1.60.0"
  },
  "categories": ["Programming Languages"],
  "contributes": {
    "languages": [{
      "id": "aljam3",
      "aliases": ["Aljam3", "aljam3"],
      "extensions": [".jm3"],
      "configuration": "./language-configuration.json"
    }],
    "grammars": [{
      "language": "aljam3",
      "scopeName": "source.aljam3",
      "path": "./syntaxes/aljam3.tmLanguage.json"
    }]
  }
}
```

3. Reload VSCode (`Developer: Reload Window`)

### For Markdown Code Blocks

To enable syntax highlighting in markdown fenced code blocks, you need to register the grammar as an embedded language:

#### Option 1: VSCode Extension with Markdown Injection

Create a second grammar file `aljam3.markdown.injection.json`:

```json
{
  "scopeName": "markdown.aljam3.codeblock",
  "injectionSelector": "L:markup.fenced_code.block.markdown",
  "patterns": [
    {
      "include": "#fenced_code_block_aljam3"
    }
  ],
  "repository": {
    "fenced_code_block_aljam3": {
      "begin": "(^|\\G)(\\s*)(`{3,}|~{3,})\\s*(?i:(aljam3|pg)(\\s+[^`~]*)?$)",
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
          "contentName": "meta.embedded.block.aljam3",
          "patterns": [
            {
              "include": "source.aljam3"
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
    "language": "aljam3",
    "scopeName": "source.aljam3",
    "path": "./syntaxes/aljam3.tmLanguage.json"
  },
  {
    "scopeName": "markdown.aljam3.codeblock",
    "path": "./syntaxes/aljam3.markdown.injection.json",
    "injectTo": ["text.html.markdown"],
    "embeddedLanguages": {
      "meta.embedded.block.aljam3": "aljam3"
    }
  }
]
```

#### Option 2: GitHub/GitLab Linguist

For GitHub/GitLab syntax highlighting, add to repository's `.gitattributes`:

```
*.jm3 linguist-language=Aljam3
```

And create `.github/linguist.yml` (if using a custom grammar repository):

```yaml
Aljam3:
  type: programming
  color: "#7e57c2"
  extensions:
    - ".jm3"
  tm_scope: source.aljam3
  ace_mode: text
```

### For Other Editors

- **Sublime Text**: Copy `aljam3.tmLanguage.json` to `Packages/User/`
- **Atom**: Create a package with the grammar in `grammars/aljam3.json`
- **TextMate**: Convert JSON to plist format and install in `~/Library/Application Support/TextMate/Bundles/`

## Syntax Highlighting Scopes

The grammar defines the following scope categories:

### Block Markers
- **Registry**: `[@]`, `[|]`, `[#]`, `[!]`, `[M]` - `keyword.control.registry.aljam3`
- **Data Flow**: `[i]`, `[o]`, `[<]`, `[>]` - `keyword.control.dataflow.aljam3`
- **Execution**: `[r]`, `[p]`, `[b]`, `[s]`, `[Y]` - `keyword.control.execution.aljam3`
- **Control Flow**: `[?]`, `[t]`, `[Q]`, `[W]` - `keyword.control.flow.aljam3`
- **Scope**: `[~]`, `[\]`, `[/]`, `[{]`, `[}]` - `keyword.control.scope.aljam3`
- **Logical**: `[&]`, `[+]`, `[-]`, `[^]`, `[.]` - `keyword.operator.logical.aljam3`
- **Special**: `[X]`, `[A]`, `[*]`, `[$]` - `keyword.control.special.aljam3`

### Operators
- **Push**: `<<` - `keyword.operator.dataflow.push.aljam3`
- **Pull**: `>>` - `keyword.operator.dataflow.pull.aljam3`
- **Default**: `<~` - `keyword.operator.dataflow.default.aljam3`
- **Comparison**: `=?`, `>?`, `<?`, `>=?`, `<=?`, `!?` - `keyword.operator.comparison.aljam3`
- **Range**: `?[`, `?(` - `keyword.operator.range.aljam3`
- **Collection**: `~*`, `~Y.*` - `keyword.operator.collection.aljam3`

### Identifiers
- **Pipeline**: `-PipelineName` - `entity.name.function.pipeline.aljam3`
- **Enumeration**: `#EnumName` - `entity.name.type.enumeration.aljam3`
- **Error**: `!ErrorName` - `entity.name.type.error.aljam3`
- **Variable**: `.variableName`, `$name` - `variable.other.aljam3`
- **Constant**: `$$True`, `$$Alias` - `constant.language.aljam3`
- **Package**: `@package/name` - `entity.name.package.aljam3`

### Types
- **Namespace**: `pg\`, `rs\`, `py\`, `js\`, `go\` - `storage.type.namespace.aljam3`
- **Primitive**: `int`, `float`, `string`, `bool`, `path`, `url`, `datetime` - `storage.type.primitive.aljam3`
- **Collection**: `array`, `map`, `set` - `storage.type.collection.aljam3`
- **Runtime Wrapper**: `RT.Python`, `RT.Rust`, etc. - `storage.type.wrapper.aljam3`

### Literals
- **String**: `"text"` - `string.quoted.double.aljam3`
- **DateTime String**: `DT"2025-12-03"` - `string.quoted.datetime.aljam3`
- **Number**: `123`, `45.67` - `constant.numeric.*.aljam3`
- **Reserved Constants**: `$$True`, `$$False`, `$$None` - `constant.language.*.aljam3`
- **Error Markers**: `!No.Input`, `!No.Output` - `constant.language.error.aljam3`

### Comments
- **Line Comment**: `// comment` - `comment.line.double-slash.aljam3`

## Customizing Colors

To customize syntax highlighting colors, add theme overrides to your VSCode `settings.json`:

```json
"editor.tokenColorCustomizations": {
  "textMateRules": [
    {
      "scope": "keyword.control.registry.aljam3",
      "settings": {
        "foreground": "#C792EA",
        "fontStyle": "bold"
      }
    },
    {
      "scope": "keyword.operator.dataflow.aljam3",
      "settings": {
        "foreground": "#89DDFF"
      }
    },
    {
      "scope": "entity.name.function.pipeline.aljam3",
      "settings": {
        "foreground": "#82AAFF",
        "fontStyle": "italic"
      }
    }
  ]
}
```

## Testing

Test the grammar with sample Aljam3 code:

```aljam3
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

1. Add new patterns to the appropriate repository section in `aljam3.tmLanguage.json`
2. Assign appropriate scope names following TextMate conventions
3. Test with sample code
4. Update this README with new scope documentation

## Resources

- [TextMate Language Grammars](https://manual.macromates.com/en/language_grammars)
- [VSCode Syntax Highlight Guide](https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide)
- [Scope Naming Conventions](https://www.sublimetext.com/docs/scope_naming.html)
