// doc-template.typ — Polyglot /docs renderer
// This is not a book. It renders the contents of docs/ as-is.

// Pandoc 3.x compatibility
#let blockquote(body) = {
   block(
      inset: (left: 14pt, top: 8pt, bottom: 8pt, right: 8pt),
      stroke: (left: 3pt + luma(160)),
      fill: luma(248),
      width: 100%,
      body,
   )
}

#let horizontalrule = {
   v(0.5em)
   line(length: 100%, stroke: 0.5pt + luma(200))
   v(0.5em)
}

// Section heading — groups files by docs/ subdirectory, appears in TOC
#let section-heading(name) = {
   heading(level: 1, name)
}

// File path label — shows which file is being rendered
#let doc-separator(path) = {
   v(1em)
   line(length: 100%, stroke: 0.3pt + luma(210))
   block(
      width: 100%,
      fill: luma(235),
      inset: (x: 8pt, y: 5pt),
      radius: 2pt,
   )[
      #text(size: 8.5pt, fill: luma(80), font: "DejaVu Sans Mono")[docs/#path]
   ]
   v(0.4em)
}

// Frontmatter metadata — renders YAML key-value pairs
#let doc-metadata(pairs) = {
   block(
      width: 100%,
      inset: (x: 8pt, y: 4pt),
      stroke: (left: 2pt + luma(200)),
      below: 0.6em,
   )[
      #set text(size: 8pt, fill: luma(100), font: "DejaVu Sans Mono")
      #for (key, val) in pairs [
         #key: #val \
      ]
   ]
}

// Cover page — states what this document is
#let cover-page(audience: none) = {
   set page(header: none, footer: none)
   v(2fr)
   align(center)[
      #text(size: 28pt, weight: "bold")[Polyglot]
      #v(0.5em)
      #text(size: 14pt, fill: luma(80))[
         Rendered contents of #raw("docs/")
      ]
      #if audience != none {
         v(0.8em)
         text(size: 12pt, fill: luma(100))[
            Filtered for: #audience
         ]
      }
      #v(2em)
      #line(length: 25%, stroke: 1pt + luma(160))
      #v(1.5em)
      #text(size: 9.5pt, fill: luma(140))[
         Auto-generated #datetime.today().display("[month repr:long] [day], [year]") \
         This PDF is a plain rendering of the project's markdown documentation. \
         Each file is shown with its source path.
      ]
   ]
   v(3fr)
   pagebreak()
}

// Main document layout
#let polyglot-book(audience: none, doc) = {
   set page(
      paper: "a4",
      margin: (top: 2.5cm, bottom: 2.5cm, left: 2.2cm, right: 2.2cm),
      header: context {
         if counter(page).get().first() > 2 {
            set text(size: 8pt, fill: luma(140))
            [docs/ rendering]
            if audience != none [ — #audience]
            h(1fr)
         }
      },
      footer: context {
         if counter(page).get().first() > 1 {
            set text(size: 8pt, fill: luma(140))
            h(1fr)
            counter(page).display("1")
            h(1fr)
         }
      },
   )

   // Typography
   set text(font: "Libertinus Serif", size: 10.5pt)
   set par(justify: true, leading: 0.65em)

   // Headings
   show heading.where(level: 1): it => {
      set text(size: 16pt, weight: "bold")
      v(0.6em)
      it
      v(0.2em)
      line(length: 100%, stroke: 0.5pt + luma(200))
      v(0.4em)
   }
   show heading.where(level: 2): it => {
      set text(size: 13pt, weight: "bold")
      v(0.4em)
      it
      v(0.2em)
   }
   show heading.where(level: 3): it => {
      set text(size: 11pt, weight: "bold")
      v(0.3em)
      it
      v(0.2em)
   }
   show heading.where(level: 4): it => {
      set text(size: 10.5pt, weight: "bold")
      v(0.2em)
      it
      v(0.1em)
   }

   // Code blocks
   show raw.where(block: true): it => {
      set text(font: "DejaVu Sans Mono", size: 8.5pt)
      block(
         fill: luma(246),
         inset: 8pt,
         radius: 3pt,
         width: 100%,
         breakable: true,
         it,
      )
   }

   // Inline code
   show raw.where(block: false): it => {
      set text(font: "DejaVu Sans Mono", size: 9pt)
      box(
         fill: luma(240),
         inset: (x: 3pt, y: 0pt),
         outset: (y: 3pt),
         radius: 2pt,
         it,
      )
   }

   // Links
   show link: set text(fill: rgb("#2563eb"))

   // Tables
   set table(
      stroke: 0.5pt + luma(200),
      inset: 6pt,
   )
   show table.cell.where(y: 0): set text(weight: "bold", size: 9.5pt)

   doc
}
