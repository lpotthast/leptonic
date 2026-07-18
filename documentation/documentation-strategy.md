# Documentation Strategy

This document defines the structure, content guidelines, and migration plan for leptonic's book-ssr documentation.

## Philosophy & Goals

1. **Concept-first navigation.** Users think "I need a button", not "I need the `use_button` hook". Documentation
   is organized around concepts, not implementation layers.

2. **Progressive disclosure.** A concept overview provides enough to get started. Layer deep-dives provide full
   detail. No reader needs to read all layer pages.

3. **Zero repetition.** Every piece of information lives in exactly one place. Other pages link to it. Conceptual
   explanations (what a button is, its accessibility semantics) live on the concept page, never on layer pages.

4. **Self-contained pages.** A user landing on `/doc/button/hook` via search must not feel lost. Each page has a brief
   context sentence linking back to its parent concept.

5. **Living examples.** Every interactive capability gets a working demo, not just a code block.

---

## Page Types

### 1. UI Concept Overview

**When it exists:** A concept has implementations at two or more layers (hook + atom, hook + component, or all three).

**URL:** `/doc/{concept}` (e.g., `/doc/button`, `/doc/slider`)

**Required sections (in order):**

| Section | Anchor ID | Content |
|---------|-----------|---------|
| Title & description | `#{concept}` | What it is (1-2 paragraphs). No code yet. |
| When to Use | `#when-to-use` | Use cases, comparison with similar concepts (e.g., button vs. link vs. toggle). |
| Choose Your Layer | `#layers` | Comparison table: hook vs. atom vs. component. What each gives you. Links to layer pages. |
| Quick Start | `#quick-start` | Code snippet for the most common use case (typically the component). One interactive demo rendered with the component. |
| Accessibility | `#accessibility` | ARIA pattern summary, keyboard interaction table. |

**Does NOT contain:** Full API reference, all configuration options, CSS variables, hook internals.

### 2. Behavioral Domain Overview

**When it exists:** A group of closely related hooks (3+) that users discover together but that don't have a shared
atom/component (e.g., Interactions, Focus, Overlays, Selection).

**URL:** `/doc/{domain}` (e.g., `/doc/interactions`, `/doc/focus`)

**Required sections (in order):**

| Section | Anchor ID | Content |
|---------|-----------|---------|
| Title & description | `#{domain}` | What this domain covers, why these hooks are grouped. |
| Overview | `#overview` | Brief description of each hook in the domain with links. When to use which. |
| Relationships | `#relationships` | How hooks in this domain relate and compose (e.g., `use_overlay` + `use_overlay_position` = `use_popover`). |
| Quick Start | `#quick-start` | Code example showing the most common hook from this domain. |

**Does NOT contain:** Full API reference for individual hooks. Each hook has its own deep-dive page.

### 3. Hook Deep-Dive

**URL:** `/doc/{concept}/hook` or `/doc/{domain}/{hook-name}` (e.g., `/doc/button/hook`, `/doc/interactions/use-press`)

**Required sections (in order):**

| Section | Anchor ID | Content |
|---------|-----------|---------|
| Title | `#{hook-name}` | Hook name. One sentence + link to concept/domain overview. |
| react-aria | `#react-aria` | "Based on [react-aria's useX](...)" with link. |
| Input | `#input` | Table of `UseFooInput` fields (field, type, default, description). |
| Return | `#return` | Table of `UseFooReturn` fields. |
| Example | `#example` | Minimal code showing hook setup and attribute spreading. |
| Demo | `#demo` | Interactive demo using `DemoShell`. |
| Options | `#options` | Configuration options with demos (omit if none). |
| Keyboard | `#keyboard` | Keyboard interaction table (omit if not applicable). |
| Deviations | `#deviations` | React-aria deviations (omit if none). |
| See Also | `#see-also` | Links to sibling layer pages (atom, component) and composed hooks. |

**Does NOT contain:** Conceptual explanation of the concept, styling/theme variables, atom or component usage.

#### Multi-Hook Pages

When a hook page documents multiple hooks (e.g., `/doc/slider/hook` documenting `use_slider_state`, `use_slider`,
`use_slider_thumb`, `use_slider_marks`), use hook names as primary `## ` headings:

| Section | Anchor ID | Content |
|---------|-----------|---------|
| Title | `#{concept}-hooks` | Concept name + "Hooks". Brief intro on how hooks compose. |
| Hook N (repeated) | `#{hook-name}` | `## hook_name` heading. Brief description, Input table, Return table, Example + Demo. |
| Keyboard | `#keyboard` | Shared keyboard interaction table (omit if not applicable). |
| Deviations | `#deviations` | React-aria deviations (omit if none). |
| See Also | `#see-also` | Links to sibling layer pages. |

