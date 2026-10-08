# Leptonic: consumers

Apps built on leptonic's working tree whose sessions report findings and requests. Their open requests are items in
`PLAN.md`; this file records who they are and what they depend on, so library changes can be announced to them.
Rules: consumer sessions don't edit the shared tree; tell them when requested work lands (`SendMessage` to their
session, found with `ListAgents`), and announce breaking changes to the APIs they use.

## crudkit

- Where: `../crudkit` (`crudkit-leptos-ui`); sessions named `crudkit-*`.
- Layers: hooks and atoms only.
- Depends on: the `use_table*`, grid and selection-checkbox hooks (its own table with multi-column sort and
  selection: announce changes to these), TextField + `TextFieldState`, `ToggleState`, `atoms::field` + `FieldContext`
  (its own date/duration widgets), Modal/Dialog atoms, Tabs atoms, disclosure and separator hooks, the NumberField.
- Localizes through the inputs it passes (German, e.g. the table selection checkboxes' `aria_label`): keep such
  overrides working until localized strings land.
- Breaking (2026-10-08, not yet told): `UseTableStateInput` has a new field `tree` (`tree: None` without a tree).
- Breaking (2026-10-08, not yet told): `Select<S>`/`ComboBox<S>` take the selection's shape as their type: `Option<V>`
  selects one value, `Vec<V>` several; `selection_mode` is gone (`crudkit-leptos/src/components/pagination.rs:93`:
  its page size select becomes `Option<..>`, e.g. typed `Option<u64>` with integer item keys).
- History: bindings for Select value, selection and table sort (2026-10-05); sibling `ModalBackdrop`s and fields
  shared one context (fixed: atoms scope their contexts with `scoped_view`/`Provider`); grid focus jumping (the
  interaction modality now tracks from the first hook); `*Input::new` removed (2026-10-07: struct literals, told
  crudkit-2f the table inputs' fields; `UseTableRowInput.on_context_menu: None` without a menu). 2026-10-07
  evening: the single `Checkbox`/`Radio`/`Switch` atoms were removed (use `CheckboxField` + `CheckboxButton`,
  ...); crudkit uses them at `crudkit-leptos/src/components/inputs.rs:171` (`Checkbox`) and
  `examples/full-stack/src/marauder/inputs.rs:121` (`Switch`): not yet told (no live session).

## agnite dev-ui

- Where: `~/dev/agnite/dev/dev-ui`; session `dev-28` (was `dev-3f`, `dev-f8`); asked for `VirtualList` (done 2026-10-07).
- Depends on the working tree directly (`path = "~/dev/leptonic/leptonic"`, features `atoms` + `clipboard`, no
  themed components): every library change reaches it at once, so the atoms feature must always compile.
- Uses: Button, ToggleButton(Group), Switch, Checkbox, RadioGroup/Radio, TextField + Input, Select + ListBox(Items),
  ListBox, GridList (with `ContextMenuTrigger`), Table, Disclosure, Link, ModalBackdrop + ModalContent + Dialog, the
  toast atoms, `use_global_shortcuts`, `ShortcutKeys`, `utils::scroll`, the theme atoms (`ThemeProvider`, `signal_ls`).
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
- History: toasts, row context menus, the `clipboard` feature without components, global shortcuts, a public
  typing check, `ShortcutKeys`, the theme without components (2026-10-06/07); the ComboBox read-order bug and the
  modal hide-outside fix were found there.

## Starter templates

- Where: `examples/leptonic-template-{csr,ssr,ssr-nightly,tauri}` (git submodules, own repositories); depend on
  leptonic from git (`branch = "hooks"`, features `atoms`), so they see library changes only once pushed.
- Use: `ThemeProvider`, `Button`, `TextField` + `Label` + `Input` (tauri), the atom theme (`style/main.scss`:
  `@use "./leptonic/leptonic-atoms"`; `style/leptonic` is the build script's copy). Ported from the components
  2026-10-07 (checked against the working tree with a `--config` patch of the git dependency; their `Cargo.lock`s
  still pin the old revision: `cargo update -p leptonic` after the push).

