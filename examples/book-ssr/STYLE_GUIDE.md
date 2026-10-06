# Book Style Guide

How the book looks and what it is built from. The book is leptonic's documentation *and* its largest user: every
part of it shows how leptonic is meant to be used. Page structure and writing rules live in
`documentation/documentation-strategy.md`; open work in the root `PLAN.md` (section "Book").

## 1. Built with leptonic

Everything the book needs that leptonic provides comes from leptonic. The book adds layout and styling, not
behavior.

- **Choose the highest layer that fits:** a themed component if it is built on leptonic's hooks, else an atom, else
  the hook with the book's own markup. Never hand-roll behavior leptonic has: no `on:click` on a raw `<button>`, no
  `<details>`, no own keyboard handling, no own focus or scroll management.
- **Book widgets are built from hooks and atoms.** Search, the source disclosure, the navigation drawer and the copy
  button are compositions of leptonic pieces (see the table). If a widget would be useful to apps, it belongs in the
  library: propose it there instead of growing the book.
- **Gaps and bugs are fixed in the library, not worked around in the book.** When a leptonic piece is missing,
  inaccessible or wrong, record it in the library roadmap of the root `PLAN.md`, list it under "Waiting on the
  library" in its "Book" section, and keep using the leptonic piece. No book-local replacements.
- **Demos** show the layer of their page: a hook demo spreads the hook onto plain elements, an atom demo uses the atom.
  Everything around the demonstrated element (open buttons, "Disabled" checkboxes, reset buttons, form fields) uses
  leptonic like any app would.

