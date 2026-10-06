# Documentation Strategy

How the book (`examples/book-ssr`) is structured and how its pages are written. How it looks and what it is built from
(leptonic pieces, design tokens, CSS conventions) is in `examples/book-ssr/STYLE_GUIDE.md`. Open work on the book is
tracked in the root `PLAN.md` (section "Book").

## Goals

1. **Concept-first navigation.** Users think "I need a button", not "I need the `use_button` hook". Pages are
   organized around concepts, not implementation layers.
2. **Progressive disclosure.** A concept overview is enough to get started. Layer pages give full detail. Nobody has
   to read every layer page.
3. **Every fact in one place.** Conceptual explanations (what a button is, its accessibility semantics) live on the
   concept page. Other pages link to it instead of repeating it.
4. **Self-contained pages.** A reader landing on `/doc/button/hook` from a search engine must not feel lost: the
   first paragraph says what the page is about and links to the concept overview.
5. **Living examples.** Every interactive capability gets a working demo.
6. **Accurate.** Pages document the current API. Props tables, types and defaults are checked against the source.

## Page Types

| Type                | Example URL                      | Kind (`nav::PageKind`) |
|---------------------|----------------------------------|------------------------|
| Guide               | `/doc/installation`              | `Guide`                |
| Domain / category   | `/doc/interactions`, `/doc/input`| `Domain`               |
| Concept overview    | `/doc/button`                    | `Concept`              |
| Hook page           | `/doc/button/hook`               | `Hook`                 |
| Atom page           | `/doc/button/atom`               | `Atom`                 |
| Component page      | `/doc/button/component`          | `Component`            |

Sections listed below appear in this order. Leave out sections that don't apply. Section ids are the slug of the
title (`"Hooks Used"` → `#hooks-used`).

**Concept overview** — a concept implemented at two or more layers, or a family of related hooks.

- Introduction (1–2 paragraphs, no code)
- When to Use — comparison with similar concepts (`DocTable`)
- Choose Your Layer — what each layer gives you, linking the layer pages (`DocTable`)
- Quick Start — the most common use case as a demo with its source shown (`source_open`)
- Accessibility — ARIA pattern and keyboard interaction (`KeyboardTable`)

Not here: full API reference, CSS variables, hook internals.

**Domain overview** — a group of hooks users discover together without a shared atom or component (Interactions,
Focus, Overlays); **category overview** — a sidebar group (Input, Layout, ...).

- Introduction — what the domain covers and why its members belong together
- Pages — `<SectionMembers overview=../>`, generated from the section's entries in `nav.rs` (never a hand-written
  member list)
- Relationships / Decision Guide — how the members compose, or how to choose between similar ones
- Quick Start — the most common member (domains only)

**Hook page**

- Introduction — one sentence, link to the concept or domain overview; then `<ReactAria hook="useX"/>`
- Input — `ApiTable kind=ApiKind::Input`
- Return — `ApiTable kind=ApiKind::Return`
- Example — minimal setup and attribute spreading
- Demo
- Further sections for options and behavior worth explaining
- Keyboard — `KeyboardTable`
- See Also

A page documenting several hooks (e.g. the slider hooks) gives each hook its own section titled with the exact hook
name (`use_slider_thumb`, not "Per-thumb behavior"), containing its Input, Return and Example as subsections. Shared
Keyboard and See Also sections follow. Give the repeated subsections ids prefixed with the hook
(`<Section title="Input" id="use-option-input">`), so that every hook's subsections have predictable anchors.

**Atom page**

- Introduction — one sentence, link to the concept overview
- Hooks Used
- Props — `ApiTable kind=ApiKind::Props`
- Example, Demo
- Data Attributes — `ApiTable kind=ApiKind::DataAttributes`
- Styling — targeting the data attributes from CSS
- Composition, See Also

**Component page**