**Key constraint:** Each hook's `## ` heading must use the exact hook function name (e.g., `## use_slider_thumb`,
not `## Per-thumb behavior`) for LLM index discoverability. The same principle applies to atom and component
deep-dive pages that document multiple items — each must have its own `## ` heading with the exact name.

### 4. Atom Deep-Dive

**URL:** `/doc/{concept}/atom` (e.g., `/doc/button/atom`, `/doc/slider/atom`)

**Required sections (in order):**

| Section | Anchor ID | Content |
|---------|-----------|---------|
| Title | `#{atom-name}` | Atom name. One sentence + link to concept overview. |
| Hooks Used | `#hooks` | Which hooks this atom wraps, with links. |
| Props | `#props` | Props table (prop, type, default, description). |
| Example | `#example` | Minimal code showing atom usage. |
| Demo | `#demo` | Interactive demo using `DemoShell`. |
| Data Attributes | `#data-attributes` | Table of `data-*` attributes (attribute, values, description). |
| Styling | `#styling` | How to target data attributes with CSS for custom styling. |
| Composition | `#composition` | How to compose with other atoms (omit if not applicable). |
| See Also | `#see-also` | Links to sibling layer pages (hook, component). |

**Does NOT contain:** Hook internals, theme CSS variables, conceptual explanation of the concept.

### 5. Component Deep-Dive

**URL:** `/doc/{concept}/component` (e.g., `/doc/button/component`, `/doc/slider/component`)

**Required sections (in order):**

| Section | Anchor ID | Content |
|---------|-----------|---------|
| Title | `#{component-name}` | Component name. One sentence + link to concept overview. |
| Props | `#props` | Props table. |
| Variants | `#variants` | Visual examples for each variant/color/size. |
| Demo | `#demo` | Interactive demos showing features. |
| CSS Variables | `#styling` | Table of CSS custom properties (variable, default, description). |
| Composition | `#composition` | Usage inside other components, slots, groups (omit if not applicable). |
| Recipes | `#recipes` | Practical usage patterns (omit if not applicable). |
| See Also | `#see-also` | Links to sibling layer pages (hook, atom). |

**Does NOT contain:** Hook internals, atom data attributes, conceptual explanation of the concept.

### 6. Standalone Page

For hooks, atoms, or components that exist at only one layer and don't belong to a behavioral domain.

**URL:** `/doc/hooks/{hook}`, `/doc/atoms/{atom}`, or `/doc/components/{component}`

**Content:** Same structure as the corresponding deep-dive page type (hook/atom/component), but with a full
conceptual introduction instead of "one sentence + link". Since there's no concept overview page to link to, the
standalone page IS the concept page.

---

## LLM-Native Content Delivery

### Requirements

1. **Single source of truth** — Content must be defined only once. Double maintenance of both a Markdown file and the
   Leptos page implementations would quickly become unmaintainable. All documentation pages must be available as Markdown
   at all times; ideally, the index is created automatically so adding, renaming, or removing pages requires no custom
   intervention.
2. **Indexed** — An index of all available documentation pages (as Markdown) must be available, mainly for LLM
   discoverability.
3. **Full freedom** — Implementing any custom logic/components with any styling on documentation pages must be preserved.
4. **Search capability** — The book's own search functionality should be driven by the Markdown-based documentation
   server-side.

### Implementation

Every `/doc/...` page is automatically available as markdown at `/doc/.../.md`. This is handled by middleware that
intercepts `.md` requests, renders the page via SSR, extracts the `<article>` content, cleans it, and converts to
markdown via `htmd`. Results are cached in memory.

### LLM Index

The index at `/doc/llm-index.md` is auto-generated from cached pages. Each entry includes:
- The page title (derived from the first `# ` heading)
- Sub-entries for each `## ` heading on the page, with anchor links

### Discoverability Rule

**Every hook, atom, or component documented on a page must appear by name in a `## ` heading.** Items without a
dedicated `## ` heading are invisible to the LLM index.

For single-item pages (e.g., `/doc/button/hook` documenting only `use_button`), the item naturally appears as the
`# ` title heading. For multi-item pages (e.g., `/doc/slider/hook` documenting `use_slider`, `use_slider_state`,
`use_slider_thumb`, `use_slider_marks`), each item must have its own `## ` heading.

