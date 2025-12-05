# Installing Polyglot Language Support for VSCode

This guide explains how to install the Polyglot syntax highlighting extension for Visual Studio Code.

## Quick Install (Local Development)

### Method 1: Symlink to Extensions Directory

1. **Create extension directory** (if it doesn't exist):
   ```bash
   mkdir -p ~/.vscode/extensions/polyglot-language-support-0.1.0
   ```

2. **Copy extension files**:
   ```bash
   cp -r vscode-extension/* ~/.vscode/extensions/polyglot-language-support-0.1.0/
   ```

3. **Reload VSCode**:
   - Open Command Palette: `Ctrl+Shift+P` (Linux/Windows) or `Cmd+Shift+P` (macOS)
   - Type: `Developer: Reload Window`
   - Press Enter

4. **Verify installation**:
   - Open a `.pg` file or create a new file with `.pg` extension
   - Check the language mode indicator in the bottom-right corner
   - Should show "Polyglot"

### Method 2: Install from VSIX Package

1. **Install vsce** (VSCode Extension Manager):
   ```bash
   npm install -g @vscode/vsce
   ```

2. **Navigate to extension directory**:
   ```bash
   cd vscode-extension
   ```

3. **Package the extension**:
   ```bash
   vsce package
   ```

   This creates `polyglot-language-support-0.1.0.vsix`

4. **Install the VSIX**:
   - Open VSCode
   - Go to Extensions view (`Ctrl+Shift+X`)
   - Click the `...` menu (top-right of Extensions view)
   - Select "Install from VSIX..."
   - Choose the generated `.vsix` file

5. **Reload VSCode**:
   - Reload window when prompted

## Testing Syntax Highlighting

### Test with .pg Files

Create a test file `test.pg`:

```polyglot
[@] @example/hello-world

[|] |HelloWorld
  [i] !No.Input
  [t] |T.Manual

  [r] .message: pg\string << "Hello, World!"
  [r] .count: pg\int << 42
  [r] .timestamp: pg\datetime << DT"2025-12-03T00:00:00Z"

  // Push to output
  [o] .message: pg\string
  [o] .count: pg\int
[X]
```

Open `test.pg` in VSCode - you should see:
- Block markers `[@]`, `[|]`, `[r]`, etc. highlighted
- Operators `<<` highlighted
- Identifiers like `.message`, `|HelloWorld` highlighted
- Types `pg\string`, `pg\int` highlighted
- Strings and datetime literals highlighted
- Comments in gray/muted color

### Test with Markdown Code Blocks

Create a test file `test.md`:

````markdown
# Polyglot Example

This is a Polyglot pipeline:

```polyglot
[|] |Example
  [i] .input: pg\string
  [t] |T.Manual

  [r] .result: pg\string << .input

  [o] .result: pg\string
[X]
```

The pipeline processes input and returns output.
````

Open `test.md` in VSCode - the code inside the fenced code block should be syntax-highlighted.

## Customizing Colors

To customize syntax highlighting colors, add to your VSCode `settings.json`:

1. **Open Settings JSON**:
   - `Ctrl+Shift+P` → `Preferences: Open User Settings (JSON)`

2. **Add custom colors**:
```json
{
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
      },
      {
        "scope": "variable.other.polyglot",
        "settings": {
          "foreground": "#FFCB6B"
        }
      },
      {
        "scope": "string.quoted.datetime.polyglot",
        "settings": {
          "foreground": "#C3E88D",
          "fontStyle": "italic"
        }
      }
    ]
  }
}
```

## Using Code Snippets

The extension includes helpful snippets. Type the prefix and press `Tab`:

| Prefix | Description |
|--------|-------------|
| `pipeline` | Basic pipeline template |
| `pipeline-io` | Pipeline with input/output |
| `exec-seq` | Sequential execution block |
| `exec-parallel` | Parallel execution block |
| `error-def` | Error definition |
| `error-catch` | Error catch block |
| `switch` | Conditional switch |
| `enum` | Enumeration definition |
| `var` | Variable declaration |
| `push` | Push assignment |
| `pull` | Pull assignment |
| `default` | Default assignment |
| `nest` | Nested scope |
| `join` | Join point |
| `wrapper-python` | Python runtime wrapper |
| `trigger-datetime` | DateTime trigger |

### Example Usage

1. Create a new `.pg` file
2. Type `pipeline` and press `Tab`
3. Fill in the placeholders (press `Tab` to move between them)

## Troubleshooting

### Syntax Highlighting Not Working

1. **Check language mode**:
   - Click the language indicator in bottom-right corner
   - Select "Polyglot" from the list

2. **Check file association**:
   - Add to `settings.json`:
   ```json
   {
     "files.associations": {
       "*.pg": "polyglot"
     }
   }
   ```

3. **Reload extension host**:
   - `Ctrl+Shift+P` → `Developer: Reload Window`

4. **Check extension installation**:
   - Go to Extensions view
   - Search for "Polyglot"
   - Should show as installed

### Markdown Code Blocks Not Highlighted

1. **Verify injection grammar**:
   - Check that `polyglot.markdown.injection.json` exists in `syntaxes/`

2. **Try different fence identifiers**:
   - Use `polyglot` or `pg` as the language identifier
   - Example: ` ```polyglot ` or ` ```pg `

3. **Reload window**:
   - Markdown injection may require a full reload

### Snippets Not Working

1. **Check snippet file**:
   - Verify `snippets/polyglot.json` exists

2. **Enable snippets in settings**:
   ```json
   {
     "editor.snippetSuggestions": "top",
     "editor.tabCompletion": "on"
   }
   ```

## Publishing to VSCode Marketplace (Optional)

To publish the extension publicly:

1. **Create publisher account**:
   - Go to https://marketplace.visualstudio.com/manage
   - Create a publisher ID

2. **Update package.json**:
   - Set `publisher` field to your publisher ID

3. **Login to vsce**:
   ```bash
   vsce login <publisher-id>
   ```

4. **Publish**:
   ```bash
   vsce publish
   ```

## Uninstalling

### Remove Extension

1. **Via VSCode UI**:
   - Go to Extensions view
   - Find "Polyglot Language Support"
   - Click Uninstall

2. **Manually**:
   ```bash
   rm -rf ~/.vscode/extensions/polyglot-language-support-0.1.0
   ```

3. **Reload VSCode**

## Support

For issues or feature requests:
- Check the [Polyglot documentation](../../docs/)
- Report issues to the project repository
- Review TextMate grammar in `syntaxes/polyglot.tmLanguage.json`