- Introduction — one sentence, link to the concept overview; then the basic demo
- Props — `ApiTable kind=ApiKind::Props`
- One section per feature (variants, colors, sizes, ...) with a demo each
- Styling — `<CssVariables prefix="--x-" scss=theme_scss!("x")/>`
- Recipes, See Also

**Standalone pages** (`/doc/hooks/use-label`, `/doc/components/toast`) exist at one layer only. They follow the
structure of their layer, but their introduction explains the concept, as there is no overview to link to. A
standalone item becomes a concept once it gains a second layer, or once three related standalone hooks exist.

## Writing Pages

### The page kit

Pages are built from the components in `src/kit/` (import with `use crate::{kit::*, routes};`). Never write raw
headings, anchor links, tables of contents or API tables.

```rust
#[component]
pub fn PageUseButton() -> impl IntoView {
    view! {
        <DocPage title="use_button">
            <p>"The "<Code inline=true>"use_button"</Code>" hook makes ... See the "
               <Link href=routes::doc::Button.materialize()>"Button overview"</Link>"."</p>
            <ReactAria hook="useButton"/>

            <Section title="Input">
                <ApiTable kind=ApiKind::Input>
                    <ApiRow name="disabled" ty="Signal<bool>" default="false">"Whether the button is disabled."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Demo">
                <Demo description="Press count tracking" source=include_str!("demos/button_basic.rs")>
                    <BasicButtonDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Enter / Space">"Press the button."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
```

| Component                         | Purpose                                                                                     |
|-----------------------------------|---------------------------------------------------------------------------------------------|
| `DocPage title`                   | The page: `<h1>`, the `<article>` exported as Markdown, the generated table of contents.   |
| `Section title [id]`              | A titled section. Nesting gives `<h2>`, `<h3>`, ... All headings land in the TOC.          |
| `ApiTable kind` + `ApiRow`        | Input, Return, Fields, Props, Data Attributes, CSS variable tables. `name` may list several. |
| `ApiTable of`                     | The documented struct (Input/Return/Fields) or component (Props). A test checks the rows.  |
| `KeyboardTable` + `KeyRow keys`   | Keyboard interaction. `keys="Shift + Tab"`, alternatives as `"Enter / Space"`.             |
| `DocTable headers`                | Any other table; rows are plain `<tr><td>`.                                                 |
| `Demo description source`         | Frame for an interactive demo with collapsible source. `source_open` expands it.           |
| `ReactAria hook`                  | "Based on react-aria's useX." with a link.                                                  |
| `ReactAriaSource path`            | The same for parts without a react-aria docs page, linking the source file.                 |
| `SectionMembers overview`         | Table of a navigation section's pages (name, layers, summary), generated from `nav.rs`.    |
| `SeeAlso`                         | The closing "See Also" section; children are `<li>` links.                                 |
| `CssVariables prefix scss`        | CSS variables of a theme stylesheet, generated from `theme_scss!("<component>")`.          |

### Prose

- Write plain, natural sentences in the present tense, addressing the reader as "you". Explain why, not only what.
- Keep introductions to one or two sentences on layer pages; the concept page holds the explanation.
- Identifiers are inline code: `<Code inline=true>"use_button"</Code>`. Link a hook, atom or component the first time
  a section mentions it.