### Alternative: `data-documents` Attribute

A page can explicitly declare what it documents via a `documents` prop on `<Article>`:

```rust
<Article documents=vec!["use_slider", "use_slider_state", "use_slider_thumb", "use_slider_marks"]>
```

This renders as `<article data-documents="use_slider,use_slider_state,use_slider_thumb,use_slider_marks">`. The
middleware extracts this list and adds items as sub-entries in the LLM index pointing to the page root.

**When to use:**
- As a **fallback** for items that don't have their own `## ` heading (e.g., helper hooks mentioned only in body text)
- As a **supplement** when a page's content structure doesn't naturally lend itself to per-item headings
- Not as a replacement for heading-based discovery — headings are the primary mechanism

If an item appears in both `## ` headings and `data-documents`, it is deduplicated (heading version wins since it has
an anchor link).

**Preferred: use `## item_name` headings. Alternative: add items to the `documents` prop on `<Article>` for items
that don't warrant their own section.**

### Demo Placeholders

Interactive demos rendered with `DemoShell` are replaced with `*[Interactive Demo]*` in markdown output. A
`description` prop on `DemoShell` produces `*[Interactive Demo: description]*` for better LLM context:

```rust
<DemoShell description="Basic slider with value display">
```

### Code Blocks

All code blocks from leptonic's `Code` component include `rust` language hints in the markdown output.

### HTTP Caching

Markdown responses include `ETag` and `Cache-Control: public, max-age=3600, must-revalidate` headers. Clients can
send `If-None-Match` for conditional requests (304 responses).

---

## URL Structure

### Multi-layer UI concepts

```
/doc/button                    -> UI Concept Overview
/doc/button/hook               -> Hook Deep-Dive
/doc/button/atom               -> Atom Deep-Dive
/doc/button/component          -> Component Deep-Dive
```

### Behavioral domains

```
/doc/interactions              -> Behavioral Domain Overview
/doc/interactions/use-press    -> Hook Deep-Dive
/doc/interactions/use-hover    -> Hook Deep-Dive
...
```

### Standalone pages

```
/doc/hooks/use-breadcrumbs     -> Standalone hook
/doc/hooks/use-toolbar         -> Standalone hook
/doc/components/stack          -> Standalone component
/doc/components/toast          -> Standalone component
...
```

### Backward compatibility

All old URLs must redirect to their new locations. Add `<Redirect>` routes for every moved page.

---

## Complete Concept Mapping

### Behavioral Domains

| Domain | Members | Page |
|--------|---------|------|
| **Interactions** | `use_press`, `use_hover`, `use_move`, `use_keyboard`, `use_interact_outside`, `use_scroll_wheel`, `use_prevent_scroll`, `use_drag_and_drop`, `use_draggable`, `use_draggable_item`, `use_droppable`, `use_droppable_collection`, `use_droppable_item` | `/doc/interactions` |
| **Focus** | `use_focus`, `use_focus_within`, `use_focusable`, `use_focus_manager`, `use_has_tabbable_child`, `use_focus_ring`, `use_focus_visible`, `FocusScope` atom, `FocusRing` atom | `/doc/focus` |
| **Overlays** | `use_overlay`, `use_overlay_position`, `use_overlay_trigger`, `use_prevent_scroll` | `/doc/overlays` |
| **Selection** | `use_selection_state`, `use_selectable_item`, `use_selectable_list`, `use_selectable_collection`, `use_type_select` | `/doc/selection` |

### Multi-Layer UI Concepts

