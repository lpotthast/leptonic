# Design note: Collections and collection state in leptonic

Status: proposal. Upstream reference: react-spectrum @ `99e6102368` (current HEAD; existing leptonic headers point at `6f664fe911`).
Scope: listbox, select, combobox, menu, grid list, grid, table, tree, tag group, tabs (and DnD, which uses `KeyboardDelegate`).

---

## 1. Decision summary

1. **One non-generic `Key` type.** Hooks use a concrete `Key` (cheap-clone newtype over `i64 | Arc<str>`) in place of today's `K: SelectionKey` generic. A thin typed layer for atoms and components maps the user's types to and from `Key`.
2. **Collections are built from data the user owns. They are never derived from the DOM.** A `Collection` is an immutable tree of `Node`s (items, sections, rows, cells, columns), produced by a `Memo` over a builder closure that reads the user's signals. It exists during SSR.
3. **State hooks are separate from ARIA hooks** (react-stately vs react-aria):
   - `use_list_state`, `use_single_select_list_state`, `use_tree_state`, `use_grid_state` and `use_table_state` own the selection signals (Hook-Owned State rule) and return a `Copy` state struct.
   - ARIA hooks (`use_listbox`, `use_select`, `use_grid_list`, ...) take that state struct as input.
   - This is how one `select` state drives both the trigger and the listbox.
4. **One `SelectionManager`.** It is a `Copy` struct of signals with real methods (no bag of `Callback`s). It is the only place that holds the selection, the focused key, `is_focused`, the disabled logic and the link lookup.
5. **`KeyboardDelegate` is a non-generic, object-safe trait** used as `Arc<dyn KeyboardDelegate>`. There are list, grid, table and tabs delegates. Page Up/Down use a `LayoutDelegate` that measures elements through a key→element registry filled by `CapturedElement`. The `data-key` querySelector lookup goes away.
6. **Migration is incremental, family by family.** It starts with listbox, then select (this fixes a real bug, see section 3.2). Each step first adds DOM/ARIA-level browser tests that keep passing across the rewrite.

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

## 3. Current leptonic approach

### 3.1 Per family

| Family | Keys / items | Selection state lives in | Navigation | Disabled |
|---|---|---|---|---|
| listbox / option | `items: Signal<Vec<K>>` (`use_listbox.rs`) | its own `use_selection_state` inside `use_selectable_list` | `ListKeyboardDelegate` over the flat `Vec<K>` | `disabled_keys` + a separate per-option `is_disabled` |
| select | `items: Signal<Vec<K>>` + `get_text_value` | its own `use_selection_state` (`use_select.rs:701`) **and** a second one inside the listbox | trigger: hand-rolled `navigate_selection` (`use_select.rs:~840`) | `disabled_keys` |
| combobox | `items: Signal<Vec<K>>`, `filter: Callback<(String, Vec<K>), Vec<K>>` | own `Option<K>` signal (`use_combobox.rs:540`); no selection manager | hand-rolled `find_adjacent_enabled` (`:403`) | `disabled_keys` |
| menu | `all_keys` + `get_key_label` | `use_selectable_list`; mode `None` is faked as Single/Replace with selection→`on_action` (`use_menu.rs:209-232`) | list delegate; separate `use_type_select` (`:255`) | `disabled_keys` |
| grid list | `all_keys: Signal<Vec<K>>` | `use_selection_state` + **its own** `focused_key` (`use_grid_list.rs:368`) | re-implemented `get_next_key` (`:227-301`) | `disabled_keys` + item `is_disabled` |
| grid | `Signal<GridCollection<K>>` (rows → cell keys) | `use_selection_state` + own `focused_key` (`use_grid.rs:291`) | `GridKeyboardDelegate` (no RTL, no page up/down) | `disabled_keys` |
| tree / table / tag / tabs | `String` keys, `Signal<Vec<String>>` / `Signal<Option<String>>` | controlled-only from the caller | caller supplies `on_focus_next/previous/...` callbacks (`use_tree_item.rs:65-84`, `use_tab_list.rs:42-51`, `use_tag.rs:48-51`, `use_table_row.rs:242-248`) | per-item prop only |

### 3.2 Concrete problems

1. **The controlled-state pattern breaks the Hook-Owned State rule and is buggy.**
   - `selected_keys: Option<Signal<Selection<K>>>` (`use_selection_state.rs:267`) is repeated in every input struct.
   - In controlled mode, `update_selection` writes only the ignored internal signal (`:476-484`).
