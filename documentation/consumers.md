# Leptonic: consumers

Apps built on leptonic's working tree whose sessions report findings and requests. Their open requests are items in
`PLAN.md`; this file records who they are and what they depend on, so library changes can be announced to them.
Rules: consumer sessions don't edit the shared tree; tell them when requested work lands (`SendMessage` to their
session, found with `ListAgents`), and announce breaking changes to the APIs they use.

## crudkit

- Where: `../crudkit` (crate `crudkit-leptos`, demo `examples/full-stack`); sessions named `crudkit-*`.
- Depends on the working tree (`path`, `default-features = false`, features `atoms`; no `intl-strings`, so the
  hooks' own messages are English in every locale).
- Depends on: the table hooks (`use_table`, `use_table_state`, `use_table_row`/`_cell`/`_column_header`/
  `_header_row`/`_header_placeholder`, `use_table_selection_checkbox`, `use_table_select_all_checkbox`,
  `use_grid_row_group`, `use_checkbox`, `TableCollection`; its own table with multi-column sort and selection:
  announce changes to these), collections (`use_collection`, `CollectionOptions`, `Selection`, `SelectionOptions`),
  the atoms Button, Select + ListBox, TextField + Input/TextArea/Label/FieldError/Description, the NumberField atoms,
  ModalBackdrop + ModalContent + Dialog, Tabs, Separator; shared types `Classes` (+ `MergeStrategy`), `NumberValue`,
  `NumberFormatOptions`, `AriaCurrent`, `CapturedElement`, `ValueBinding`, `DialogRole`, `PressEvent`,
  `ValidationBehavior`, `CellFocusMode`. (Its checkout of 2026-10-07 no longer uses `TextFieldState`, `ToggleState`,
  `FieldContext` or the disclosure and separator hooks.)
- Localizes through the inputs it passes (German, e.g. the table selection checkboxes' `aria_label`): keep such
  overrides working; it doesn't enable `intl-strings`.
- Breaking (2026-10-08, not yet told): `UseTableStateInput` has a new field `tree` (`tree: None` without a tree).
- Breaking (2026-10-08, not yet told): `Select<S>`/`ComboBox<S>` take the selection's shape as their type: `Option<V>`
  selects one value, `Vec<V>` several; `selection_mode` is gone (`crudkit-leptos/src/components/pagination.rs:93`:
  its page size select becomes `Option<..>`, e.g. typed `Option<u64>` with integer item keys).
- Breaking (2026-10-09, not yet told): no prelude, one path per item (`documentation/conventions.md`): its
  `leptonic::atoms::prelude::{..}` imports become `leptonic::atoms::<module>::..` (`atoms::button::Button`),
  `leptonic::hooks::{..}` the family modules (`hooks::table::use_table`, `hooks::collections::Key`),
  `leptonic::utils::..` the crate root (`leptonic::NumberValue`, `leptonic::AriaCurrent`,
  `leptonic::leptos_classes::Classes`). `TableOptions` and `TableCollection::build_with` are gone:
  `show_selection_checkboxes` is a field of `UseTableStateInput` (`crudkit-leptos/src/hooks/table.rs:299`).
- History: bindings for Select value, selection and table sort (2026-10-05); sibling `ModalBackdrop`s and fields
  shared one context (fixed: atoms scope their contexts with `scoped_view`/`Provider`); grid focus jumping (the
  interaction modality now tracks from the first hook); `*Input::new` removed (2026-10-07: struct literals, told
  crudkit-2f the table inputs' fields; `UseTableRowInput.on_context_menu: None` without a menu). 2026-10-07
  evening: the single `Checkbox`/`Radio`/`Switch` atoms were removed (use `CheckboxField` + `CheckboxButton`,
  ...); crudkit uses them at `crudkit-leptos/src/components/inputs.rs:171` (`Checkbox`) and
  `examples/full-stack/src/marauder/inputs.rs:121` (`Switch`): not yet told (no live session).

## agnite dev-ui

- Where: `~/dev/agnite/dev/dev-ui`; session `dev-28` (was `dev-3f`, `dev-f8`); asked for `VirtualList` (done 2026-10-07).
- Depends on the working tree directly (`path = "../../../leptonic/leptonic"`, features `atoms` + `clipboard` and
  the default `intl-strings`): every library change reaches it at once, so the atoms feature must always compile.
- Uses: the atoms Button, Link (`CurrentMatch`) + AnchorLink, ToggleButton(Group), Switch, Checkbox,
  RadioGroup/Radio, TextField + Input, SearchField, Select + ListBox(Items), ComboBox, ListBox, GridList, Menu +
  MenuTrigger + `ContextMenuTrigger`, Popover, Tooltip, Table, Disclosure, Breadcrumbs, Meter, ProgressBar, Toolbar,
  Form, Focusable, VisuallyHidden, ModalBackdrop + ModalContent + Dialog, the toast atoms, `VirtualList`,
  `ShortcutKeys`, the theme atoms (`ThemeProvider`, `LeptonicTheme`, `ThemeContext`); the hooks
  `use_global_shortcuts`, `use_collection`/`use_list_collection`, `TableCollection`, `use_focus_ring`, `ToastQueue`;
  the utilities `use_locale`, `Collator`/`Filter`, the live announcer, `KeyboardShortcuts`, `ListFormatter`,
  `scroll_into_viewport`, `write_text`, `plural_category`.
- Breaking (2026-10-07 evening, not yet told: no live session): the single `Checkbox`/`Radio`/`Switch` atoms were
  removed (state props go on `CheckboxField`/`RadioField`/`SwitchField`, `classes` and children on the
  `*Button`); dev-ui uses them in `src/app/process_card.rs:365`, `environments_view.rs:383-399`, `theme.rs:48`,
  `shell.rs:148`.
- Done for it (2026-10-08, not yet told): tree tables (`Table`'s `tree_column` + expansion props,
  `TableExpandButton`, child rows with `ItemBuilder::children`).
- Breaking (2026-10-08, not yet told): selection atoms are generic over typed values (`SelectionValue`);
  `ToggleButtonGroup`'s `selected_keys`/`set_selected_keys`/`default_selected_keys`/`on_selection_change` are now
  `value`/`set_value`/`default_value`/`on_change` (`src/app/tests/catalog_view.rs:219`). Its `RadioGroup`, `Select`,
  `ComboBox` uses compile if their props are `Key`-typed (else name the type: `<atoms::Select<Key> ..>`).
  Then (2026-10-08): `Select<S>`/`ComboBox<S>` take the selection's shape as their type, `Option<V>` (one value) or
  `Vec<V>` (several), and lose `selection_mode` (`catalog_view.rs:337`, `log_panel.rs:259`, `palette.rs:132`).
- Breaking (2026-10-09, not yet told): no prelude, one path per item (`documentation/conventions.md`): its
  `use leptonic::atoms::prelude as atoms` (28 files) becomes imports from the atom modules
  (`leptonic::atoms::button::Button`), `leptonic::hooks::{..}` the family modules (`hooks::collections::Key`),
  `leptonic::utils::..` the crate root (`leptonic::use_locale`, `leptonic::write_text`, `leptonic::flag`).
  `TableOptions` and `TableCollection::build_with` are gone: `show_selection_checkboxes` is a field of
  `UseTableStateInput` (`src/app/tests/catalog_view.rs:156`).
- History: toasts, row context menus, the `clipboard` feature without components, global shortcuts, a public
  typing check, `ShortcutKeys`, the theme without components (2026-10-06/07); the ComboBox read-order bug and the
  modal hide-outside fix were found there.

## Starter templates

- Where: `examples/leptonic-template-{csr,ssr,ssr-nightly,tauri}` (git submodules, own repositories); depend on
  leptonic from git (`branch = "hooks"`, features `atoms`), so they see library changes only once pushed.
- Use: `ThemeProvider` + `LeptonicTheme`, `Button`, `TextField` + `Label` + `Input` (tauri), the atom theme
  (`style/main.scss`: `@use "./leptonic/leptonic-atoms"`; `style/leptonic` is the build script's copy); ssr-nightly
  adds the `nightly` feature. Ported from the components 2026-10-07 (checked against the working tree with a
  `--config` patch of the git dependency; their committed `Cargo.lock`s list leptonic without a source, as that
  patch resolved it, so a plain build resolves `branch = "hooks"` afresh).
- Breaking once pushed (2026-10-09): they import `leptonic::atoms::prelude::*` (`src/pages/welcome.rs`, tauri
  `src/app.rs`), which no longer exists: import from the atom modules (`leptonic::atoms::button::Button`, ...).

## agnite admin-frontend (dormant)

- Where: `~/dev/agnite/admin-frontend`: a path dependency on the working tree (`../../leptonic/leptonic`, default
  features), but it still uses the removed components layer (`leptonic::components::prelude`, `leptonic::prelude`)
  and depends on agnite's own crudkit copy, which uses leptonic from git (branch `migrate-to-leptos-0.8`). It
  doesn't build against the tree (checkout of 2026-10-05).

