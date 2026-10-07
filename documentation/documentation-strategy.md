# Documentation Strategy

How the book (`examples/book-ssr`) is structured and how its pages are written. How it looks and what it is built from
(leptonic pieces, design tokens, CSS conventions) is in `examples/book-ssr/STYLE_GUIDE.md`. Open work on the book is
tracked in the root `PLAN.md` (section "Book").

## Goals

1. **Concept-first navigation.** Users think "I need a button", not "I need the `use_button` hook". The sidebar lists
   concepts; their layers are tabs of the concept. Behavior shared by many concepts is listed apart, as building
   blocks.
2. **Progressive disclosure.** A concept overview is enough to get started. Layer pages give full detail. Nobody has
   to read every layer page.
3. **Every fact in one place.** Explanations (what a button is, its accessibility semantics) live on the concept
   overview. Other pages link to it instead of repeating it.
4. **Self-contained pages.** A reader landing on `/doc/button/hook` from a search engine must not feel lost: the
   first paragraph says what the page is about and links to the concept overview.
5. **Living examples.** Every interactive capability gets a working demo.
6. **Accurate.** Pages document the current API. Props tables, types and defaults are checked against the source.
7. **A reason for every place.** Where a page lives, what its entry is called and which marker it carries follow
   from the rules under "Navigation", and `nav.rs` enforces them. Nothing is placed by taste.

## Terminology

Leptonic is two layers, hooks and atoms, plus an optional CSS theme for the atoms (the user's decision, 2026-10-07:
the styled components layer is removed). The book gives each meaning its own word and uses it consistently, in prose,
page titles, the sidebar and the code of the book (`nav.rs`, the kit).

- **Concept**: a UI element an app places on the page (Button, Table, Toolbar), documented on one set of pages
  across its layers. The sidebar part "Concepts" lists them. Never "component" or "widget".
- **Layer**: hook or atom, i.e. how much of a concept leptonic implements for you.
- **Leptos component**: a `#[component]` function. Atoms are Leptos components; say so only where that is the point
  ("an atom is a Leptos component rendering one element"). Never use the bare word "component" for a leptonic piece:
  leptonic has no styled components (the changelog names the removed ones).
- **Atom theme**: the optional stylesheet `leptonic-atoms` (`leptonic-theme/scss/leptonic-atoms.scss`) styling the
  atoms by their default classes and data attributes. The book never loads it; pages mention it, never rely on it.
- **Recipe**: markup and CSS for a part without behavior that leptonic has no piece for (card, stack, app bar,
  alert, drawer), shown on the overview of its group.
- **ARIA pattern**: the WAI-ARIA Authoring Practices pattern a concept implements (listbox, menu button,
  disclosure). Never "ARIA component".
- **Building block**: a hook, atom or utility that gives an element one behavior that many concepts share (`use_press`,
  `FocusScope`). The sidebar part "Building blocks" lists them, grouped into areas (Interactions, Focus, ...).

## Page Types

| Type                    | Example URL                                   | Kind (`nav::PageKind`)    |
|-------------------------|-----------------------------------------------|---------------------------|
| Guide                   | `/doc/installation`, `/doc/event-propagation` | `Guide`                   |
| Group overview          | `/doc/fields`, `/doc/interactions`            | `Overview`                |
| Concept overview        | `/doc/button`                                 | `Concept`                 |
| Layer page of a concept | `/doc/button/hook` (also `/atom`)             | `Hook`, `Atom`            |
| Single-layer concept    | `/doc/tree`, `/doc/kbd`                       | its layer's kind          |
| Building block          | `/doc/interactions/use-press`                 | `Hook`, `Atom`, `Utility` |

Layer pages are titled like their tab: "Checkbox Hooks", "Checkbox Atoms". So is the single page of a single-layer
concept ("Tree Hooks", "Kbd Atoms"). A hook page documenting a single hook is titled with
the hook's name ("use_button"), so that search finds it. Building-block pages are titled with their identifier.

Sections listed below appear in this order. Leave out sections that don't apply. Section ids are the slug of the
title (`"Hooks Used"` → `#hooks-used`).