2. **Select ↔ listbox composition is two states glued together.**
   - `use_select` passes its signal as "controlled" to the listbox (`use_select.rs:1210-1215`). The listbox's `on_selection_change` → `on_internal_selection_change` (`:692-700`) only calls the user callback and closes the menu.
   - **It never writes `use_select`'s own state.** An uncontrolled `Select` very likely does not update when an option is clicked. This needs a browser test to confirm.
   - The focused key is mirrored back through an `Effect` (`atoms/listbox.rs:236-241`). `UseSelectMenuConfig` copies ~20 fields.
3. **Focus state is duplicated.** `use_selection_state` owns `focused_key`, but grid and grid list create a second one and never use the first.
4. **`Selection::All` semantics differ from upstream.**
   - `toggle`/`deselect`/`select` on `All` are no-ops (`use_selection_state.rs:518, 569, 592`).
   - `is_selected` ignores disabled keys under `All`.
   - Grid Shift+Arrow calls `select` where it should call `extend_selection` (`use_grid.rs:329-343`).
5. **Type-ahead is effectively missing or wrong.**
   - `use_listbox` drops `get_text_value` (`use_listbox.rs:271`, `get_key_label: None` at `:310`), so listboxes have no type-ahead.
   - `use_type_select` (`:225-256`) does not skip disabled keys.
   - The menu wires a second type-select by hand.
6. **No collection structure.**
   - Flat `Vec<K>` everywhere. Sections (`use_listbox_section.rs`, `use_menu_section.rs`) are purely presentational.
   - Trees have no expansion model. `use_tree` uses `role="tree"` (`use_tree.rs:254`) where upstream uses `treegrid` via `useGridList`.
   - `extend_selection` slices `Vec` indices (`use_selectable_collection.rs:284-338`).
7. **Two sources of truth for disabled state.** Item hooks take their own `is_disabled` (`use_option.rs`, `use_grid_list_item.rs`), but the delegates only skip `disabled_keys`, so a prop-disabled item can still be reached with the keyboard.
8. **DOM vs data order.**
   - Navigation follows `all_keys`.
   - Scroll and focus look elements up with `[data-collection][data-key]` querySelector built from `Display` (`selection/utils.rs:416-430`, uuid `collection_id` at `use_selectable_collection.rs:253`). These two diverge silently.
9. **Missing delegate features.** No page up/down (`list_keyboard_delegate.rs:246`, `grid_keyboard_delegate.rs`), no RTL in grids, and RTL is hard-coded to `Ltr` with a TODO (`use_listbox.rs:304`, `use_menu.rs:241`).
10. **Duplication.**
    - Five `SelectionMode` enums (`use_selection_state.rs:19`, `table/use_table.rs:28`, `tree/use_tree.rs:29`, `tag/use_tag_group.rs:28`, plus tabs).
    - Several orientation enums; `Orientation` lives in `form/use_checkbox_group.rs`.
    - Four copies of "next enabled key".
    - Three copies of the grid-level keydown handler.
11. **SSR rule violations.** `web_sys::window()` in `use_grid.rs:451`, `use_grid_list.rs:493`, `use_grid_cell.rs:206`.
12. **Generic cost.**
    - ~15k lines are generic over `K` (and `D: KeyboardDelegate<K> + Copy`), so they are monomorphized per key type.
    - DnD is already forced to `dyn KeyboardDelegate<String>` (`dnd/use_droppable_collection.rs:50`).
    - `get_untracked()` clones the whole key `Vec` on every keypress.
13. **No browser tests** for any collection hook (only focus/press/button fixtures exist).

---

## 4. Proposed design

Module layout:
- `hooks/collections/`: `key.rs`, `node.rs`, `collection.rs`, `builder.rs`, `use_collection.rs`, `item_elements.rs`.
- `hooks/selection/`: `selection.rs`, `use_multiple_selection_state.rs`, `selection_manager.rs`, `keyboard_delegate.rs`, `layout_delegate.rs`, `list_keyboard_delegate.rs`, `use_selectable_collection.rs`, `use_selectable_list.rs`, `use_selectable_item.rs`, `use_type_select.rs`.
- `hooks/list/`: `use_list_state.rs`, `use_single_select_list_state.rs`.
- Per family: `use_tree_state.rs`, `use_grid_state.rs`, `use_table_state.rs`, `tabs_keyboard_delegate.rs`, `table_keyboard_delegate.rs`.

### 4.1 `Key`: concrete, not generic

