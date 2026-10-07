# Book Style Guide

How the book looks and what it is built from. The book is leptonic's documentation *and* its largest user: every
part of it shows how leptonic is meant to be used. Page structure and writing rules live in
`documentation/documentation-strategy.md`; open work in the root `PLAN.md` (section "Book").

## 1. Built with leptonic

Everything the book needs that leptonic provides comes from leptonic. The book adds layout and styling, not
behavior. Leptonic is hooks and unstyled atoms (its components layer is gone), so all of the book's look is its own.

- **Use the atom where leptonic has one,** else the hook with the book's own markup. Never hand-roll behavior
  leptonic has: no `on:click` on a raw `<button>`, no `<details>`, no own keyboard handling, no own focus or scroll
  management.
- **The book styles every atom itself** with its tokens (section 2), through the atoms' default classes
  (`leptonic-<AtomName>`), its own classes and the data attributes. It never loads a leptonic stylesheet, not even the
  optional atom theme (`leptonic-atoms`), which is for apps.
- **Book widgets are built from hooks and atoms.** Search, the source disclosure, the navigation menus and the copy
  button are compositions of leptonic pieces (see the table). If a widget would be useful to apps, it belongs in the
  library: propose it there instead of growing the book.
- **Gaps and bugs are fixed in the library, not worked around in the book.** When a leptonic piece is missing,
  inaccessible or wrong, record it in the library roadmap of the root `PLAN.md`, list it under "Waiting on the
  library" in its "Book" section, and keep using the leptonic piece. No book-local replacements.
- **Demos** show the layer of their page: a hook demo spreads the hook onto plain elements, an atom demo uses the atom.
  Everything around the demonstrated element (open buttons, "Disabled" checkboxes, reset buttons, form fields) uses
  leptonic's atoms like any app would, styled by the book (see "Demo controls" in section 5).