| Concept | Hook(s) | Atom(s) | Component(s) | URL |
|---------|---------|---------|---------------|-----|
| **Button** | `use_button` | `Button`, `LinkButton` | `Button`, `ButtonGroup` | `/doc/button` |
| **Link** | `use_link`, `use_anchor_link` | `Link`, `AnchorLink` | — | `/doc/link` |
| **Slider** | `use_slider`, `use_slider_state`, `use_slider_thumb`, `use_slider_marks` | `Slider` (composite) | `Slider`, `RangeSlider` | `/doc/slider` |
| **Popover** | `use_popover` | `Popover`, `PopoverTrigger`, `PopoverContent` | `Popover` | `/doc/popover` |
| **Modal** | `use_modal`, `use_modal_state`, `use_modal_backdrop`, `use_dialog`, `use_dialog_state` | `Modal` | `Modal` | `/doc/modal` |
| **Checkbox** | `use_checkbox`, `use_checkbox_state`, `use_checkbox_group` | — | `Checkbox` | `/doc/checkbox` |
| **Radio** | `use_radio`, `use_radio_group`, `use_radio_group_state` | — | `Radio`, `RadioGroup` | `/doc/radio` |
| **Toggle** | `use_switch`, `use_switch_state`, `use_toggle`, `use_toggle_state` | — | `Toggle` | `/doc/toggle` |
| **Color** | `use_color_area`, `use_color_slider`, `use_color_wheel`, `use_color_field`, `use_color_swatch`, `use_color_channel_field` | — | `ColorPicker` | `/doc/color` |
| **Text Field** | `use_text_field`, `use_text_field_state`, `use_number_field`, `use_number_field_state`, `use_search_field`, `use_search_field_state` | — | `TextInput`, `NumberInput`, `PasswordInput` | `/doc/text-field` |
| **Select** | `use_select`, `use_hidden_select` | — | `Select`, `OptionalSelect`, `Multiselect` | `/doc/select` |
| **Grid** | `use_grid`, `use_grid_row`, `use_grid_cell`, `use_grid_list`, `use_grid_list_item` | `Grid`, `GridList` | `Grid` | `/doc/grid` |
| **Table** | `use_table`, `use_table_header`, `use_table_row`, `use_table_cell`, etc. | — | `Table` | `/doc/table` |
| **Tabs** | `use_tabs`, `use_tab_list`, `use_tab`, `use_tab_panel` | — | `Tabs` | `/doc/tabs` |
| **Separator** | `use_separator` | — | `Separator` | `/doc/separator` |
| **Collapsible** | `use_disclosure`, `use_disclosure_state` | — | `Collapsible`, `Collapsibles` | `/doc/collapsible` |
| **Progress** | `use_progress_bar` | — | `ProgressBar` | `/doc/progress` |
| **Chip** | `use_tag`, `use_tag_group` | — | `Chip` | `/doc/chip` |
| **Tooltip** | `use_tooltip`, `use_tooltip_trigger`, `use_tooltip_trigger_state` | — | — | `/doc/tooltip` |
| **Menu** | `use_menu`, `use_menu_trigger`, `use_menu_trigger_state`, `use_menu_item`, `use_menu_section` | — | — | `/doc/menu` |
| **Listbox** | `use_listbox`, `use_option`, `use_listbox_section` | — | — | `/doc/listbox` |
| **Combobox** | `use_combobox` | — | — | `/doc/combobox` |
| **Calendar** ¹ | `use_calendar_state`, `use_calendar_grid`, `use_calendar_cell`, `use_range_calendar`, `use_date_picker`, `use_date_field`, `use_date_segment`, `use_time_field` | — | `DateSelector`, `DateTimeInput` | `/doc/calendar` |

¹ Calendar is not yet routed in `routes.rs`; see "Remaining Phase 1 closeout" above. The
existing `DateSelector` / `DateTimeInput` content is currently reachable at
`/doc/components/date-time` and will move under `/doc/calendar/component` when the concept
route lands.

Note: Tooltip, Menu, Listbox, and Combobox are listed as multi-layer because they have multiple related hooks that
warrant a concept overview even without atoms/components yet. When atoms or components are added, they naturally gain
layer sub-pages.

### Standalone Pages

| Item | Type | URL |
|------|------|-----|
| `use_label` | hook | `/doc/hooks/use-label` |
| `use_breadcrumbs` | hook | `/doc/hooks/use-breadcrumbs` |
| `use_meter` | hook | `/doc/hooks/use-meter` |
| `use_toolbar` | hook | `/doc/hooks/use-toolbar` |
| `use_tree` | hook | `/doc/hooks/use-tree` |
| `use_enter_animation` | hook | `/doc/hooks/use-enter-animation` |
| `use_exit_animation` | hook | `/doc/hooks/use-exit-animation` |
| `use_form_validation_state` | hook | `/doc/hooks/use-form-validation-state` |
| `use_form_validation` | hook | `/doc/hooks/use-form-validation` |
| `use_form_reset` | hook | `/doc/hooks/use-form-reset` |
| `DismissButton` | atom | `/doc/atoms/dismiss-button` |
| `Stack` | component | `/doc/components/stack` |
| `Skeleton` | component | `/doc/components/skeleton` |
| `App Bar` | component | `/doc/components/app-bar` |
| `Drawer` | component | `/doc/components/drawer` |
| `Alert` | component | `/doc/components/alert` |
| `Toast` | component | `/doc/components/toast` |
| `Kbd` | component | `/doc/components/kbd` |
| `Typography` | component | `/doc/components/typography` |
| `Icon` | component | `/doc/components/icon` |
| `Callback` | component | `/doc/components/callback` |
| `Tiptap Editor` | component | `/doc/components/tiptap-editor` |