```rust
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Key(Repr);
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Repr { Int(i64), Str(Arc<str>), Pair(Arc<(Key, Key)>) } // Pair: generated cell keys (row, column)

impl Key {
    pub fn as_str(&self) -> Option<&str>;
    pub fn as_i64(&self) -> Option<i64>;
    pub fn cell(row: &Key, column: &Key) -> Key;
}
// From<&str>, From<String>, From<Arc<str>>, From<i32|i64|u32|u16|u8|usize>, TryFrom<u64>, Display, Debug

/// Optional typed round-trip for atoms/components (impl for String, integers; users impl for enums).
pub trait TypedKey: Clone + Send + Sync + 'static {
    fn to_key(&self) -> Key;
    fn from_key(key: &Key) -> Option<Self>;
}
```

Why not generic `K`:
- The hook layer never needs the user's type. It only needs identity, order, text, flags and links.
- A concrete key compiles the large hook bodies once. That means smaller wasm, faster builds, object-safe delegates, and no `where K: SelectionKey` noise.
- Type safety comes back at the atom/component layer, where values are already owned (see 4.10). This mirrors react-aria (`Key = string | number`).
- Trade-off: hook-level callbacks hand out `Key`, not `MyEnum`. `TypedKey::from_key` closes that gap.
- `Int(1)` and `Str("1")` are different keys, as in JS sets. DOM ids derived from keys must normalize whitespace, as `getItemId` does.

### 4.2 `Node` and `Collection`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind { Item, Section, Header, Separator, Loader, Row, HeaderRow, Cell, RowHeader, Column, Body }

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub key: Key, pub kind: NodeKind,
    pub text_value: Arc<str>, pub aria_label: Option<Arc<str>>,
    pub level: u16, pub index: usize,              // index among same-kind siblings (upstream `index`)
    pub parent_key: Option<Key>,
    pub has_child_items: bool,
    pub is_disabled: bool, pub disabled_behavior: Option<DisabledBehavior>, // per-item override
    pub link: Option<LinkTarget>,                  // links-as-items
    pub col_index: Option<usize>, pub col_span: Option<usize>,
}
pub struct LinkTarget { pub href: Arc<str>, pub target: Option<Arc<str>>, pub rel: Option<Arc<str>>, pub download: Option<Arc<str>> }

