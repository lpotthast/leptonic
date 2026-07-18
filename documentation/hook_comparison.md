Cross-Check: Leptonic Hooks vs React-Aria — Deviations Audit

Context

Systematic source-level comparison of every major leptonic hook against its react-aria counterpart at ~
/dev/react-spectrum. Goal: identify features we missed
(unintentional omissions) vs features we deliberately skipped (intentional deviations). Findings are organized by
severity.

 ---
P0 — Bugs / Incorrect Behavior

These are cases where leptonic's implementation is actively wrong or produces incorrect results.

┌─────┬────────────────────┬──────────────────────────────────────┬──────────────────────────────────────────────────────────────────────────────────────────────────┐
│ # │ Hook │ Issue │ Details │
├─────┼────────────────────┼──────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────┤
│ │ │ │ DONE. Implemented full iOS Safari path (touchmove prevention, overscroll-behavior, │
│ 1 │ use_prevent_scroll │ Missing mobile Safari strategy │ keyboard visibility, visual viewport resize, focus override,
range input exception). Also fixed │
│ │ │ entirely │ lossy style restoration, added scrollbar-gutter: stable support, and first-caller-only guard. │
│ │ │ │ File: leptonic/src/hooks/interactions/use_prevent_scroll.rs │
├─────┼────────────────────┼──────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────┤
│ │ │ │ DONE. use_modal was refactored into composable hooks. is_open is now managed in │
│ 2 │ use_modal │ is_open input accepted but unused │ use_modal_backdrop where it gates use_overlay, use_prevent_scroll,
and aria_hide_outside. │
│ │ │ │ File: leptonic/src/hooks/modal/use_modal_backdrop.rs │
├─────┼────────────────────┼──────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────┤
│ │ │ should_close_on_interact_outside │ DONE. Moved to use_modal_backdrop as Option<Callback<web_sys::Element, bool>> (
a filter │
│ 3 │ use_modal │ accepted but unused │ function, not boolean). Passed through to use_overlay which applies it in both
interact-outside │
│ │ │ │ and blur-within handlers. File: leptonic/src/hooks/modal/use_modal_backdrop.rs │
├─────┼────────────────────┼──────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────┤
│ │ │ │ DONE. use_modal no longer sets any role. Role is handled by use_dialog which accepts DialogRole │
│ 4 │ use_modal │ Hardcoded role="dialog"              │ enum (Dialog | AlertDialog). The modal hook system was split
into use_modal (aria-modal marker), │
│ │ │ │ use_modal_backdrop (dismiss/scroll), and use_dialog (ARIA role/labeling/focus-on-mount). │
│ │ │ │ File: leptonic/src/hooks/dialog/use_dialog.rs │
├─────┼────────────────────┼──────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────┤
│ │ │ │ DONE. aria-level, aria-setsize, aria-posinset now included in UseTreeItemProps output. │
│ 5 │ use_tree_item │ aria-level, aria-setsize, │ Also added isComposing guard, Home/End key support, on_focus_self
callback, and fixed │
│ │ │ aria-posinset accepted but not │ demo to pass correct position_in_set/set_size values. │
│ │ │ output │ File: leptonic/src/hooks/tree/use_tree_item.rs │
├─────┼────────────────────┼──────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 6 │ use_slider_thumb │ Hidden input missing │ DONE. Added native `min`, `max`, `step` attrs on the hidden input alongside the
existing ARIA  │
│ │ │ min/max/step/value attributes │ attrs and `value`. `step` resolves to `"any"` for continuous sliders (state.step
== None).      │
│ │ │ │ File: leptonic/src/hooks/slider/use_slider_thumb.rs │
└─────┴────────────────────┴──────────────────────────────────────┴──────────────────────────────────────────────────────────────────────────────────────────────────┘

 ---
P1 — Missing Core Features (Functional Gaps)

Features react-aria provides that most consumers would expect to work.

Interactions

