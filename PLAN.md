# Leptonic: working plan

The only todo list of the repository: open work of the library (`leptonic/`, `leptonic-theme/`, `testing/`) and
the book (`examples/book-ssr/`, section "Book"). Keep it to open items: finished work goes to
`documentation/history.md`, long-term rules and knowledge elsewhere:
- guiding decisions and API conventions (C1–C15): `documentation/conventions.md`;
- implementation patterns: `documentation/{hooks,atoms,components}-implementation.md`;
- Leptos/browser/tooling pitfalls: `documentation/lessons.md`;
- consumers (crudkit, agnite dev-ui) and what they depend on: `documentation/consumers.md`;
- the 2026-10-05 audit's raw findings ("audit §n"): `documentation/audit-2026-10-05.md`.

## Waiting on the user

- **leptos-styles suggestion:** an `add_reactive_unchecked` method; today an always-present reactive unchecked value
  needs `add_optional_unchecked(prop, move || Some(..))`. leptos-css lacks `Margin::all`-style helpers.
- **leptos-tiptap suggestion** (2026-10-06): attributes for the editable element (tiptap's `editorProps.attributes`;
  0.10's `UseTiptapEditorInput` has none), so it can be labelled (`aria-label`/`aria-labelledby`; leptonic's
  `TiptapEditor` labels only its group).
- **leptos-element-capture suggestions:** `PartialEq`/`Eq`/`Hash` (identity) for `CapturedElement` (the collection
  item registry tracks registration ids instead); `try_get_untracked` for deferred callbacks (audit §2 B5).
- **Leptos bug report** (2026-10-06): reactive_graph 0.2.15 can skip an effect run. A memo recomputed while the
  effect checks a downstream memo doesn't mark the effect dirty (`Observer::is` skip in `MemoInner::update_if_
  necessary`), and is clean when the effect checks it next; if the downstream memo came out unchanged, the effect
  doesn't run. Leptonic reads upstream first ("Effect Read Order" in `documentation/hooks-implementation.md`). Repro
  and issue draft ready: `~/dev/leptos-issue-repros/a-effect-skips-run` (also fails on 0.9.0-beta2): file it?
- **Name of the third layer:** "component" is the styled layer (`leptonic::components`, feature `components`), but
  readers also know it as a Leptos `#[component]` and as a UI element. The book now says "concept" for UI elements
  and "styled component" where needed (2026-10-06). Renaming the layer itself (e.g. `styled`) would remove the last
  ambiguity but changes the public module and feature names: decide.
- **Leptos `erase_components` report** (2026-10-06): attributes spread onto a component (`attr:id`) are lost when
  its root element is replaced in erased builds (details in `documentation/lessons.md`). Repro and issue draft:
  `~/dev/leptos-issue-repros/b-spread-attrs-lost` (passes on 0.9.0-beta2: ask for a 0.8 fix): file it?
- **Leptos bug, listeners leak on rebuild** (2026-10-07, from agnite dev-ui; tachys 0.2.19 and Leptos main of
  2026-03-14): with `erase_components`, attributes spread onto a component (`<Comp {..hook.props.into_attrs()} />`)
  go through `AnyViewWithAttrs`, whose `rebuild` removes nothing of the previous set (it rebuilds against an empty
  state) and drops the new attributes' states: a view rebuilt in place keeps the disposed owner's handlers (which
  panic when they run) and loses the new ones. Repros: `component_spread_rebuild_known_issues`
  (`BROWSER_TEST_KNOWN_ISSUES=1`) and `~/dev/leptos-issue-repros/c-spread-listener-leak` with an issue draft
  (passes on 0.9.0-beta2: ask for a 0.8 fix): file it? Meanwhile atoms avoid needing such spreads (`VirtualList`'s
  `is_focusable`).
- **Leptos bug, keyed `For` moves kept rows** (2026-10-07): the keyed diff moves a kept item whose relative order
  didn't change (items removed before it, added after it), which collapses a text selection in it (why
  `render_visible_items` mounts rows itself). Repro and issue draft: `~/dev/leptos-issue-repros/d-keyed-for-moves`
  (also fails on 0.9.0-beta2): file it?
- **Commits:** the latest is 842585b (2026-10-07); the working tree holds the changes since.
- **Decisions from the react-aria fidelity review (2026-10-07):**
  - Modal focus on open: RAC's `useDialog` focuses the dialog element. Our `ModalContent` defaults `auto_focus` to
    true (`atoms/modal.rs:195-223`), so the first tabbable element gets focus, which in an alert dialog may be the
    destructive button; `test_dialog.rs` asserts this. Follow RAC?
  - `aria-modal="true"` on the role-less `ModalContent` div (`use_modal`): upstream sets none (WebKit bug 211934),
    and on a div without a role it is invalid ARIA. Drop `use_modal` there (inert outside gives modality)?
  - `LinkButton` renders `<a href role="button">`, so screen readers announce a navigation as a button. RAC has no
    such component. Make it a `Link` with a button look (link role), or document the role?
  - Delete `documentation/audit-2026-10-05.md` once its unabsorbed rest is in this plan (item under "Documentation")?

## Library roadmap

Order: open bugs first, then families (each a complete package: API onto the conventions, missing react-aria
features, re-sync to the current upstream commit, deviation block, fixture + browser test from upstream's tests,
atoms, components, book heads-up), then cross-cutting work. Port react-aria's behavior and structure (cross-check
what it provides before designing); adapt only the API shape (`documentation/conventions.md`).

### Remove the components layer (the user's decision, 2026-10-07)
leptonic becomes hooks + atoms + an optional CSS theme for the atoms (`documentation/conventions.md`). Order, so the
tree keeps compiling:
- Step 1 (default classes `leptonic-<AtomName>`) done 2026-10-07. Note for the theme: `ColorSwatchPicker`/
  `ColorSwatchPickerItem` render through `ListBox`/`ListBoxItem` and carry their classes too (style
  `.leptonic-ListBox:not(.leptonic-ColorSwatchPicker)` or scope the listbox rules).