/// Immutable; arena of nodes in document order + `HashMap<Key, u32>`; sibling/child links are u32 indices.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Collection { /* private */ }
impl Collection {
    pub fn build(f: impl FnOnce(&mut CollectionBuilder)) -> Self;
    pub fn size(&self) -> usize;                                   // items + sections, as ListCollection
    pub fn get(&self, key: &Key) -> Option<&Node>;
    pub fn first_key(&self) -> Option<&Key>;  pub fn last_key(&self) -> Option<&Key>;
    pub fn key_after(&self, key: &Key) -> Option<&Key>;  pub fn key_before(&self, key: &Key) -> Option<&Key>; // BaseCollection semantics
    pub fn children(&self, parent: Option<&Key>) -> impl Iterator<Item = &Node>;
    pub fn items(&self) -> impl Iterator<Item = &Node>;            // all Item nodes, document order
    pub fn compare_order(&self, a: &Key, b: &Key) -> Option<Ordering>;
    pub fn filter(&self, keep: impl Fn(&str, &Node) -> bool) -> Collection;          // prunes empty sections
    pub fn flatten_expanded(&self, expanded: &HashSet<Key>) -> Collection;            // TreeCollection
}
```

Builder (sections, tree children, grids and tables share one API):

```rust
impl CollectionBuilder {
    pub fn item(&mut self, key: impl Into<Key>, text_value: impl Into<Arc<str>>) -> ItemBuilder<'_>;
    pub fn section(&mut self, key: impl Into<Key>, heading: Option<&str>, f: impl FnOnce(&mut CollectionBuilder));
    pub fn separator(&mut self, key: impl Into<Key>);
    pub fn row(&mut self, key: impl Into<Key>, f: impl FnOnce(&mut RowBuilder)) -> ItemBuilder<'_>;
}
impl ItemBuilder<'_> {
    pub fn disabled(self, v: bool) -> Self;  pub fn disabled_behavior(self, b: DisabledBehavior) -> Self;
    pub fn aria_label(self, l: impl Into<Arc<str>>) -> Self;  pub fn link(self, l: LinkTarget) -> Self;
    pub fn children(self, f: impl FnOnce(&mut CollectionBuilder)) -> Self;   // tree items
}
// RowBuilder::cell(text) auto-keys cells with Key::cell(row, column). TableCollection adds
// columns/header rows: `TableCollection::build(|t| { t.columns(|c| ..); t.body(|b| ..) })`.
```

Duplicate keys log a `leptos::logging::warn!` and the first one wins.

### 4.3 Reactive source: data-driven, SSR-safe

```rust
pub type CollectionMemo = Memo<Arc<Collection>>;
/// Tracks every signal read inside `build`; rebuilds O(n) on change, notifies only if structurally changed.
pub fn use_collection(build: impl Fn(&mut CollectionBuilder) + Send + Sync + 'static) -> CollectionMemo;
pub fn use_list_collection<T: Send + Sync + 'static>(
    items: Signal<Vec<T>>, key: impl Fn(&T) -> Key + ..., text: impl Fn(&T) -> String + ...,
) -> CollectionMemo;
```

Implement it with `Memo::new_with_compare` plus `PartialEq` on `Collection`, so a rebuild that produces the same structure does not wake dependents.

Example:

```rust
let fruits: Signal<Vec<Fruit>> = /* ... */;
let collection = use_collection(move |b| {
    for group in fruits_by_group.read().iter() {
        b.section(group.id, Some(&group.name), |s| {
            for f in &group.items { s.item(f.id, &*f.name).disabled(f.sold_out); }
        });
    }
});
```

**Rejected: building from rendered children (the RAC approach, or registration from item components).**
- Leptos renders once; there is no two-pass or fake-DOM render.
- During SSR the parent's attributes are emitted before the children run. Anything that depends on the collection would render empty on the server and then mismatch on hydration: select display value, hidden `<option>`s, `aria-activedescendant`, empty state, initial focus key, `aria-setsize`.
- `<For>` reorders would make registration order differ from DOM order.

With data-driven collections, all of that is computed on the server from the same data (compare RAC's `*.ssr.test.js`).

**Rendering contract.** The user (or an atom) renders from the same data, in collection order. Item hooks take only the `Key` and read everything else from the node. In debug builds, an item hook whose key is not in the collection logs a warning.

**Item element registry (the one DOM-derived piece).** `ItemElements` is a `StoredValue<HashMap<Key, CapturedElement>>` plus a `Trigger`.
- Item hooks register their `CapturedElement` and unregister in `on_cleanup`.
- It is used only in effects and event handlers: focus, scroll-into-view, `DomLayoutDelegate`, `open_link`.
- It is empty during SSR, which is fine.
- It replaces `data-collection` uuids and `[data-key]` selectors. `data-key` stays only as a styling/test hook.

### 4.4 Selection: `use_multiple_selection_state` + `SelectionManager`

```rust
pub struct SelectionOptions {                       // part of every *StateInput
    pub selection_mode: Signal<SelectionMode>,
    pub selection_behavior: SelectionBehavior,       // initial; runtime switch via manager
    pub default_selected_keys: Selection,            // hook-owned; no controlled variant
    pub on_selection_change: Option<Callback<Selection>>,
    pub disallow_empty_selection: Signal<bool>,
    pub disabled_keys: Signal<HashSet<Key>>,
    pub disabled_behavior: DisabledBehavior,
    pub allow_duplicate_selection_events: bool,
}