### When a Standalone Item Gains a Concept Page

A standalone item gets promoted to a concept page when:
- It gains an implementation at a second layer (e.g., a hook gains an atom), OR
- A group of related standalone hooks reaches 3+ members.

Until then, it stays standalone. Don't create empty concept pages speculatively.

### Edge Cases

- **Collapsible / Disclosure**: The hook is `use_disclosure` (WAI-ARIA pattern name) but the component is `Collapsible`
  (user-facing name). The concept page is named "Collapsible" with a note explaining the ARIA pattern name.
- **DismissButton**: A utility atom used inside Popover and Modal. Stays standalone — it's not a user-facing concept.
- **`use_prevent_scroll`**: Appears in both Interactions and Overlays. Primary home is Interactions
  (where its deep-dive page lives). Overlays links to it.

---

## Sidebar Navigation

### Structure

Top-level groups remain the same (Getting Started, Interactions, Focus, Overlays, Input, Data Display, Layout,
Feedback, Navigation, General). The change is how items within groups are displayed.

**Behavioral domain groups** (Interactions, Focus, Overlays) render the domain header as a
clickable link to the domain overview page AND keep their member hooks expanded beneath as
sidebar entries. This deviates from the original "single link" intent but preserves
discoverability of individual hooks at a glance. **Selection** is the lone exception: it
appears as a single link with no children because none of its hooks have dedicated pages yet.

**Category groups** (Input, Data Display, Layout, Feedback, Navigation, General) remain as
sidebar organizers. Within them:
- **UI concepts** (multi-layer) appear as a single entry without badges, linking to the concept overview.
- **Standalone items** appear with their layer badge (`[hook]`, `[atom]`, `[comp]`) rendered
  by the `DocBadge` component in `doc_layout.rs`.
- Concepts are listed first, then standalone items. Both sub-groups are alphabetical.

### Example: Input section

```
Input                      -> /doc/input
  Button                   -> /doc/button
  Checkbox                 -> /doc/checkbox
  Color                    -> /doc/color
  Combobox                 -> /doc/combobox
  Listbox                  -> /doc/listbox
  Radio                    -> /doc/radio
  Select                   -> /doc/select
  Slider                   -> /doc/slider
  Text Field               -> /doc/text-field
  Toggle                   -> /doc/toggle
  [comp] Date & Time       -> /doc/components/date-time
  [comp] Tiptap Editor     -> /doc/components/tiptap-editor
  [hook] use_label         -> /doc/hooks/use-label
```

### Example: Interactions section

```
Interactions               -> /doc/interactions
  use_press                -> /doc/interactions/use-press
  PressResponder           -> /doc/interactions/press-responder
  use_hover                -> /doc/interactions/use-hover
  use_move                 -> /doc/interactions/use-move
  use_keyboard             -> /doc/interactions/use-keyboard
  use_interact_outside     -> /doc/interactions/use-interact-outside
  use_scroll_wheel         -> /doc/interactions/use-scroll-wheel
  use_prevent_scroll       -> /doc/interactions/use-prevent-scroll
  Drag & Drop              -> /doc/interactions/dnd
```

### Example: Selection section

```
Selection                  -> /doc/selection
```

---

## DemoShell

### Purpose

A book-ssr component providing consistent visual presentation for hook and atom demos. It replaces ad-hoc inline
styles currently scattered across hook pages.

### Location

`examples/book-ssr/src/pages/documentation/demo_shell.rs`

### What it provides

- Container styled via the `.demo-shell` and `.demo` CSS classes defined in
  `examples/book-ssr/style/demo-classes.scss` (border, padding, border-radius, theme-aware via
  CSS custom properties).
- Optional title rendered above the demo area.
- Optional `data-demo-description` attribute consumed by the markdown middleware to produce
  `*[Interactive Demo: <description>]*` placeholders in the markdown export.
- Optional collapsible "View source" / "View styles" `<details>` block when a `source` prop
  is provided; renders the source via the leptonic `Code` component and includes the demo
  CSS via `include_str!` so readers can reproduce the styling.

### What it does NOT provide

