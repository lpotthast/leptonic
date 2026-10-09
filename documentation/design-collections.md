# Design note: Collections and collection state in leptonic

Status: **implemented** (2026-10; history in `documentation/history.md`). This note keeps the design's rationale; the
code (`leptonic/src/hooks/collections/`, the family hooks) is the reference where they differ. Upstream reference:
react-spectrum @ `99e6102368`.

Implemented differently from the proposal below:
- No `TypedKey`: values name their key through `From<T> for Key` or `SelectionValue` (`collections/key.rs`).
  Integer keys hold any integer type's value (`i128`); `Key::id_fragment` gives unambiguous element ids.
- §4.2's node storage was not built as an arena: `Collection` is a `HashMap<Key, Node>` with key links between
  nodes (an arena with structural sharing is an open item in `PLAN.md`).
- Per-key notification (built 2026-10-09, in place of the design's `is_*_signal(key)` helpers over a leptos
  `Selector`, which updates in an effect): `SelectionManager::is_selected(key)` and `is_focused_key(key)` subscribe
  their reader to that key only (`KeySubscribers`); `set_focused_key` notifies the old and new key, selection
  changes the keys whose selection changed (synchronously; a change the app makes to a bound selection, in an
  effect).
- State inputs follow C4 (`documentation/conventions.md`): besides `default_*`, a `ValueBinding` to app state
  (`SelectionOptions::selection`, `expanded_keys`, `sort_descriptor`, select/combo box `value`), see §4.11.
- Nodes carry their document position (crate-private `position`): `Collection::compare_order` is O(1), and
  `sorted_keys` puts keys in collection order (`first/last_selected_key`, range selection, the values of
  `use_select_state`/`use_combobox_state`).
- `Key::cell(row, column)` takes the column as a `usize`.
- `NodeKind` is `Item, Section, Header, Separator, Loader, Cell, HeaderRow, Column, Placeholder` (rows are `Item`s with
  `Cell` children; no `Row`/`RowHeader`/`Body`); `Node` has `prev_key`/`next_key`/`first_child_key`/`last_child_key`
  and `has_child_nodes`, links are `ItemLink`. Tables use `TableCollection` (`hooks/table/table_collection.rs`).
- Module layout: everything of §4's `collections`/`selection`/`list` lives in `hooks/collections/` (see §4).
- §4.10's typed atom layer is built only for values: `Select<S>`/`ComboBox<S>` (`S: SelectedValues`: `Option<V>`
  or `HashSet<V>`, the shape is the selection mode), `RadioGroup<V>`, `CheckboxGroup<V>`, `ToggleButtonGroup<V>`
  take `V: SelectionValue` (`atoms/typed_values.rs`). Collections are not typed: atoms take a `CollectionMemo` the
  app builds (`use_collection`, `use_list_collection`), and the other collection atoms (`ListBox`, `GridList`,
  `Table`, `TagGroup`, `Menu`) take `default_selection: Selection` and hand out `Key`s.

Scope: listbox, select, combobox, menu, grid list, grid, table, tree, tag group, tabs (and DnD, which uses `KeyboardDelegate`).

---

## 1. Decision summary

1. **One non-generic `Key` type.** Hooks use a concrete `Key` (cheap-clone newtype over an integer or `Arc<str>`) in place of the former `K: SelectionKey` generic. A thin typed layer for atoms and components maps the user's types to and from `Key`.
2. **Collections are built from data the user owns. They are never derived from the DOM.** A `Collection` is an immutable tree of `Node`s (items, sections, rows, cells, columns), produced by a `Memo` over a builder closure that reads the user's signals. It exists during SSR.
3. **State hooks are separate from ARIA hooks** (react-stately vs react-aria):
   - `use_list_state`, `use_single_select_list_state`, `use_tree_state`, `use_grid_state` and `use_table_state` own the selection signals (Hook-Owned State rule) and return a `Copy` state struct.
   - ARIA hooks (`use_listbox`, `use_select`, `use_grid_list`, ...) take that state struct as input.
   - This is how one `select` state drives both the trigger and the listbox.
4. **One `SelectionManager`.** It is a `Copy` struct of signals with real methods (no bag of `Callback`s). It is the only place that holds the selection, the focused key, `is_focused`, the disabled logic and the link lookup.
5. **`KeyboardDelegate` is a non-generic, object-safe trait** used as `Arc<dyn KeyboardDelegate>`. There are list, grid, table and tabs delegates. Page Up/Down use a `LayoutDelegate` that measures elements through a key→element registry filled by `CapturedElement`. The `data-key` querySelector lookup goes away.
6. **Migration is incremental, family by family.** It starts with listbox, then select (this fixes a real bug, see section 3). Each step first adds DOM/ARIA-level browser tests that keep passing across the rewrite.

---

## 2. How react-stately / react-aria model this