#[derive(Clone, Copy)]
pub struct SelectionManager { /* collection, full_collection: Option<CollectionMemo>, signals, layout: Option<StoredValue<Arc<dyn LayoutDelegate>>> */ }
impl SelectionManager {
    // queries: tracked (use inside Signal::derive / views)
    pub fn selection_mode(&self) -> SelectionMode;   pub fn selection_behavior(&self) -> SelectionBehavior;
    pub fn raw_selection(&self) -> Selection;        pub fn selected_keys(&self) -> HashSet<Key>;
    pub fn is_selected(&self, key: &Key) -> bool;    pub fn is_empty(&self) -> bool;  pub fn is_select_all(&self) -> bool;
    pub fn first_selected_key(&self) -> Option<Key>; pub fn last_selected_key(&self) -> Option<Key>;
    pub fn focused_key(&self) -> Option<Key>;        pub fn is_focused(&self) -> bool;
    pub fn child_focus_strategy(&self) -> Option<FocusStrategy>;
    pub fn is_disabled(&self, key: &Key) -> bool;    pub fn can_select_item(&self, key: &Key) -> bool;
    pub fn is_link(&self, key: &Key) -> bool;
    // per-item reactive helpers (consider leptos `Selector` for focused key → O(1) notifications)
    pub fn is_selected_signal(&self, key: Key) -> Signal<bool>;
    pub fn is_focused_signal(&self, key: Key) -> Signal<bool>;
    // mutations: never track; fire on_selection_change
    pub fn set_focused(&self, v: bool);  pub fn set_focused_key(&self, key: Option<Key>, child: Option<FocusStrategy>);
    pub fn select(&self, key: &Key, pointer: Option<PointerType>);
    pub fn toggle_selection(&self, key: &Key);  pub fn replace_selection(&self, key: &Key);
    pub fn extend_selection(&self, to: &Key);   pub fn set_selected_keys(&self, keys: impl IntoIterator<Item = Key>);
    pub fn select_all(&self);  pub fn clear_selection(&self);  pub fn toggle_select_all(&self);
    pub fn set_selection_behavior(&self, b: SelectionBehavior);
    pub fn with_collection(&self, view: CollectionMemo) -> Self;   // filtered views (combobox)
}
```

- The methods are a faithful port of `SelectionManager.ts`: `'all'` handling, `getKey` parent mapping for cells, `getKeyRange` over collection order (sections and headers skipped), disabled override per item.
- Convention: queries read tracked, mutations read untracked. Event handlers that need queries call `untrack(|| ...)`.
- Remove `Selection::contains(all_keys)`. `Selection`/`SelectionSet` otherwise stay, with `Key`.

### 4.5 State hooks (all return `Copy` structs)

```rust
pub struct ListState { pub collection: CollectionMemo, pub selection: SelectionManager, pub item_elements: ItemElements }
pub fn use_list_state(i: UseListStateInput /* collection, selection: SelectionOptions, layout_delegate? */) -> ListState;
// installs the useFocusedKeyReset effect (walk forward in the old collection, then backward)

pub struct SingleSelectListState { pub list: ListState, pub selected_key: Signal<Option<Key>> }
impl SingleSelectListState { pub fn set_selected_key(&self, key: Option<Key>); pub fn selected_node(&self) -> Option<Node>; }

pub struct TreeState { pub full: CollectionMemo, pub list: ListState /* visible = flatten_expanded */, pub expanded_keys: Signal<HashSet<Key>> }
impl TreeState { pub fn toggle_key(&self, k: &Key); pub fn set_expanded_keys(&self, ks: HashSet<Key>); }

pub struct GridState { pub list: ListState, pub focus_mode: GridFocusMode }  // row-aware focus reset
pub struct TableState { pub grid: GridState, pub table: Memo<Arc<TableCollection>>, pub sort: Signal<Option<SortDescriptor>> }
pub struct SelectState { pub list: SingleSelectListState /* or ListState for multiple */, pub is_open: Signal<bool>, pub focus_strategy: Signal<Option<FocusStrategy>> /* + open/close/toggle */ }
pub struct ComboBoxState { pub list: ListState /* filtered view via with_collection */, pub input_value: Signal<String>, /* ... */ }
```

### 4.6 Delegates

```rust
#[derive(Default, Clone, Copy)] pub struct NavOptions { pub include_disabled: bool } // needed by DnD
pub trait KeyboardDelegate: Send + Sync {
    fn key_below(&self, k: &Key, o: NavOptions) -> Option<Key> { None }
    fn key_above(&self, k: &Key, o: NavOptions) -> Option<Key> { None }
    fn key_left_of(&self, k: &Key, o: NavOptions) -> Option<Key> { None }
    fn key_right_of(&self, k: &Key, o: NavOptions) -> Option<Key> { None }
    fn supports_horizontal(&self) -> bool { true }   // replaces upstream deleting getKeyLeftOf on vertical stacks
    fn key_page_above(&self, k: &Key) -> Option<Key> { None }
    fn key_page_below(&self, k: &Key) -> Option<Key> { None }
    fn first_key(&self, from: Option<&Key>, global: bool) -> Option<Key>;
    fn last_key(&self, from: Option<&Key>, global: bool) -> Option<Key>;
    fn key_for_search(&self, search: &str, from: Option<&Key>) -> Option<Key> { None }
}
pub trait LayoutDelegate: Send + Sync {
    fn item_rect(&self, k: &Key) -> Option<Rect>;  fn visible_rect(&self) -> Rect;  fn content_size(&self) -> Size;
    fn key_range(&self, from: &Key, to: &Key) -> Option<Vec<Key>> { None }
}
pub struct DomLayoutDelegate { container: CapturedElement, items: ItemElements } // port of DOMLayoutDelegate; None on SSR
pub struct ListKeyboardDelegate { collection: CollectionMemo, selection: SelectionManager, layout: ListLayout /* Stack|Grid */,
    orientation: Orientation, direction: Signal<WritingDirection>, layout_delegate: Arc<dyn LayoutDelegate>, collator: Filter }