┌─────┬───────────┬────────────────────────────────────┬─────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ # │ Hook │ Missing Feature │ React-aria Reference │
├─────┼───────────┼────────────────────────────────────┼─────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 7 │ use_press │ No virtual click detection │ isVirtualClick(e.nativeEvent) — fires pressStart→pressUp→pressEnd for
screen reader clicks. │
│ │ │ │ usePress.ts:546-551 │
├─────┼───────────┼────────────────────────────────────┼─────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 8 │ use_press │ No macOS Meta key workaround │ Tracks keydown events while Meta pressed, synthesizes keyup. Prevents
stuck-key on Cmd+Tab. │
│ │ │ │ usePress.ts:366-375 │
├─────┼───────────┼────────────────────────────────────┼─────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 9 │ use_press │ No explicit pointer capture │ releasePointerCapture(e.pointerId) on cancel/end. usePress.ts:602-613 │
│ │ │ release │ │
├─────┼───────────┼────────────────────────────────────┼─────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 10 │ use_press │ No isComposing guard on Escape │ DONE. Audit misattributed to use_press; react-aria's check lives in useOverlay.ts:122,
not │
│ │ │ │ usePress.ts. Leptonic matches at leptonic/src/hooks/overlay/use_overlay.rs:242 with
│
│ │ │ │ `e.key() == "Escape" && !is_keyboard_dismiss_disabled && !e.is_composing()`. │
└─────┴───────────┴────────────────────────────────────┴─────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

Focus

┌─────┬───────────────────┬──────────────────────────────┬───────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ # │ Hook │ Missing Feature │ React-aria Reference │
├─────┼───────────────────┼──────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 11 │ use_focus_visible │ No pointer sub-type tracking │ React-aria distinguishes mouse/pen/touch via
currentPointerType. Leptonic only tracks Pointer vs Keyboard │
│ │ │ │ vs Virtual. │
├─────┼───────────────────┼──────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 12 │ use_focus_manager │ Always focuses without │ React-aria uses focusElement(node, true) allowing scroll. Leptonic
always passes false — focused elements │
│ │ │ scrolling │ may be off-screen. │
└─────┴───────────────────┴──────────────────────────────┴───────────────────────────────────────────────────────────────────────────────────────────────────────────┘

Form

┌─────┬─────────────────┬────────────────────────────────┬───────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ # │ Hook │ Missing Feature │ React-aria Reference │
├─────┼─────────────────┼────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 13 │ use_radio_group │ No keyboard navigation (arrow │ DONE. Added on_keydown to UseRadioGroupProps that walks the group container via
                  │
│ │ │ keys)                          │ get_focusable_tree_walker, focuses next/prev radio input, and dispatches click so the
radio's │
│ │ │ │ change handler invokes set_selected_value. RTL flips Left/Right when orientation is
horizontal. │
│ │ │ │ File: leptonic/src/hooks/form/use_radio_group.rs │
├─────┼─────────────────┼────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 14 │ use_button │ No element type polymorphism │ React-aria supports rendering as <a>, <div>, <input>, <span> with
appropriate ARIA. Leptonic only handles │
│ │ │ │  <button>. │
├─────┼─────────────────┼────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 15 │ use_button │ Missing form attributes │ form, formAction, formEncType, formMethod, formNoValidate, formTarget,
name, value — all absent. │
├─────┼─────────────────┼────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 16 │ use_slider │ Label click doesn't focus │ React-aria label has onClick that focuses first thumb + sets interaction
modality to keyboard. Leptonic │
│ │ │ thumb │ label only has an ID. │
└─────┴─────────────────┴────────────────────────────────┴───────────────────────────────────────────────────────────────────────────────────────────────────────────┘

Selection / Collections

┌─────┬───────────────────────────┬──────────────────────────────────┬───────────────────────────────────────────────────────────────────────────────────────────────┐
│ # │ Hook │ Missing Feature │ React-aria Reference │
├─────┼───────────────────────────┼──────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────┤
│ 17 │ use_selectable_collection │ No collectionProps DOM output │ Produces no event handlers or ARIA attributes.
Hook-plan P0. │
├─────┼───────────────────────────┼──────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────┤
│ 18 │ use_selectable_collection │ No KeyboardDelegate integration │ Uses flat vector instead of delegate pattern.
Hook-plan P0. │
├─────┼───────────────────────────┼──────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────┤
│ 19 │ use_type_select │ Disabled keys not skipped │ Iterates all keys with simple prefix matching. React-aria delegates
to │
│ │ │ │ keyboardDelegate.getKeyForSearch(). │
├─────┼───────────────────────────┼──────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────┤
│ 20 │ use_type_select │ Spacebar not special-cased │ React-aria ignores spacebar when search empty, prevents default
when active. Leptonic treats │
│ │ │ │ it uniformly. │
├─────┼───────────────────────────┼──────────────────────────────────┼───────────────────────────────────────────────────────────────────────────────────────────────┤
│ 21 │ use_type_select │ Character matching rejects │ Uses key.len() != 1 (byte length). React-aria accepts Unicode
characters. │
│ │ │ non-ASCII │ │
└─────┴───────────────────────────┴──────────────────────────────────┴───────────────────────────────────────────────────────────────────────────────────────────────┘