- **`Collection<Node<T>>`.** A `Node` carries `key, type ('item'|'section'|'header'|'cell'|'row'|'column'|'loader'…), textValue, level, index, parentKey, prev/nextKey, first/lastChildKey, hasChildNodes, props (isDisabled, href, disabledBehavior)`.
  - `BaseCollection.getKeyAfter/Before` (`react-aria/src/collections/BaseCollection.ts:200-245`) walks document order. It descends into non-item nodes (sections) but not into items.
  - `filter()` prunes items by `textValue` and drops sections left empty (`SectionNode.filter`).
- **Building.** The old `CollectionBuilder` (react-stately) walks the JSX `<Item>/<Section>` elements. The RAC builder (`react-aria/src/collections/CollectionBuilder.tsx`, `Document.ts`) renders the children a second time into a fake DOM inside a hidden portal, then publishes immutable snapshots through `useSyncExternalStore`. SSR needs a special `SSRContext` path (lines 128-196).
- **`useMultipleSelectionState`** holds `selectedKeys | 'all'` (`Selection` with `anchorKey`/`currentKey`), `focusedKey`, `childFocusStrategy`, `isFocused`, `selectionBehavior`, `disabledKeys`, `disabledBehavior` and `allowDuplicateSelectionEvents`.
- **`SelectionManager(collection, state, {layoutDelegate, allowsCellSelection, fullCollection})`** (`react-stately/src/selection/SelectionManager.ts`):
  - `extendSelection` uses `getKeyRange`, which follows collection order (or `layoutDelegate.getKeyRange`).
  - `select(key, e)` toggles for touch/virtual pointers.
  - `toggleSelection` turns `'all'` into an explicit set.
  - `isSelected` returns `canSelectItem` when the selection is `'all'`.
  - `isDisabled` checks `disabledBehavior == 'all'`, then `disabledKeys`/`props.isDisabled`, with a per-item override.
  - `isLink(key)` checks `props.href`.
  - `withCollection()` gives a filtered view.
- **State hooks.**
  - `useListState` = collection + selection + `useFocusedKeyReset`: when the focused item disappears, focus moves to the next surviving item, else the previous one (`useListState.ts:107-150`).
  - `useSingleSelectListState` wraps it with `selectedKey`, `disallowEmptySelection` and duplicate events.
  - `useTreeState` adds `expandedKeys` and a flattened `TreeCollection` (only expanded children are visited).
  - `useGridState` adds `focusMode` (row/cell) and a row-aware focus reset.
  - `useTableState` adds columns, header rows and sorting.
- **Delegates.**
  - `ListKeyboardDelegate` (`react-aria/src/selection/ListKeyboardDelegate.ts`) takes `layout: stack|grid`, orientation, direction, `disabledBehavior`, a `LayoutDelegate` (`DOMLayoutDelegate`: item rect, visible rect, content size) and a collator for `getKeyForSearch`. It skips non-item nodes and disabled items, and handles reversed layouts and page up/down.
  - `GridKeyboardDelegate` handles row vs cell focus mode, RTL and page up/down. `TableKeyboardDelegate` extends it to cover column headers.
  - `TabsKeyboardDelegate` wraps and flips direction for RTL.
- **Who uses what.** `useSelectableCollection` (keys, focus, scroll, type-ahead through `useTypeSelect` → `delegate.getKeyForSearch`, virtual focus) is used by:
  - `useSelectableList`, which serves listbox, menu, grid list, tag group, and tree (tree = grid list with `role="treegrid"`).
  - `useSelect` and `useComboBox` directly; combobox uses virtual focus on the input.
- **Item hooks** read per-container data (`id`, `onAction`, `linkBehavior`, …) from a `WeakMap` keyed by `state`. `useSelectableItem` implements primary vs secondary actions and `linkBehavior` (`action|selection|override|none`).

---

## 3. The approach it replaced

Before (2026-10-05): every collection family had its own key list (`items: Signal<Vec<K>>`, generic over a
`SelectionKey`), its own selection and focus state (often two, e.g. select + its listbox, grid + `focused_key`),
hand-rolled "next enabled key" navigation, controlled-mode bugs, no type-ahead in listboxes, no sections or tree
structure, DOM lookups by `[data-key]`, RTL hard-coded to LTR, five `SelectionMode` enums, and no browser tests. The
design below replaced all of it.

---

## 4. Design

