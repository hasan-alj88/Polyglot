#!/usr/bin/env bash
# generate-docs-pdf.sh — Combine docs/ markdown into PDF(s) via Pandoc + Typst
#
# Usage:
#   ./scripts/generate-docs-pdf.sh                    # full documentation book
#   ./scripts/generate-docs-pdf.sh docs/user           # only user docs
#   ./scripts/generate-docs-pdf.sh --by-audience       # one PDF per audience in docs/pdf/
#   ./scripts/generate-docs-pdf.sh --audience=architect # single audience PDF
#
# Requirements: pandoc (>=3.x), typst (>=0.14)
# Output:
#   Default:       Polyglot-Documentation.pdf in repo root
#   --by-audience: docs/pdf/{audience}.pdf for each audience

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DOCS_DIR="$REPO_ROOT/docs"
TEMPLATE="$REPO_ROOT/scripts/doc-template.typ"
BUILD_DIR="$REPO_ROOT/.docs-build"
OUTPUT_PDF="$REPO_ROOT/Polyglot-Documentation.pdf"

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

# --- Argument parsing ---
BY_AUDIENCE=false
AUDIENCE_FILTER=""
TARGET="$DOCS_DIR"

while [[ $# -gt 0 ]]; do
   case "$1" in
      --by-audience)
         BY_AUDIENCE=true
         shift
         ;;
      --audience=*)
         AUDIENCE_FILTER="${1#--audience=}"
         BY_AUDIENCE=true
         shift
         ;;
      *)
         TARGET="$1"
         if [[ ! "$TARGET" = /* ]]; then
            TARGET="$REPO_ROOT/$TARGET"
         fi
         shift
         ;;
   esac
done

# --- Document ordering ---
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

# Audience display names
declare -A AUDIENCE_DISPLAY=(
   ["pg-coder"]="Polyglot Coder"
   ["developer"]="Developer"
   ["designer"]="Language Designer"
   ["architect"]="System Architect"
   ["integrator"]="Integrator"
)

# Audiences excluded from per-audience PDF generation
EXCLUDED_AUDIENCES=("ai")

# --- Helper functions ---

# Collect files in section order (INDEX.md first, then sorted)
collect_section_files() {
   local section="$1"
   local full_path="$DOCS_DIR/$section"

   if [[ -f "$full_path" ]]; then
      echo "$full_path"
   elif [[ -d "$full_path" ]]; then
      if [[ -f "$full_path/INDEX.md" ]]; then
         echo "$full_path/INDEX.md"
      fi
      find "$full_path" -name '*.md' -type f ! -name 'INDEX.md' | sort
   fi
}

# Convert one markdown file to typst content (no template wrapper)
convert_one() {
   local md_file="$1"

   sed 's/<!--.*-->//g' "$md_file" | pandoc \
      --from markdown+yaml_metadata_block \
      --to typst \
      --wrap=none 2>/dev/null | sed -E \
         -e 's/^<[a-z][a-z0-9_-]*>$//g' \
         -e 's/\\\[#link\(<[^>]*>\)\[([^]]*)\]\\\]/\1/g'
}

# Extract audience values from YAML frontmatter (one per line)
extract_audiences() {
   local md_file="$1"

   # Read only the YAML frontmatter block, extract audience line
   sed -n '/^---$/,/^---$/p' "$md_file" | grep '^audience:' | head -1 \
      | sed 's/^audience: *//' \
      | sed 's/ *#.*//' \
      | sed 's/^\[//; s/\]$//' \
      | tr ',' '\n' \
      | tr '|' '\n' \
      | sed 's/^ *//; s/ *$//' \
      | grep -v '^$'
}

# Guess audience from file path when frontmatter is missing
guess_audience() {
   local md_file="$1"
   local rel_path="${md_file#"$DOCS_DIR"/}"

   case "$rel_path" in
      vision.md)
         # vision applies to all non-excluded audiences
         for aud in "${!AUDIENCE_DISPLAY[@]}"; do
            echo "$aud"
         done
         ;;
      user/*)       echo "pg-coder" ;;
      technical/spec/*|technical/plan/*|technical/compile-rules/*|technical/COMPILE-RULES.md)
                    echo "architect" ;;
      technical/ebnf/*|technical/edge-cases/*|technical/brainstorming/*)
                    echo "designer" ;;
      technical/*)  echo "developer" ;;
      audit/*)      ;; # ai-facing, excluded
      *)            echo "developer" ;;
   esac
}

# Get audiences for a file (frontmatter or guessed)
get_file_audiences() {
   local md_file="$1"
   local audiences
   audiences="$(extract_audiences "$md_file")"

   if [[ -z "$audiences" ]]; then
      guess_audience "$md_file"
   else
      echo "$audiences"
   fi
}

# Check if an audience is excluded
is_excluded_audience() {
   local aud="$1"
   for excl in "${EXCLUDED_AUDIENCES[@]}"; do
      [[ "$aud" == "$excl" ]] && return 0
   done
   return 1
}

# Check if a file matches a given audience filter
file_matches_audience() {
   local md_file="$1"
   local filter="$2"

   # No filter = include everything
   [[ -z "$filter" ]] && return 0

   get_file_audiences "$md_file" | grep -qx "$filter"
}

# --- PDF generation function ---

generate_pdf() {
   local output_pdf="$1"
   local audience_filter="$2"    # empty = all files
   local audience_label="$3"     # display name for title page, empty for monolithic

   local combined_typ="$BUILD_DIR/book.typ"

   # Counters
   local total=0 success=0 skipped=0 failed=0

   # Write typst header
   {
      echo '#import "/scripts/doc-template.typ": *'
      if [[ -n "$audience_label" ]]; then
         echo "#show: polyglot-book.with(audience: \"$audience_label\")"
         echo ""
         echo "#polyglot-title-page(audience: \"$audience_label\")"
      else
         echo '#show: polyglot-book'
         echo ''
         echo '#polyglot-title-page()'
      fi
      echo ''
      echo '// Table of contents'
      echo '#outline(title: "Table of Contents", depth: 2, indent: 1.5em)'
      echo '#pagebreak()'
   } > "$combined_typ"

   # Process each section
   for section in "${SECTIONS[@]}"; do
      local full_path="$DOCS_DIR/$section"

      # If targeting a subdirectory, skip sections outside it
      if [[ "$TARGET" != "$DOCS_DIR" ]]; then
         if [[ -f "$TARGET" ]]; then
            [[ "$full_path" != "$TARGET" ]] && continue
         elif [[ -d "$TARGET" ]]; then
            [[ "$full_path" != "$TARGET"* ]] && continue
         fi
      fi

      # Skip if section doesn't exist
      [[ ! -e "$full_path" ]] && continue

      local section_name="${SECTION_NAMES[$section]:-$section}"
      local section_has_files=false

      # Check if section has any matching files before adding part heading
      local section_content=""
      while IFS= read -r md_file; do
         [[ -z "$md_file" ]] && continue

         # Audience filter
         if ! file_matches_audience "$md_file" "$audience_filter"; then
            continue
         fi

         section_has_files=true
         break
      done < <(collect_section_files "$section")

      [[ "$section_has_files" == false ]] && continue

      info "Section: $section_name"

      # Add part heading
      cat >> "$combined_typ" <<PART

// ═══════════════════════════════════════════
// Part: $section_name
// ═══════════════════════════════════════════
#part-heading("$section_name")
PART

      # Process files in this section
      while IFS= read -r md_file; do
         [[ -z "$md_file" ]] && continue

         # Audience filter
         if ! file_matches_audience "$md_file" "$audience_filter"; then
            continue
         fi

         local rel_path="${md_file#"$DOCS_DIR"/}"
         ((total++)) || true

         local content
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

         cat >> "$combined_typ" <<DOC

// --- $rel_path ---
#doc-separator("$rel_path")
$content
DOC

         ((success++)) || true
         log "$rel_path"

      done < <(collect_section_files "$section")
   done

   # Compile
   info "Compiling Typst to PDF..."
   if typst compile --root "$REPO_ROOT" "$combined_typ" "$output_pdf" 2>&1; then
      local pdf_size
      pdf_size="$(du -h "$output_pdf" | cut -f1)"
      log "Output: $output_pdf ($pdf_size)"
      log "$success documents included, $skipped skipped, $failed failed (of $total total)"
   else
      fail "Typst compilation failed. Check $combined_typ for errors."
      return 1
   fi
}

# --- Main execution ---

if $BY_AUDIENCE; then
   # Discover all unique audiences
   declare -A ALL_AUDIENCES
   for section in "${SECTIONS[@]}"; do
      while IFS= read -r md_file; do
         [[ -z "$md_file" ]] && continue
         while IFS= read -r aud; do
            if [[ -n "$aud" ]] && ! is_excluded_audience "$aud"; then
               ALL_AUDIENCES["$aud"]=1
            fi
         done < <(get_file_audiences "$md_file")
      done < <(collect_section_files "$section")
   done

   # Determine which audiences to generate
   if [[ -n "$AUDIENCE_FILTER" ]]; then
      AUDIENCE_LIST=("$AUDIENCE_FILTER")
   else
      AUDIENCE_LIST=("${!ALL_AUDIENCES[@]}")
   fi

   mkdir -p "$REPO_ROOT/docs/pdf"

   info "Generating per-audience PDFs for: ${AUDIENCE_LIST[*]}"
   echo ""

   for aud in "${AUDIENCE_LIST[@]}"; do
      local_label="${AUDIENCE_DISPLAY[$aud]:-$aud}"
      local_output="$REPO_ROOT/docs/pdf/${aud}.pdf"

      echo ""
      info "════════════════════════════════════════"
      info "Audience: $local_label ($aud)"
      info "════════════════════════════════════════"

      rm -rf "$BUILD_DIR"
      mkdir -p "$BUILD_DIR"
      generate_pdf "$local_output" "$aud" "$local_label"
   done

   rm -rf "$BUILD_DIR"

   echo ""
   info "All per-audience PDFs generated in docs/pdf/"
   ls -lh "$REPO_ROOT/docs/pdf/"*.pdf 2>/dev/null | while read -r line; do
      log "$line"
   done
else
   # Monolithic mode (original behavior)
   rm -rf "$BUILD_DIR"
   mkdir -p "$BUILD_DIR"

   info "Building combined documentation PDF..."
   info "Source: $TARGET"
   echo ""

   generate_pdf "$OUTPUT_PDF" "" ""

   rm -rf "$BUILD_DIR"
fi