pub struct GridKeyboardDelegate { /* collection, selection, focus_mode, direction, layout_delegate, collator */ }
pub struct TableKeyboardDelegate { grid: GridKeyboardDelegate, table: Memo<Arc<TableCollection>> } // composition, not inheritance
pub struct TabsKeyboardDelegate { /* wraps, flips left/right in RTL */ }
```

- Delegates read through `collection.with_untracked(|c| ...)`. There are no `Vec` clones.
- Disabled checks go through `SelectionManager::is_disabled`, so there is one source of truth.
- `key_for_search` uses the ICU4X `Filter` (`utils/filter.rs`). `use_type_select` becomes a thin port that calls `delegate.key_for_search`, which also skips disabled items.
- Direction comes from `use_locale().direction()`, replacing the `Ltr` TODOs.
- Hooks accept `keyboard_delegate: Option<Arc<dyn KeyboardDelegate>>` as an override, as upstream does.

### 4.7 ARIA layer

- `use_selectable_collection(UseSelectableCollectionInput { manager, keyboard_delegate: Arc<dyn KeyboardDelegate>, collection_element: CapturedElement, item_elements, auto_focus, should_focus_wrap, select_on_focus, disallow_empty_selection, disallow_select_all, disallow_type_ahead, escape_key_behavior, should_use_virtual_focus, allows_tab_navigation, link_behavior, is_virtualized })`.
  - It is no longer generic, and no longer creates selection state.
  - It gains virtual focus (`aria-activedescendant`) for combobox.
- Each container hook returns an **item context**: the Rust equivalent of upstream's `listMap`/`listData` `WeakMap`. Example: `ListBoxItemCtx { state: ListState, id: Arc<str>, should_select_on_press_up, should_focus_on_hover, should_use_virtual_focus, link_behavior, on_action }`.
  - Item hooks take `{ ctx, key }`. Selected, focused, disabled, text value, index, level, set size and link are all derived from state.
  - This removes `row_index`, `is_disabled`, `text_value`, `focused_key`, `on_focus`, … from item inputs.
- `use_selectable_item(UseSelectableItemInput { manager, key, element: CapturedElement, item_elements, should_select_on_press_up, allows_different_press_origin, should_use_virtual_focus, on_action, link_behavior, ... })`.
  - This is the full port of the primary/secondary action model and `linkBehavior`. Opening a link goes through `utils/open_link.rs` on the registered element.
  - It registers its element in `item_elements`.

### 4.8 Cross-cutting behavior

- **Disabled state:** `disabled_keys` (state input) and the node flag `.disabled()` are combined only in `SelectionManager::is_disabled/can_select_item`, with upstream semantics. `DisabledBehavior::Selection` keeps items focusable and actionable.
- **Sections:** these are `Section` nodes. Navigation skips non-item nodes. `use_listbox_section`/`use_menu_section` take `{ ctx, key }` and read the heading and `aria-label` from the node.
- **Focus reset:** handled by the state hooks (list: walk forward, then back; grid: row-aware; tree: clear). It also covers combobox filtering.
- **Tree:** `TreeState.list.collection` is the flattened visible collection, so the list delegate and selection work unchanged. `use_tree` is `use_grid_list` with `role=treegrid`. `use_tree_item` adds `aria-expanded/level/posinset/setsize` from nodes and ArrowRight/ArrowLeft expand, collapse and parent navigation, following upstream `useGridListItem` tree mode.
- **Links as items:** `Node::link` feeds `manager.is_link` and the item renders `<a href>`. The tag group uses `link_behavior: Override`, as upstream.

### 4.9 Consumer sketches

```rust
// listbox + option
let state = use_list_state(UseListStateInput { collection, selection: SelectionOptions { selection_mode: SelectionMode::Multiple.into(), ..Default::default() }, ..Default::default() });
let lb = use_listbox(UseListBoxInput { state, aria_label: Some("Fruits".into()), ..Default::default() });
view! { <ul {..lb.props.into_attrs()}>
  <For each=move || fruits.get() key=|f| f.id children=move |f| {
      let opt = use_option(UseOptionInput { ctx: lb.item_ctx, key: f.id.into() });
      view! { <li {..opt.props.into_attrs()}>{f.name}</li> } } />
</ul> }