Overlays

┌─────┬──────────────────────┬────────────────────────────────────────┬──────────────────────────────────────────────────────────────────────────────────────────────┐
│ # │ Hook │ Missing Feature │ React-aria Reference │
├─────┼──────────────────────┼────────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────┤
│ 22 │ use_overlay_position │ No offset (main axis spacing)          │ Core positioning feature. All consumers need
trigger-to-overlay gap. │
├─────┼──────────────────────┼────────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────┤
│ 23 │ use_overlay_position │ No containerPadding / boundary nudging │ Overlay can overflow viewport edges. React-aria
nudges position to stay within bounds. │
├─────┼──────────────────────┼────────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────┤
│ 24 │ use_overlay_position │ No maxHeight computation │ React-aria computes and returns available height. Critical for
dropdowns near viewport edge. │
├─────┼──────────────────────┼────────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────┤
│ 25 │ use_overlay_position │ No resolved placement output │ Consumer can't know if overlay flipped. React-aria returns
computed placement. │
├─────┼──────────────────────┼────────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────┤
│ 26 │ use_overlay_position │ No window resize listener │ Position doesn't update when window resizes. │
└─────┴──────────────────────┴────────────────────────────────────────┴──────────────────────────────────────────────────────────────────────────────────────────────┘

Table / Tree

┌─────┬───────────┬───────────────────────────┬──────────────────────────────────────────────────────────────────────────────────┐
│ # │ Hook │ Missing Feature │ React-aria Reference │
├─────┼───────────┼───────────────────────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ 27 │ use_table │ No arrow key navigation │ PARTIAL. ArrowUp/Down ARE handled by use_table_row via on_focus_next /
on_focus_previous       │
│ │ │ │ callbacks delegated to the consumer. What's still missing is centralized
TableKeyboardDelegate │
│ │ │ │ (cell-level Left/Right/Home/End/PageUp/PageDown, sticky-header support, RTL).
Larger design   │
│ │ │ │ work — needs explicit decision on whether to mirror react-aria's centralized
delegate model.  │
├─────┼───────────┼───────────────────────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ 28 │ use_table │ No sort announcements │ DONE. Effect watching (sorted_column, sort_direction) calls announce_polite on
change      │
│ │ │ │ ("Sorted by {col}, {ascending|descending}.") with initial-run guard so it
doesn't fire on │
│ │ │ │ mount. File: leptonic/src/hooks/table/use_table.rs │
├─────┼───────────┼───────────────────────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ 29 │ use_tree │ No KeyboardDelegate │ Arrow key callbacks not wired up. * key (expand all) commented as unimplemented.
│
├─────┼───────────┼───────────────────────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ 30 │ use_tree │ No collection-based state │ Uses flat Vec<String> instead of tree structure. Hook-plan P0. │
└─────┴───────────┴───────────────────────────┴──────────────────────────────────────────────────────────────────────────────────┘

Combobox

┌─────┬──────────────┬──────────────────────────────┬──────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ # │ Hook │ Missing Feature │ React-aria Reference │
├─────┼──────────────┼──────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 31 │ use_combobox │ No live region announcements │ React-aria announces filtered count, focused item, and selection
changes. Huge accessibility regression. │
├─────┼──────────────┼──────────────────────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 32 │ use_combobox │ No useTextField delegation │ Builds input props from scratch instead of composing. │
└─────┴──────────────┴──────────────────────────────┴──────────────────────────────────────────────────────────────────────────────────────────────────────────┘

Menu