- No styling for elements inside it. Hook and atom outputs remain unstyled. Pages style
  individual elements inside the shell with their own CSS classes (see
  `style/demo-classes.scss` for the conventions used by existing demos).

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Children` | required | Demo content |
| `title` | `Option<&'static str>` | `None` | Optional title rendered above the demo |
| `description` | `Option<&'static str>` | `None` | Surfaced as `data-demo-description`; consumed by the markdown middleware to produce `*[Interactive Demo: <description>]*` |
| `source` | `Option<&'static str>` | `None` | When present, renders a "View source" / "View styles" `<details>` block beneath the demo |

---

## Cross-Referencing Rules

1. **Concept overview pages** link to all their layer deep-dive pages in the "Choose Your Layer" section.
2. **Layer deep-dive pages** link back to their concept overview in the first sentence.
3. **Layer deep-dive pages** link to sibling layer pages in a "See Also" section at the bottom.
4. **Hook pages** link to other hooks they compose (e.g., `use_button` links to `use_press`, `use_hover`,
   `use_focus_ring`).
5. **Atom pages** link to the hooks they wrap in the "Hooks Used" section.
6. **Behavioral domain pages** link to all their member hooks.
7. **Shared hooks** (e.g., `use_prevent_scroll`) have one primary home; other pages link to it.

---

## Interactive Example Guidelines

### When to use interactive demos

- **Always** for: user interaction (press, hover, focus, drag), visual state changes (disabled, selected, variants),
  overlays/modals.
- **Code-only acceptable** for: static structural patterns (how to compose atoms), CSS variable listings, API
  reference tables.

### Standard pattern

Every interactive demo follows this order:

1. `<Code>` block showing the relevant code (using `indoc!()` macro).
2. Interactive demo immediately after, rendered inline.

### Demo conventions

- Demos are self-contained: all state is defined within the page component.
- Demos show state feedback: "Is pressed: true", event logs, visual indicators.
- Use the CSS classes from `examples/book-ssr/style/demo-classes.scss` for consistent styling
  (not inline style strings).
- Event logs use `ringbuf::HeapRb` capped at 50 entries.
- Include a "Disabled" toggle when the concept supports disabling.
- Demos must be keyboard-accessible (Tab, Enter, Space, Escape where applicable).

---

## Migration Strategy

### Phase 1: Infrastructure

- **Calendar concept route** - listed in the Multi-Layer UI Concepts table but not yet in
  `examples/book-ssr/src/routes.rs`.
- **Standalone hook routes** for `use_form_validation_state`, `use_form_validation`,
  `use_form_reset`, `use_enter_animation`, `use_exit_animation` - listed in the Standalone
  Pages table but not yet in `routes.rs`.
- **`infer_layer::CONCEPTS` list** in `examples/book-ssr/src/markdown.rs` must be updated
  whenever a new concept is added; the markdown middleware consults it to classify page URLs
  as Concept / Hook / Atom / Component for the LLM index.
- **DismissButton placement decision.** Currently nested under `/doc/overlays/dismiss-button`
  via `mod overlays::dismiss_button` in `routes.rs`. The Standalone Pages table proposes
  `/doc/atoms/dismiss-button`. Pick one and unify.

### Phase 2: Concept-by-concept migration

For each concept (one at a time):

1. Create the concept overview page.
2. Move conceptual content out of hook/atom/component pages into the overview.
3. Replace removed content with "one sentence + link to overview".
4. Add "See Also" cross-links to each layer page.
5. Ensure each layer page has proper Input/Return/Props tables.
6. Replace inline styles with `DemoShell` and the CSS classes from `style/demo-classes.scss`.

### Phase 3: Quality pass

1. Audit all atom pages for empty/`"..."` content and fill them.
2. Ensure every hook page uses the shared `style/demo-classes.scss` classes instead of
   ad-hoc inline styles.
3. Add demos to pages that only have code blocks.
4. Verify all cross-links.

### Migration priority

1. **Button** — has all three layers, well-developed, good reference implementation.
2. **Slider** — has all three layers, atom page is well-developed.
3. **Modal / Popover** — complex, multi-hook, high value.
4. **Remaining Input** — Checkbox, Radio, Toggle, Select, Text Field.
5. **Focus domain** — group hooks under domain overview.
6. **Interactions domain** — group hooks under domain overview.
7. **Layout** — Tabs, Separator, Collapsible.
8. **Feedback** — Progress, Chip.
9. **Navigation** — Link, Anchor Link.
10. **Standalone hooks** — apply DemoShell, quality improvements.
11. **Standalone components** — verify quality.