// select: ONE state shared by trigger and listbox (removes UseSelectMenuConfig + the sync Effect)
let select = use_select(UseSelectInput { collection, default_selected_key: None, on_selection_change: Some(cb), ..Default::default() });
let lb = use_listbox(UseListBoxInput { aria_label: None, ..select.menu_props });   // menu_props: UseListBoxInput { state: select.state.list.list, auto_focus, should_select_on_press_up: true, ... }

// combobox: same list state, filtered view; input gets use_selectable_collection with virtual focus
let state = use_combobox_state(UseComboBoxStateInput { collection, default_input_value: String::new(), ..Default::default() });
let cb = use_combobox(UseComboBoxInput { state, ..Default::default() });          // input props: aria-activedescendant from focused key
let lb = use_listbox(UseListBoxInput { should_use_virtual_focus: true, ..cb.listbox_props });

// grid list
let gl = use_grid_list(UseGridListInput { state: use_list_state(..), on_action: Some(open), ..Default::default() });
let item = use_grid_list_item(UseGridListItemInput { ctx: gl.item_ctx, key });

// tree
let tree = use_tree_state(UseTreeStateInput { collection /* built with .children() */, default_expanded_keys, ..Default::default() });
let t = use_tree(UseTreeInput { state: tree, ..Default::default() });
let row = use_tree_item(UseTreeItemInput { ctx: t.item_ctx, key });
```

### 4.10 Typed layer for atoms and components

- Atoms and components stay ergonomic and typed: for example `ListBox<T> items: Signal<Vec<T>>, key: fn(&T) -> K, text_value: ...` and `Select<O>`.
- They build the `CollectionMemo` from `items` and keep a `Memo<HashMap<Key, T>>`, so they can emit `on_change: Callback<O>` / `Vec<T>`.
- The generic surface is a few dozen lines per atom; the hook bodies are compiled once.
- `Keyed` is replaced by `TypedKey` (for keys), or by key closures.
- Inside `Select`, `ListBox` reads `SelectCtx { state }` in place of `menu_config`.

### 4.11 Deviations to record

Record these in `hooks/mod.rs` (global) and per file:
- Concrete `Key`.
- Data-driven collections (no JSX `CollectionBuilder`).
- Hook-owned selection and expansion: no `selectedKeys`/`expandedKeys` controlled props, only `default_*` plus manager methods.
- Item element registry in place of `getItemElement`.
- Composition in place of class inheritance for the table delegate.
- Tracked queries vs untracked mutations.

---

## 5. Migration plan

Every step must pass `cargo check -p leptonic --features full`, `just clippy`, `just test` and `just browser-test`.

**Browser test pattern for every step:**
- Fixture `testing/test-app/src/pages/hooks/<name>.rs`, registered in `FIXTURES`.
- Page object `leptonic/tests/pages/<name>.rs`.
- Test `leptonic/tests/ui_tests/test_<name>.rs`, registered in `ui_tests::all()`.

Write the assertions against DOM/ARIA (roles, `aria-selected`, `document.activeElement`, `aria-activedescendant`), never against hook APIs. Only the fixture code changes during a rewrite; the tests must keep passing unchanged. Use RAC `ListBox.test.js`, `GridList.test.js`, `Select.test.js`, … as the behavioral spec, and RAC `*.ssr.test.js` for SSR expectations.

0. **Safety net (no library changes).**
   - Add fixtures and tests for the current listbox, select, grid list and menu: arrow navigation that skips disabled items, Home/End, wrap, Tab single stop with focus restore on re-entry, Shift+Arrow range, Ctrl+A, Escape, select open → click → trigger text, closed-trigger ArrowDown.
   - Mark known-broken cases (uncontrolled select update, listbox type-ahead, type-ahead onto disabled items) as expected failures in a separate test, or fix them later in step 4. Do not hide them.
1. **`hooks/collections`.** `Key`, `Node`, `Collection`, builder, `use_collection`, `filter`, `flatten_expanded`, `ItemElements`. Pure code with `assertr` unit tests: traversal across sections, filter pruning, tree flattening, duplicate keys, `PartialEq` memo stability. No consumers yet.
2. **Selection core.** `use_multiple_selection_state` + `SelectionManager` + `ListState` / `use_list_state` / `use_single_select_list_state`, next to the old generic code. Port the `SelectionManager.ts` semantics with unit tests under an `Owner`: `All` toggling, `extend_selection` across sections, `disabled_behavior` override, duplicate events, focus reset.
3. **Delegates.** New `KeyboardDelegate`/`LayoutDelegate`, `DomLayoutDelegate`, `ListKeyboardDelegate` (stack/grid, RTL, page up/down) and delegate-based `use_type_select`. Unit tests use a fake `LayoutDelegate`. Move DnD (`dnd/use_droppable_collection.rs`, `drop_target_keyboard_navigation.rs`) to the non-generic trait in the same step, since it already uses `dyn`.
4. **Listbox family.** New `use_selectable_collection/list/item`; `use_listbox`, `use_option` and `use_listbox_section` on `ListState`; `ListBox` atom. Old generic selection files stay only for the not-yet-migrated families. New tests: sections (arrows cross sections, headings skipped), type-ahead (multi-character, space inside the search, skips disabled), PageUp/PageDown in a scrollable listbox, horizontal RTL, `disabled_behavior=Selection`, link items.
5. **Select.** `use_select_state`; `use_select` returns `menu_props`. Delete `UseSelectMenuConfig` and the focused-key `Effect`. `use_hidden_select` reads the collection. Update the `Select` atom and component. Tests:
   - Uncontrolled click selection updates the trigger text and the hidden select value.
   - Type-to-select on the closed trigger.
   - SSR: a raw HTTP GET of the fixture (before hydration) contains the default selected text and the hidden `<option>`s.
6. **Menu.** Map `SelectionMode::None` to real `None` plus `on_action`; menu items go through `use_selectable_item` (`should_select_on_press_up`, `allows_different_press_origin`). Tests: action vs radio vs checkbox roles and `aria-checked`, type-ahead, sections, link items.
7. **Combobox.** `use_combobox_state` (filtered `with_collection`), virtual focus in `use_selectable_collection`. Tests: DOM focus stays on the input while `aria-activedescendant` moves; filtering removes the focused item and focus resets to the next one; Enter commits; Escape reverts.
8. **Grid list + tag group.** `use_grid_list` on `use_selectable_list` (drop `get_next_key`); `use_tag_group` on grid list (horizontal list delegate, `link_behavior: Override`, keyboard navigation behavior `tab`). Tests: row navigation, ArrowLeft/Right into cell children, remove-tag focus moves to a neighbor.
9. **Tree.** `use_tree_state`; `use_tree` on grid list with `treegrid`. Tests: expand/collapse keys, navigation over visible nodes only, `aria-level/posinset/setsize`, collapse while a child is focused.
10. **Grid + table.** `GridState`, `GridKeyboardDelegate` v2 (RTL, page up/down), `TableCollection` + `TableKeyboardDelegate`. Delete `GridCollection`. Fix `web_sys::window()` uses. Tests: row vs cell focus mode, RTL left/right, PageDown, column header navigation, select-all checkbox, Shift+Arrow range.
11. **Tabs.** `TabsKeyboardDelegate`, tab-list state on `SingleSelectListState`. Tests: wrap, RTL, disabled skip, automatic vs manual activation.
12. **Cleanup.**
    - Delete `SelectionKey`, `Keyed`, generic `use_selection_state`, the per-family `*SelectionMode` enums and duplicate orientation enums; move `Orientation` to `utils`.
    - Update book-ssr pages (CLAUDE.md requires it) and `// Upstream:` headers (`--mark-synced` at the new sha).

