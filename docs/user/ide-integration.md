---
audience: [user, developer]
type: guide
scope: ide-integration
updated: 2026-05-04
---

# Aljam3 IDE Integration Guide

This document outlines the official intelligent features provided by the Aljam3 VS Code / Antigravity extension, including our **"Symbology-Only" Semantic Highlighting** and **Context-Aware Auto-Complete**.

## 1. "Symbology-Only" Semantic Highlighting

Aljam3 employs a highly intentional **"Symbology-Only"** syntax highlighting philosophy (refer to the core language design principles). 

Unlike traditional programming languages that arbitrarily colorize variable names, keywords, and functions based on their type, Aljam3 **only colors structural prefixes and brackets**, leaving the actual identifier text completely plain. This forces the eye to parse the *structural and operational meaning* of the code rather than reading the names, drastically reducing visual noise and cognitive load.

### Highlight Criteria and Color Taxonomy

The extension assigns colors based on a strict semantic state-machine mapped to the language's prefixes and markers (see [[user/syntax/identifiers]] and [[user/syntax/blocks]]):

| Semantic State | Element | Prefix | Bracket | Assigned Color | Hex Code |
|----------------|---------|--------|---------|----------------|----------|
| **Go** | Sequential Execution | `-` | `[-]` | Spring Green | `#00FF7F` |
| **Go** | Parallel Execution | `=` | `[=]` | Yellow Green | `#9ACD32` |
| **Go** | Wrapper | `-W.` | `[W]` | Medium Sea Green | `#3CB371` |
| **Go** | Environment | `;` | `[;]` | Dark Sea Green | `#8FBC8F` |
| **Wait** | Trigger | `-T.` | `[T]` | Gold | `#FFD700` |
| **Wait** | Queue | `-Q.` | `[Q]` | Khaki | `#F0E68C` |
| **Wait** | Collector | `*` | `[*]` | Pale Goldenrod | `#EEE8AA` |
| **Stop** | Error | `!` | `[!]` | Crimson | `#DC143C` |
| **Stop** | Permission | `_` | `[_]` | Orange Red | `#FF4500` |
| **Logic** | Conditional | `?` | `[?]` | Dodger Blue | `#1E90FF` |
| **Royal** | Package | `@` | `[@]` | Medium Purple | `#9370DB` |
| **Royal** | Metadata | `%` | `[%]` | Orchid | `#DA70D6` |
| **Data** | Data Type | `#` | `[#]` | Orange | `#FFA500` |
| **Data** | Data Instance | `$` | `[$]` | Dark Goldenrod | `#B8860B` |
| **Data** | Subfield/Accessor | `.` or `:` | `[.]` / `[:]` | Hot Pink | `#FF69B4` |
| **Flow** | Input Port | `<` / `<<` | `[<]` | Lime Green | `#32CD32` |
| **Flow** | Output Port | `>` / `>>` | `[>]` | Lawn Green | `#7CFC00` |

*Note: String (`" "`) literals are colored Coral (`#FF7F50`) to group them loosely with the Orange Data family. Number literals maintain standard light blue coloration. Comments (`//`, `{ }`, `[ ]`) are muted to grey to prevent visual distraction from the operational symbology.*

---

## 2. Context-Aware Auto-Complete

The Aljam3 extension provides a highly advanced, context-aware autocomplete engine that acts as a structural assistant.

### A. Static Library Dictionary (`jm3lib`)
The extension ships with a complete dictionary of the Aljam3 Standard Library ([[user/jm3lib/INDEX]]). When you trigger specific prefixes, the extension instantly provides the available library implementations:
- **`#` (Types):** Triggers `#String`, `#Int`, `#Boolean`, `#Map`, `#Array`, etc.
- **`-` (Pipelines):** Triggers standard executors like `-File.Read`, `-Math.Add`, `-Sys.Log`.
- **`!` (Errors):** Triggers `!TypeMismatch`, `!FileNotFound`, etc.
- **`$$` (System Constants):** Triggers `$$True`, `$$False`, `$$Null`.

### B. Chained Triggers
The autocomplete engine understands Aljam3's compositional nature. If you are constructing a conditional query and type `?` and select `#`, the menu will automatically re-trigger to offer the type list (`#String`, `#Int`, etc.), allowing for fluid, uninterrupted typing.

### C. Dynamic Local Scanning
Because Aljam3 strictly enforces the `{PrefixMarker}` definition structure, the extension dynamically scans your active document in real-time. 
- If you define `{#} #MyCustomType`, typing `#` anywhere in the file will automatically offer `#MyCustomType`.
- If you define `{T} -T.MyTrigger`, typing `-` will automatically offer `-T.MyTrigger`.

### D. Block-Aware Intelligence
When typing `[`, the extension looks at the enclosing definition block (`{...}`) to offer only valid execution blocks.
- Inside a **Pipeline Block (`{-}`)**, typing `[` offers: `T`, `W`, `Q`, `-`, `=`, `b`, `\`, `/`, `?`, `!`, `$`.
- Inside an **Import Block (`{@}`)**, typing `[` offers: `@`, `$`.
- Inside a **Type Block (`{#}`)**, typing `[` offers: `#`, `.`, `:`.

Selecting a block (e.g., `T`) will automatically expand the structure (e.g., inserting `[T]\n    -T.`) and instantly trigger the corresponding auto-complete menu for that prefix!

---

## 3. Installation and Application

The extension is built and managed via the `syntax-highlighting/vscode-extension/` directory in the repository.

### For Antigravity (Local Development Editor)
If you are developing the language or testing the extension locally within the Antigravity editor:
1. Navigate to the extension directory: `cd syntax-highlighting/vscode-extension/`
2. Run the dedicated Antigravity script: `./install.sh`
3. This script will purge the local cache, uninstall the old version, compile the TypeScript extension, package the new VSIX, and force-install it.
4. **CRITICAL:** You must *completely quit* the Antigravity application (close all windows) and reopen it to force the editor to dump the cached TextMate grammar and load the new Semantic Color Theory.

### For VS Code (Standard User Installation)
If you are installing the extension into a standard VS Code environment:
1. Navigate to the extension directory: `cd syntax-highlighting/vscode-extension/`
2. Run the VS Code script: `./install_vscode.sh`
3. Once the installation completes, press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac) and execute the **"Developer: Reload Window"** command.
4. To see the colors, you **must explicitly select the custom theme**. Open the Command Palette (`Ctrl+Shift+P`), type **"Preferences: Color Theme"**, and select the **"Aljam3 Semantic Theme"** from the dropdown menu.