Module layout (as implemented; the design split it into `collections/`, `selection/` and `list/`):
- `hooks/collections/`: `key.rs` (`Key`, `SelectionValue`, `selection_value!`), `node.rs`, `collection.rs` (with the
  builders), `use_collection.rs`, `item_elements.rs`, `selection.rs`, `selection_manager.rs` (with
  `SelectionOptions`, upstream's `useMultipleSelectionState`), `keyboard_delegate.rs` (trait + list delegate),
  `layout.rs` (`LayoutDelegate`, `DomLayoutDelegate`), `list_state.rs` (`use_list_state`,
  `use_single_select_list_state`), `use_selectable_collection.rs`, `use_selectable_list.rs`,
  `use_selectable_item.rs`, `use_type_select.rs`, `modifiers.rs`.
- Per family: `tree/use_tree_state.rs`, `grid/use_grid_state.rs`, `grid/grid_keyboard_delegate.rs`,
  `table/use_table_state.rs`, `table/table_collection.rs`, `table/table_keyboard_delegate.rs`,
  `tabs/tabs_keyboard_delegate.rs`.

### 4.1 `Key`: concrete, not generic

As implemented (`hooks/collections/key.rs`; the design had `Int(i64)`, a `Pair(Key, Key)` cell key and `TypedKey`):

```rust
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Key(Repr);
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Repr {
    Int(i128),                     // every integer type's values: Key::from(1_u8) == Key::from(1_i64)
    Str(Arc<str>),
    Cell(Arc<(Key, usize)>),       // generated cell keys (row, column index)
    Generated(&'static str, usize) // structural nodes (table header rows, placeholders), crate-internal
}

impl Key {
    pub fn as_str(&self) -> Option<&str>;
    pub fn as_i64(&self) -> Option<i64>;
    pub fn cell(row: &Key, column: usize) -> Key;
    pub fn id_fragment(&self) -> String;   // unambiguous, whitespace-free part of element ids
}
// From<&str|String|&String|Arc<str>>, From<i8..i64|isize|u8..u64|usize>, Display, Debug

/// A typed value atoms select, and its key (impl for Key, String, integers; enums via `selection_value!`).
pub trait SelectionValue: Clone + Eq + Hash + Send + Sync + 'static {
    fn to_key(&self) -> Key;
    fn from_key(key: &Key) -> Option<Self>;
}
// selection_value!(Size { Small = "s", Large = "l" }): SelectionValue + From<Size> for Key (distinct keys checked)
```

Why not generic `K`:
- The hook layer never needs the user's type. It only needs identity, order, text, flags and links.
- A concrete key compiles the large hook bodies once. That means smaller wasm, faster builds, object-safe delegates, and no `where K: SelectionKey` noise.
- Type safety comes back at the atom/component layer, where values are already owned (see 4.10). This mirrors react-aria (`Key = string | number`).
- Trade-off: hook-level callbacks hand out `Key`, not `MyEnum`. `SelectionValue::from_key` closes that gap.
- `Int(1)` and `Str("1")` are different keys, as in JS sets. DOM ids derived from keys must normalize whitespace, as `getItemId` does: `Key::id_fragment` escapes whitespace and `%` and prefixes non-string keys (`%i1`), so `Int(1)` and `Str("1")` also get different ids (`Display` prints them alike).

### 4.2 `Node` and `Collection`

As implemented (`node.rs`, `collection.rs`; the design's arena storage and `Row`/`RowHeader`/`Body` kinds were not
built, see the notes at the top):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKind { Item, Section, Header, Separator, Loader, Cell, HeaderRow, Column, Placeholder }

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Node {
    pub key: Key, pub kind: NodeKind,
    pub text_value: Arc<str>, pub aria_label: Option<Arc<str>>,
    pub level: usize,                              // tree depth; sections don't add a level
    pub index: usize,                              // position among the parent's children
    pub parent_key: Option<Key>, pub prev_key: Option<Key>, pub next_key: Option<Key>,
    pub first_child_key: Option<Key>, pub last_child_key: Option<Key>,
    pub has_child_nodes: bool,
    pub is_disabled: bool, pub disabled_behavior: Option<DisabledBehavior>, // per-item override
    pub link: Option<ItemLink>,                    // links-as-items
    pub col_index: Option<usize>, pub col_span: Option<usize>,
    pub(crate) position: usize,                    // document order, for O(1) compare_order
}
pub struct ItemLink { pub href: Arc<str>, pub target: Option<Arc<str>>, pub rel: Option<Arc<str>> }

/// Immutable; `HashMap<Key, Node>` with key links between nodes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Collection { /* private */ }
impl Collection {
    pub fn build(f: impl FnOnce(&mut CollectionBuilder)) -> Self;
    pub fn size(&self) -> usize;                                   // items only
    pub fn get(&self, key: &Key) -> Option<&Node>;  pub fn contains_key(&self, key: &Key) -> bool;
    pub fn first_key(&self) -> Option<&Key>;  pub fn last_key(&self) -> Option<&Key>;
    pub fn key_after(&self, key: &Key) -> Option<&Key>;  pub fn key_before(&self, key: &Key) -> Option<&Key>; // BaseCollection semantics
    pub fn iter(&self) -> impl Iterator<Item = &Node>;             // top-level nodes
    pub fn children(&self, parent: &Key) -> impl Iterator<Item = &Node>;
    pub fn cells(&self, row: &Key) -> impl Iterator<Item = &Node>;
    pub fn items(&self) -> impl Iterator<Item = &Node>;            // all Item nodes, document order
    pub fn compare_order(&self, a: &Key, b: &Key) -> Option<Ordering>;
    pub fn sorted_keys(&self, keys: impl IntoIterator<Item = Key>) -> Vec<Key>;
    pub fn item_keys_between(&self, from: &Key, to: &Key) -> Vec<Key>;  // range selection
    pub fn filter(&self, keep: impl Fn(&str, &Node) -> bool) -> Collection;  // prunes empty sections
    pub fn with_expanded(&self, expanded: &HashSet<Key>) -> Collection;      // a tree's visible part
}
```

Builder (sections, tree children, grids and tables share one API):

```rust
impl CollectionBuilder {
    pub fn item(&mut self, key: impl Into<Key>, text_value: impl Into<Arc<str>>) -> ItemBuilder<'_>;
    pub fn section(&mut self, key: impl Into<Key>, contents: impl FnOnce(&mut CollectionBuilder)) -> SectionBuilder<'_>;
    pub fn header(&mut self, key: impl Into<Key>, text: impl Into<Arc<str>>);  // a section's heading, first in it
    pub fn separator(&mut self, key: impl Into<Key>);
    pub fn loader(&mut self, key: impl Into<Key>);
    pub fn row(&mut self, key: impl Into<Key>, text_value: impl Into<Arc<str>>, cells: impl FnOnce(&mut RowBuilder)) -> ItemBuilder<'_>;
}
impl ItemBuilder<'_> {
    pub fn disabled(self, v: bool) -> Self;  pub fn disabled_behavior(self, b: DisabledBehavior) -> Self;
    pub fn aria_label(self, l: impl Into<Arc<str>>) -> Self;  pub fn link(self, l: ItemLink) -> Self;
    pub fn children(self, f: impl FnOnce(&mut CollectionBuilder)) -> Self;   // tree items
}
// RowBuilder::cell(text) -> CellBuilder (col_span, aria_label) keys cells with Key::cell(row, i). TableCollection
// (hooks/table/table_collection.rs) adds columns and header rows:
// `TableCollection::build(|t| { t.column(..); t.column_group(.., |g| ..); t.row(..) /* or t.rows(|b| ..) */ })`.
```

Duplicate keys log a warning in debug builds (`dev_warn!`) and the first one (with its subtree) wins.

### 4.3 Reactive source: data-driven, SSR-safe

```rust
pub type CollectionMemo = Memo<Arc<Collection>>;
/// Tracks every signal read inside `build`; rebuilds O(n) on change, notifies only if structurally changed.
pub fn use_collection(build: impl Fn(&mut CollectionBuilder) + Send + Sync + 'static) -> CollectionMemo;
pub fn use_list_collection<T, K, Tx, S>(input: UseListCollectionInput<T, K, Tx, S>) -> CollectionMemo;
// UseListCollectionInput { items: Signal<Vec<T>>, key: K /* Fn(&T) -> Key */, text_value: Tx /* Fn(&T) -> S, S: Into<Arc<str>> */ }
```

It is a `Memo` (which compares with `PartialEq`) over `Arc<Collection>`, so a rebuild that produces the same
structure does not wake dependents.

Example:

```rust
let collection = use_collection(move |b| {
    for group in fruits_by_group.read().iter() {
        b.section(group.id, |s| {
            s.header(format!("{}-header", group.id), group.name.clone());
            for f in &group.items { s.item(f.id, f.name.clone()).disabled(f.sold_out); }
        });
    }
});
```

**Rejected: building from rendered children (the RAC approach, or registration from item components).**
- Leptos renders once; there is no two-pass or fake-DOM render.
- During SSR the parent's attributes are emitted before the children run. Anything that depends on the collection would render empty on the server and then mismatch on hydration: select display value, hidden `<option>`s, `aria-activedescendant`, empty state, initial focus key, `aria-setsize`.
- `<For>` reorders would make registration order differ from DOM order.

With data-driven collections, all of that is computed on the server from the same data (compare RAC's `*.ssr.test.js`).

**Rendering contract.** The user (or an atom) renders from the same data, in collection order. Item hooks take only the `Key` and read everything else from the node. In debug builds, `use_option` warns about a key that is not in the collection, and `ItemElements` about a key rendered twice.

**Item element registry (the one DOM-derived piece).** `ItemElements` is a `StoredValue` of each key's live registrations (`CapturedElement`s with registration ids, the latest last) plus a `Trigger`.
- Item hooks register their `CapturedElement` and unregister (only their own registration) in `on_cleanup`.
- It is used in effects and event handlers: focus, scroll-into-view, `DomLayoutDelegate`, opening links. `get_tracked` lets a reactive context follow an item being rendered (e.g. by a virtualizer).
- It is empty during SSR, which is fine.
- It replaces the `[data-key]` selectors; items render no `data-key`. `data-collection` stays (as upstream, now an SSR-stable `use_id`): items use it to tell their own interactive children from nested collections.

### 4.4 Selection: `use_multiple_selection_state` + `SelectionManager`

As implemented (`selection_manager.rs`; the design had `default_selected_keys` without a binding, a layout delegate
in the manager and `is_*_signal(key)` helpers; there is no `use_multiple_selection_state`: `SelectionManager::new`
creates the state from `SelectionOptions`):

```rust
pub struct SelectionOptions {                       // part of every *StateInput
    pub selection_mode: Signal<SelectionMode>,
    pub selection_behavior: Signal<SelectionBehavior>, // a change applies right away
    pub default_selection: Selection,                // ignored when `selection` is bound
    pub selection: Option<ValueBinding<Selection>>,  // C4: the selection as app state
    pub on_selection_change: Option<Callback<Selection>>,
    pub disallow_empty_selection: Signal<bool>,
    pub disabled_keys: Signal<HashSet<Key>>,
    pub disabled_behavior: DisabledBehavior,
    pub allow_duplicate_selection_events: bool,
}

#[derive(Clone, Copy)]
pub struct SelectionManager { /* collection, full_collection: Option<CollectionMemo>, signals + KeySubscribers, cell_focus */ }
impl SelectionManager {
    pub fn new(collection: CollectionMemo, options: SelectionOptions) -> Self;
    // queries: tracked (use inside Signal::derive / views)
    pub fn selection_mode(&self) -> SelectionMode;   pub fn selection_behavior(&self) -> SelectionBehavior;
    pub fn raw_selection(&self) -> Selection;        pub fn selected_keys(&self) -> HashSet<Key>;
    pub fn is_selected(&self, key: &Key) -> bool;    // tracks only this key
    pub fn is_empty(&self) -> bool;  pub fn is_select_all(&self) -> bool;
    pub fn first_selected_key(&self) -> Option<Key>; pub fn last_selected_key(&self) -> Option<Key>;
    pub fn focused_key(&self) -> Option<Key>;        pub fn is_focused(&self) -> bool;
    pub fn is_focused_key(&self, key: &Key) -> bool; // tracks only this key
    pub fn has_focused_key(&self) -> bool;
    pub fn child_focus_strategy(&self) -> Option<FocusStrategy>;
    pub fn is_disabled(&self, key: &Key) -> bool;    pub fn is_item_disabled(&self, key: &Key) -> bool;
    pub fn can_select_item(&self, key: &Key) -> bool; pub fn is_link(&self, key: &Key) -> bool;
    // mutations: never track; fire on_selection_change
    pub fn set_focused(&self, v: bool);  pub fn set_focused_key(&self, key: Option<Key>, child: Option<FocusStrategy>);
    pub fn select(&self, key: &Key, pointer: Option<&PointerType>);
    pub fn toggle_selection(&self, key: &Key);  pub fn replace_selection(&self, key: &Key);
    pub fn extend_selection(&self, to: &Key);   pub fn set_selected_keys(&self, keys: impl IntoIterator<Item = Key>);
    pub fn select_all(&self);  pub fn clear_selection(&self);  pub fn toggle_select_all(&self);
    pub fn set_selection_behavior(&self, b: SelectionBehavior);
    // views
    pub fn with_collection(&self, view: CollectionMemo) -> Self;   // filtered views (combobox)
    pub fn with_own_selection(&self, options: SelectionOptions) -> Self; // a group's selection (menu sections)
    pub fn with_cell_focus(&self) -> Self;                          // grids in cell focus mode
}
```

- The methods are a faithful port of `SelectionManager.ts`: `'all'` handling, `getKey` parent mapping for cells, `getKeyRange` over collection order (sections and headers skipped), disabled override per item.
- Convention: queries read tracked, mutations read untracked. Event handlers that need queries call `untrack(|| ...)`.
- `Selection` is `All | Keys(SelectedKeys)` (`SelectedKeys`: the keys plus the range anchor and current key; equality
  compares the keys only). The old `Selection::contains(all_keys)` is gone.

### 4.5 State hooks (all return `Copy` structs)

As implemented (state inputs follow C4: `default_*` + `on_*_change`, or a `ValueBinding` to app state):

```rust
pub struct ListState { pub collection: CollectionMemo, pub selection: SelectionManager, pub item_elements: ItemElements }
pub fn use_list_state(i: UseListStateInput /* collection, selection: SelectionOptions */) -> ListState;
// installs the useFocusedKeyReset effect (walk forward in the old collection, then backward)
pub fn use_list_state_view(i: UseListStateViewInput /* state, collection: a subset */) -> ListState; // shared selection

pub struct SingleSelectListState { pub list: ListState }
impl SingleSelectListState {
    pub fn selected_key(&self) -> Option<Key>;  pub fn set_selected_key(&self, key: Option<Key>);
    pub fn selected_item(&self) -> Option<Node>;
}

pub struct TreeState { pub list: ListState /* visible = with_expanded */, pub expansion: TreeExpansion }
impl TreeExpansion { pub fn is_expanded(&self, k: &Key) -> bool; pub fn toggle_key(&self, k: Key); } // + expanded_keys signal
impl TreeState { pub fn set_expanded_keys(&self, ks: HashSet<Key>); }

pub struct GridState { pub list: ListState, pub focus_mode: GridFocusMode, /* keyboard navigation disabled */ } // row-aware focus reset
pub struct TableState { pub grid: GridState, pub table: Memo<Arc<TableCollection>>, pub columns: Memo<Arc<[Column]>>,
    pub tree: Option<TableTree>, pub sort_descriptor: Signal<Option<SortDescriptor>> /* + sort(column, direction) */ }
pub struct SelectState { pub list: ListState, pub selection_mode: SelectMode, pub menu_trigger: MenuTriggerState,
    pub validation: FormValidationState /* + value() -> Vec<Key>, set_value, open/close/toggle, focus_strategy */ }
pub struct ComboBoxState { pub list: ListState /* filtered view while open */, pub selection_mode: SelectMode,
    pub validation: FormValidationState /* + value(), input_value(), set_input_value, open/close/toggle, commit, revert */ }
pub struct TabListState { pub list: SingleSelectListState, pub is_disabled: Signal<bool> /* + tab_id, tab_panel_id */ }
```

Select and combo box values are `Vec<Key>` in collection order in both modes (at most one key in `Single`), not a
`SingleSelectListState`; the atoms map them to `S: SelectedValues`.

### 4.6 Delegates

As implemented (`keyboard_delegate.rs`, `layout.rs`, the family delegates; the design had `NavOptions`,
`supports_horizontal`, `LayoutDelegate::key_range` and signal-valued direction):

```rust
#[derive(Default, Clone, Copy)] pub struct NavigationOptions { pub include_disabled: bool } // needed by DnD
pub trait KeyboardDelegate: Send + Sync {        // every method defaults to None ("no such key")
    fn key_below(&self, k: &Key, o: NavigationOptions) -> Option<Key> { None }
    fn key_above(&self, k: &Key, o: NavigationOptions) -> Option<Key> { None }
    fn key_left_of(&self, k: &Key, o: NavigationOptions) -> Option<Key> { None } // None replaces upstream deleting
    fn key_right_of(&self, k: &Key, o: NavigationOptions) -> Option<Key> { None } // getKeyLeftOf on vertical stacks
    fn key_page_above(&self, k: &Key) -> Option<Key> { None }
    fn key_page_below(&self, k: &Key) -> Option<Key> { None }
    fn first_key(&self, from: Option<&Key>, global: bool) -> Option<Key> { None }
    fn last_key(&self, from: Option<&Key>, global: bool) -> Option<Key> { None }
    fn key_for_search(&self, search: &str, from: Option<&Key>) -> Option<Key> { None }
}
pub fn keyboard_delegate_memo(build: impl Fn() -> Arc<dyn KeyboardDelegate> + ..) -> Signal<Arc<dyn KeyboardDelegate>>;
pub trait LayoutDelegate: Send + Sync {
    fn item_rect(&self, k: &Key) -> Option<Rect>;  fn visible_rect(&self) -> Rect;  fn content_size(&self) -> Size;
    fn is_scrollable(&self) -> bool { /* content larger than visible */ }
}
pub struct DomLayoutDelegate { container: CapturedElement, items: ItemElements } // port of DOMLayoutDelegate; empty on SSR
pub struct ListKeyboardDelegate { collection: CollectionMemo, selection: SelectionManager, layout: ListLayout /* Stack|Grid */,
    orientation: Orientation, direction: WritingDirection, layout_delegate: Arc<dyn LayoutDelegate>, collator: Option<Arc<Collator>> }
pub struct GridKeyboardDelegate { /* collection, selection, layout_delegate, direction, collator, focus_mode */ }
pub struct TableKeyboardDelegate { grid: GridKeyboardDelegate, table: Memo<Arc<TableCollection>>, /* direction, collator */ } // composition, not inheritance
pub struct TabsKeyboardDelegate { /* collection, selection, direction, orientation; wraps, flips left/right in RTL */ }
```

- Delegates read through `collection.with_untracked(|c| ...)`. There are no `Vec` clones.
- Disabled checks go through `SelectionManager::is_disabled`, so there is one source of truth.
- `key_for_search` uses the ICU4X `Collator` (`utils/filter.rs`, `use_collator`). `use_type_select` is a thin port that calls `delegate.key_for_search`, which also skips disabled items.
- Direction comes from `use_direction()` (C5), replacing the `Ltr` TODOs. Delegates hold a plain `WritingDirection`
  and are rebuilt when it changes: hooks take them as `Signal<Arc<dyn KeyboardDelegate>>` (`keyboard_delegate_memo`).
- Hooks accept `keyboard_delegate: Option<Signal<Arc<dyn KeyboardDelegate>>>` as an override, as upstream does.

### 4.7 ARIA layer

> As implemented (listbox + select): behavior switches live in `CollectionOptions` (shared by
> `use_selectable_collection`, `use_selectable_list`, `use_listbox`, ...), `auto_focus` is a
> `Signal<Option<AutoFocus>>` (`Selected | First | Last`, read at mount), keyboard delegates are passed as
> `Signal<Arc<dyn KeyboardDelegate>>` (they depend on the locale), and the listbox's item context is
> `ListBoxData`, returned by `use_listbox` and passed to `use_option(UseOptionInput { list, key })`.
>
> As implemented (grid): grids are ordinary collections whose rows (`Item` nodes, `CollectionBuilder::row`) have
> `Cell` children with generated keys (`Key::cell(row, column)`, optional `col_span`; `col_index` is set in rows
> with spanning cells). There is no separate `GridCollection`. In cell focus mode, the `SelectionManager` maps a
> focused row to its first/last cell; selection maps cells to their row. `use_grid` returns `GridData` for
> `use_grid_row`/`use_grid_cell`.

- `use_selectable_collection(UseSelectableCollectionInput { selection, item_elements, delegate: Signal<Arc<dyn KeyboardDelegate>>, element: CapturedElement, options: CollectionOptions })`; `CollectionOptions` holds `auto_focus, should_focus_wrap, disallow_empty_selection, disallow_select_all, escape_key_behavior, select_on_focus, disallow_type_ahead, allows_tab_navigation, should_use_virtual_focus, link_behavior` (`is_virtualized` is on the listbox input).
  - It is no longer generic, and no longer creates selection state.
  - It gains virtual focus (`aria-activedescendant`) for combobox.
- Each container hook returns an **item context**: the Rust equivalent of upstream's `listMap`/`listData` `WeakMap`. Example: `ListBoxData { state: ListState, id: String, collection_id, should_select_on_press_up, should_focus_on_hover, link_behavior, on_action, should_use_virtual_focus, is_virtualized }` (likewise `MenuData`, `GridListData`, `GridData`, `TagGroupData`).
  - Item hooks take the container's data and the key (`UseOptionInput { list, key, .. }`, `UseMenuItemInput { menu, key, .. }`, `UseTreeItemInput { tree, key, .. }`, `UseGridRowInput { grid, key, .. }`, `UseTagInput { group, key }`). Selected, focused, disabled, text value, index, level, set size and link are all derived from state.
  - This removes `row_index`, `is_disabled`, `text_value`, `focused_key`, `on_focus`, … from item inputs.
- `use_selectable_item(UseSelectableItemInput { selection, item_elements, key, element: CapturedElement, id, collection_id, is_disabled, should_select_on_press_up, allows_different_press_origin, on_action, link_behavior, focus, should_use_virtual_focus, on_context_menu })`.
  - This is the full port of the primary/secondary action model and `linkBehavior`. Opening a link goes through `ItemLink::open` (`node.rs`) on the registered element: an `<a>` item is clicked itself (`utils/open_link.rs`), other elements through a temporary `<a>`.
  - It registers its element in `item_elements`.

### 4.8 Cross-cutting behavior

- **Disabled state:** `disabled_keys` (state input) and the node flag `.disabled()` are combined only in `SelectionManager::is_disabled/can_select_item`, with upstream semantics. `DisabledBehavior::Selection` keeps items focusable and actionable.
- **Sections:** these are `Section` nodes (their heading a `Header` node). Navigation skips non-item nodes. `use_listbox_section`/`use_menu_section` take `{ list, key }`/`{ menu, key }` and read the heading and `aria-label` from the nodes.
- **Focus reset:** handled by the state hooks (list: walk forward, then back; grid: row-aware; tree: clear). It also covers combobox filtering.
- **Tree:** `TreeState.list.collection` is the flattened visible collection, so the list delegate and selection work unchanged. `use_tree` is `use_grid_list` with `role=treegrid`. `use_tree_item` adds `aria-expanded/level/posinset/setsize` from nodes and ArrowRight/ArrowLeft expand, collapse and parent navigation, following upstream `useGridListItem` tree mode.
- **Links as items:** `Node::link` feeds `manager.is_link`; activating the item opens it (`ItemLink::open`, see 4.7). The item atoms render no `<a href>` for link items. The tag group uses `link_behavior: Override`, as upstream.

### 4.9 Consumer sketches

Updated to the implemented API. The inputs have no `Default` (C7): `/* .. */` stands for the remaining fields, which
callers name.

```rust
// listbox + option
let state = use_list_state(UseListStateInput { collection, selection: SelectionOptions { selection_mode: Signal::stored(SelectionMode::Multiple), ..Default::default() } });
let UseListBoxReturn { props, data } = use_listbox(UseListBoxInput { state, aria_label: "Fruits".into(), /* .. */ });
view! { <ul {..props.into_attrs()}>
  <For each=move || collection.with(|c| c.items().map(|n| n.key.clone()).collect::<Vec<_>>()) key=Clone::clone children=move |key| {
      let (attrs, styles) = use_option(UseOptionInput { list: data.clone(), key, on_context_menu: None }).props.into_parts();
      view! { <li {..attrs} style=styles>/* .. */</li> } } />
</ul> }

// select: ONE state shared by trigger and listbox (removes UseSelectMenuConfig + the sync Effect)
let state = use_select_state(UseSelectStateInput { collection, selection_mode: SelectMode::Single, default_value: vec![], on_change: Some(cb), /* .. */ });
let select = use_select(UseSelectInput { state, /* .. */ });
let lb = use_listbox(select.listbox);   // UseListBoxInput { state: state.list, auto focus, should_select_on_press_up: true, .. }

// combobox: same list state, filtered view; the input keeps DOM focus, the listbox uses virtual focus
let state = use_combobox_state(UseComboBoxStateInput { collection, filter: Some(use_contains_filter()), default_input_value: None, /* .. */ });
let cb = use_combobox(UseComboBoxInput { state, /* .. */ });   // input props: aria-activedescendant from the focused key
let lb = use_listbox(cb.listbox);                               // its CollectionOptions: should_use_virtual_focus

// grid list
let UseGridListReturn { props, data } = use_grid_list(UseGridListInput { state: use_list_state(/* .. */), on_action: Some(open), /* .. */ });
let item = use_grid_list_item(UseGridListItemInput { list: data.clone(), key, /* .. */ });

// tree
let state = use_tree_state(UseTreeStateInput { collection /* built with .children() */, default_expanded_keys, /* .. */ });
let tree = use_tree(UseTreeInput { state, /* .. */ });          // returns UseGridListReturn
let row = use_tree_item(UseTreeItemInput { tree: tree.data.clone(), key, /* .. */ });
```

### 4.10 Typed layer for atoms and components (values built, item collections not)

- Atoms and components stay ergonomic and typed: for example `ListBox<T> items: Signal<Vec<T>>, key: fn(&T) -> K, text_value: ...` and `Select<O>`.
- They build the `CollectionMemo` from `items` and keep a `Memo<HashMap<Key, T>>`, so they can emit `on_change: Callback<O>` / `Vec<T>`.
- The generic surface is a few dozen lines per atom; the hook bodies are compiled once.
- `Keyed` is replaced by `TypedKey` (for keys), or by key closures.
- Inside `Select`, `ListBox` reads `SelectCtx { state }` in place of `menu_config`.

As implemented:
- Typed values: `SelectionValue` (in place of `TypedKey`, see 4.1) for `RadioGroup<V>`, `CheckboxGroup<V>`,
  `ToggleButtonGroup<V>`; `Select<S>`/`ComboBox<S>` with `S: SelectedValues` (sealed; `Option<V>` selects one value,
  `HashSet<V>` several, so the type is the selection mode). `atoms/typed_values.rs` converts the typed state props
  (C4) to the hooks' keys at the boundary (a crate-private `Keyed`, unrelated to the old trait of that name).
- Not built: `items` + key/text closures and the `Memo<HashMap<Key, T>>`; collection atoms take a `CollectionMemo`
  and emit `Key`s (`on_action: Callback<Key>`). Open in `PLAN.md` ("Typed item collections").
- Inside `Select` (and `ComboBox`), `ListBox` reads a `ListBoxParent` context (the `UseListBoxInput` that
  `use_select`/`use_combobox` return) in place of `menu_config`; the listbox clears it for its own children
  (`clear_context`, `utils/scoped_context.rs`), so a nested `ListBox` is not the select's.

### 4.11 Deviations to record

Record these in `hooks/mod.rs` (global) and per file:
- Concrete `Key`.
- Data-driven collections (no JSX `CollectionBuilder`).
- Hook-owned selection and expansion: no `selectedKeys`/`expandedKeys` controlled props, only `default_*` plus manager methods.
- Item element registry in place of `getItemElement`.
- Composition in place of class inheritance for the table delegate.
- Tracked queries vs untracked mutations.

As implemented: `hooks/mod.rs` records only hook-owned state (C4) globally, and C4 now also allows a `ValueBinding`
to app state in place of `default_*` (`SelectionOptions::selection`, `UseTreeStateInput::expanded_keys`; the
mutation path stays the hook's). The others are recorded per file: `key.rs` (doc comment), `use_collection.rs` and
`item_elements.rs` (`// No upstream:` headers), the deviation blocks of `list_state.rs`, `selection_manager.rs`,
`use_selectable_item.rs` and `table_keyboard_delegate.rs`.

---

## 5. Migration (done)

Migrated family by family behind browser tests asserting DOM/ARIA only (roles, `aria-selected`,
`document.activeElement`, `aria-activedescendant`), so fixtures changed during rewrites and tests didn't: safety net,
`hooks/collections`, selection core, delegates, listbox, select, menu, combo box, grid list + tag group, tree, grid +
table, tabs, cleanup (generic `SelectionKey`/`Keyed`/`use_selection_state` deleted, `Orientation` in `utils`).
Collection item ids are the container's SSR-stable `use_id` plus `Key::id_fragment` (e.g. `{listbox}-option-{key}`).

---

## 6. Open questions

- The virtualization API (`is_virtualized`, `Loader` nodes), deferred by the design, is built: `hooks/virtualizer/`
  (react-stately's virtualizer and `ListLayout`, which lays out `Loader` nodes), the `Virtualizer`/`VirtualList`
  atoms (`atoms/virtualizer.rs`; a virtualized `ListBox` renders only the visible items through `ListBoxItems`), and
  `is_virtualized` on the listbox input (`aria-setsize`/`aria-posinset` from the collection).
  `LayoutDelegate::key_range` was not needed (range selection follows collection order). Virtualized `GridList`,
  `Table` and select, combo box and menu popovers are open in `PLAN.md` ("Virtualizer").
- Whether atoms should also accept a pre-built `CollectionMemo` (power users) in addition to `items` + key/text closures. Recommendation: yes, via an enum prop.
  Status: atoms take only a `CollectionMemo` so far (`ListBox`/`Menu`: an `Option`, required outside a parent);
  typed item collections are open in `PLAN.md`.
