# Design: collections and collection state

Why leptonic's collections layer (listbox, select, combo box, menu, grid list, grid, table, tree, tag group, tabs;
DnD uses its keyboard delegates) is built as it is. The code (`leptonic/src/hooks/collections/`, the family hooks) is
the reference for the API; this note keeps the decisions and their reasons. Upstream reference: react-spectrum
@ `99e6102368`.

## Decisions

1. **One concrete `Key`**, no generic `K` (`collections/key.rs`): an integer (`i128`, so `Key::from(1_u8) ==
   Key::from(1_i64)`), a string (`Arc<str>`), a cell key (row key, column key) or a crate-internal structural
   key. Why: the hook layer only needs identity, order, text, flags and links, never the user's type; a concrete key
   compiles the large hook bodies once (smaller wasm, faster builds), keeps delegates object-safe and drops
   `where K: ...` noise. Type safety returns at the atom boundary (`SelectionValue`, C16), as react-aria has
   `Key = string | number`. `Int(1)` and `Str("1")` are different keys, as in JS sets, and `Key::id_fragment` gives
   them different, whitespace-free element ids (`Display` prints them alike).
2. **Collections are built from data the user owns, never derived from the DOM.** A `Collection` is an immutable tree
   of `Node`s (items, sections, headers, rows, cells, columns, loaders) built by a `Memo` over a builder closure that
   reads the user's signals (`use_collection`, `use_list_collection`; tables `TableCollection`). It exists during SSR.
   Rejected: building from rendered children (react-aria-components' second render into a fake DOM, or registration
   from item components): Leptos renders once; during SSR a parent's attributes are emitted before its children run,
   so everything depending on the collection (the select's value, hidden `<option>`s, `aria-activedescendant`, empty
   state, initial focus, `aria-setsize`) would render empty on the server and mismatch on hydration; and `<For>`
   reorders would make registration order differ from DOM order.
3. **State hooks are separate from ARIA hooks** (react-stately vs react-aria): `use_list_state`,
   `use_single_select_list_state`, `use_tree_state`, `use_grid_state`, `use_table_state`, `use_select_state`, ... own
   the state and return `Copy` structs; the ARIA hooks take them as input. That is how one select state drives both
   the trigger and the listbox.
4. **One `SelectionManager`**: a `Copy` struct of signals with methods, a faithful port of `SelectionManager.ts`
   (`'all'` handling, cell-to-row mapping, ranges in collection order, per-item disabled override). The only holder of
   the selection, the focused key, `is_focused`, the disabled logic and the link lookup. Queries track, mutations
   don't (event handlers needing queries use `untrack`).
5. **`KeyboardDelegate` is a non-generic, object-safe trait** (`Arc<dyn KeyboardDelegate>`): list, grid, table
   (composition over the grid delegate, not inheritance) and tabs delegates. Page Up/Down measure through a
   `LayoutDelegate` (`DomLayoutDelegate`) over the item element registry, not `[data-key]` queries. Delegates hold a
   plain `WritingDirection` and are rebuilt when it changes (`keyboard_delegate_memo`); hooks take a
   `Signal<Arc<dyn KeyboardDelegate>>`, overridable as upstream.

## How it works

- **Storage**: the collection is an arena: nodes in a `Vec` in document order, their links (parent, siblings, first
  and last child) by position in a parallel `Vec`, and a `HashMap<Key, u32>` of positions with std's seeded hasher
  (keys are often user-controlled and collections are built on the server: `lessons.md`). Traversal doesn't hash;
  `compare_order` is O(1); `sorted_keys` puts keys in collection order. No structural sharing between rebuilds: nodes
  are compared by content (`Node::same_content`) where it matters (item renderers, the list layout's cache). The
  `Memo` compares with `PartialEq`, so a rebuild with the same structure wakes nobody. Duplicate keys warn in debug
  builds; the first one wins.
- **Rendering contract**: the user (or an atom) renders from the same data, in collection order. Item hooks take
  only the container's data and the `Key` (`UseOptionInput { list, key, .. }`) and derive everything else (selected,
  focused, disabled, text, index, level, set size, link) from state. Item hooks warn in debug builds about a key
  missing from the collection, `ItemElements` about a key rendered twice.
- **Item element registry** (`ItemElements`, the one DOM-derived piece): each key's live registrations
  (`CapturedElement`s), registered by item hooks and removed by their own cleanup; used by focus, scroll-into-view,
  `DomLayoutDelegate` and link opening; empty during SSR. Items render no `data-key`; `data-collection` (an SSR-stable
  `use_id`) lets items tell their own interactive children from nested collections.
