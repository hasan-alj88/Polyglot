#!/usr/bin/env bash
# generate-docs-pdf.sh — Combine all docs/ markdown into a single PDF via Pandoc + Typst
#
# Usage:
#   ./scripts/generate-docs-pdf.sh              # full documentation book
#   ./scripts/generate-docs-pdf.sh docs/user    # only user docs
#
# Requirements: pandoc (>=3.x), typst (>=0.14)
# Output: Polyglot-Documentation.pdf in repo root

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DOCS_DIR="$REPO_ROOT/docs"
TEMPLATE="$REPO_ROOT/scripts/doc-template.typ"
BUILD_DIR="$REPO_ROOT/.docs-build"
OUTPUT_PDF="$REPO_ROOT/Polyglot-Documentation.pdf"
COMBINED_TYP="$BUILD_DIR/book.typ"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m'

log()   { echo -e "${GREEN}[OK]${NC} $1"; }
warn()  { echo -e "${YELLOW}[SKIP]${NC} $1"; }
fail()  { echo -e "${RED}[FAIL]${NC} $1"; }
info()  { echo -e "${CYAN}[INFO]${NC} $1"; }

# Check dependencies
for cmd in pandoc typst; do
   if ! command -v "$cmd" &>/dev/null; then
      echo "Error: $cmd not found. Install it first." >&2
      exit 1
   fi
done

# Determine target
TARGET="${1:-$DOCS_DIR}"
if [[ ! "$TARGET" = /* ]]; then
   TARGET="$REPO_ROOT/$TARGET"
fi

# Clean build directory
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# --- Document ordering ---
# Define the canonical section order. Files within each section are sorted.
# This ensures the book reads logically.
SECTIONS=(
   "vision.md"
   "user/SPEC-INDEX.md"
   "user/syntax"
   "user/concepts"
   "user/pglib"
   "user/scenarios"
   "technical/ebnf"
   "technical/edge-cases"
   "technical/COMPILE-RULES.md"
   "technical/compile-rules/algorithms"
   "technical/compile-rules/PGE"
   "technical/compile-rules/PGW"
   "technical/compile-rules/TYPE-IDENTITY.md"
   "technical/spec"
   "technical/plan"
   "technical/brainstorming"
   "audit"
)

# Section display names for part headings
declare -A SECTION_NAMES=(
   ["vision.md"]="Vision"
   ["user/SPEC-INDEX.md"]="Language Reference Index"
   ["user/syntax"]="Syntax Foundations"
   ["user/concepts"]="Core Concepts"
   ["user/pglib"]="Standard Library"
   ["user/scenarios"]="Automation Scenarios"
   ["technical/ebnf"]="EBNF Grammar"
   ["technical/edge-cases"]="Edge Cases"
   ["technical/COMPILE-RULES.md"]="Compile Rules Overview"
   ["technical/compile-rules/algorithms"]="Compiler Algorithms"
   ["technical/compile-rules/PGE"]="Error Rules (PGE)"
   ["technical/compile-rules/PGW"]="Warning Rules (PGW)"
   ["technical/compile-rules/TYPE-IDENTITY.md"]="Type Identity"
   ["technical/spec"]="Technical Specifications"
   ["technical/plan"]="Architecture Plans"
   ["technical/brainstorming"]="Design Brainstorming"
   ["audit"]="Documentation Audit"
)

# Collect files in order
collect_section_files() {
   local section="$1"
   local full_path="$DOCS_DIR/$section"

   if [[ -f "$full_path" ]]; then
      echo "$full_path"
   elif [[ -d "$full_path" ]]; then
      # INDEX.md first if it exists, then everything else sorted
      if [[ -f "$full_path/INDEX.md" ]]; then
         echo "$full_path/INDEX.md"
      fi
      find "$full_path" -name '*.md' -type f ! -name 'INDEX.md' | sort
   fi
}

# Convert one markdown file to typst content (no template wrapper)
convert_one() {
   local md_file="$1"
   local rel_path="${md_file#"$DOCS_DIR"/}"

   # Strip HTML comments, convert, then:
   # 1. Remove standalone label anchors (e.g. <my-heading>)
   # 2. Replace \[#link(<label>)[text]\] with just the display text
   sed 's/<!--.*-->//g' "$md_file" | pandoc \
      --from markdown+yaml_metadata_block \
      --to typst \
      --wrap=none 2>/dev/null | sed -E \
         -e 's/^<[a-z][a-z0-9_-]*>$//g' \
         -e 's/\\\[#link\(<[^>]*>\)\[([^]]*)\]\\\]/\1/g'
}

info "Building combined documentation PDF..."
info "Source: $TARGET"
echo ""

# Counters
total=0
success=0
skipped=0
failed=0

# Start building the combined typst file
cat > "$COMBINED_TYP" <<'HEADER'
#import "/scripts/doc-template.typ": *
#show: polyglot-book

// Title page
#polyglot-title-page()

// Table of contents
#outline(title: "Table of Contents", depth: 2, indent: 1.5em)
#pagebreak()
HEADER

# Process each section
for section in "${SECTIONS[@]}"; do
   full_path="$DOCS_DIR/$section"

   # If targeting a subdirectory, skip sections outside it
   if [[ "$TARGET" != "$DOCS_DIR" ]]; then
      if [[ -f "$TARGET" ]]; then
         [[ "$full_path" != "$TARGET" ]] && continue
      elif [[ -d "$TARGET" ]]; then
         # Check if section is under target
         [[ "$full_path" != "$TARGET"* ]] && continue
      fi
   fi

   # Skip if section doesn't exist
   if [[ ! -e "$full_path" ]]; then
      continue
   fi

   section_name="${SECTION_NAMES[$section]:-$section}"
   info "Section: $section_name"

   # Add part heading
   cat >> "$COMBINED_TYP" <<PART

// ═══════════════════════════════════════════
// Part: $section_name
// ═══════════════════════════════════════════
#part-heading("$section_name")
PART

   # Process files in this section
   while IFS= read -r md_file; do
      [[ -z "$md_file" ]] && continue

      rel_path="${md_file#"$DOCS_DIR"/}"
      ((total++)) || true

      content="$(convert_one "$md_file" 2>/dev/null)" || {
         ((failed++)) || true
         fail "$rel_path (pandoc error)"
         continue
      }

      if [[ -z "$content" ]]; then
         ((skipped++)) || true
         warn "$rel_path (empty)"
         continue
      fi

      # Add a page break and source path annotation before each document
      cat >> "$COMBINED_TYP" <<DOC

// --- $rel_path ---
#doc-separator("$rel_path")
$content
DOC

      ((success++)) || true
      log "$rel_path"

   done < <(collect_section_files "$section")
done

echo ""
info "Compiling combined Typst file to PDF..."

# Compile
if typst compile --root "$REPO_ROOT" "$COMBINED_TYP" "$OUTPUT_PDF" 2>&1; then
   echo ""
   pdf_size="$(du -h "$OUTPUT_PDF" | cut -f1)"
   page_count="$(typst compile --root "$REPO_ROOT" "$COMBINED_TYP" - 2>/dev/null | wc -c || echo "?")"
   log "Output: $OUTPUT_PDF ($pdf_size)"
   log "$success documents included, $skipped skipped, $failed failed (of $total total)"
else
   echo ""
   fail "Typst compilation failed. Check $COMBINED_TYP for errors."
   exit 1
fi

# Clean up build directory on success
rm -rf "$BUILD_DIR"