┌─────┬───────────────┬──────────────────────────────┬───────────────────────────────────────────────────────────────────┐
│ # │ Hook │ Missing Feature │ React-aria Reference │
├─────┼───────────────┼──────────────────────────────┼───────────────────────────────────────────────────────────────────┤
│ 33 │ use_menu_item │ No submenu support │ Missing aria-haspopup, aria-expanded, submenu hover behavior. │
├─────┼───────────────┼──────────────────────────────┼───────────────────────────────────────────────────────────────────┤
│ 34 │ use_menu_item │ No drag-from-trigger-to-item │ React-aria tracks interaction type for trigger→item pointer drag.
│
└─────┴───────────────┴──────────────────────────────┴───────────────────────────────────────────────────────────────────┘

 ---
P2 — Missing Secondary Features

Important but not critical for basic functionality.

┌─────┬──────────────────────────┬──────────────────────────────────────────────────────────────────────────┐
│ # │ Hook │ Missing Feature │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 35 │ use_overlay_position │ No crossOffset input │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 36 │ use_overlay_position │ No arrow positioning (arrowSize, arrowRef, arrowBoundaryOffset)          │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 37 │ use_overlay_position │ No close-on-scroll behavior │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 38 │ use_overlay_position │ No isOpen gating (computes when closed)                                  │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 39 │ use_overlay_position │ No boundaryElement input (hardcoded to body)                             │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 40 │ use_modal │ DONE — Now Option<Callback<Element, bool>> in use_modal_backdrop. │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 41 │ use_modal │ DONE — Firefox underlay pointer-down prevention in use_overlay.rs. │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 42 │ use_modal │ DONE — Overlay stacking via visible_overlays module in use_overlay. │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 43 │ use_modal │ RESOLVED — ModalBackdrop atom defaults false (react-aria match). │
│ │ │ Modal component defaults true as intentional UX choice. │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 44 │ use_disclosure │ No animation support │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 45 │ use_disclosure │ No hidden="until-found" for find-in-page support │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 46 │ use_slider_state │ No locale-aware number formatting (Intl.NumberFormat / ICU4X equivalent) │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 47 │ use_slider_thumb │ No form reset support (useFormReset)                                     │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 48 │ use_progress_bar │ No locale-aware number formatting │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 49 │ use_meter │ Missing role="meter progressbar" dual role fallback (only emits meter)   │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 50 │ use_combobox │ No collection freezing during close animation │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 51 │ use_combobox │ Button not excluded from tab order (excludeFromTabOrder)                 │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 52 │ use_option │ No aria-posinset/aria-setsize for virtualized lists │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 53 │ use_option │ No Safari VoiceOver workaround (omitting aria-labelledby when misread)   │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 54 │ use_date_picker │ No focusWithin tracking │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 55 │ use_date_picker │ No RTL segment navigation │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 56 │ use_select │ No arrow key handling on closed select (value cycling)                   │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 57 │ use_grid │ No virtualization (aria-rowcount, aria-colcount)                         │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 58 │ use_grid │ No RTL direction swapping │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 59 │ use_grid_list │ No type-ahead search │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 60 │ use_grid_list │ No selection announcements │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 61 │ use_calendar_cell │ No drag-to-select for range calendars │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 62 │ use_range_calendar_state │ No drag-to-select support │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 63 │ use_breadcrumbs │ Hardcoded English label "Breadcrumbs" (no i18n)                          │
├─────┼──────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
│ 64 │ use_breadcrumb_item │ Does not delegate to use_link (reimplements inline)                      │
└─────┴──────────────────────────┴──────────────────────────────────────────────────────────────────────────┘

 ---
P3 — Nice-to-Have / Platform-Specific

