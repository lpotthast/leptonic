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
- History: bindings for Select value, selection and table sort (2026-10-05); sibling `ModalBackdrop`s and fields
  shared one context (fixed: atoms scope their contexts with `scoped_view`/`Provider`); grid focus jumping (the
  interaction modality now tracks from the first hook); `*Input::new` removed (2026-10-07: struct literals, told
  crudkit-2f the table inputs' fields; `UseTableRowInput.on_context_menu: None` without a menu).

## agnite dev-ui

- Where: `~/dev/agnite/dev/dev-ui`; session `dev-28` (was `dev-3f`, `dev-f8`); asked for `VirtualList` (done 2026-10-07).
- Depends on the working tree directly (`path = "~/dev/leptonic/leptonic"`, features `atoms` + `clipboard`, no
  themed components): every library change reaches it at once, so the atoms feature must always compile.
- Uses: Button, ToggleButton(Group), Switch, Checkbox, RadioGroup/Radio, TextField + Input, Select + ListBox(Items),
  ListBox, GridList (with `ContextMenuTrigger`), Table, Disclosure, Link, ModalBackdrop + ModalContent + Dialog, the
  toast atoms, `use_global_shortcuts`, `ShortcutKeys`, `utils::scroll`, the theme atoms (`ThemeProvider`, `signal_ls`).
- History: toasts, row context menus, the `clipboard` feature without components, global shortcuts, a public
  typing check, `ShortcutKeys`, the theme without components (2026-10-06/07); the ComboBox read-order bug and the
  modal hide-outside fix were found there.