**Concept overview** — a concept implemented at both layers. Its layer pages are its tabs.

- Introduction (1–2 paragraphs, no code)
- When to Use — comparison with similar concepts (`DocTable`)
- Choose Your Layer — what each layer gives you, linking the layer pages (`DocTable`)
- Quick Start — the most common use case as a demo with its source shown (`source_open`)
- Accessibility — ARIA pattern and keyboard interaction (`KeyboardTable`)

Not here: full API reference, styling, hook internals.

**Group overview** — a concept group (Buttons, Fields, ...) or a building-block area (Interactions, Focus, ...).

- Introduction — what the group covers and why its members belong together
- Pages — `<SectionMembers overview=../>`, generated from the group's entries in `nav.rs` (never a hand-written
  member list)
- Relationships / Decision Guide — how the members compose, or how to choose between similar ones
- Quick Start — the most common member (building-block areas, and concept groups whose members are usually
  combined, like Color and Date & Time)
- Recipes — one subsection per part without behavior that belongs to the group (Content & Layout: app bar, stack,
  grid layout, card, skeleton, typography, icons, rich content; Status: alert): what it is for, the semantic markup
  and its accessibility (landmarks, headings, `role`), and a CSS snippet with stand-in tokens or a demo. The old
  URLs of such parts redirect to the recipe's anchor.

**Hook page**

- Introduction — one sentence, link to the concept overview or building-block area; then `<ReactAria hook="useX"/>`
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
- Styling — how an app styles the atom, since atoms bring no styles: its default class (`leptonic-<AtomName>`, one
  per atom of the family), the data attributes to select, the markup the app renders inside it (decorative parts
  such as a checkbox's box or a select's caret, `aria-hidden`), and the book's own CSS for it as the example
  (stand-in tokens, see `STYLE_GUIDE.md`). Variants of a concept (sizes, a secondary button) are data attributes or
  classes the app sets (`attr:data-variant="secondary"`). It ends with one sentence on the optional atom theme, for
  apps that don't want to start from scratch: `@use "leptonic/leptonic-atoms";`. Animation (`data-entering`,
  `data-exiting`) is a subsection.
- Composition, See Also

**Pages documenting several hooks or atoms** (layer pages with a plural tab) keep the order of their
layer's outline, with the per-item reference in the middle: Introduction, Hooks Used (atoms), Example, Demo, then
one section per item titled with its identifier and holding its Props (or Input and Return) as subsections with
prefixed ids (`<Section title="Props" id="slider-thumb-props">`), then Data Attributes, Styling, Composition and
See Also. Keyboard and Accessibility sections belong on the concept overview, not on layer pages.

**Single-layer concepts** (`/doc/tree`, `/doc/kbd`) have one page and no tabs. It follows the structure of its
layer, but its introduction explains the concept, as there is no overview to link to. A concept gets an overview and
tabs once it gains a second layer.

**Building-block pages** follow the structure of their layer; their introduction explains the behavior and links
the area overview. Closely related hooks share one page (`use_enter_animation` and `use_exit_animation`), each with
its own section.

## Writing Pages

### The page kit

Pages are built from the Leptos components in `src/kit/` (import with `use crate::{kit::*, routes};`). Never write raw
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

| Kit piece                       | Purpose                                                                                      |
|---------------------------------|----------------------------------------------------------------------------------------------|
| `DocPage title`                 | The page: `<h1>`, the `<article>` exported as Markdown, the generated table of contents.     |
| `Section title [id]`            | A titled section. Nesting gives `<h2>`, `<h3>`, ... All headings land in the TOC.            |
| `ApiTable kind` + `ApiRow`      | Input, Return, Fields, Props and Data Attributes tables. `name` may list several.            |
| `ApiTable of`                   | The documented struct (Input/Return/Fields) or atom (Props). A test checks the rows.         |
| `KeyboardTable` + `KeyRow keys` | Keyboard interaction. `keys="Shift + Tab"`, alternatives as `"Enter / Space"`.               |
| `DocTable headers`              | Any other table; rows are plain `<tr><td>`.                                                  |
| `Demo description source`       | Frame for an interactive demo with collapsible source. `source_open` expands it.             |
| `ReactAria hook`                | "Based on react-aria's useX." with a link.                                                   |
| `ReactAriaSource path`          | The same for parts without a react-aria docs page, linking the source file.                  |
| `SectionMembers overview`       | Table of a navigation section's pages (name, layers, summary), generated from `nav.rs`.      |
| `SeeAlso`                       | The closing "See Also" section; children are `<li>` links.                                   |