- **Per-key notification**: `SelectionManager::is_selected(key)` and `is_focused_key(key)` subscribe their reader to
  that key only; `set_focused_key` notifies the old and new key, a selection change the keys whose selection changed
  (synchronously; a change the app makes to a bound selection, in an effect). Item hooks read focus key-first, so only
  the focused item tracks `is_focused()`.
- **Item contexts**: each container hook returns its items' context (`ListBoxData`, `MenuData`, `GridListData`,
  `GridData`, `TagGroupData`), the Rust counterpart of upstream's per-state `WeakMap`s. `CollectionOptions` holds the
  behavior switches shared by `use_selectable_collection`, `use_selectable_list` and the container hooks.
- **Grids** are ordinary collections whose rows (`Item` nodes) have `Cell` children keyed `Key::cell(row, column)`;
  there is no separate grid collection. Grid cells' column is their position (`Key::cell(&row, 0)`); table cells' is the
  key of the column they start in, the selection checkbox cell's `TableCollection::SELECTION_COLUMN` (react-stately's
  `${rowKey}${column.key}` and `'header'`): a table cell keeps its key, element and focus when columns before it come
  and go (index keys shifted every cell, re-rendered all of them and lost the focus). In cell focus mode the manager
  maps a focused row to its first/last cell, and selection maps cells to their row.
- **Trees**: `TreeState.list.collection` is the flattened visible collection (`with_expanded`), so the list delegate
  and selection work unchanged; `use_tree` is `use_grid_list` with `role="treegrid"`.
- **Focus reset** when the focused item disappears is the state hooks' (list: walk forward in the old collection,
  then backward; grid: row-aware; tree: clear); it covers combo box filtering too.
- **Links as items**: `Node::link` feeds `is_link`; activating the item opens it (`ItemLink::open`: an `<a>` item is
  clicked itself, others through a temporary `<a>`). Item atoms render no `<a href>` for link items yet (`PLAN.md`).
- **Select and combo box values** are `Vec<Key>` in collection order in both modes (at most one in single mode); the
  atoms map them to `S: SelectedValues`. Inside a `Select`/`ComboBox`, `ListBox` reads a `ListBoxParent` context (the
  `UseListBoxInput` the parent's hook returns) and clears it for its children, so a nested `ListBox` isn't the
  select's.

## State inputs

State inputs follow C4 (`conventions.md`): `default_*` + `on_*_change`, or a `ValueBinding` to app state
(`SelectionOptions::selection`, `UseTreeStateInput::expanded_keys`, the sort descriptor, select/combo box values,
tabs); every change goes through the manager's methods. `hooks/mod.rs` records hook-owned state globally; the other
collection deviations are recorded per file (`key.rs`, the `// No upstream:` headers of `use_collection.rs` and
`item_elements.rs`, the deviation blocks of `list_state.rs`, `selection_manager.rs`, `use_selectable_item.rs` and
`table_keyboard_delegate.rs`).

## Typed layer: values built, item collections not

Typed values are built (C16): `RadioGroup<V>`, `CheckboxGroup<V>`, `ToggleButtonGroup<V>` and `Select<S>`/
`ComboBox<S>` convert at the boundary (`atoms/typed_values.rs`). Typed item collections are not: collection atoms
(`ListBox`, `GridList`, `Table`, `TagGroup`, `Menu`) take a `CollectionMemo` the app builds and hand out `Key`s. The
plan (atoms taking `items` + key/text closures, keeping a `Memo<HashMap<Key, T>>` to emit typed callbacks; the generic
surface a few dozen lines per atom, hook bodies compiled once) is open in `PLAN.md`, "Typed item collections".

## Upstream's model, for porting

- `Collection<Node<T>>`: `BaseCollection.getKeyAfter/Before` walks document order, descending into sections but not
  items; `filter()` prunes items by `textValue` and drops sections left empty.
- `useMultipleSelectionState` (selected keys or `'all'` with anchor and current key, focused key, child focus
  strategy, behavior, disabled keys and behavior, duplicate events) + `SelectionManager` (`select` toggles for
  touch/virtual pointers, `toggleSelection` turns `'all'` into a set, `isDisabled` checks `disabledBehavior == 'all'`
  first, `withCollection()` gives a filtered view).
- `useListState` = collection + selection + `useFocusedKeyReset`; `useSingleSelectListState` adds `selectedKey`;
  `useTreeState` adds expanded keys and a flattened collection; `useGridState` focus mode and a row-aware reset;
  `useTableState` columns, header rows and sorting.
- `useSelectableCollection` (keys, focus, scroll, type-ahead through `useTypeSelect` → `getKeyForSearch`, virtual
  focus) serves `useSelectableList` (listbox, menu, grid list, tag group, tree) and `useSelect`/`useComboBox`
  directly; `useSelectableItem` implements primary vs secondary actions and `linkBehavior`.
