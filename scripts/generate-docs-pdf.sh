#!/usr/bin/env bash
# generate-docs-pdf.sh — Render docs/ markdown into PDF(s) via Pandoc + Typst
#
# Usage:
#   ./scripts/generate-docs-pdf.sh                    # full docs rendering
#   ./scripts/generate-docs-pdf.sh docs/user           # only user docs
#   ./scripts/generate-docs-pdf.sh --by-audience       # one PDF per audience in docs/pdf/
#   ./scripts/generate-docs-pdf.sh --audience=architect # single audience PDF
#
# Requirements: pandoc (>=3.x), typst (>=0.14), mmdc (mermaid-cli, optional)
# Output:
#   Default:       Aljam3-Documentation.pdf in repo root
#   --by-audience: docs/pdf/{audience}.pdf for each audience

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DOCS_DIR="$REPO_ROOT/docs"
TEMPLATE="$REPO_ROOT/scripts/doc-template.typ"
BUILD_DIR="$REPO_ROOT/.docs-build"
MERMAID_DIR="$BUILD_DIR/mermaid"
OUTPUT_PDF="$REPO_ROOT/Aljam3-Documentation.pdf"

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

HAS_MERMAID=false
if command -v mmdc &>/dev/null; then
   HAS_MERMAID=true
   info "mermaid-cli found — diagrams will be rendered"
else
   warn "mmdc not found — mermaid blocks will render as code"
fi

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
   "user/aj3lib"
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