### Prose

- Every page except group overviews and guides ends with `SeeAlso`: the concept overview, the sibling layers, and
  closely related concepts or building blocks.
- Link texts of layer pages are their titles ("Slider Hooks", "Slider Atoms"), in layer tables, See Also and prose
  alike.
- In-page links use leptonic's `AnchorLink` (`<AnchorLink href="#marks">"Marks"</AnchorLink>`), never a raw `<a>`.
- Keys are always `Keys`/`KeyRow`. A shortcut with the platform's primary modifier names both:
  `<Keys keys="Control + A"/>" (" <Keys keys="Meta + A"/>" on macOS)"` — leptonic matches `Command` on Apple devices.
- Code examples compile against the current API: include imports that aren't in the preludes, and never use book
  helpers in them.
- Input and Props tables fill the Default column from the `Default` implementation (for inputs without one: the value
  a caller writes when they don't need the field) and mark required fields with "Required." in the description.
  Inputs have no constructors: samples write them as struct literals with every field named, or with
  `..Default::default()` where the type implements `Default`.
- A library gap is never described as intended behavior. Say it in one sentence where it affects the reader ("The
  arrow keys currently move the thumb by one pixel."), and record it in `PLAN.md`.

- Write plain, natural sentences in the present tense, addressing the reader as "you". Explain why, not only what.
- Keep introductions to one or two sentences on layer pages; the concept overview holds the explanation.
- Identifiers are inline code: `<Code inline=true>"use_button"</Code>`. Link a hook or atom the first time a section
  mentions it.
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

- One Leptos component per demo in a `demos/` module next to the page, shown with `Demo` and `source=include_str!(..)`.
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

`src/nav.rs` defines all pages: the sidebar parts, their groups and entries, and the tabs of concepts. It drives the
sidebar, the concept tabs, the member tables of group overviews and the page kinds of the Markdown export. Adding a
page means adding its route in `src/routes.rs` and its entry in `src/nav.rs`; a page missing from the navigation is
logged as a warning at startup. Every entry has a one-line `summary` (no trailing period), shown in the
`SectionMembers` table of its group's overview page; a unit test enforces it.

### Sidebar parts

A reader arrives with one of two questions: "which UI element do I need?" or "how do I give my own element a
behavior?". The sidebar answers each in its own part, framed by the guides:

1. **Getting started**: what leptonic is, installing it, the changelog.
2. **Guides**: topics every page builds on (the layers and styling atoms, event propagation, classes and styles,
   themes and the atom theme, forms and validation, SSR, accessibility).
3. **Concepts**: every concept, in groups by purpose: Buttons, Fields, Pickers, Collections, Date & Time, Color,
   Overlays, Navigation, Status, Content & Layout. The groups follow react-aria-components' catalog, so readers
   who know it find things where they expect them.
4. **Building blocks**: hooks, atoms and utilities shared by many concepts, in areas by behavior: Interactions,
   Focus, Overlay Behavior, Collection State, Drag & Drop, Animation, Screen Readers, Utilities.

### Placement rule

An element the app uses directly as a UI element (usually with an ARIA pattern of its own) is a **concept**. A hook or
atom that other concepts are built from is a **building block**. `use_spin_button` has an ARIA role but is a part of
Number Field and date segments, so it is a building block.

A concept with many hooks is one of two cases:

- The hooks render **parts of one concept** (`use_table`, `use_table_row`, `use_table_cell`, ...): they are sections
  of the concept's Hooks tab, never sidebar entries. Each section is titled with the exact hook name, so search, the
  table of contents and the Markdown index find every hook.
- The hooks render **separate concepts of one family** (color area, color slider, color wheel, ...): each is its own
  concept, and the family is a concept group (Color).

### Entries and markers

- Concept entries are named by the concept ("Toolbar", "Text Field"), never by an identifier. Each shows the layers it
  has as markers (`H A`, an absent layer dimmed).
- Building-block entries are named by their identifier (`use_press`, `FocusScope`) and carry the badge of their
  kind (hook, atom, utility). Every entry has a marker or a badge; whether it does never depends on its
  section.
- Entries of concept groups are sorted alphabetically. Guides are in reading order. In building-block areas the most
  used member comes first and related members follow each other (`use_press`, `PressResponder`, `use_hover`,
  `Hoverable`).
- Groups with entries are collapsible; the guides and the group of the current page are expanded.
- An area whose hooks are only used together documents them on its overview page (Collection State, Drag & Drop,
  Animation); its entries are the members with pages of their own.

### Concept tabs

A concept with both layers has the tabs **Overview · Hook(s) · Atom(s)**, in this order. The labels follow from the page kind and the number of documented items (`Hook` / `Hooks`), never
from free text. Several hooks of one concept are sections of the one Hooks tab, not tabs of their own.

### Enforced by `nav.rs`

The rules above are checked, not just written down:

- Tab labels derive from `PageKind`; a sidebar part decides whether its entries show layer markers or badges.
- Unit tests fail when a concept group contains a building block or the other way around, when a hook and an atom of
  the same name are separate entries instead of one concept, and when entries are out of order.

### URL structure

```
/doc/button                    concept overview
/doc/button/hook               layer page (also /atom)
/doc/tree                      single-layer concept
/doc/fields                    concept group overview
/doc/interactions              building-block area overview
/doc/interactions/use-press    building block
/doc/event-propagation         guide
```

URLs are flat slugs, so group, area, concept and guide names must not collide (the concept group is "Fields" because
the guide "Forms & Validation" is `/doc/forms`). When a page moves, add a `<Redirect>` route for its old URL: the
removed Component tabs (`/doc/<concept>/component`) redirect to the "Styling" section of the atom page, removed
component-only pages to their recipe or replacement.

## Verification

- `cargo test --features ssr --lib` in `examples/book-ssr`: unit tests of the kit, the navigation (every entry has
  a summary) and `kit::api_check` (API tables against the library source).
- `just book-browser-test` (`examples/book-ssr/tests/`): visits every page of the navigation and fails on page
  errors, demos unreadable in the dark theme, internal links or anchors that don't resolve, pages wider than a 390px
  screen, and pages missing from the Markdown export. Add checks there, in Rust, in the style of the library's browser
  tests. `BOOK_TEST_PAGES=<text>` limits the page checks to matching pages. Never run two book suites at the same time:
  they share the app's build directory.
- Clippy for both builds, zero findings: `cargo clippy --features ssr --tests` and
  `cargo clippy --lib --no-default-features --features hydrate --target wasm32-unknown-unknown`.
- Screenshots of the affected pages when changing visuals (`just book-serve-isolated` serves a second instance).
- Checks build against the live library, as the user's `just serve` does, in the shared agent target directory.

## Markdown Export (LLM-native docs)

Every `/doc/...` page is also served as Markdown at `/doc/....md`, and `/doc/llm-index.md` lists all pages with
their sections, in the order of the sidebar (generated from `nav.rs`): Getting started, Guides, the concepts by group
and the building blocks by area. Content is defined once, in the Leptos page; the Markdown is derived from it
(`src/markdown/`):

- The middleware renders the page through SSR, converts its `<article>` with `htmd` and caches the result. All pages
  are converted at startup, which also feeds the book's search (which matches the plain text of the article, without
  frontmatter and demos).
- Frontmatter: title (`<h1>`), kind (page kind from `nav.rs`: `guide`, `overview`, `concept`, `hook`, `atom` or
  `utility`), path, description (first paragraph) and related pages (the See Also links).
- Demos become `*[Interactive Demo: <description>]*` followed by their Rust source.
- Links to other pages point to their Markdown export.
- Every hook or atom documented on a page must appear by name in a section title, or it is invisible to
  the index.
- Responses carry an `ETag` and `Cache-Control: public, max-age=3600, must-revalidate`.
