#!/usr/bin/env python3
"""Fix pipe character escaping in markdown tables for Polyglot syntax."""

import re
import sys
from pathlib import Path

def fix_pipes_in_table_cell(cell):
    """Escape pipes in Polyglot syntax within a table cell."""
    # Escape |Pipeline, |T., |U., |W., |Y., |Q., |Macro patterns
    cell = re.sub(r'(\|[A-Z][A-Za-z0-9._]*)', r'&#124;\1', cell)
    # Escape standalone | operator references
    cell = re.sub(r'`(\|)`', r'`&#124;`', cell)
    return cell

def fix_table_row(line):
    """Fix pipes in a table row, preserving table delimiters."""
    if not line.strip().startswith('|'):
        return line

    # Skip separator rows (e.g., |---|---|)
    if re.match(r'^\|[\s\-:]+\|', line):
        return line

    # Split by | but preserve them
    parts = line.split('|')

    # First and last parts are empty (line starts and ends with |)
    if len(parts) < 3:
        return line

    # Fix each cell (skip first empty part)
    fixed_parts = [parts[0]]  # Keep empty first part
    for i, part in enumerate(parts[1:-1], 1):
        fixed_parts.append(fix_pipes_in_table_cell(part))
    fixed_parts.append(parts[-1])  # Keep last part (usually empty or newline)

    return '|'.join(fixed_parts)

def fix_file(filepath):
    """Fix pipe escaping in a markdown file."""
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    fixed_lines = []
    in_table = False

    for line in lines:
        # Detect table start
        if line.strip().startswith('|') and '|' in line[1:]:
            in_table = True
            fixed_lines.append(fix_table_row(line))
        elif in_table and line.strip() == '':
            # Table ended
            in_table = False
            fixed_lines.append(line)
        elif in_table:
            fixed_lines.append(fix_table_row(line))
        else:
            fixed_lines.append(line)

    with open(filepath, 'w', encoding='utf-8') as f:
        f.writelines(fixed_lines)

    return len([l for l in fixed_lines if '&#124;' in l])

def main():
    docs_dir = Path('docs/user')

    if not docs_dir.exists():
        print(f"Error: {docs_dir} not found")
        sys.exit(1)

    md_files = list(docs_dir.rglob('*.md'))

    total_fixed = 0
    for filepath in md_files:
        fixes = fix_file(filepath)
        if fixes > 0:
            print(f"Fixed {fixes} lines in {filepath.relative_to(docs_dir.parent)}")
            total_fixed += fixes

    print(f"\nTotal: Fixed pipes in {total_fixed} lines across {len(md_files)} files")

if __name__ == '__main__':
    main()