# Section display names
declare -A SECTION_NAMES=(
   ["vision.md"]="Vision"
   ["user/SPEC-INDEX.md"]="Language Reference Index"
   ["user/syntax"]="Syntax Foundations"
   ["user/concepts"]="Core Concepts"
   ["user/aj3lib"]="Standard Library"
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
   ["pg-coder"]="Aljam3 Coder"
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

# Pre-process mermaid blocks in a markdown file:
#   Extracts ```mermaid ... ``` blocks, renders to SVG, replaces with image refs.
#   Returns the processed markdown content on stdout.
preprocess_mermaid() {
   local md_file="$1"
   local file_id="$2"   # unique id for naming output images

   if [[ "$HAS_MERMAID" != true ]]; then
      cat "$md_file"
      return
   fi

   local content
   content="$(<"$md_file")"

   # Check if file contains mermaid blocks at all
   if ! grep -q '```mermaid' <<< "$content"; then
      echo "$content"
      return
   fi

   local diagram_idx=0
   local in_mermaid=false
   local mermaid_buf=""
   local result=""

   while IFS= read -r line || [[ -n "$line" ]]; do
      if [[ "$in_mermaid" == true ]]; then
         if [[ "$line" == '```' ]]; then
            # End of mermaid block — render it
            in_mermaid=false
            ((diagram_idx++)) || true
            local png_file="$MERMAID_DIR/${file_id}_${diagram_idx}.png"

            if echo "$mermaid_buf" | mmdc -i /dev/stdin -o "$png_file" -b transparent -s 2 >/dev/null 2>&1; then
               # Use path relative to repo root for Typst --root
               local rel_png="${png_file#"$REPO_ROOT"/}"
               result+=$'\n'"![Diagram](/${rel_png})"$'\n'
            else
               # Fallback: keep as code block
               result+=$'\n'"\`\`\`"$'\n'"$mermaid_buf"$'\n'"\`\`\`"$'\n'
            fi
            mermaid_buf=""
         else
            mermaid_buf+="$line"$'\n'
         fi
      elif [[ "$line" == '```mermaid' ]]; then
         in_mermaid=true
         mermaid_buf=""
      else
         result+="$line"$'\n'
      fi
   done <<< "$content"

   echo "$result"
}

# Extract YAML frontmatter as key: value lines (empty if none)
extract_frontmatter() {
   local md_file="$1"
   sed -n '1{/^---$/!q}; 1,/^---$/{/^---$/d;p}' "$md_file"
}

# Convert one markdown file to typst content (no template wrapper)
convert_one() {
   local md_file="$1"
   local file_id="$2"

   local preprocessed
   preprocessed="$(preprocess_mermaid "$md_file" "$file_id")" || true

   echo "$preprocessed" \
      | sed 's/<!--.*-->//g' \
      | pandoc \
         --from markdown+yaml_metadata_block \
         --to typst \
         --wrap=none 2>/dev/null \
      | sed -E \
         -e 's/^<[a-z][a-z0-9_-]*>$//g' \
         -e 's/\\\[#link\(<[^>]*>\)\[([^]]*)\]\\\]/\1/g' \
         -e 's/#cite\("[^"]*"\)//g'
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
   local audience_label="$3"     # display name for cover page, empty for full render

   local combined_typ="$BUILD_DIR/book.typ"
   mkdir -p "$MERMAID_DIR"

   # Counters
   local total=0 success=0 skipped=0 failed=0
   local file_counter=0

   # Write typst header
   {
      echo '#import "/scripts/doc-template.typ": *'
      if [[ -n "$audience_label" ]]; then
         echo "#show: aljam3-book.with(audience: \"$audience_label\")"
         echo ""
         echo "#cover-page(audience: \"$audience_label\")"
      else
         echo '#show: aljam3-book'
         echo ''
         echo '#cover-page()'
      fi
      echo ''
      echo '#outline(title: "Contents", depth: 2, indent: 1.5em)'
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

      # Check if section has any matching files before adding heading
      local section_has_files=false
      while IFS= read -r md_file; do
         [[ -z "$md_file" ]] && continue
         if file_matches_audience "$md_file" "$audience_filter"; then
            section_has_files=true
            break
         fi
      done < <(collect_section_files "$section")

      [[ "$section_has_files" == false ]] && continue

      # Section heading
      cat >> "$combined_typ" <<SEC

// ═══ Section: $section_name ═══
#section-heading("$section_name")
SEC
      info "Section: $section_name"

      # Process files in this section
      while IFS= read -r md_file; do
         [[ -z "$md_file" ]] && continue

         # Audience filter
         if ! file_matches_audience "$md_file" "$audience_filter"; then
            continue
         fi

         local rel_path="${md_file#"$DOCS_DIR"/}"
         ((total++)) || true
         ((file_counter++)) || true

         # Extract frontmatter for metadata display
         local fm
         fm="$(extract_frontmatter "$md_file")"

         local content
         content="$(convert_one "$md_file" "f${file_counter}" 2>/dev/null)" || {
            ((failed++)) || true
            fail "$rel_path (pandoc error)"
            continue
         }

         if [[ -z "$content" ]]; then
            ((skipped++)) || true
            warn "$rel_path (empty)"
            continue
         fi

         # Emit file header with path and frontmatter
         cat >> "$combined_typ" <<DOC

// --- $rel_path ---
#doc-separator("$rel_path")
DOC
         # Add frontmatter block if present
         if [[ -n "$fm" ]]; then
            {
               echo '#doc-metadata(('
               while IFS= read -r fm_line; do
                  local key="${fm_line%%:*}"
                  local val="${fm_line#*: }"
                  # Escape quotes in values
                  val="${val//\"/\\\"}"
                  echo "  (\"$key\", \"$val\"),"
               done <<< "$fm"
               echo '))'
            } >> "$combined_typ"
         fi

         cat >> "$combined_typ" <<DOC
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
      log "$success files rendered, $skipped skipped, $failed failed (of $total total)"
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
   # Full render mode
   rm -rf "$BUILD_DIR"
   mkdir -p "$BUILD_DIR"

   info "Rendering docs/ to PDF..."
   info "Source: $TARGET"
   echo ""

   generate_pdf "$OUTPUT_PDF" "" ""

   rm -rf "$BUILD_DIR"
fi