- Link with the generated routes (`routes::doc::button::Hook.materialize()`), never with string paths.
- Use real typography: `\u{2019}` for apostrophes in contractions, `\u{2014}` for dashes, `\u{2026}` for ellipses.
- Never show deviations from react-aria: no "Deviations from react-aria" sections, no comparisons ("unlike
  react-aria", "as in react-aria", "react-aria calls it ..."). Describe leptonic's API and behavior on its own terms.
  Deviations are documented in the library source (`API DIFFERENCES` comments), not in the book. The only references
  to react-aria are `ReactAria` / `ReactAriaSource` ("Based on react-aria's useX") and provenance in the getting
  started pages.
- Tag every Input, Return, Fields and Props table with `of="<Item>"` (qualified, e.g. `atoms::button::Button`, where
  the name is ambiguous). The unit test `kit::api_check` fails for untagged tables and when a table's rows differ from
  the item's public fields or props, so the book can't silently drift from the library. Document function arguments
  and methods in prose or a `DocTable`, not in an `ApiTable`.
- Don't document react-aria legacy or deprecated API (e.g. `onClick` on press hooks).

### Demos

- One component per demo in a `demos/` module next to the page, shown with `Demo` and `source=include_str!(..)`.
  The source must be meaningful on its own: readers copy it.
- Don't repeat a demo's code as a separate snippet; use `source_open` where the code matters as much as the demo.
  A separate snippet is fine when it shows something different (a minimal setup, a configuration variant).
- Demos show their state ("Pressed 3 times", event logs capped with `ringbuf::HeapRb` at 50 entries) and include a
  disabled toggle where the concept supports disabling.
- Demos are keyboard accessible and work in light and dark theme.
- Demos are built with leptonic and styled with the book's tokens: see `examples/book-ssr/STYLE_GUIDE.md` (which
  leptonic piece to use, design tokens, demo stylesheets).
- No blocking browser dialogs (`alert`) in demos.

### Code conventions

- Book code follows the library's quality bar: zero clippy findings (`all` + `pedantic`), SSR-safe browser access
  (`leptos_use::use_window()`), no `unwrap()` on fallible operations.
- Imports: `use crate::{kit::*, routes};` for pages; leptonic through its preludes.

## Navigation

`src/nav.rs` defines all pages: the sidebar sections, their entries, the tabs of concept pages, and pages reached only
through other pages. It drives the sidebar, the concept tabs (`ConceptLayout`) and the page kinds of the Markdown
export. Adding a page means adding its route in `src/routes.rs` and its entry in `src/nav.rs`; a page missing from the
navigation is logged as a warning at startup. Every entry has a one-line `summary` (no trailing period), shown in
the `SectionMembers` table of its section's overview page; a unit test enforces it.

Sidebar conventions: domain sections list their member hooks and atoms. Category sections list concepts first, then
standalone pages with a layer badge, each group alphabetically.

### URL structure

```
/doc/button                    concept overview
/doc/button/hook               hook page (also /atom, /component)
/doc/interactions              domain overview
/doc/interactions/use-press    member of a domain
/doc/hooks/use-label           standalone hook (also /doc/components/...)
```

When a page moves, add a `<Redirect>` route for its old URL.

## Verification

- `cargo test --features ssr --lib` in `examples/book-ssr`: unit tests of the kit, the navigation (every entry has
  a summary) and `kit::api_check` (API tables against the library source).
- `just book-browser-test` (`examples/book-ssr/tests/`): visits every page of the navigation and fails on page
  errors, demos unreadable in the dark theme, internal links or anchors that don't resolve, pages wider than a 390px
  screen, and pages missing from the Markdown export. Add checks there, in Rust, in the style of the library's browser
  tests.

## Markdown Export (LLM-native docs)

Every `/doc/...` page is also served as Markdown at `/doc/....md`, and `/doc/llm-index.md` lists all pages with
their sections. Content is defined once, in the Leptos page; the Markdown is derived from it (`src/markdown/`):

- The middleware renders the page through SSR, converts its `<article>` with `htmd` and caches the result. All pages
  are converted at startup, which also feeds the book's search.
- Frontmatter: title (`<h1>`), layer (page kind from `nav.rs`), path, description (first paragraph) and related pages
  (the See Also links).
- Demos become `*[Interactive Demo: <description>]*` followed by their Rust source.
- Links to other pages point to their Markdown export.
- Every hook, atom or component documented on a page must appear by name in a section title, or it is invisible to
  the index.
- Responses carry an `ETag` and `Cache-Control: public, max-age=3600, must-revalidate`.
