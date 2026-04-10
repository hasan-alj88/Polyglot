// doc-template.typ — Polyglot documentation book template

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

// Part heading — used between major sections
#let part-heading(name) = {
   pagebreak(weak: true)
   v(1fr)
   align(center)[
      #text(size: 12pt, fill: luma(100), tracking: 2pt, weight: "regular")[PART]
      #v(0.5em)
      #text(size: 28pt, weight: "bold")[#name]
      #v(0.5em)
      #line(length: 40%, stroke: 1pt + luma(150))
   ]
   v(1fr)
   pagebreak()
}

// Document separator — source path + page break between docs
#let doc-separator(path) = {
   pagebreak(weak: true)
   block(
      width: 100%,
      inset: (bottom: 6pt),
   )[
      #text(size: 8pt, fill: luma(150), font: "DejaVu Sans Mono")[#path]
   ]
}

// Title page
#let polyglot-title-page(audience: none) = {
   set page(header: none, footer: none)
   v(2fr)
   align(center)[
      #text(size: 36pt, weight: "bold")[Polyglot]
      #v(0.3em)
      #text(size: 16pt, fill: luma(80))[Documentation]
      #if audience != none {
         v(1em)
         text(size: 13pt, fill: rgb("#2563eb"), weight: "medium")[
            #upper(audience) Edition
         ]
      }
      #v(2em)
      #line(length: 30%, stroke: 1.5pt + luma(120))
      #v(2em)
      #text(size: 11pt, fill: luma(100))[
         Async-centric programming language and platform
      ]
      #v(1em)
      #text(size: 10pt, fill: luma(140))[
         Generated #datetime.today().display("[month repr:long] [day], [year]")
      ]
   ]
   v(3fr)
   pagebreak()
}

// Main book layout
#let polyglot-book(audience: none, doc) = {
   set page(
      paper: "a4",
      margin: (top: 2.5cm, bottom: 2.5cm, left: 2.2cm, right: 2.2cm),
      header: context {
         if counter(page).get().first() > 2 {
            set text(size: 8.5pt, fill: luma(130))
            if audience != none [Polyglot Documentation (#audience)] else [Polyglot Documentation]
            h(1fr)
            context {
               let headings = query(selector(heading.where(level: 1)).before(here()))
               if headings.len() > 0 {
                  headings.last().body
               }
            }
         }
      },
      footer: context {
         if counter(page).get().first() > 1 {
            set text(size: 8.5pt, fill: luma(130))
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
      set text(size: 18pt, weight: "bold")
      v(0.8em)
      it
      v(0.2em)
      line(length: 100%, stroke: 0.5pt + luma(180))
      v(0.5em)
   }
   show heading.where(level: 2): it => {
      set text(size: 13pt, weight: "bold")
      v(0.5em)
      it
      v(0.3em)
   }
   show heading.where(level: 3): it => {
      set text(size: 11pt, weight: "bold")
      v(0.3em)
      it
      v(0.2em)
   }

   // Code blocks
   show raw.where(block: true): it => {
      set text(font: "DejaVu Sans Mono", size: 8.5pt)
      block(
         fill: luma(246),
         inset: 8pt,
         radius: 3pt,
         width: 100%,
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