Adjacent issue, out of scope but blocking SSR fidelity: about 50 `Uuid::new_v4()` ids are generated independently on the server and the client. Collection item ids (`aria-activedescendant`, `aria-controls`) need an SSR-stable `use_id` first. Plan that as step 0.5.

---

## 6. Open questions

- Whether to expose `Selection` as tracked-by-key (`Selector`-like) to cut per-item notifications. Measure on a 1k-item listbox first.
- The virtualization API (`is_virtualized`, `Loader` nodes) is deferred, but `NodeKind::Loader` and `LayoutDelegate::key_range` leave room for it.
- Whether atoms should also accept a pre-built `CollectionMemo` (power users) in addition to `items` + key/text closures. Recommendation: yes, via an enum prop.

### Critical Files for Implementation
- leptonic/src/hooks/selection/use_selection_state.rs
- leptonic/src/hooks/selection/use_selectable_collection.rs
- leptonic/src/hooks/select/use_select.rs
- leptonic/src/atoms/listbox.rs
- leptonic/src/hooks/grid/use_grid_list.rs
- (upstream spec) ../react-spectrum/packages/react-stately/src/selection/SelectionManager.ts, ../react-spectrum/packages/react-aria/src/selection/ListKeyboardDelegate.ts, ../react-spectrum/packages/react-aria/src/collections/BaseCollection.ts