┌─────┬──────────────────────────┬─────────────────────────────────────────────────────────────────────┐
│ # │ Hook │ Missing Feature │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 65 │ use_overlay_position │ No visual viewport handling (iOS virtual keyboard, pinch zoom)      │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 66 │ use_overlay_position │ No scroll anchor preservation │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 67 │ use_overlay_position │ No containing block detection (uses position:fixed unconditionally) │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 68 │ use_overlay_position │ No margin awareness │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 69 │ use_overlay_position │ No updatePosition imperative API │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 70 │ use_modal │ DONE — aria_hide_outside in use_modal_backdrop.rs using inert. │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 71 │ use_modal │ RESOLVED (intentional) — aria_hide_outside with inert + ref │
│ │ │ counting replaces ModalProvider. DOM walk handles portals directly. │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 72 │ use_modal │ RESOLVED (intentional) — Modals don't close on blur. use_overlay │
│ │ │ supports blur dismiss; use_modal_backdrop sets it to false. │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 73 │ Overlay hooks │ No Safari iOS VoiceOver focus-trap workarounds │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 74 │ use_combobox │ No iOS VoiceOver virtual touch detection │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 75 │ use_calendar_state │ English-only month names (no ICU4X integration yet)                 │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 76 │ use_date_picker_state │ No multi-calendar system support │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 77 │ use_range_calendar_state │ No time preservation across selection │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 78 │ Grid hooks │ Page Up/Down returns None (requires layout measurement)             │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 79 │ use_grid_list │ No link behavior support │
├─────┼──────────────────────────┼─────────────────────────────────────────────────────────────────────┤
│ 80 │ use_draggable │ No platform-specific modifier keys for virtual drag operations │
└─────┴──────────────────────────┴─────────────────────────────────────────────────────────────────────┘

 ---
Cross-Cutting Gaps (Affect Multiple Hooks)

1. RTL / Writing Direction

Hooks affected: use_listbox, use_menu, use_grid, use_grid_list, use_date_picker, use_tabs
Issue: Hardcoded WritingDirection::Ltr with // TODO: get from i18n context in listbox and menu. No direction swap for
arrow keys.

2. i18n / Localized Strings

Hooks affected: use_combobox, use_breadcrumbs, use_date_picker, use_calendar_state, use_slider_state, use_progress_bar,
use_meter
Issue: English-only ARIA labels and value formatting. React-aria uses useLocalizedStringFormatter throughout. Leptonic
has ICU4X infrastructure in utils but most hooks
don't use it yet.

3. Live Region Announcements

Hooks affected: use_combobox, use_table (sort), use_grid (selection), use_tree
Issue: No aria-live regions for dynamic content changes. React-aria announces filtered counts, selections, sort changes.

4. Form Reset / Validation Integration

Hooks affected: use_slider_thumb, use_combobox, use_select
Issue: No useFormReset equivalent. Hidden inputs don't track initial values for reset.

5. Link Item Support

Hooks affected: use_listbox, use_option, use_combobox, use_menu_item, use_grid_list
Issue: No concept of items-as-links (href on options). React-aria supports this across all collection components.

6. Virtualization

Hooks affected: use_grid, use_grid_list, use_table, use_listbox, use_option
Issue: No aria-rowcount/aria-colcount/aria-posinset/aria-setsize for virtualized collections.

 ---
Hooks That Are Feature-Complete (No Significant Gaps)

These hooks closely match or exceed their react-aria counterparts:

- use_hover — Nearly identical implementation
- use_scroll_wheel — Functionally equivalent
- use_keyboard — Enhanced with propagation control
- use_interact_outside — Only differs in event type (intentional)
- use_focus — Complete including Firefox workaround
- use_focus_within — Enhanced with reactive is_focus_within signal
- use_focus_ring — Enhanced with data-focus-visible attribute
- use_focusable — Enhanced with FocusHandle and auto-capture
- use_has_tabbable_child — Complete
- use_text_field — Comprehensive, returns separate label/description/error props
- use_switch — More complete than react-aria (explicit keyboard + hidden input)
- use_checkbox — Good parity
- use_dialog — ~90% match, both handle iOS Safari workaround
- use_separator — Complete with type-safe element enum
- use_tooltip / use_tooltip_trigger — Good parity with better state integration

 ---
Statistics

- Total deviations catalogued: 80
- P0 (bugs/incorrect): 6
- P1 (missing core features): 28
- P2 (missing secondary): 30
- P3 (nice-to-have): 16
- Cross-cutting gaps: 6 themes
- Feature-complete hooks: 15

Key Files Referenced

- Hook-plan documents: documentation/hook-plans/*.md (148 plans with detailed deviation analysis)
- Hooks source: leptonic/src/hooks/ (all subdirectories)
- React-aria source: ~/dev/react-spectrum/packages/@react-aria/*/src/
