import * as vscode from 'vscode';
import * as jm3libCompletionsRaw from './jm3lib_completions.json';

type Jm3libCompletions = {
    [key: string]: {
        label: string;
        detail?: string;
        insertText?: string;
        kind: keyof typeof vscode.CompletionItemKind;
        doc?: string;
        command?: string;
    }[];
};

const jm3libCompletions = jm3libCompletionsRaw as Jm3libCompletions;

// Universal extractor for Aljam3 local definitions
function extractLocalDefinitions(text: string, trigger: string): string[] {
    let blockMarker = trigger.charAt(0);
    
    // For -T. the block marker is 'T', for -W. it is 'W', etc.
    if (trigger.startsWith('-') && trigger.length > 1) {
        const match = trigger.match(/^-([A-Z]+)\.?/);
        if (match) {
            blockMarker = match[1];
        }
    }
    
    const escMarker = blockMarker.replace(/[-[\]{}()*+?.,\\^$|#\s]/g, '\\$&');
    const escTrigger = trigger.replace(/[-[\]{}()*+?.,\\^$|#\s]/g, '\\$&');
    
    const regexPattern = new RegExp(`^\\{${escMarker}\\}\\s*${escTrigger}([a-zA-Z0-9_]+)`, 'gm');
    
    const results: string[] = [];
    let match;
    while ((match = regexPattern.exec(text)) !== null) {
        results.push(match[1]);
    }
    return results;
}

// Extractor for package aliases defined in [@] blocks
// Matches: @aliasImport << @Company.pkg
function extractImportAliases(text: string): string[] {
    const aliasRegex = /^\s*@([a-zA-Z0-9_]+)\s*<</gm;
    const results: string[] = [];
    let match;
    while ((match = aliasRegex.exec(text)) !== null) {
        results.push(match[1]); // Captures 'aliasImport'
    }
    return results;
}

export function activate(context: vscode.ExtensionContext) {
    const aljam3CompletionProvider = vscode.languages.registerCompletionItemProvider(
        'aljam3',
        {
            provideCompletionItems(document, position, token, context) {
                const triggerCharacter = context.triggerCharacter;
                let lookupKey = triggerCharacter;
                const completionItems: vscode.CompletionItem[] = [];
                const text = document.getText();
                const linePrefix = document.lineAt(position).text.substr(0, position.character);

                // If trigger is undefined (manual Ctrl+Space or command), deduce from text
                if (!lookupKey) {
                    const match = linePrefix.match(/(-[A-Za-z]+\.|[-#?!@\[=]|\$\$|\*\*|__)\s*$/);
                    if (match) {
                        lookupKey = match[1].trim();
                    } else {
                        return undefined;
                    }
                } else if (lookupKey === '.') {
                    const dotMatch = linePrefix.match(/(-[a-zA-Z]+)\.$/);
                    if (dotMatch) {
                        lookupKey = dotMatch[1] + '.';
                    } else {
                        return undefined;
                    }
                } else if (lookupKey === '[') {
                    // Context-aware block completion based on the enclosing definition block
                    let contextBlock = '[-'; // Default to pipeline blocks
                    for (let i = position.line; i >= 0; i--) {
                        const lineText = document.lineAt(i).text.trim();
                        const defMatch = lineText.match(/^\{([^\}]+)\}/);
                        if (defMatch) {
                            const marker = defMatch[1];
                            if (marker === '@') {
                                contextBlock = '[@';
                            } else if (marker.startsWith('#') || marker.startsWith('!')) {
                                contextBlock = '[#';
                            }
                            break;
                        }
                    }
                    lookupKey = contextBlock;
                }
                
                // --- FUTURE IMPLEMENTATION PLACEHOLDER ---
                // If user typed an alias followed by ANY prefix: @aliasImport# or @aliasImport-
                // We need cross-file package retrieval to show definitions from that bound file.
                // For now, return empty. When package loading is implemented, populate here.
                // Escape trigger char for regex
                const escapedTriggerForAlias = lookupKey.replace(/[-[\]{}()*+?.,\\^$|#\s]/g, '\\$&');
                const aliasRegex = new RegExp(`@([a-zA-Z0-9_]+)${escapedTriggerForAlias}$`);
                
                const aliasMatch = linePrefix.match(aliasRegex);
                if (aliasMatch) {
                    // eslint-disable-next-line @typescript-eslint/no-unused-vars
                    const aliasName = aliasMatch[1]; // e.g., 'aliasImport'
                    // TODO: 1. Lookup 'aliasName' in [@] block to find file path
                    //       2. Parse that file to find definitions matching 'triggerCharacter'
                    return []; 
                }

                // 1. Load static jm3lib items for this trigger
                // @NOTE: ALWAYS cross-reference with jm3lib docs. If 'INDEX.md' changes, update the JSON.
                const staticItems = jm3libCompletions[lookupKey] || [];
                for (const item of staticItems) {
                    const kind = (vscode.CompletionItemKind as any)[item.kind] as vscode.CompletionItemKind;
                    const completion = new vscode.CompletionItem(item.label, kind);
                    if (item.detail) completion.detail = item.detail;
                    if (item.doc) completion.documentation = new vscode.MarkdownString(item.doc);
                    if (item.insertText) completion.insertText = new vscode.SnippetString(item.insertText);
                    if (item.command) {
                        completion.command = { command: 'editor.action.triggerSuggest', title: 'Re-trigger' };
                    }
                    completionItems.push(completion);
                }

                // 2. Scan document for local definitions (Types, Pipelines, etc)
                if (['#', '##', '-', '!', '@', '-T.', '-Q.', '-W.'].includes(lookupKey)) {
                    const localNames = extractLocalDefinitions(text, lookupKey);
                    for (const name of localNames) {
                        const item = new vscode.CompletionItem(name, vscode.CompletionItemKind.Reference);
                        item.detail = "Local Definition";
                        completionItems.push(item);
                    }
                }
                
                // 3. Scan document for Import Aliases when typing `@`
                if (lookupKey === '@') {
                    const aliasNames = extractImportAliases(text);
                    for (const name of aliasNames) {
                        const item = new vscode.CompletionItem(name, vscode.CompletionItemKind.Module);
                        item.detail = "Import Alias";
                        // Setup the chained trigger for '@alias#' in the future
                        item.command = { command: 'editor.action.triggerSuggest', title: 'Re-trigger' };
                        completionItems.push(item);
                    }
                }
                
                return completionItems;
            }
        },
        // Trigger characters
        '@', '?', '=', '#', '-', '$', '!', '_', '.', '['
    );

    context.subscriptions.push(aljam3CompletionProvider);
}