| Need                                   | Use                                                                                                                                                |
|----------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|
| Button (text or icon)                  | `Button` atom with `book-button` (`attr:data-variant="secondary"` outlined); icon-only: `book-icon-button` + kit `Icon` + `aria_label`             |
| Link to a page / external / in-page    | `Link`, `AnchorLink` atoms (kit `Link`, `AnchorLink` in pages); a link that looks like a button: `Link` with `book-button`                         |
| Dialog                                 | `ModalBackdrop` + `ModalContent` + `Dialog` (with `DialogTitle`) atoms                                                                             |
| Panel covering the page (mobile menu)  | the shell's `MenuDrawer`: the modal atoms laid out at one edge, with a close button inside                                                         |
| Show/hide section                      | `use_disclosure` + `use_disclosure_state` (kit `Disclosure`)                                                                                       |
| Checkbox, switch, radio, toggle button | `Checkbox`, `Switch` (`is_selected` + `set_selected`), `RadioGroup` + `Radio` (`value` + `set_value`), `ToggleButton(Group)` atoms                 |
| Text, search and number fields         | `TextField`, `SearchField`, `NumberField` atoms (`value` + `set_value`) with `Label`, `Input`, `FieldError`                                        |
| Keyboard shortcut                      | `utils::keyboard_shortcut::Shortcut` (`Shortcut::key("k").primary()`)                                                                              |
| Key caps                               | `ShortcutKeys` atom (a `Shortcut`, per platform), `Keys` atom (keys as given); in pages kit `Keys keys="Shift + Tab"`, in tables `KeyRow`          |
| Data table                             | `Table*` atoms; in pages kit `DocTable` with `TableRow`/`TableCell` rows                                                                           |
| Code                                   | kit `Code` (inline or with `language`; blocks are highlighted and get a copy button)                                                               |
| Icons                                  | kit `Icon` with `icondata` Bootstrap icons (`Bs*`)                                                                                                 |
| Toasts                                 | `BookToasts::show` (the shell's `ToastRegion` and toast atoms on a `ToastQueue`)                                                                   |
| Screen reader announcements            | `utils::live_announcer::announce_polite`                                                                                                           |
| Theme switching                        | the shell's `ThemeToggle` (a `Switch` atom on `use_theme`) inside the app's `ThemeProvider`                                                        |
| Layout                                 | CSS (flex/grid) with the tokens below                                                                                                              |

## 2. Design tokens

All tokens are CSS custom properties defined in `style/book/_theme.scss`. Book code uses tokens, not literal values.
A few general names, kept from leptonic's former component theme, are defined by the book like all others
(`--brand-color`, `--main-color`, `--main-background-color`, the status colors, `--font-family`,
`--typography-code-font-family`); `--book-*` variables cover the rest.

### Color

| Token                                                               | Use                                                                                     |
|---------------------------------------------------------------------|-----------------------------------------------------------------------------------------|
| `--main-color`, `--main-background-color`                           | Page text and background                                                                |
| `--brand-color`                                                     | Accent fills and borders: demo frames, current item backgrounds, focus                  |
| `--book-brand-text-color`                                           | Brand-colored text (links in prose, current tab, demo triggers): 4.5:1 on every surface |
| `--brand-color-subtle`                                              | Tinted backgrounds (selected rows, highlights)                                          |
| `--book-on-brand-color`                                             | Text on `--brand-color` or a badge color                                                |
| `--book-surface-color`, `--book-surface-color-raised`               | Cards, panels, popovers; raised: inputs, key caps, hovered surfaces                     |
| `--book-hover-background-color`                                     | Hovered rows and list items                                                             |
| `--book-border-color`                                               | Borders and separators                                                                  |
| `--book-muted-color`                                                | Secondary text (descriptions, labels, placeholders)                                     |
| `--book-focus-color`                                                | Focus rings drawn by the book                                                           |
| `--book-backdrop-color`                                             | Behind modal overlays                                                                   |
| `--success-color`, `--warn-color`, `--danger-color`, `--info-color` | Status: valid/invalid, meters, errors                                                   |
| `--book-categorical-1` ... `-4`                                     | Distinguishing series in demos (slider accents, chart bars)                             |
| `--book-badge-{hook,atom,util}-color`                               | Layer badges and layer markers                                                          |
| `--book-app-bar-background-color`, `--book-app-bar-shadow`          | The app bar                                                                             |
| `--book-switch-off-color`, `--book-switch-knob-color`               | A switch's track when off, its knob (the theme toggle, switch demos)                    |

Every color works in both themes: the dark theme redefines the same tokens. Never write `#fff`, `white`, `black` or
`rgba(...)` outside `_theme.scss`; the one exception is `transparent`. Text meets WCAG AA in both themes: 4.5:1
against its background (3:1 for text of 24px and more). Brand-colored text uses `--book-brand-text-color`, never
`--brand-color`; text on a colored background (badges, the current sidebar item) uses a pair checked for 4.5:1.

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
  `-l` (1.25em: card titles), `-xl` (1.5em: icon buttons). Headings in pages are styled in `_article.scss`; the
  welcome and 404 pages may use larger display sizes.
- Weights: 400 text, 600 emphasis in UI (titles of list items), 700 headings and table heads. No other weights.
- Code, logs and keys use `--typography-code-font-family`, never a bare `monospace`.

### Motion and layering

- Transitions: `var(--book-transition)` (0.15s ease) for color, background and border changes. Anything that moves
  is disabled under `prefers-reduced-motion: reduce`.
- Layers: `--book-z-sticky` (sticky concept tabs), `--book-z-app-bar` (the app bar, fixed above the page),
  `--book-z-backdrop` (backdrops of demo overlays, and panels positioned like them: they cover the app bar),
  `--book-z-modal` (the book's own overlays: the menus of small screens, the search, toasts). Popovers and tooltips
  are layered by leptonic (positioned overlays) and need no z-index from the book.
- Sizes: `--book-app-bar-height`, `--book-concept-tabs-height`, `--book-sidebar-width`, `--book-toc-width`,
  `--book-article-max-width`, in rem, so that they resolve to the same length in every element.
- Breakpoints: 800px (sidebar becomes a menu, tables stack into cards), 1200px (table of contents hidden), as `$small`
  and `$medium` in `_theme.scss` (`@use "theme" as *;`, then `@media (width <= $small)`). `$small` is coupled with
  `SMALL_SCREEN_MAX_WIDTH` in `src/app.rs`. Pages must fit a 390px screen.

## 3. CSS conventions

- **Files:** book chrome in `style/book/` (one file per area: `_shell`, `_doc-layout`, `_article`, `_demo`,
  `_search`, `_pages`; tokens in `_theme`). Demo styles in `style/demos/`: one file per sidebar group, named after it
  (concept groups `_buttons`, `_fields`, `_pickers`, `_collections`, `_date-time`, `_color`, `_overlays`,
  `_navigation`, `_status`, `_layout`; building-block areas `_interactions`, `_focus`, `_overlay-behavior`,
  `_collection-state`, `_drag-and-drop`, `_animation`, `_screen-readers`, `_utilities`; `_guides` for the guides),
  optionally split further per concept when a group's file grows large (`_table.scss`), and `_shared.scss` for
  classes used by demos of several groups (status line, control rows, buttons around demos, forms). A new stylesheet is registered in `style/main.scss` and in `STYLESHEETS` of
  `src/kit/demo_styles.rs` (a test checks both lists match; "View styles" shows the rules a demo uses).
- **Names:** chrome uses `book-` ids for unique landmarks (`#book-app-bar`) and `doc-` classes for page building
  blocks (`.doc-table`); demo classes start with `demo-`. Kebab case, no BEM modifiers: state comes from attributes.
- **Style state through attributes** leptonic sets: `[aria-current]`, `[aria-expanded]`, `[data-selected]`,
  `[data-focus-visible]`, `[data-disabled]`, ... Don't mirror state into classes.
- **Style atoms through their classes:** a book class passed as `classes` (`book-button`, `demo-btn`), or the
  atom's default class (`.leptonic-ListBoxItem`) scoped under a book class. Don't restyle a default class globally
  outside the stylesheet that owns the widget.
- **Mixins:** `key-cap` (`_theme.scss`, `@use "theme" as *;`) draws a `<kbd>` key cap: the kit's `Keys`, the search
  button's shortcut and key caps in demos.
- **No inline styles.** Values computed at runtime use the typed `leptos-styles` API.
- **Focus** is visible on every interactive element: an outline in `--book-focus-color` on the atom's
  `[data-focus-visible]` (`:focus-visible` on plain elements). Never remove an outline without replacing it.
- **No literal z-index in demos:** popovers and tooltips are layered by leptonic; backdrops use `--book-z-backdrop`. **No literal durations:** `--book-transition`.
- **Both themes, all widths:** check light and dark and a 390px screen (the browser tests do: `just
  book-browser-test`).

## 4. Patterns

- **Icon buttons** (app bar menus, close buttons): a `Button` atom with `book-icon-button` (no frame, icon at
  `--book-font-size-xl`), an `aria_label` naming the action, `aria_expanded` when they open something.
- **Cards** (welcome features, demo frames): `--book-surface-color`, 1px border, `--book-radius-m`, padding
  `--book-space-l`.
- **Lists of links** (sidebar, table of contents, search results): full-width rows, `--book-space-xs`/`-s` padding,
  `--book-radius-s`, hover `--book-hover-background-color`, current item `[aria-current]` in `--brand-color`.
- **Badges** (kind of a building block: hook, atom, util) and **layer markers** (`H A` of a concept):
  `--book-font-size-xs`, bold, `--book-radius-s`, text `--book-on-brand-color` on the
  badge color.
- **Demo frame** (`Demo`): `--brand-color` border, `--book-radius-m`; "View source" and "View styles" are
  disclosures below the demo.
- **Status text in demos:** errors in `--danger-color` with an `aria-invalid` / error message wiring from the hook,
  never color alone.

## 5. Demos

A demo is code readers copy. It shows one thing well, and everything around the demonstrated element uses leptonic.

- **State is visible:** a `<p class="demo-status">` below the demo shows the current value or the last event ("Last
  action: copy."), with correct grammar ("1 time", "2 times"). Event logs use `ringbuf::HeapRb` capped at 50 entries
  in a `demo-event-log`. No other status classes.
- **Disabling:** where the concept supports disabling, the standard demo control (a `Checkbox` atom, see "Demo
  controls" below) labelled "Disabled" in a `demo-controls` row below the demo toggles it. Further switches of a demo
  (read-only, orientation, ...) go into the same row.
- **The book styles every demo itself:** every demo is styled by the book's demo stylesheets (`style/demos/`), with
  the book's tokens. Leptonic's optional atom theme (`leptonic-atoms`) is for apps; the book never loads it. Demos use
  atoms and hooks only.
- **The layer of the page is styled through what it renders:** atom demos style the atom only through its data
  attributes (`[data-pressed]`, `[data-selected]`, `[data-focus-visible]`, ...), on a book class passed as `classes`
  or on the atom's default class `leptonic-<AtomName>` (e.g. `.leptonic-ListBoxItem`); hook demos style the plain
  elements they render with book classes, through what the hook sets (ARIA attributes, the hook's state
  attributes). No mirrored classes (`.selected`, `.pressed`), no `:hover`, `:focus` or `:disabled` where the layer
  provides an attribute. Markup an atom doesn't render (a checkbox's box, a switch's track, a select's caret) is the
  demo's own, `aria-hidden`.
- **Standalone:** a demo's source compiles on its own in an app (no `crate::routes`, no book helpers); a Quick Start
  demo is short (a few dozen lines). Names in demos are real words, not `foo`.
- **Accessible:** every control has a name, decorative glyphs and emoji are `aria-hidden` (or use `Icon`), demos
  don't trap focus, and nothing relies on color or hover alone.
- **No restated defaults** in demo sources: set only what the demo is about, with struct update syntax.

### Demo controls

The controls around a demo (options like "Disabled", a button opening an overlay, a field feeding a value) are
leptonic atoms with the book's demo classes. They read like any app's code: copy the markup, then replace the classes
with your own styles. The classes used by demos of several groups live in `style/demos/_shared.scss`; a concept's own
classes in its group's stylesheet (move a rule to `_shared.scss` once another group uses it).

The standard control, a checkbox (`_shared.scss`; `atoms/demos/{button,checkbox,link,switch}.rs` use it):

```rust
use leptonic::atoms::checkbox::Checkbox;

<div class="demo-controls">
    <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
        <span class="demo-check-box" aria-hidden="true"></span>
        "Disabled"
    </Checkbox>
</div>
```

The markup of the other atoms demos use most (their classes: `_shared.scss` unless named):

```rust
// Button: `demo-btn`, or `demo-btn-primary` / `demo-btn-danger` for a demo's main or destructive action.
<Button on_press=move |_| open.set(true) classes="demo-btn">"Open"</Button>

// Toggle button: `demo-toggle-button`; a group of them: `ToggleButtonGroup` with `demo-toggle-group`.
<ToggleButton is_selected=bold set_selected=bold classes="demo-toggle-button">"Bold"</ToggleButton>

// Switch (`_fields.scss`): the track and thumb are the demo's own markup.
<Switch is_selected=wifi set_selected=wifi classes="demo-switch">
    <span class="demo-switch-track" aria-hidden="true"><span class="demo-switch-thumb"></span></span>
    "Wi-Fi"
</Switch>

// Radio group (`demo-radio`, `demo-radio-circle` in `_fields.scss`; the group classes in `_fields.scss`).
<RadioGroup value=plan set_value=plan classes="demo-choice-group">
    <Label classes="demo-choice-group-label">"Plan"</Label>
    <div class="demo-choice-group-items">
        <Radio value="free" classes="demo-radio"><span class="demo-radio-circle" aria-hidden="true"></span>"Free"</Radio>
        <Radio value="pro" classes="demo-radio"><span class="demo-radio-circle" aria-hidden="true"></span>"Pro"</Radio>
    </div>
</RadioGroup>

// Text field: `demo-field`, `demo-field-label`, `demo-atom-input`, `demo-field-error`, `demo-field-description`.
<TextField value=name set_value=name classes="demo-field">
    <Label classes="demo-field-label">"Name"</Label>
    <Input classes="demo-atom-input"/>
    <FieldError classes="demo-field-error"/>
</TextField>

// Select (`demo-sel-*` in `_pickers.scss`): the caret is the demo's own markup.
<Select collection=offices value=office set_value=office classes="demo-sel">
    <Label classes="demo-sel-label">"Office"</Label>
    <SelectTrigger classes="demo-sel-trigger">
        <SelectValue placeholder="Choose an office" classes="demo-sel-value"/>
        <span class="demo-sel-caret" aria-hidden="true">"\u{25bc}"</span>
    </SelectTrigger>
    <SelectPopover classes="demo-sel-popover">
        <ListBox classes="demo-sel-listbox">/* ListBoxItem classes="demo-sel-item" */</ListBox>
    </SelectPopover>
</Select>

// Slider (`demo-slider-*` in `_fields.scss`).
<Slider min_value=0 max_value=100 default_values=vec![50_u8] classes="demo-slider">
    <Label classes="demo-slider-label">"Volume"</Label>
    <SliderTrack classes="demo-slider-track">
        <SliderFill classes="demo-slider-fill"/>
        <SliderThumb classes="demo-slider-thumb"/>
    </SliderTrack>
    <SliderOutput classes="demo-slider-output"/>
</Slider>
```

### Demo stylesheets

- **Atom demos** are styled by the book's demo classes (or the atoms' default classes `leptonic-<AtomName>`) and the
  atoms' data attributes, nothing else. A "Styling" section of an atom page shows how to style the atom (its default
  class, data attributes and the markup to render inside it) with the book's own CSS as the example, and ends with
  one sentence on the optional atom theme (`documentation/documentation-strategy.md`, "Atom page").
- **Recipes** (parts without behavior: card, stack, app bar, alert, drawer) live on their group's overview, as
  semantic markup plus CSS with stand-in tokens or a demo styled in the group's stylesheet.
- **Hook demos** render plain elements with book classes and style them through the attributes the hooks set.
- **Layout** classes (`demo-controls`, `demo-control-row`, `demo-form`, ...) arrange demos; they don't style atoms.
- **Rules of removed demos are removed** with them (the unit test `every_rule_belongs_to_a_used_class` lists them).

**Styling snippets on pages** (the CSS in "Styling" sections) use stand-ins for the reader's own design tokens,
`var(--accent)`, `var(--surface)`, `var(--border)`, `var(--muted)` and `var(--focus)`, never literal colors. Short
rules take one line each. A snippet that animates includes its `@media (prefers-reduced-motion: reduce)` rule.