| Need                                   | Use                                                                                         |
|----------------------------------------|---------------------------------------------------------------------------------------------|
| Button (text or icon)                  | `components::Button`; icon-only: `variant=ButtonVariant::Flat` + `Icon` + `attr:aria-label` |
| Link to a page / external / in-page    | `Link`, `LinkExt`, `AnchorLink`; a link that looks like a button: `LinkButton`              |
| Dialog                                 | `Modal` (+ `ModalHeader`, `ModalTitle`, `ModalBody`, `ModalFooter`)                         |
| Panel covering the page (mobile menu)  | atoms `ModalBackdrop` + `ModalContent` + `Dialog` (until the Drawer is rebuilt on them)     |
| Show/hide section                      | `use_disclosure` + `use_disclosure_state` (kit `Disclosure`)                                |
| Checkbox, switch, radio, toggle button | `Checkbox`, `Switch`, `RadioGroup` + `Radio` (bind a signal: `state=signal`); toggle buttons: atoms `ToggleButton(Group)` |
| Text, search and number fields         | `TextField`, `SearchField`, `NumberField` (bind a signal: `state=signal`)                   |
| Keyboard shortcut                      | `utils::keyboard_shortcut::Shortcut` (`Shortcut::key("k").primary()`)                       |
| Key caps                               | `KbdKey`, `KbdShortcut`; in pages kit `Keys keys="Shift + Tab"`, in tables `KeyRow`        |
| Data table                             | `Table*` components; in pages kit `DocTable` with `TableRow`/`TableCell` rows            |
| Code                                   | `Code` (inline or with `language`; blocks get leptonic's copy button)                      |
| Icons                                  | `Icon` with `icondata` Bootstrap icons (`Bs*`)                                              |
| Screen reader announcements            | `utils::live_announcer::announce_polite`                                                    |
| Theme switching                        | `ThemeToggle` inside `Root`                                                                 |
| Layout                                 | CSS (flex/grid) with the tokens below; `Stack` where a plain stack is meant                 |

## 2. Design tokens

All tokens are CSS custom properties defined in `style/book/_theme.scss`. Book code uses tokens, not literal values.
Leptonic's own theme variables (`--brand-color`, `--danger-color`, `--typography-*`, ...) are used where they exist;
`--book-*` variables cover the rest.

### Color

| Token                                           | Use                                                              |
|-------------------------------------------------|------------------------------------------------------------------|
| `--main-color`, `--main-background-color`       | Page text and background                                         |
| `--brand-color`                                 | Accent: active navigation, demo frames, links in prose, focus    |
| `--brand-color-subtle`                          | Tinted backgrounds (selected rows, highlights)                   |
| `--book-on-brand-color`                         | Text on `--brand-color` or a badge color                         |
| `--book-surface-color`, `--book-surface-color-raised` | Cards, panels, popovers; raised: inputs, key caps, hovered surfaces |
| `--book-hover-background-color`                 | Hovered rows and list items                                      |
| `--book-border-color`                           | Borders and separators                                           |
| `--book-muted-color`                            | Secondary text (descriptions, labels, placeholders)              |
| `--book-focus-color`                            | Focus rings drawn by the book                                    |
| `--book-backdrop-color`                         | Behind modal overlays                                            |
| `--success-color`, `--warn-color`, `--danger-color`, `--info-color` | Status (leptonic theme): valid/invalid, meters, errors |
| `--book-categorical-1` ... `-4`                 | Distinguishing series in demos (slider accents, chart bars)      |
| `--book-badge-{hook,atom,comp}-color`           | Layer badges                                                     |

Every color works in both themes: the dark theme redefines the same tokens. Never write `#fff`, `white`, `black` or
`rgba(...)` outside `_theme.scss`; the one exception is `transparent`.

### Spacing

`--book-space-xs` (0.25em), `-s` (0.5em), `-m` (1em), `-l` (1.5em), `-xl` (2em), `-xxl` (3em). Relative to the
element's font size, so smaller text gets tighter spacing. Use them for padding, margin and gap.

### Shape

- Radii: `--book-radius-s` (4px: key caps, inline badges, small controls), `--book-radius-m` (8px: buttons, inputs,
  cards, demo frames), `--book-radius-l` (12px: dialogs and large panels), `--book-radius-pill` (pills). Circles use
  `50%`.
- Borders: `1px solid var(--book-border-color)`; `2px` only for emphasis (demo frames, table heads, active tabs).
- Shadows: `--book-shadow-s` (thumbs, small raised controls), `--book-shadow` (popovers, dialogs, menus).

### Type

- Fonts: Roboto (`--font-family`) for text, JetBrains Mono (`--typography-code-font-family`) for code and keys.
- Sizes: `--book-font-size-xs` (0.75em: badges, captions), `-s` (0.85em: secondary text, small controls), `-m` (1em),
  `-l` (1.25em: card titles), `-xl` (1.5em: icon buttons). Headings in pages come from leptonic's typography; the
  welcome and 404 pages may use larger display sizes.
- Weights: 400 text, 600 emphasis in UI (titles of list items), 700 headings and table heads.

### Motion and layering

- Transitions: `var(--book-transition)` (0.15s ease) for color, background and border changes. Anything that moves
  is disabled under `prefers-reduced-motion: reduce`.
- Layers: `--book-z-sticky` (sticky concept tabs, the copy button). The app bar is layered by leptonic's theme;
  overlays render in leptonic's portal and need no z-index from the book.
- Breakpoints: 800px (sidebar becomes a menu, tables stack into cards), 1200px (table of contents hidden). Pages
  must fit a 390px screen.

## 3. CSS conventions

- **Files:** book chrome in `style/book/` (one file per area: `_shell`, `_doc-layout`, `_article`, `_demo`,
  `_search`, `_pages`; tokens in `_theme`). Demo styles in `style/demos/`: one file per sidebar section
  (`_input.scss`, ...), a file per concept with many rules (`_table.scss`, ...), and `_shared.scss` for classes used
  across sections. A new stylesheet is registered in `style/main.scss` and in `STYLESHEETS` of
  `src/kit/demo_styles.rs` (a test checks both lists match; "View styles" shows the rules a demo uses).
- **Names:** chrome uses `book-` ids for unique landmarks (`#book-app-bar`) and `doc-` classes for page building
  blocks (`.doc-table`); demo classes start with `demo-`. Kebab case, no BEM modifiers: state comes from attributes.
- **Style state through attributes** leptonic sets: `[aria-current]`, `[aria-expanded]`, `[data-selected]`,
  `[data-focus-visible]`, `[data-disabled]`, ... Don't mirror state into classes.
- **Style leptonic components through their classes and CSS variables** (`.leptonic-btn`, `--button-*`), scoped
  under a book class. Don't restyle leptonic globally; a fix that every app needs belongs in the theme.
- **No inline styles.** Values computed at runtime use the typed `leptos-styles` API.
- **Focus** is visible on every interactive element: leptonic's `data-focus-visible` / `:focus-visible` outline in
  `--book-focus-color`. Never remove an outline without replacing it.
- **Both themes, all widths:** check light and dark and a 390px screen (the browser tests do: `just
  book-browser-test`).

## 4. Components and patterns

- **Icon buttons** (app bar menus, close buttons): flat `Button`, icon at `--book-font-size-xl`, an `aria-label`
  naming the action, `aria-expanded` when they open something.
- **Cards** (welcome features, demo frames): `--book-surface-color`, 1px border, `--book-radius-m`, padding
  `--book-space-l`.
- **Lists of links** (sidebar, table of contents, search results): full-width rows, `--book-space-xs`/`-s` padding,
  `--book-radius-s`, hover `--book-hover-background-color`, current item `[aria-current]` in `--brand-color`.
- **Badges** (layer of a page): `--book-font-size-xs`, bold, `--book-radius-s`, text `--book-on-brand-color` on the
  badge color.
- **Demo frame** (`Demo`): `--brand-color` border, `--book-radius-m`; "View source" and "View styles" are
  disclosures below the demo.
- **Status text in demos:** errors in `--danger-color` with an `aria-invalid` / error message wiring from the hook,
  never color alone.
