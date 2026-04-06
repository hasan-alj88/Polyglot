#!/usr/bin/env bash
# docs-pdf-hook.sh — Post-commit hook: regenerate PDF if docs/ changed
#
# Called by Claude Code PostToolUse hook after Bash(git commit) commands.
# Only runs the full generation if docs/ files were modified in the last commit.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Only trigger on git commit tool calls
if [[ "${TOOL_NAME:-}" != "Bash" ]]; then
   exit 0
fi

# Check if the tool input looks like a git commit
if [[ "${TOOL_INPUT:-}" != *"git commit"* ]]; then
   exit 0
fi

# Check if the last commit touched docs/
if ! git -C "$REPO_ROOT" diff --name-only HEAD~1 HEAD 2>/dev/null | grep -q '^docs/'; then
   exit 0
fi

# Regenerate PDF
echo "docs/ changed — regenerating Polyglot-Documentation.pdf..."
"$REPO_ROOT/scripts/generate-docs-pdf.sh" 2>&1 | tail -3