- [ ] 2. The atom theme in `leptonic-theme` (`documentation/atom-theme.md`): a port of react-aria-components'
  starter styles (`starters/docs/src/*.css`) to `leptonic-theme/scss/atoms/`, entry `leptonic-atoms.scss`. Ported
  2026-10-07: every family with atoms (51 stylesheets, compiled with dart-sass); not yet checked in a browser. The
  user confirmed (2026-10-07) to keep and maintain it; the book won't use it (its own styling). Open:
  - A showcase to check it visually (light and dark, every family), e.g. an `examples/atom-theme` app with the
    starter's markup per family; then fix what's off. Not the test-app (the user, 2026-10-07: it needs no theme).
  - Atom gaps the stylesheets need (else those rules don't apply; done 2026-10-07: `ListBox`/`GridList` root
    `data-empty`/`data-focused`/`data-focus-visible`/`data-layout` (+ `data-orientation` on `ListBox`), section
    heading default classes, `Menu` `data-empty`, `MenuItem` `data-selection-mode` + public `MenuItemCtx` state,
    `GridListItem`'s gridcell `display: contents`, `TabList` `data-orientation`, `TabPanel` focus ring,
    `ComboBoxButton` `data-hovered`/`data-disabled`; `ModalBackdrop`'s `--page-*`/`--visual-viewport-*`):
    `Table` `data-focus-visible`, `TableBody` `data-empty`,
    rows/cells/column headers/resizer `data-focus-visible`; custom column header content (a flex wrapper, sort
    indicator: the resizer rules assume it); a `SelectionIndicator` atom (sliding tab indicator), `TabPanels`;
    `ProgressBarFill`/`MeterFill` exposing `--percent`; `DateInput` with
    `part` rendering `slot="start"`/`"end"`; `CalendarCellButton` children as a function of the cell state (range
    calendar: selection start/end, hovered, pressed, the formatted day); a `ToastQueue` `wrap_update` hook (view
    transitions); the table's selection checkbox and `GridListItem`'s as `Checkbox` atoms; links in list
    box/menu/grid list/tag items (`data-href` rules dropped).
  - Decide: `ColorPicker` renders no element (the theme's `.color-picker` trigger is the app's class, and a
    `leptonic-Button` trigger brings button looks it doesn't undo); calendar buttons and the toast close button are
    quiet only with `attr:data-variant="quiet"` (the starter's components set it), maybe quiet by default in the
    theme; the `Disclosure` trigger is a `Button` (the theme undoes button looks).
  - Not ported (no atoms yet): `CommandPalette`, `DropZone`, `NavigationTree`, `SegmentedControl`, `Sheet`,
    `TokenField`, `Tree`. The component stylesheets go with step 4.
  - What becomes of each component (decided 2026-10-07):
  - covered by atoms already, the component goes: `Button`, `Link`/`LinkButton` (atoms `Link`, `AnchorLink`,
    `LinkButton`), `Checkbox`, `Radio`, `Switch`, `TextField`, `NumberField`, `Slider`, `Select` (incl. multiple
    selection: `SelectMode`), `Tabs`/`Tab`, `Table`, `Meter`, `ProgressBar`, `Separator`, `Modal`, `Popover`,
    `DateSelector`/`DatePicker` (Calendar/DatePicker atoms), `ColorPicker`, `Collapsible` (`Disclosure`),
    `Toasts`/`ToastRoot` (`ToastRegion` + `Toast` atoms on a `ToastQueue`), `Kbd`/`KbdShortcut` (`ShortcutKeys`
    for a `Shortcut` in the platform's form, `Keys` for literal keys);
  - layout and decoration only, gone (plain markup + CSS; the book shows recipes): `Card`, `Stack`, `Grid`, `Tile`,
    `Skeleton`, `Typography`, `AppBar` (a `<header>`), `Drawer` (modal atoms + CSS), `Alert` (`role="alert"`
    markup), `Icon` (use `leptos_icons`), the transitions (atoms expose `data-entering`/`data-exiting` like RAC;
    animate with CSS);
  - `Root`: gone; `ThemeProvider` + `ToastRegion` cover it, `--leptonic-vh` is replaced by `dvh` units;
  - `Code`: gone; `utils::syntax_highlight` (feature `syntax-highlight`) and the clipboard util stay;
  - `Chip`: gone; the `TagGroup`/`Tag` atoms replace it (done 2026-10-07);
  - `TiptapEditor` (feature `tiptap`) and `SanitizedHtml` (feature `sanitize`): gone with their features (use
    `leptos-tiptap` / `ammonia` directly).
- [ ] 3. The book moves off `leptonic::components` (509 files: its shell, kit and every demo) onto atoms + the
  atom theme or its own styles; component tabs become styling sections of the atom pages (book session).
- [ ] 4. Delete `leptonic/src/components/`, the feature `components` (and features only it needed), the component
  stylesheets (`testing/` and `leptonic/tests/` use no components, but the test-app depends on `features =
  ["full"]`: narrow it); drop the component items in
  this plan (themed `Breadcrumbs`/`ToggleButton`/`DateRangePicker`, the legacy `TableHeaderCell`, `Select`
  search styling, ...).

### Consumer requests
- [ ] Virtualizer, rest (see history: `ListBox` and `VirtualList` done 2026-10-07): virtualized `GridList`, `Table`,
  `ComboBox`/`Select` popovers and menus (RAC's virtualized `GridList`/`Table`/`ComboBox`/`VirtualizedMenu` tests;
  no consumer asks yet; dev-ui's collections are all short), sections/headers in virtualized lists,
  `GridLayout`/`TableLayout`/`WaterfallLayout`, drop targets. Check `VirtualList` with dev-ui's load (20,000 lines,
  10–100 appended per second): the collection is rebuilt per append, the rebuild is O(n²) and every layout pass
  clones the whole collection (see the fidelity review's bugs below).
- [ ] Tree tables (agnite dev-ui, 2026-10-07): react-aria-components' expandable `Table` rows (`treeColumn`, nested
  rows, `expandedKeys` + `onExpandedChange`, ←/→ collapse/expand, `role="treegrid"`, `aria-level`/`aria-expanded`/
  `aria-setsize`/`aria-posinset`): `TableCollection` rows with child rows, `expanded_keys` + `set_expanded_keys` on
  `Table` (C4), an expander button atom for the tree column's cells.

### Bugs and review leftovers
- [ ] From crudkit (2026-10-05, not reproduced): in a `use_table_cell` cell with `CellFocusMode::Child`, Enter on a
  `Button` atom (not the cell's first child) first moves focus to the cell's first focusable child, then the
  button's press fires. Repro: a fixture row with two buttons in one cell.
- [ ] Review of 2026-10-07, open: toast focus tracking by key instead of index (a new toast above the focused one
  shifts it; as upstream); `use_table` replaces instead of merging `aria-describedby` (latent until the grid has a
  description); `use_landmark` tests for nested landmark order, the duplicate-role warnings, `LandmarkController`,
  browser tab/window toggling.
- [ ] Test gaps from the 2026-10-06 overlay/link review: `aria_hide_outside` MutationObserver cases (elements added
  outside, into hidden containers, inside a target, reparented, top-layer) and "unhide after reorder"; popover
  reopened without its dialog (containment reset); link hover/focus/press data attributes and Enter; `AnchorLink`
  (scroll, hash without history entry); disabled `LinkButton`; disclosure group `on_expanded_change`, nested groups,
  focus ring.
- [ ] `use_breadcrumbs`' doc says its props and `aria_label` belong to the navigation landmark, while the attrs type
  and the atom put them on the `<ol>` (upstream: `navProps` on the `<nav>`): check and align.
- [ ] Legacy `TableHeaderCell` always attaches `use_press`, so static table headers (the book's documentation tables)
  are press targets.
- [ ] `classes="a b"` compiles but panics during SSR (`ClassName::from(&'static str)` in leptos-classes, the user's
  crate): accept/split it or reject it at compile time.
- [ ] **Bugs from the react-aria fidelity review (2026-10-07; nine read-only passes against react-spectrum
  @ 99e6102368).** Most severe first. "Verified" means checked in the code and upstream, or reproduced natively.
  Each fix also gets the regression test named.
  - High, verified: `use_press` registers its document `keyup` listener in the bubble phase (`use_press.rs:1066`;
    upstream uses the capture phase, usePress.ts:424-429). A keyup handler that stops propagation (`use_keyboard`
    does so by default once `on_key_up` is set, e.g. via `focusable_press.rs:127`, or any ancestor) leaves a
    keyboard press stuck: no `on_press`, `is_pressed` stays true, and later presses are ignored. Test with
    `on_key_up` on the pressed element.
  - High, verified: `use_press`'s `handle_dragstart` cancels with a `PointerEvent` that was never dispatched
    (`use_press.rs:1443-1447`). Its `current_target` is null, so `coordinates()` → `expect_current_target()` panics
    whenever `on_press_end`/`on_long_press_end` is set (toggle, calendar cells, spin buttons) and the user drags a
    draggable `<a>`/`<img>` or text inside a pressable. Fix: cancel with `EventRef::Synthetic(&current_target)` after
    a containment check, as upstream does.
  - High, verified: slider and color slider thumbs swallow the arrow keys of the other axis. `use_move` calls
    prevent default and stop propagation before zeroing the off-axis delta (`use_move.rs:740-749`), and the thumb
    passes its orientation as the axis (`use_slider_thumb.rs:328`). Upstream's `useMove` has no axis filter and
    `useSliderThumb` steps on all four arrows. Mirror useSliderThumb.test "can be moved with keys (vertical)".
  - High, verified: a quick touch tap on a range calendar cell selects a one-day range instead of starting one
    (`use_calendar_cell.rs:379-395`). The touch-timer branch sets the anchor, and the next branch re-reads it live and
    completes the range; upstream reads the `anchorDate` captured at render. Capture the anchor at the start of
    `on_press_up`; add a touch browser test.
  - High, verified: `NumberParser` breaks for locales whose default digits aren't Latin (ar-EG, fa, bn, mr, ...).
    The separator probe looks for an ASCII "111" and takes the first non-ASCII-digit character as the group
    separator (`number_parser.rs:276-311`), so a native digit becomes the "group separator" and "١٢" parses as 2. The
    `sep_end - 1` byte slice panics on a multi-byte decimal separator. Fix ahead of the full re-port; add ar-EG
    unit tests.
  - High, verified: `Filter::contains`/`starts_with`/`ends_with` slide windows of `substring.len()` *bytes*
    (`utils/filter.rs:136-171`), so `contains("café", "cafe")` and `contains("Müller", "mul")` are false. The doc
    example at :116 claims the opposite (doctests aren't run). Use windows by `char` count (upstream: JS string
    length) and add umlaut/accent tests. This hits crudkit's German combo boxes.
  - High, reproduced natively: `commit_custom_value` in the combo box (`use_combobox_state.rs:419-431` with
    :541-555) resets `last_reported` and sets the value to `[]`. The selection callback then takes the "picked again"
    path and clears the input, and `on_change` never fires. `revert` without a selection does the same. As a result,
    `allows_custom_value` is unusable in single mode. Upstream (useComboBoxState.ts:536-549) keeps the text and calls
    `onChange(null)`. Add `*_state` unit tests (commit, revert, custom value, `menu_trigger`, show all).
  - High, verified: the combo box's validation never reaches its input (`use_combobox.rs:394-398`: `validation:
    None`, `ValidationBehavior::default()`). `FieldError` shows the text field's own, empty state, `validate` messages
    never appear, and native mode never blocks submission. Pass `validation: Some(state.validation)` and the real
    behavior (upstream: `privateValidationStateProp`, useComboBox.ts:326).
  - High, reproduced natively: `Collection::build` is O(n²) (`collection.rs:482`, `keys.contains` on a `Vec`).
    20,000 items take 1.3 s in a debug build, and dev-ui rebuilds 20,000 lines up to 100 times a second. Use a
    `HashSet`.
  - High: toggling follow mode in `VirtualList` drops every measured row size. `anchor_to` changes whenever the user
    scrolls away from the end (`atoms/virtualizer.rs:291-297,326-332`), which makes `should_invalidate_everything`
    true (`list_layout.rs:558`): content jumps and every row is measured again. Leave `anchor_to` and
    `scroll_end_threshold` out of the check, and test from the middle of a variable-height log.
  - Medium: `ThemeProvider` (`atoms/theme.rs:94,100`), `TagGroup` (`atoms/tag_group.rs:156-157`) and
    `atoms/virtualizer.rs:185` call `provide_context` in the component body, so the contexts leak to later
    siblings. A second `ThemeProvider` never sets `<html data-theme>`, and a field after a `TagGroup` picks up its
    Label/Field context. Use `<Provider>`.
  - Medium: in a tree, ArrowRight on an expanded row focuses its expand button. `data-react-aria-prevent-focus`
    (useTreeItem.ts:77, checked in isElementVisible.ts) isn't ported. Port the attribute and the walker check
    (`utils/focusability.rs`), and set it in `use_hidden_select` too, where focusable walks (grid cell child focus,
    FocusScope fallbacks) can land on the hidden select.
  - Medium: unevenly nested table column groups produce duplicate group header rows (`table_collection.rs:281`;
    upstream `buildHeaderRows` shifts shared parents up). Traced by hand, not run. Add a two-level unit test.
  - Medium: table atoms read the collection once, untracked (`atoms/table.rs:326,352,578`, plus `use_grid_cell`'s
    col span/index and `use_table_column_header`). Header text, `allows_sorting`, the select-all column and a kept
    cell's column index go stale when columns change or are reordered, and changes to `selection_mode` (a `Signal`)
    are ignored.
  - Medium: `use_press` leftovers:
    - A descendant that stops `click` still gets the press fired by the 80 ms virtual-click fallback
      (`use_press.rs:1276-1290`; upstream records clicks in a capture listener and cancels).
    - The macOS Meta keyup synthesis is dead code (`:956-964`, `:995-1003`): it returns before the Meta branch and
      would dispatch the stored keydown through a null `current_target`. history.md R1 calls it done.
    - `on_press_up` on keyup ignores the target and `repeat` (`:1023`).
    - The long-press `contextmenu` blocker (`prevent_default_once`, `:1434`) is never removed and suppresses the
      next real context menu (upstream removes it 100 ms after pointerup).
  - Medium: the hidden range inputs of the color area and wheel set `value` as an attribute and listen to `change`
    instead of `input` (`use_color_area.rs:228,245`, `use_color_wheel.rs:184,195`). After one assistive-technology
    adjustment the input is dirty and stops following. `test_color_wheel.rs:86` asserts the attribute, which hides
    this. Use the property + `input`, as the slider thumb does.
  - Medium: color area gradients always stack `[y, x]` (`utils/color.rs:923,1404`), while upstream orders them by
    channel, brightness/lightness on top. Wrong for `y = Hue` or `x = Brightness, y = Saturation`. Add an HSV
    fixture with swapped axes.
  - Medium: slider:
    - `values[index]` panics for a thumb index without a value (`use_slider_state.rs:101-126`).
    - Bound values aren't restricted (`:405`; upstream `restrictValues`), so out-of-range or out-of-order values
      render as they are.
  - Medium: controlled text fields keep text their state rejected (`use_text_field.rs:488-501`). Re-sync the DOM
    value in `on_input`, as `use_toggle.rs:508` does.
  - Medium: integer number fields:
    - A typed value outside the type's range reverts instead of clamping (`number_parser.rs:116` →
      `number_value.rs:160`).
    - ArrowUp then jumps to the minimum (`use_number_field_state.rs:335-361`).
    - Fix: saturate in `from_decimal`.
  - Medium: server errors don't come back when the server returns an equal error map after the user edited
    (`use_form_validation_state.rs:331-339`; upstream resets on every new object).
  - Medium, likely (verify in a browser): in a Validate + Native number field, `use_native_range_validation` never
    clears its own custom validity (`use_number_field.rs:653-655`), so Enter after a correction may still be blocked.
    It also creates a probe `<input type=number>` per sync; cache one.
  - Medium, not reproduced: DnD drop-target registration runs a keyboard drag's `get_drop_operation` inside the
    registration Effects (`drag_manager.rs:122,130`, `use_drop.rs:445`, `use_droppable_collection.rs:420`), which
    subscribes them to user signals. A re-registration then loses the current target. Wrap in `untrack`.
  - Medium: SSR calendar without a value or focused date:
    - The client computes its own `today()`, while the server HTML shows the server's month
      (`use_calendar_state.rs:524-530`). tachys doesn't patch server text when hydrating.
    - Across a month boundary (a UTC server, users in the Americas in the evening on the month's last day), cells
      show the server's dates while their `date` signals hold the client's. Runtime effect unverified.
    - Fix: hand the server date to the client, compute after mount, or require `default_focused_value` under SSR.
      Fix the deviation note (:28-30).
  - Medium: `synthetic_blur` leaks a MutationObserver closure per focus of a form element (`synthetic_blur.rs:87,152`
    `forget()`), keeping detached nodes alive.
  - Low, forms:
    - `HiddenSelect` with more than 300 options captures no element (`use_hidden_select.rs:184-202,269-281`), so form
      reset and native validation do nothing (upstream: `inputRef` on the first input). Its hidden `<label>` has no
      text.
    - `SearchField` ignores its `input_type` prop (`search_field.rs:123-127`).
    - `ToggleButton` documents `data-focus-visible` but doesn't render it (`toggle_button.rs:104-115`).
    - The checkbox group's error order follows the `HashMap` (`use_checkbox_group_state.rs:167-174`).
    - `use_form_reset` resets even when the reset event is `defaultPrevented` (`use_form_reset.rs:316-318`).
    - Committing empty text with a binding that rejects `None` leaves `""` (`use_number_field_state.rs:330-334`).
  - Low, overlays:
    - A reopened popover or tooltip starts from the previous opening's position (`use_overlay_position.rs:209`
      never resets it; `popover.rs:423`, `tooltip.rs:242`).
    - `DismissButton` drops `aria_label` when `aria_labelledby` is given (`dismiss_button.rs:139-155`).
    - Enter on a menu item ends with Virtual modality (`use_menu_item.rs:309-321`: set Keyboard after the click, as
      upstream does).
    - Tooltip trigger ids get their prefix twice (`use_tooltip_trigger.rs:189-191`).
    - `use_toast_region` calls `StoredValue::new_local` unconditionally (`:147`); it is safe only inside the Portal.
  - Low, links and tabs:
    - `use_anchor_link` calls `preventDefault` on modified clicks (`use_anchor_link.rs:138`), so Ctrl/Cmd-click
      can't open a new tab.
    - A disabled `use_link` `<a>` loses its link role (`use_link.rs:338-341`: no href, no role; set `role="link"`).
    - `use_disclosure_group_state` skips `on_expanded_change` when it normalizes a bound value (`:116`).
    - `use_tab_list_state` keeps a disabled bound key, though its doc says it is replaced (`:31-32,106`). It also
      reads its binding before `collection`, against "Effect Read Order".
  - Low, collections:
    - Arrow keys onto a link item under `LinkBehavior::Override` return Ignored
      (`use_selectable_collection.rs:267-269,333`; upstream treats them as handled), so ListBox in toggle behavior
      scrolls natively.
    - `Collection::filter` keeps leading and doubled separators (`collection.rs:260-276`).
    - A key reused across tree levels is silently overwritten (`:479-549`).
  - Low, grid/table/tree:
    - Empty tables keep keyboard navigation on (upstream: `collection.size === 0`).
    - Wrapping `GridKeyboardDelegate` loses upstream's virtual dispatch (`table_keyboard_delegate.rs:184,212-218`):
      PageUp/Down never reach the headers, and Ctrl+Home/End from a header returns a cell in row mode.
    - Tree row positions cost O(n²) per expand (`use_grid_list_item.rs:234-248`), and expansion keys are compared
      as strings (`:353`).
  - Low, virtualizer:
    - Section headers keep the 48 px estimate and are never measured (`list_layout.rs:442-465`).
    - An RTL horizontal `scroll_to` writes a positive `scrollLeft` (`use_scroll_view.rs:383`).
    - Each layout pass clones every `Node`/`LayoutNode` and deep-compares in `is_valid` (`list_layout.rs:232,247,329`;
      use `Arc` + pointer comparison).
    - Visible rows search `root.visible` again each scroll frame (`atoms/virtualizer.rs:605-609`; use a
      `Memo<LayoutInfo>`).
    - A dead `Weak` child owner stays behind per mounted row (`:533`; cost unverified).
  - Low, DnD: `use_drag` with `is_disabled` still swallows Enter and keeps its pointerdown handler
    (`use_drag.rs:347,368,455`).
  - Low, color:
    - `ColorAreaState::display_color` keeps alpha (`use_color_area_state.rs:194`).
    - The take-once color parts panic on remount inside `<Show>` (`color_thumb.rs:102`, `color_slider.rs:202,247`,
      `color_wheel.rs:164`).
    - Slider input attributes format through `f64` (f32 0.1 renders as "0.10000000149011612"; i64 precision is lost
      against C15).
    - `css_arguments` rejects `rgb(r,g,b,a)` and `rgba(r,g,b)` (`color.rs:597`).
  - Low, focus:
    - `utils/focus.rs:85-91` skips SVG elements.
    - `text_selection.rs:153-176` keeps its state in a DOM attribute and can strip an app's own `user-select: none`.
    - `use_focus_within` uses a bubble-phase `focusin` (`:275`; upstream: a capture-phase `focus` listener).
    - FocusScope containment Tab doesn't `select()` inputs, and calls preventDefault even when focus is outside the
      scope (`focus_scope.rs:274-292`).
  - Low, dates:
    - Segment RTL styles are fixed at creation (`use_date_segment.rs:683-692`).
    - `DateTimeFormatter::format()` cuts localized short names too (`date_time_formatter.rs:421-424,470-473`: "周四"
      becomes "周"), not only the English fallback. It also isn't localized: it joins with spaces, defaults `hour12`
      to false, and ignores time zone and era. Port it onto ICU field sets or narrow the API.

- [ ] Atoms forward their element (react-aria-components forwards `ref` everywhere): a `node_ref` prop on every atom
  rendering an element (done for `Input`/`TextArea`, 2026-10-07).

### Families
- [ ] **Text inputs and fields:** re-port `NumberParser` fully from `@internationalized/number` (other numbering
  systems when pasting, literal stripping from formatted parts, unit plurals, accounting sign, fr-FR/Swiss group
  characters, percent rounding) with a formatter that knows currencies/units/percent patterns from CLDR
  (`format_percent` appends `%`; German expects `50 %`); `NumberFormatOptions`' currency and unit are strings (typed
  values instead). Also: non-Latin default digits (a bug, see above); Arabic `,`/`،` accepted as the decimal
  separator; keep the typed numbering system when formatting (`getNumberingSystem`). `number_formatter.rs` has no
  upstream drift: only bump its marker. The `Select` search still styles through the legacy `leptonic-input` classes
  (`input.scss`).
- [ ] **Forms** (fidelity review 2026-10-07):
  - RAC's `CheckboxField`/`CheckboxButton`, `RadioField`/`RadioButton` and `SwitchField`/`SwitchButton` aren't
    ported. They give a single checkbox, radio or switch its own Description and FieldError through `FieldContext`
    (C14); the hooks already return the props. Port them, or list them as omitted in the atoms' deviation blocks.
  - A bound number value isn't snapped for display (`use_number_field_state.rs:282-291`).
  - `%` is accepted in Decimal style (`number_parser.rs:104,186`; upstream treats it as invalid): document or align.
  - `UseCheckboxGroupItemInput.options.validate`/`is_invalid`/`validation_behavior` are silently ignored
    (`use_checkbox_group.rs:246`, `use_toggle.rs:404`).
  - The number field steppers and `SearchFieldClearButton` add a second `use_hover` on top of `use_button`'s and
    lack `data-focused`/`data-focus-visible` (`number_field.rs:321`, `search_field.rs:218`).
  - `use_slot_id` is unused (`slot_id.rs:62`).
  - The `use_form_validation_state.rs` deviation block is stale: `builtinValidation` exists (:168), and the single
    `name` isn't mentioned.
  - `number_parser.rs` and `use_spin_button.rs` carry the legacy line instead of `// Upstream:`.
  - Tests:
    - NumberParser: 14 of ~66 upstream cases (add ar-EG, currencies, units, percents, round trips, out-of-range
      integers).
    - Browser:
      - number field wheel stepping, and "no grouping characters in de-DE";
      - RTL radio arrow keys;
      - form reset for Checkbox/CheckboxGroup/Radio/Switch;
      - implicit submit with Enter from a checkbox, radio or switch;
      - server errors coming back after an edit;
      - Validate mode + Enter submit;
      - checkbox group realtime re-validation.
- [ ] **Button, link, tabs, disclosure, separator, meter** (fidelity review 2026-10-07):
  - `Button` pending state (RAC `isPending`, 5 upstream tests):
    - press is disabled, but focus and hover stay;
    - `aria-disabled` and `data-pending`;
    - a submit button becomes `type=button`;
    - the progress id joins `aria-labelledby`, plus an announcement.
  - `atoms/button.rs` and `atoms/tabs.rs` have no `// Upstream:` header or deviation block. Document `LinkButton`
    and the RAC props `Button` omits: `on_press_start/end/up/change`, focus/key events, `auto_focus`,
    `prevent_focus_on_press`.
  - `LinkButton`'s ARIA props are `Option<Signal<Option<_>>>`. Its role is under "Waiting on the user".
  - `use_button`/`use_link` add `rel="noopener"` for `_blank`: document it as an addition.
  - Tabs atom:
    - `Tab` has no `is_disabled` (`tabs.rs:155`).
    - A force-mounted unselected panel keeps its role, id, `aria-labelledby` and tabindex (RAC drops them,
      Tabs.tsx:623-625).
    - The `collection` prop and the children can drift apart.
    - No `TabPanels`, `SelectionIndicator` or link tabs.
  - `use_tab_panel`:
    - Labels each panel by its own tab, where upstream uses the selected tab. Keep it and document it; the header
      says "no deviations".
    - Has no `aria-describedby`/`aria-details`.
    - Takes `TabListData`, while `use_tab` takes `TabListItemData`.
  - C9 `tab_props`/`tab_panel_props`; C13: meter/progress `percentage` is 0..100, not a `Fraction`.
  - Small API gaps:
    - `ThemeProvider` uses `leptonic-theme-provider` instead of the default class and has no `classes` prop.
    - The `Separator` atom lacks `id`/`aria_labelledby`.
    - `use_visually_hidden`'s Props derive `Clone`, and `is_focusable` is a plain `bool`.
    - Toolbar/Tabs/Separator `orientation`/`keyboard_activation` aren't signals (C11).
    - `UseTabReturn`/`UseSeparatorReturn` lack `Debug`.
    - The `Breadcrumb` atom re-implements `use_breadcrumb_item` (`atoms/breadcrumbs.rs:104-110`) instead of
      calling it, so that hook is reached by no fixture.
  - Tests:
    - VisuallyHidden: none yet ("hides element", "unhides if focused and focusable").
    - Tabs: RTL, force mount (`inert`), data attributes, adding/removing tabs while keeping or losing the selection,
      nested tabs, a controlled key, `is_disabled`.
    - Disclosure: controlled, disabled but expanded, `beforematch` while controlled and closed,
      `data-focus-visible-within`.
    - Button: data attributes and form props; `use_button`'s input element, `rel` and `target`.
    - Separator: a DOM test.
    - Toolbar: RTL vertical, "all the aria example children".
  - The breadcrumbs' localized default label goes with "Localized strings". The themed `Breadcrumbs`/`ToggleButton`
    components are dropped with the components layer.
- [ ] **Calendar and date picker:**
  - calendar test gaps: `pageBehavior: single` and the 2-week view, held arrow keys, announcements, `weeks_in_month`,
    commit behaviors `Clear`/`Reset` and on focus leaving;
  - the format options as signals (C11: `hour_cycle`, `granularity`, `hide_time_zone`, `should_force_leading_zeros`,
    `placeholder_value`, `max_granularity`); several names per validation state (react-aria `name: string |
    string[]`: server errors under the range's end name);
  - date picker test gaps: close on select `false`, the range's placeholder time on closing, a disabled picker,
    required/FieldError for pickers and time fields, segment hover/focus-visible;
  - range formatting with shared fields ("June 1 – 15, 2024": ICU4X has none yet); a themed `DateRangePicker`;
  - fidelity review 2026-10-07:
    - The calendar layout props (`visible_duration`, `first_day_of_week`, `page_behavior`, `selection_alignment`,
      `weeks_in_month`) are fixed at creation (C11). Upstream re-aligns when `visibleDuration` changes (RAC test).
    - Cell labels build an ICU formatter per evaluation, ~42 per hover during range selection
      (`use_calendar_cell.rs:252-284,469-478`). Share one memoized formatter in `CalendarData` and use `Memo`s.
    - BC dates get no era (upstream `getEraFormat`; `format_date` ignores `DateTimeFormatOptions.era`).
    - No autofill: RAC's `HiddenDateInput` is a hidden `<input type=date|datetime-local autocomplete>`.
    - The picker's contexts reach its popover (RAC `clearContexts`, DatePicker.test.js:340).
    - `DatePickerButton` has no `data-pressed` while open.
    - The range picker prefers popover selections over a complete value
      (`use_date_range_picker_state.rs:200-207,389-396`).
    - Ported files without a deviation banner: `calendar/states.rs`, `calendar/utils.rs`,
      `datepicker/{format,placeholders,types}.rs`, `date_time_formatter.rs`, `list_formatter.rs`. Document the
      first-day override in `align_*`, the ICU field sets, era stripping, `resolve_hour_cycle`, `DateValue` and the
      `to_zoned` disambiguation.
    - Hygiene: misplaced doc comments (`use_calendar_state.rs:383`, `date_time_formatter.rs:108`,
      `use_date_picker.rs:72`); `let _ = moved;` (`use_date_field.rs:177-185`).
    - Tests:
      - touch range selection;
      - month/year pickers (none yet);
      - date fields outside en-US in the browser: de-DE order, RTL segments, LRI/PDI time isolation, ja-JP h11;
      - RAC DateField: "Enter does nothing", held keys, reset to placeholders when deleting a partial field, the
        selection not collapsed while another element is focused;
      - `useDatePicker`: a programmatic `setValue` on an empty field;
      - unit tests for `day_periods()`/`eras()`;
      - weak "unchanged" assertions without a settle in `test_calendar.rs:318-321,337-338,364-365,381-383`.
- [ ] **Collections:** tree expansion and tree input (keyboard delegate, Tab navigation, select on press up);
  focus-mode enum names; tag group options; menu item links; per-item `on_action`; combobox item actions/links,
  `aria-labelledby` fallback, missing props (`should_focus_wrap`, `on_open_change`, ...); tab links; grid list
  selection checkbox (labelled by its row); `SelectionManager` cell selection/layout ranges; load more; DnD on the
  collection atoms + drag preview + tree drops; Tree atoms; Autocomplete (with upstream's
  `getPointerType` from `useFocusVisible`, which only `useAutocomplete` reads), then searchable Select/Multiselect
  (search keeps focus, collection unfiltered for the trigger text); legacy `components::select` issues; combo box
  label click shows no focus ring (check upstream first); `atoms::prelude` with `Grid*`/`Table*` once the legacy
  names are gone; table resizing leftovers (cursor overlay while mouse-resizing, resizer `data-focused`/
  `data-focus-visible`/`data-hovered` via focus ring + hover, scrollable ancestor, empty tables).
  - Fidelity review 2026-10-07:
    - Performance: the ICU collator is rebuilt on every `delegate()` read (`use_selectable_list.rs:93-102`, 2–3 per
      arrow key), and `use_contains_filter` builds a `Filter` per item comparison and reads the locale untracked
      (`use_combobox_state.rs:66-75`). Memoize per locale.
    - Combo box:
      - No live announcements on Apple devices (the focused option, the option count, the selection; VoiceOver
        ignores `aria-activedescendant`).
      - No hidden form input (`formValue`).
      - When both `value` and `input_value` are bound, the text is still synced (upstream leaves it to the app):
        nicer, document it.
    - Escape propagates when empty selection is disallowed (upstream swallows it): document. Missing: per-item
      `disabled_behavior`, and reacting to `selection_behavior` changes.
    - The `is_focused_key` doc claims per-item notification (`selection_manager.rs:252`): use a `Selector` or fix
      the doc.
    - Select/ComboBox API:
      - `is_required: bool` (C1).
      - `name`/`validation_behavior`/`is_read_only` are repeated on the state and the hook input (C8).
      - The `SelectValue` placeholder default (RAC: "Select an item").
      - `has_aria_label` is read untracked (`use_select.rs:242`).
      - No `is_open` + `set_open` on Select.
      - The Select root lacks `data-focused`/`data-focus-visible`/`data-required`, the ComboBox root
        `data-required`.
    - ListBox:
      - `collection` and `state` are both optional; giving neither only triggers a `dev_warn`.
      - `ListBoxSection` renders 3 elements.
      - It provides no `SeparatorContext`, so a separator renders an `<hr>` inside `role=listbox` (RAC
        ListBox.tsx:422).
    - Hygiene: inputs without `Debug`; a loose `label_on_click` return; the unused `use_filtered_list_state`
      (`list_state.rs:63`); stale headers `use_option.rs:41-42` (posinset/setsize are implemented),
      `selection_manager.rs:26` and `use_selectable_collection.rs:54` (the `auto_focus` type).
    - GridList/Tag:
      - The row isn't set to `tabindex=-1` in child focus mode with Tab navigation (useGridListItem.ts:436).
      - `use_tag` takes its disabled state from the grid list instead of `disabled_keys`.
      - `GridListItem` drops `description_props`.
      - There is no `GridListSection` atom (so `use_grid_list_section` is unused).
    - Tests:
      - Combo box: `allows_custom_value`, `menu_trigger` focus/manual, commit on blur, required and validation
        (native and aria), multiple selection, disabled keys, sections, Enter with no focused option.
      - Select: multiple, required/native + FieldError, autofill `change` on the hidden select, disabled, an empty
        collection doesn't open.
      - ListBox: sections, replace selection with Ctrl/Shift, `on_action` (Enter and double-click), links,
        horizontal/grid layout + RTL, PageUp/Down, `should_focus_wrap`, `disabledBehavior: selection`, empty state,
        focus after removal.
      - GridList: arrows into row children, Tab navigation, `on_action`, replace selection, links, type-ahead.
      - No fixture uses sections, links, `on_action` or replace selection yet.
- [ ] **Grid, table, tree, DnD, virtualizer** (fidelity review 2026-10-07):
  - Grid and table:
    - `use_grid_cell` lacks upstream's pointerdown tabindex removal (useGridCell.ts:390-410), and the column header
      lacks `useFocusable`.
    - API: `GridState.is_keyboard_navigation_disabled` is a public `RwSignal` (C3/C4);
      `use_table_selection_checkbox(table, key)` and `use_table_select_all_checkbox(&table)` take positional
      arguments (C8); `UseTableColumnResizeInput.aria_label: String` (C2); `atoms/table.rs:41,152` exposes a private
      `ColumnSizes` alias in public props, and `disabled_behavior` is an `Option`.
    - Hygiene: ad-hoc `data-pressed` in GridCell/TableCell/column header (use `flag`); `clone_keyboard_event`
      duplicated, with an `.expect` (`use_grid_cell.rs`, `use_grid_list_item.rs`); nested-dispatch comments
      missing.
    - Optional: `GridRow` runs `use_focus_visible` per row (`atoms/grid.rs:167`); one grid-level signal would do.
  - Tree:
    - `use_tree_state` offers no `ValueBinding` for `expanded_keys` (tree tables need it).
    - `use_tree_item` reads `has_child_items` once, untracked; `expand_button_label` is returned separately.
  - Stale notes:
    - `table_collection.rs:22` points to "PLAN step 11". DnD exists, so drag-button columns can be built now.
    - The `use_table_state.rs`/`use_table_row.rs` omission notes say "trees are grid lists", but tree tables are now
      planned.
    - `use_tree.rs` says "No intentional deviations".
    - `use_table_header_row.rs` has an "API ADDITIONS" heading.
    - English labels are listed under API DIFFERENCES (`use_tree_item`, `use_grid_selection_checkbox`,
      `use_table_selection_checkbox`).
    - The `on_resize_*` HashMap entry gives no reason.
  - DnD:
    - A keyboard drag hides the page with `aria-hidden` instead of `inert` (`drag_manager.rs:624`; upstream
      `shouldUseInert: true`).
    - DnD events lack `Propagation`.
    - API: `get_allowed_drop_operations: Callback<(), _>`, `on_key_down: Callback<SendWrapper<KeyboardEvent>>`,
      the `DragPreview.offset` tuple, and `ListDropTargetDelegate` defaulting to LTR (C5).
    - `use_draggable_item.rs:97-111`: touch + action keeps `aria-describedby`, and `selection_mode` is read
      untracked.
    - `use_droppable_item`'s target is fixed at creation, yet it claims "No intentional deviations".
    - `use_drag.rs:50` still has the React 17 entry.
    - Document the good deviations: wildcard `DragTypes` at drop time, the virtual-pointer OR fix.
  - Virtualizer:
    - Public `RwSignal`s `visible` and `content_size` (C3); a tuple `Callback<(Key, Size)>` (C10); scroll-view
      options that aren't signals.
    - `drop_indicator_thickness` is never read.
    - Undocumented `use_scroll_view` differences: the window offset, no re-measure when scrollbars appear, no
      `onScroll`.
  - Tests:
    - DnD:
      - 3 browser cases against ~110 upstream. Missing: Tab/Shift+Tab cycling, cancel, targets added during a drag,
        the inert/hidden checks, disabled drag/drop, Alt+Enter, drops on items, selection after a drop.
      - Native drags can be synthesized with `DataTransfer` + `DragEvent`, so `test_dnd.rs:13`'s claim that they
        can't be driven is too broad.
      - `use_clipboard` has no test (16 upstream cases).
    - Grid and table: `KeyboardNavigationBehavior::Tab` and RAC's "textfields in rows", RTL, PageUp/Down,
      `escape_key_behavior` None, `SelectionBehavior::Replace`, `should_select_on_press_up`, row/cell actions, empty
      state, removing and re-adding a column, colspan up/down.
    - Tree: RTL, ArrowRight on an expanded row, disabled items, collapsing the parent of the focused row.
    - Unit tests: `use_grid_state::refocus`, `use_tree_state`, nested column groups, `ListLayout`,
      `OverscanManager`.
    - Virtualizer:
      - The "measured variable heights" test never checks for overlap.
      - `test_virtual_list.rs:52` uses fixed 400 ms sleeps.
- [ ] **Interactions and focus** (fidelity review 2026-10-07):
  - Upstream drift to absorb (`scripts/upstream-drift.sh -v`):
    - `use_focus_visible`:
      - keep the modality on window refocus (11cfbcccb; Safari shows a ring after a tab switch);
      - a global `invalid` listener (d478f30f3);
      - the `Reflect.defineProperty` override;
      - `useShowFocusIndicator`;
      - pointer-type tracking (needed by Autocomplete).
    - `scroll.rs` (5804bad7e): the root among the scroll parents, the root's scrollbar/borders, re-scroll after
      centering. Without it, keyboard navigation in scroll-locked overlays can clip items.
    - `use_prevent_scroll` on iOS: `is_scrollable(target, true)`, an `isConnected` guard, iOS && WebKit.
    - `platform.rs`: CriOS/CrMo, `is_firefox`/`is_safari`, the iPad threshold, caching, a header.
    - `open_link.rs`: the Mac WebKit keydown path always, not only with modifiers.
    - Only `--mark-synced` (already absorbed): `focusable_tree_walker.rs`, `use_focus_manager.rs`,
      `shadow_dom.rs`, `use_form_reset.rs` (after its reset fix).
  - `use_press`:
    - Drag out/in uses a global pointermove + bounding rect (`is_over`, `:1182-1215`) instead of
      pointerenter/leave, which is wrong for `display: contents`. Document or port.
    - `handle_click` lacks the `openLink.isOpening` guard.
  - Stale deviation docs:
    - `use_focus_visible.rs:45` claims "No ignoreFocusEvent guard", but the guard is used.
    - `use_focus_manager.rs` says "No defaultOptions merging", but `with_default_accept` exists.
    - FocusScope has a scope-level focusin listener that duplicates the document one (`focus_scope.rs:308-329`).
  - Tests:
    - usePress.test.js:
      - keyup stopped on the element, dragstart, a child stopping the click;
      - virtual click, leave/re-enter, `should_cancel_on_pointer_exit`, pointercancel;
      - links with Space, `on_double_press`, `PressPropagation::Continue`;
      - the element removed mid-press, a contextmenu after a long press.
    - `use_constrained_move`/`MoveAxis` (~300 lines) are untested.
    - FocusScope: input select on Tab, runtime `contain`, cancelling the restore event, Tab out of a restoring
      non-containing scope.
    - Focus visible: virtual modality, window refocus, `invalid`. Focus within: removal of the focused child.
    - Unit tests for `focus_scope_tree.rs` and the platform UA functions.
    - Weak tests:
      - `test_focus_scope.rs:150-167` passes when focus never left.
      - `test_press.rs:88` doesn't re-check after a delay.
      - Immediate reads instead of `wait_for_*` in test_focus_scope (:44-100), test_focus (:80-95),
        test_focus_visible and test_focus_ring.
      - Long-press sleeps sit around the 500 ms threshold.
- [ ] **Overlays, modal, menu, tooltip** (fidelity review 2026-10-07):
  - Modal/dialog:
    - `ModalContent` renders no `DismissButton` when dismissable (RAC Modal.tsx). VoiceOver iOS users then have no
      way out.
    - Its `contain_focus`/`restore_focus`/`auto_focus` are plain bools (C11).
    - `use_dialog`'s "already focused" check isn't shadow-DOM aware (`:208-213`).
    - `use_dialog`'s `tabindex: &'static str` (C12).
    - Focus on open and `aria-modal` are under "Waiting on the user".
  - Popover:
    - No `dir` on the portalled popover (`popover.rs:376-393`; RAC sets it, and our ToastRegion already does), so
      popovers in an RTL subtree render LTR.
    - Closed overlays keep observers and listeners running: `use_element_bounding` per popover/select/combobox,
      including leptos-use's `window_scroll` (`popover.rs:299`); `use_overlay_position`'s resize observers and
      viewport listeners (`:401-442`); `use_viewport_size` in every closed ModalBackdrop (`modal.rs:117`). Create
      them per opening.
    - `runAfterKeyboard` (5a6c32500) is neither ported nor listed, for Popover and ModalOverlay alike.
  - Tooltip:
    - `should_skip_animation` is ignored (`tooltip.rs:228-246`), so a warm-up swap shows two tooltips.
    - Its deviation block wrongly says the animations are omitted.
    - `use_tooltip` returns no `role`.
  - Menu:
    - `MenuSection` renders three elements (`menu.rs:777-786`); RAC renders one `<section>` plus a header via
      HeaderContext.
    - Per-section `selection_mode`/`should_close_on_select` are missing (RAC GroupSelectionManager).
    - Neither is documented.
  - API leftovers:
    - `use_overlay_trigger`'s `show` should be `is_open`.
    - `use_tooltip` takes `state: Option` or `on_open`/`on_close` as alternatives.
    - `use_modal.is_disabled: bool`.
    - The menu trigger's id is a `Signal<String>` read untracked.
    - DismissButton's `on_dismiss` is optional.
    - `ModalBackdrop` adds the class `leptonic-modal-backdrop` (`modal.rs:159`).
  - Stale docs:
    - `popover.rs:36-37`: the animations and submenu triggers it lists as omitted exist. The real omissions are
      unlisted: `dir`, `runAfterKeyboard`, skip animation, aria-label only in dialog mode.
    - `overlay/mod.rs:24-35`: the iOS VoiceOver workarounds exist (`use_dialog.rs:218-244`).
    - `modal/mod.rs:9-16`: numbering.
    - The doc examples of `use_overlay`/`use_popover`/`use_tooltip`/`use_menu_trigger` use `disabled` and
      `menu_trigger.props`.
    - Still @ 6f664fe911 with ad-hoc blocks: `use_overlay_trigger`, `use_modal`, `use_tooltip(_trigger)`,
      `animation/mod.rs`. No banner at all: `use_close_on_scroll`, `use_exit_animation`, `atoms/modal.rs`.
  - Tests:
    - A `use_overlay` fixture:
      - `should_close_on_interact_outside`, `is_keyboard_dismiss_disabled`, a non-dismissable modal;
      - nested modals: only the top one hides; also exercises `aria_hide_outside`'s `reveal` vs FocusScope auto
        focus.
    - DismissButton: its 4 upstream cases.
    - Tooltip: hide on scroll, `should_close_on_press = false`, the `Focus` trigger mode, `close_delay`, and an
      animated warm-up swap instead of the fixed `sleep(100ms)` (`test_tooltip.rs:47`).
    - `use_prevent_scroll` with nested modals.
    - `use_safely_mouse_to_submenu` (pointer-events).
    - RTL submenu keys.
- [ ] **Slider and color** (fidelity review 2026-10-07):
  - Slider:
    - No per-thumb `Label`: `slider.rs:245` hard-codes `has_label: false`, while RAC's SliderThumb provides a
      `LabelContext`. A Label in a thumb duplicates the slider label's id.
    - The thumb input lacks `aria-errormessage`/`aria-details`; the Slider atom lacks `is_required`.
    - `SliderMark(s).name: Option<Cow>` (C2).
    - `use_slider_marks` allocates signals inside a derived closure.
  - Color:
    - `ColorWheel<Ch: ColorChannel>` accepts any channel, and RGB colors can't use a wheel at all. Derive the hue
      channel, or convert like upstream does.
    - Channel values are formatted as `"{:.0}%"`/`"{}°"` (`format_channel_value`) instead of by locale (ICU4X is
      there).
    - `ColorSlider` puts `role=group` on the root (upstream: on the track) and lacks RAC's `Label` default children
      (the channel name).
    - RGB→HSV/HSL conversions aren't rounded to 2 decimals (`toFixedNumber`).
    - `color.rs` leftovers:
      - `get_*` names, the tuple `get_color_space_axes`, `HSV.value` vs Brightness;
      - the field state hard-wired to `RGB8`, `RGB8::from_hex` instead of `FromStr`, no `toString(format)`;
      - 2361 lines: split it.
    - Hygiene: dead `math::calculate_page_size`; two step-snapping implementations; `math.rs` still
      @ 6f664fe911; `ColorChannelField` mutates 7 fields after its literal.
    - Document that the 0..1 saturation/brightness/lightness range also changes `aria-valuenow` and the form
      values.
  - Tests:
    - Slider:
      - useSlider.test.js: track drag, stacked thumbs, disabled track press, vertical track.
      - An RTL fixture; cross-axis arrows.
      - RAC: multiple controlled thumbs, three-thumb output, repeated Page Up/Down, the form prop.
      - A per-thumb disabled thumb, and the `input`-event path.
      - `test_slider.rs`'s vertical check reads `value` without waiting.
    - Color:
      - HSV/HSL areas (only RGB is tested), RTL.
      - ColorSlider: the vertical cases and thumb drags.
      - Wheel: a thumb drag, and the `value` property.

- [ ] `utils::syntax_highlight`: syntect's default syntaxes lack TOML (the book's TOML blocks stay plain, 2026-10-07):
  bundle a TOML `.sublime-syntax` (license check) or use `two-face`'s extra syntaxes (heavier); decide by binary size.

- [ ] `utils::clipboard`: copy text that is still loading (the book's "Copy as Markdown", 2026-10-07; Safari only
  allows clipboard writes during the press): `navigator.clipboard.write([new ClipboardItem({"text/plain":
  promise})])` issued synchronously in the handler, e.g. `write_text_deferred(impl Future<Output = Option<String>>)`
  (Chrome ≥ 97, Firefox ≥ 127, Safari). Tell the book when it lands.

### Conventions still to apply
- [ ] C2 text types, what's left (recounted 2026-10-07; the form and color hooks, breadcrumbs, `use_anchor_link`'s
  label and `Dialog` are done):
  - `SliderMark`/`ComputedSliderMark.name: Option<Cow>` (`use_slider_marks.rs:27,47`);
  - `UseButtonInput.id` and `ButtonFormAttributes`' `form`/`form_action`/`name`/`value` as `Option<Oco>`
    (`use_button.rs:117-184`);
  - `UseOverlayTriggerInput.overlay_id: Oco` (`use_overlay_trigger.rs:56`);
  - `Href(Oco)` (`use_anchor_link.rs:17`): check whether it's fine as a newtype;
  - not `MaybeProp`: `UseTableColumnResizeInput.aria_label: String` (`:63`), and the `UseComboBoxInput`/
    `SelectValue` placeholders (`Option<String>`).
- [ ] C12: 22 enum-valued ARIA attributes or tabindex values are still strings, in 10 files (recounted
  2026-10-07; the breadcrumb item is done):
  - `use_calendar_cell` (5), `use_calendar_grid` (3), `use_date_segment` (4), `use_spin_button` (3);
  - tag group (2: `aria_atomic`/`aria_relevant`);
  - `use_date_field` (1), `use_number_field` (1), `use_menu_item` (1: `aria_expanded`), toast (1: `aria_hidden`);
  - `use_dialog` (1: `tabindex: &'static str`, `:86`).
- [ ] C1/C3/C7/C8/C10 leftovers (fidelity review 2026-10-07):
  - C1: plain `bool` flags: `UseComboBoxInput.is_required` (`:79`), `UseModalInput.is_disabled` (`:43`),
    `UseHiddenSelectInput.is_required`, `UseSelectInput.is_required` (`:65`).
  - C3: `UseFormValidationStateReturn` (`use_form_validation_state.rs:222`, a bag of `update`/`reset`/
    `commit_validation` callbacks) → `FormValidationState` with methods.
  - C8: 29 public hooks take more than one parameter, e.g.:
    - calendar: `use_calendar(input, state)`, `use_range_calendar(.., commit_behavior)`, `use_calendar_heading`,
      the month/year pickers;
    - date: `use_date_field`/`use_time_field` (4), `use_date_picker`, `use_date_segment`;
    - overlays and landmarks: `use_landmark`, `use_toast`, `use_toast_region`, `use_tooltip_trigger`;
    - virtualizer: `use_scroll_view`, `use_virtualizer_item`;
    - others: `use_constrained_move`, `use_formatted_text_field`, `use_list_keyboard_delegate` (4),
      `use_draggable_collection`, `use_table_selection_checkbox`.
  - C10:
    - String pointer-type comparisons: `use_slider.rs:283`, `use_color_wheel.rs:362`, `use_color_area.rs:494`,
      `use_safely_mouse_to_submenu.rs:170`, `utils/virtual_click.rs:63`.
    - String arrow keys in `use_move`.
    - Tuple callbacks: `UseComboBoxStateInput.on_open_change`, `is_date_unavailable` (`use_range_calendar_state.rs:41`,
      `atoms/calendar.rs:85`), `update_item_size`.
    - `Callback<()>` used for configuration or getters: `UseDragInput.get_items`, `get_allowed_drop_operations`
      (×2), `UseFormValidationInput.focus`, `UseSelectableItemInput.focus`, `UseDateFieldInput.open`.
    - The mode bool of `use_date_picker_group(element, bool, ..)` (`use_date_field.rs:147`).
  - C7: `MoveConstraintOptions::new` (`use_move.rs:162`; against C7's spirit).
- [ ] Generics sweep (user, 2026-10-05): typed values instead of the dynamic collection `Key` for value-like
  selections (RadioGroup, Select, ComboBox, ToggleButtonGroup, CheckboxGroup values), spin button values, color
  channel values (slider, meter and progress are generic over `NumberValue` already). Decide per family.
- [ ] Optional `Callback` props with generic arguments (`on_selection_change: Option<Callback<HashSet<Key>>>`) can't
  infer an untyped closure in `view!` (E0282).

### Cross-cutting
- [ ] Localized strings: a `use_localized_string_formatter` equivalent with upstream's message bundles (~36 hooks
  hard-code English), `useDefaultLocale` (browser language + `languagechange`), reactive `I18nProvider`. crudkit
  (German) overrides labels through the inputs meanwhile; keep that working.
- [ ] Theme flash: the server renders `Root`'s default theme, the stored choice applies one frame after hydration
  (`signal_ls`). dev-ui solved it with a cookie (leptos-use `use_cookie_with_options`, its `axum` feature on the
  server) feeding `ThemeProvider`'s `theme`/`set_theme`, and `<html data-theme>` rendered on the server through
  leptos_meta's `<Html>` (2026-10-07). For `Root`/the docs: recommend a cookie in `signal_ls`'s and
  `ThemeProvider`'s docs; let `ThemeProvider` render `<html data-theme>` on the server itself (optionally via
  leptos_meta), so apps need no own `<Html>`.
- [ ] Atom hygiene:
  - `data-hovered` everywhere (`use_option` must return `is_hovered`);
  - `data-focus-visible` on `TableRow` (Tab/GridRow/ComboBoxButton have it);
  - context for render state (selected/pressed) in Tab/GridRow/TableRow/GridListItem;
  - no classes besides the default one: `leptonic-modal-backdrop` (`modal.rs:159`), `leptonic-theme-provider`
    (`theme.rs:113`; also `FocusScope` has no default class);
  - a complete `atoms::prelude` (Tabs, calendar, date picker missing);
  - one rule for `Children` vs `ChildrenFn`;
  - atoms dropping `attr:` attributes.
- [ ] Port hooks onto `use_keyboard` shortcuts (upstream uses them in ~20 hooks).
- [ ] Input composition: which `MergeWith` types become unnecessary (e.g. link + press, tooltip trigger)?
- [ ] Theme: the global `[data-focus-visible]` outline also hits virtually focused listbox options.
- [ ] `use_clipboard` lives in `hooks::dnd`: move it to its own module?
- [ ] Later: step list, token field, preview trigger, `S/data` list helpers.
- [ ] Hygiene (recounted 2026-10-07):
  - Upstream markers:
    - 27 files (31 headers; 16 hooks, 11 utils) are still on 6f664fe911, and `shadow_dom.rs` is on eef7319215.
    - 9 of them have unabsorbed upstream commits (36; see "Interactions and focus"); the other 18 only need
      `--mark-synced`.
    - Header SHAs mix 10 and 9 characters: `scripts/port-starter-css.py:23` uses `--short=9`, while the drift
      script's `--mark-synced` uses `--short=10`. Align them.
  - Deviation blocks:
    - 39 of 197 ported hook files have neither a deviation block nor "No deviations": use_press, use_hover,
      use_move, use_focus*, use_tooltip*, use_modal, use_overlay_trigger, use_interact_outside, ...
    - Also: ad-hoc formats, wrong "no deviations" claims, reasons for every API DIFFERENCES entry.
  - Headers and legacy lines:
    - 12 hook files have neither `// Upstream:` nor `// No upstream:`: use_global_shortcuts, visible_overlays,
      tooltip_registry, use_collection, item_elements, use_anchor_link, use_slider_marks, all `merged/*`.
    - Ported utils without headers: focusability, synthetic_blur, text_selection, platform, focus_scope_tree,
      interaction_rect, `atoms/focus_ring`.
    - 27 legacy "based on work in" lines; 14 `cfg_attr(ssr, allow(dead_code))` → precise `cfg`s.
  - Dead code: the `PressEvents` enum (`use_press.rs:122`); `focus_scope_tree::{get_active_scope,
    innermost_containing_ancestor}`; interaction_rect's `KeyboardEvent` impl (an `expect`, a TODO at :102);
    `use_slot_id`; `use_filtered_list_state`; `math::calculate_page_size`.
  - `Propagation` isn't implemented on the Hover/Move/LongPress/FocusWithin/Scroll/DnD events: implement it or
    narrow CLAUDE.md's "all user-facing event types".
  - Raw `.target()` reads to move onto `EventAccessors`: `use_focus.rs:122`, `focus_scope.rs:318,338,403`,
    `use_context_menu.rs:112,129`, `use_number_field.rs:262,459`, `use_radio_group.rs:213`.
  - Split the largest functions (`use_press`, `use_move`, `use_selectable_collection`, `FocusScope`) and
    `utils/color.rs` (2361 lines).
  - Docs for public items:
    - Undocumented pub fields of `UseFocusInput`, DnD states, `SelectionOptions.disabled_behavior`,
      `UseListBoxInput`/`Return`, `CollectionOptions`, `SelectState`, `Column`,
      `TableColumnResizeState.table_state`, `TabListState.list`, `TreeState.expansion`, `UseGridCellReturn`.
    - Most atom props.
  - Run the book's `api_check` after API changes.

### Documentation (fidelity review 2026-10-07)
- [ ] `atoms-implementation.md`:
  - Its "No CSS Classes" section contradicts the default classes (CLAUDE.md's "Headless/unstyled" too). Rewrite it
    around the default class, the data attributes and the optional atom theme.
  - The atom example uses an API that is gone (`disabled`, nested `use_press_input`/..., `props.into_attrs()`).
    Take it from `atoms/button.rs` and show `is_*`, `x` + `set_x: Out<T>`, the default class and `node_ref`.
  - The Context Pattern cites `SliderCtx`/`expect_context`; the code uses `SliderContext` with `use_context` +
    `dev_warn`.
  - Add a "Shared atom infrastructure" section: `Out<T>` (its only reference implementation is
    `components/date_selector.rs`, which goes with step 4), `LabelPresence`, `FieldContext`, `SlotProps`/`slot_id`,
    `use_description`, `scoped_view` (attr forwarding), virtual focus, `OwnerAlive` (also in "Blur After
    Disposal").
- [ ] `architecture.md` and `hooks-implementation.md`:
  - The examples call `props.into_attrs()` on a `PropsWithStyles`, which only has `into_parts()`. Fix them and
    document `PropsWithStyles`/`IntoAttrs` (`hooks/mod.rs:144-200`, 44 uses).
  - Drop architecture.md's Components example and its form-validation/animation sections, which duplicate
    hooks-implementation.md.
  - The `MergeWith` table lists 4 of the 7 merged types.
  - `select_components_tests` (`:756`) is now `select_tests`.
  - "`*Return` carries fields of type `Callback`" fights C3.
  - Form validation section: the field is `state`, not `validation_state`, and `setCustomValidity` goes on the
    field, not the form.
- [ ] Hook-owned state is described three ways, all against C4 and the 31 hooks taking a `ValueBinding`:
  - CLAUDE.md "always own their WriteSignal ... mutation callbacks";
  - hooks-implementation.md's C4 row "no controlled inputs";
  - hooks-implementation.md "Hook-Owned State": "only `default_open`".

  Use one wording (`default_*` + `on_*_change`, or a `ValueBinding`; mutation through state methods). Also update
  `hooks/mod.rs:6-9` (its global deviation block says C1–C13, recommends `new(required)`, which C7 forbids, and
  lacks `ValueBinding`).
- [ ] `design-collections.md` still says "Status: proposal", but the design is implemented, with differences:
  `TypedKey` is `ToKey`, `Key::cell` takes a `usize`, the `NodeKind` list differs, and the typed atom layer of
  §4.10 was never built. Mark it implemented and condense §3/§5.
- [ ] Smaller fixes:
  - `components-implementation.md` goes with step 4.
  - CLAUDE.md names `BrowserTestFailurePolicy::RunAll`; the type is `FailurePolicy::RunAll`.
  - conventions.md lists C13 after C14/C15.
  - Atom test files use both `_atoms` and `_atom`: rename `test_focusable_atom`, `test_number_field_atom` and
    `test_text_field_atom` to `test_<family>_atoms.rs`.
- [ ] `audit-2026-10-05.md`: move its unabsorbed rest (§3 missing modules, §4 atom coverage) into this plan, then
  delete it (see "Waiting on the user"), together with this plan's "audit §n" references.

### Testing infrastructure
- [ ] Every hook/atom family gets a fixture + browser test from upstream's tests as part of its package. Recounted
  2026-10-07 (the family items above list the missing upstream cases):
  - Hooks reached by no fixture, even transitively: `use_breadcrumb_item`, `use_clipboard`,
    `use_grid_list_section`.
  - 19 more hooks are reached only through other hooks, e.g. `use_selectable_collection`, `use_form_validation`,
    `use_overlay`, `use_prevent_scroll`, `use_visually_hidden`.
  - Atoms in no fixture: LinkButton, DismissButton, FocusManagerProvider, AnchorLink, ListBoxItemDescription,
    ListBoxItemLabel, ListBoxSection, ClearPressResponder, SliderMark, SliderMarks, VisuallyHidden.
  - Browser tests without an `// Upstream:` header (14): test_button, test_context_menu_atoms, test_focusable,
    test_focus_manager, test_focus, test_focus_visible, test_focus_within, test_global_shortcuts, test_label_slots,
    test_live_announcer. The leptonic-only ones (test_hydration_ids, test_server_panics, test_virtual_list,
    test_has_tabbable_child) get `// No upstream:`.
- [ ] Native test helper for hooks (`with_owner`) and a way to run Effects natively, for `*_state` hooks.
  - State modules without native tests: `calendar/states.rs`, `use_color_channel_field_state`,
    `use_color_picker_state`, `use_color_slider_state`, `use_form_validation_state`, `use_text_field_state`,
    `use_grid_state`, `use_tree_state`, `use_virtualizer_state`.
  - `use_combobox_state` has 1 test.
- [ ] Extend the hydration id test to every fixture that generates ids. It lists 40 of 74 pages; missing are
  `/atoms/slider`, `/atoms/tag-group`, `/atoms/label-slots`, `/atoms/submenu`, `/atoms/context-menu`,
  `/hooks/context-menu`, `/hooks/landmark`, `/atoms/toast`, `/atoms/toast-single`, `/atoms/virtualizer`.
- [ ] Waiting instead of sleeping:
  - Sleeps before positive checks: test_checkbox.rs:311, test_toast (7), test_long_press (5, partly timer-bound).
  - 28 hand-rolled 50 ms poll loops (e.g. test_combobox) → the polling helpers.
  - Sleeps before negative checks ("nothing changed") stay.
- [ ] `just clippy` never checks `atoms` without `components`, which is dev-ui's set (consumers.md: "the atoms
  feature must always compile"). Add `cargo clippy -p leptonic --no-default-features --features atoms,clipboard
  --tests`.
- [ ] `testing/test-app/style/leptonic` (107 generated theme files) is tracked in git and rewritten by the harness
  (`leptonic/tests/browser_test.rs:41-47`), but `main.scss` doesn't use it, and the test-app needs no theme. Stop
  generating it and untrack it (ask before touching the index).
- [ ] Clippy in release too: some lints depend on type sizes that differ in release (`MaybeProp`, `StoredValue`:
  `trivially_copy_pass_by_ref` fires only there). `just clippy` checks debug builds only; add a release run once the
  user decides on the cost.
- [ ] Chrome profiles leak (browser-test crate, the user's): chromedriver's `/tmp/org.chromium.Chromium.scoped_dir.*`
  profiles stay behind when a session isn't quit cleanly; 2026-10-06 they filled the /tmp quota (14 GB), again
  2026-10-07 (245 dirs, ~17 GB; Chrome sessions then fail to start: "Devtools port number file"). Fix in
  browser-test: an own `--user-data-dir` per session, removed on drop, or a sweep at startup.

## Book

Page and writing rules: `documentation/documentation-strategy.md` (incl. "Verification"); look, design tokens and
which leptonic piece to use: `examples/book-ssr/STYLE_GUIDE.md`. Library gaps the book hits go into the roadmap above
and, while the book waits for them, under "Waiting on the library" below. Finished book work:
`documentation/history.md` ("Book").

### Next
- [ ] B1 follow-up (state 2026-10-07 evening): all four groups converted to atoms + book CSS (component pages, Chip →
  Tag Group, Drawer/Alert/layout recipes, Kbd as one atom page, Transitions removed; welcome Showcase on atoms).
  Open: the library's API wave broke `kit::api_check` on many atom tables (Button, ComboBox, Menu/MenuSection,
  ListBox/ListBoxSection, Select, GridList, ColorWheel, DismissButton, ...) — update them from leptonic-e9's
  consolidated list; clippy (raw-string hashes in `atoms/combobox.rs`, `atoms/field.rs`; `hooks/demos/dnd_reorder.rs`);
  browser runs not done for menu, table, calendar, date-*, time-field, collection, tag-group, tree, checkbox, form,
  radio, slider, switch, combobox, select, interactions (use_move page) and screenshots (Drawer, Toast, popovers);
  then remove `leptonic-themes` from `style/main.scss` and `ComponentDemoContexts` from `app.rs` (`app.rs` still imports
  components); Slider keyboard tables (all four arrows); library gaps: no `data-hovered` on ListBoxItem/GridListItem/
  GridRow/MenuItem, no `data-hovered`/`data-focus-visible` on table header/row/cell; toast strings English only.
  Agents' temp: use per-agent scratch subfolders.
- [ ] The user's rule (2026-10-07, done for the library): every dependency of `examples/book-ssr/Cargo.toml` at its
  latest version (breaking updates one at a time) and declared with `default-features = false` plus only the features
  it needs. Note: leptonic's `atoms`/`components` features now gate their modules, and `icondata` comes only with
  `components` (Bootstrap and VS Code icons).
- [ ] Off the components layer (the user's decision 2026-10-07, step 3 of "Remove the components layer"):
  - A: done 2026-10-07 (shell, `Root` → `ThemeProvider` + the book's toast queue and `ToastRegion`, kit `Icon`/`Link`/
    `Keys`/`DocTable`, pages import the kit). Left: the kit's `Code` once `utils::syntax_highlight` is public and the
    `syntax-highlight` feature no longer needs `components` (then drop the `Code`/`Language` re-export in
    `kit/mod.rs`); remove `ComponentDemoContexts` in `app.rs` (provides `ToastRoot`/`Leptonic` for the component
    demos) and the welcome `Showcase`'s components with phase B.
  - B (the user, 2026-10-07: "The book should use its own unique styling. I think that even maintaining a
    stylesheet is too much for leptonic."): the book never uses a library theme; it styles every atom itself with
    its tokens, selecting on the atoms' default classes `leptonic-<AtomName>`, its own classes and the data
    attributes. Stages: B0 one agent — Component tabs out of nav/routes (redirects to the atom pages' styling
    sections), component pages and demos deleted, the standard demo-control markup (atoms + book CSS) in STYLE_GUIDE
    §5; B1 four agents by sidebar groups — every atom page's styling section (default class, data attributes, inner
    markup, the book's CSS as the example), all demos on atoms + book CSS, Quick Starts, recipes for component-only
    concepts (Card, Stack, Grid, Skeleton, Typography, AppBar, Drawer, Alert, Icon, transitions; Chip → TagGroup
    atoms), group stylesheets; at the end `leptonic-themes` leaves `style/main.scss` and `ComponentDemoContexts`
    leaves `app.rs`. Replacements decided by the library: Button, Link/LinkButton, Checkbox, Radio, Switch,
    TextField, NumberField, Slider, Select (`SelectMode` multiple), Tabs, Table, Meter, ProgressBar, Separator, Modal,
    Popover, Calendar/DatePicker, ColorPicker, Collapsible → Disclosure, toasts → `ToastRegion`/`Toast`, Kbd →
    `ShortcutKeys`/`Keys`; gone with their features: Root, Code, TiptapEditor, SanitizedHtml. The library keeps its
    optional atom theme (`@use "leptonic/leptonic-atoms";`, the user's decision): the styling sections mention it in
    one sentence for apps that don't want to style from scratch; the Themes guide explains it (stage C).
  - C: `documentation-strategy.md`, `STYLE_GUIDE.md` (also its new tokens `--book-z-app-bar`, `--book-z-modal`,
    `--book-app-bar-*`, `--book-switch-*` and the `key-cap` mixin), the guides (Overview, Architecture, Installation, Themes,
    Classes & Styles) and the sidebar markers (H/A/C → H/A) drop the component layer; changelog.
- [ ] The kit's `Keys` draws group descriptions inside combinations ("Shift + Arrow keys") as key caps: compose them
  as key caps for the keys and plain text for the description (the library's advice; `KeyboardKey::Other` is a key).
- [ ] Browser checks for the book's own code blocks and key caps (styles, copy button).
- [ ] Browser test speed: implicit wait 0 with explicit waits only?; book page loads hydrate debug wasm (~900 ms
  each), a release/wasm-opt build for the browser tests would cut that. Step timing: `BROWSER_TEST_LOG_STEPS=1`.
- [ ] The Dockerfile builds from the repository root (path dependency on `../../leptonic`); not yet test-built.

### Waiting on the library
- "Copy as Markdown" downloads the export on press and then writes it with `write_text`; Safari may reject a write
  after the await. Switch to a deferred clipboard write (`ClipboardItem` with a promise) once `utils::clipboard`
  offers one (requested 2026-10-07).
- TOML code blocks stay unhighlighted until the library bundles a TOML syntax.
- The search trigger (`doc_search.rs`) works out the platform itself for its `KbdShortcut` and `aria-keyshortcuts`:
  once the themed `KbdShortcut` takes a `Shortcut` (or `ShortcutKeys` gets the theme's look) and `Shortcut` gives an
  `aria-keyshortcuts` value, use them.
- Search results → Autocomplete with arrow keys through results (R3j).
- The `DateSelector`'s English labels ("choose a year", "Previous years") and the calendar strings wait for
  localized strings (R4).
- Document once implemented: tree tables; the `Label` atom (renders a `for` pointing at a generated id nothing has,
  R3c).
