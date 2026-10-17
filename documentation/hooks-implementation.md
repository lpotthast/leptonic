# Hook Implementation Patterns

How leptonic's hooks are shaped: types, props, events, state, ids and elements. Rules for any code running in Leptos
and the browser: `leptos-and-dom.md`; porting (upstream headers, deviation blocks): `porting.md`; tests:
`testing.md`; atoms: `atoms-implementation.md`; the API conventions C1–C16: `conventions.md`.

## Shape of a Hook

A hook is a function that makes an element *behave like* something (a button, an option, a slider thumb): it returns
the element's props as a `Part` the caller spreads onto the element it renders.

```rust
let UseButtonReturn { button, .. } = use_button(UseButtonInput {
    element_type: ButtonElementType::Other,
    ..UseButtonInput::default()
});
// `use_button` also sets inline styles: its part gives attributes and styles ("Props and Styles").
let (attrs, styles) = button.attrs();
view! { <div {..attrs} style=styles>"Press me"</div> }
```

- **Naming:** `use_<capability>` (`use_press`, `use_move`). Every public hook starts with `crate::metrics::subject!();`,
  as every atom does: with the `metrics` feature (only the test-app enables it) it reports that the hook ran, named
  after its function, so the performance tests know what each measurement ran; without the feature it is nothing.
- **Modules:** one hook per file, also for closely related hooks (`use_grid_list` and `use_grid_list_item`). A
  family directory's `mod.rs` re-exports its public items (the hook files are `pub(crate) mod`); the family module is
  the hook's only public path (`leptonic::hooks::button::use_button`).
- **One input, one return:** `use_x(UseXInput) -> UseXReturn`, plus `UseXProps` and `UseXAttrs`. The hook
  destructures its input at the very top of its body (`let UseFooInput { is_disabled, on_change, value } = input;`),
  so every input is visible at a glance.
- **Shared state** between hooks of a concept (`use_slider_*`) is a `use_<concept>_state` hook returning a `Copy`
  state struct (C3) passed to the others, not fields repeated on each input.

## `*Input`, `*Return`, `*Props` and `*Attrs`

- `*Input` derives `Debug` and `Clone`. No constructors (C7): callers write struct literals naming every field
  (`UseFooInput { state, on_bar: Some(cb), is_disabled: Signal::stored(false) }`); `Default` only when every field has
  a meaningful default ("everything off"), never placeholder defaults building an invalid configuration (an empty
  state, a never-attached element). Reactive values are `Signal<T>` (constants via `.into()`).
- `*Return` hands out every element's props as a `Part<P>` (`Part<PropsWithStyles<*Props>>` when the hook also sets
  inline styles), named after the element (C9: `button`, `listbox`, `option`, `label`, `input`; `target` for hooks
  adding a behavior to any element); `Signal` fields for reactive data (`is_pressed`, ...); no `Callback` fields for
  programmatic control (state mutations are methods of the state struct, C3).
- `*Props` holds attributes as `Signal`s, event handlers as `EventHandler<E>`s and special attributes like
  `ElementCaptureAttr` (`leptos-element-capture`). It implements `IntoAttrs` (`fn into_attrs(self) -> Self::Attrs`;
  `leptonic::IntoAttrs`, `attrs.rs`); `*Attrs` is the tuple of Leptos `Attr`/`OnEvent` types spreadable with
  `{..props.into_attrs()}`. Returning raw `*Props` instead of attributes keeps the signals and handlers accessible,
  e.g. for chaining handlers; there is no generic `mergeProps`.
- `*Props` and `*Return` derive `Debug` but **not** `Clone` (the user; see "Props are Single-Use"); props implement
  `Remount` (public: a copy for another mount of their `Part`, written field-wise). Exception: a `*Return` of signals only
  (`UseFocusVisibleReturn`) may be `Copy`. Structs passing a hook's handlers to another hook
  (a submenu trigger's keyboard handling) carry `EventHandler`s, not the single-use `*Props`.

```rust
pub type UseFooAttrs = (Attr<attr::Tabindex, Signal<i32>>, ElementCaptureAttr, OnEvent<ev::keydown>);

#[derive(Debug)] // `IntoAttrs` requires `Debug`.
pub struct UseFooProps {
    pub tabindex: Signal<i32>,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseFooProps {
    type Attrs = UseFooAttrs;
    fn into_attrs(self) -> UseFooAttrs {
        (Attr(attr::Tabindex, self.tabindex), self.element_capture, self.on_keydown.into_on(ev::keydown))
    }
}
```

### Props and Styles (`PropsWithStyles`)

Hooks that set inline styles (positions, `touch-action`, `user-select`, ...; `use_press`, `use_button`, `use_link`,
`use_slider_thumb`, collection items) hand out `Part<PropsWithStyles<*Props>>`. `PropsWithStyles` has no `into_attrs`
and isn't an attribute, because a spread `style` would be replaced by the caller's `style=`, or replace it ("One
writer of `style`", `leptos-and-dom.md`). Such a part's `attrs()` gives the attributes and the styles together; merge
the styles with the caller's:

```rust
let (attrs, hook_styles) = button.attrs();
view! { <button {..attrs} style=hook_styles.merge(styles)>"Save"</button> }
```

`part.props()` gives the `PropsWithStyles` itself: `into_parts()` → `(Attrs, Styles)`, `into_inner()` → `(Props,
Styles)` to change the props first.

### Props are Single-Use (non-Clone); parts are `Part`s

Props bind a hook to exactly one element; not being `Clone` enforces it at compile time. Spread twice, they would
capture two elements into one `CapturedElement`, share handlers mutating the same state, and duplicate ARIA ids.

**Every props value a public hook returns is a `Part<P>`** (`utils/part.rs`; the user, 2026-10-17): the hook's own
element and every further one (a label, a description, a toggle's input, a disclosure panel, a popover). Each mount
calls `part.props()` (or `part.attrs()`) in its body and gets **fresh** props built from refcounted or `Copy`
ingredients, one element at a time (a second element gets `None` and a development warning); the lease ends when the
mount's owner is cleaned up. `Part::observed` also reports presence (`is_held()`, e.g. a slot's `referenced_id`);
`Presence` is such a lease without props (a select's listbox, which the trigger names only while rendered). Never
store `into_attrs()` output (or props) to clone it per render. Users adjusting props take `part.props()`, change
fields, then call `into_attrs()`; `Part::map` (public) derives a part from another one sharing its lease (an atom of
their own built on a hook's part).

```rust
let UseButtonReturn { button, .. } = use_button(input);
let (attrs, styles) = button.attrs();
view! { <button {..attrs} style=styles>"Save"</button> }
```

**Building parts.** A hook's body computes its props once and returns them as `Part::stored(props)`: each mount gets
`props.remount()` (one implementation per props type, shared by every hook offering it: smaller than a closure per
site). `Part::new(move || P { .. })` builds props from ingredients where there is no props value to copy (a slot, a
part the atom assembles), `part.map(..)` adds to another part's props (`InputContext` erases the input's attributes, a
color slider's thumb input gets the slider's input styles). `Part`, `Part::stored`, `Part::new`, `Part::observed`,
`Part::into_props` and `Remount` are public, so a user's hook is written like leptonic's.

**One way: leptonic composes its public hooks** (the user, 2026-10-17: "I want users to be able to look at how we
implement atoms and be able to copy most code for their own spin"). There is no second, crate-private body of a hook:
hooks building on hooks and atoms call the public `use_x` and use its parts as a user would.

- **A hook building its own props from another hook's for the same element** (`use_button` on `use_press`,
  `use_focusable`, `use_hover`, `use_focus_ring`) consumes the inner part in its body with `into_props()` (the props
  for good: no lease cleanup, a stored part hands over its props without a copy) and puts them into its own props:

  ```rust
  let UsePressReturn { target: press, is_pressed } = use_press(UsePressInput { .. });
  let (press, press_styles) = press.into_props().into_inner();
  let props = UseButtonProps { on_pointerdown: press.on_pointerdown, on_keydown: press.on_keydown.chain(..), .. };
  UseButtonReturn { button: Part::stored(PropsWithStyles::new(props, press_styles)), is_pressed, .. }
  ```

- **A hook handing out another hook's element unchanged** passes the part through (`use_field`'s `label` is
  `use_label`'s; `use_color_slider` returns `use_slider`'s return). To change a field of an inner part, consume and
  store again: `let mut input = field.input.into_props(); input.value = ..; field.input = Part::stored(input);`.
- **An atom rendering a hook's element** spreads it with `attrs()` in its body (`let (attrs, styles) =
  button.attrs();`, a re-render takes the lease again); to change the props first it takes them with `into_props()`.
  Parts for child components travel in contexts as they come from the hook (`LabelContext::label(field.label)`,
  `ThumbParts::Area { thumb: area.thumb, .. }`); two parts on one element are spread side by side (a color slider's
  track: `{..group.attrs()} {..track.attrs()}`).
- **No crate-private variants** (the user, 2026-10-17): what a composing hook needs from an inner one is a
  documented field of the inner hook's public input, typed Rust-natively, defaulting to the plain behavior:
  `UseFocusInput::mark` (a `FocusMark`: the focus ring's state, marked before the callbacks, `performance.md` R8),
  `UseToggleInput::role` (`ToggleRole`: `use_switch`), `UseToggleInput::validation`/`UseCheckboxInput::validation`
  (a `FormValidationState` managed elsewhere: a checkbox group item's, built with `FormValidationState::from_parts`),
  `UseProgressBarInput::role` (`ProgressRole`: `use_meter`). Each is an `API DIFFERENCES` deviation where react-aria
  patches the returned props (`useSwitch`, `useMeter`) or uses a private prop (`privateValidationStateProp`).

**A hook of your own** (users, and the book's "Parts" section): compute the props, implement `Remount` field-wise for
the props type (every field named; `Copy` signals copied, `Id`s, `EventHandler`s and the element capture cloned:
refcounts), return `Part::stored(props)`; the caller spreads `part.attrs()`.

```rust
impl Remount for UseFooProps {
    fn remount(&self) -> Self {
        Self { tabindex: self.tabindex, element_capture: self.element_capture.clone(), on_keydown: self.on_keydown.clone() }
    }
}
```

Costs (`performance.md`, "Part costs"): a part is 1 allocation (the `Arc`) when the hook runs, also an inner hook's
part consumed with `into_props()`, and 1 per mount (the lease's cleanup; copying chained handlers is a refcount bump).

## Event Handlers

| Location                                       | Convention  | Example                                      |
|------------------------------------------------|-------------|----------------------------------------------|
| `*Input` fields (user callbacks)               | `on_*`      | `on_press`, `on_change`                      |
| Event handler closures in the hook body        | `handle_*`  | `handle_click`, `handle_keydown`             |
| `*Props` fields (`EventHandler<E>`)            | `on_*`      | `on_keydown`, `on_click`                     |
| Helpers that aren't event handlers             | descriptive | `trigger_press_start`, `toggle`, `increment` |

`on_*` is the public surface; `handle_*` keeps a `handle_click` closure from shadowing the `on_click` callback it
captures.

`EventHandler<E>` is a chainable, clonable handler: `EventHandler::new(f)` (no allocation for one handler),
`.chain(other)`, `.then(f)`, and `.into_on(ev::keydown)`, which gives the `OnEvent` attribute: a re-entrant DOM
listener ("Nested Event Dispatch", `leptos-and-dom.md`), or nothing for `EventHandler::empty()`. Hooks leave handlers
they don't need empty instead of attaching no-op listeners.

## Event Propagation Control

**Rule** (the user): an event type implements the sealed `Propagation` trait (`utils/propagation_control.rs`) exactly
when its react-aria counterpart has `continuePropagation()`. Those events stop propagation by default; a handler calls
`continue_propagation()` to let the native event bubble (`is_propagation_stopped()` reads the state; there is no
`stop_propagation()`). Every other event type keeps upstream's fixed behavior and has no `Propagation`.

- **Implement it:** `PressEvent` (`use_press` and everything on it: buttons, links, toggles, items;
  `PressEvent.continuePropagation()`) and `KeyboardEventWrapper` (`use_keyboard`, type select, toggles, keyboard
  shortcuts; react-aria's `BaseEvent<KeyboardEvent>` from `createEventHandler`).
- **Never stop:** hover (`useHover`), focus and focus within (`useFocus`/`useFocusWithin`: stopping
  `focusin`/`focusout` would break nested focus-within containers and collection listeners above the element), long
  press (`LongPressEvent` omits `continuePropagation`; `useLongPress` continues its press events).
- **Always stop** the native events they handle: move (`useMove`), scroll wheel (`useScrollWheel`, except Ctrl+wheel
  zoom), drag and drop (`useDrag`/`useDrop`/`DragManager`).
- **A new event type:** look it up in `@react-types/shared/src/events.d.ts` (and the hook's own types): with
  `continuePropagation()`, implement `Propagation` through `PropagationControl`; without, port the fixed behavior.

How it works: the hook creates a `PropagationControl` (an `Arc<AtomicBool>` shared by clones), builds the event with
a clone, runs the user's handler, and afterwards calls `stop_propagation()` on the native event unless the handler
continued propagation. **Reference**: `PressEvent` in `hooks/interactions/use_press.rs`.

## Combining Hooks

### Input Composition (preferred)

When a hook *configures* an element another hook renders, it returns that hook's **input**, not DOM props: a menu
trigger configures a button, so `use_menu_trigger` returns a `UseButtonInput`; a spin button returns two. The caller
hands it on, adding its own settings with struct update syntax:

```rust
let menu_trigger = use_menu_trigger(UseMenuTriggerInput { .. });
let button = use_button(UseButtonInput { on_hover_start: Some(cb), ..menu_trigger.button });
```

This keeps exactly one press, focus and hover state machine per element (merging a menu trigger's and a button's DOM
props once attached two independent press handlers). A configuring hook that adds to a callback the caller may also
set chains them (`chain_optional_callbacks` from `use_press`; long press groups: `merge_long_press`). Inputs meant to
be composed implement `Default`.

### Independent Hooks on One Element

Independent hooks (`use_press` + `use_hover`) are each spread onto the element; Leptos attaches every spread's
listeners, in spread order:

```rust
let (press_attrs, styles) = use_press(press_input).target.attrs();
let hover = use_hover(hover_input);
view! { <button {..press_attrs} {..hover.target.attrs()} style=styles>"Hover and click me"</button> }
```

There is no merge mechanism (typed `MergeWith` structs were removed: one struct per pair of hooks, and spreading
works as well). Two hooks never set the same plain attribute on one element; where several contribute to one id list
(`aria-describedby`, `aria-labelledby`), the combining hook joins them with `IdRefs::derive([..])` (signals of `Option<Id>`;
constants are folded, a single reactive contribution passed on) or, for values, `utils::id::join_ids`, never by
hand. An element labelled by an `aria-label` and other elements
gets its labelling from `utils::labels` (react-aria's `useLabels`).

## Attributes

### ARIA Attribute Types

ARIA attributes use the typed enums of `utils/aria.rs` (`AriaRole` with every WAI-ARIA role, `AriaDisabled`,
`AriaExpanded`, `AriaChecked`/`AriaPressed` with `Mixed`, `AriaSort`, `AriaHasPopup`, ...), **never `bool`** and no
string literals: Leptos renders a `bool` as an HTML boolean attribute (present or absent), while ARIA needs
`"true"`/`"false"`, so a `bool` silently renders no attribute. No `Undefined` variants: an absent attribute is `None`.

```rust
pub aria_disabled: Signal<Option<AriaDisabled>>, // not Signal<bool>
let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));
```

Id references are `Signal<Option<String>>` (C2), lists joined with `IdRefs`. For a typed ARIA value present only while
true: `utils::aria::aria_true`.

### Custom Data Attributes

`data-*` and unlisted ARIA attributes use `leptos::attr::custom::custom_attribute` (`CustomAttr<&'static str,
Signal<Option<&'static str>>>` in the `*Attrs` tuple), `"true"` while true and absent otherwise, as `flag` renders it.
**Reference**: `hooks/focus/use_focus_ring.rs`.

### Element Ids

Hooks linking elements (`aria-labelledby`, `aria-controls`, `aria-activedescendant`, `for`, ...) create ids with
`crate::utils::id::use_id("prefix")`, never random values: `use_id` draws from Leptos' hydration counter, so server
and client produce the same ids. Call it in the hook body, unconditionally and in the same order on both sides: never
in an effect or event handler, never in a code path only one side has (after a `#[cfg(feature = "ssr")]` early
return), or every later id differs. `test_hydration_ids.rs` compares the server's ids with the hydrated DOM on every
fixture.

## Elements

Hooks needing the DOM element (focus, measuring) capture it with `CapturedElement` (`leptos-element-capture`) through
an attribute in their props, instead of making users bind a `NodeRef`. `CapturedElement::get()` is tracked: an Effect
reading it re-runs once the element is captured, which matters when it is captured after the Effects' first run
(inside `<Show>`/`<Suspense>` during client-side navigation; during hydration it is captured before Effects run).

```rust
let element = CapturedElement::new();
Effect::new(move |_| {
    let Some(el) = element.get() else { return };
    // ...
});
// in the props: `element_capture: element.attr()`
```

A hook that doesn't spread onto the element takes a `CapturedElement` from the hook that does
(`use_overlay_position` from the trigger's). `leptos_use`'s `IntoElementMaybeSignal` (a user-bound `NodeRef`) is a
last resort for a hook that spreads nothing and has no spreading companion; no hook needs it today.
**Reference**: `use_menu_item`.

## Hook-Owned State (C4)

react-aria's state hooks use `useControlledState` for controlled (`isOpen`) and uncontrolled (`defaultOpen`) state,
because React components can't share mutable state. Leptos signals are shared, but accepting a writable signal would
let callers bypass the hook's mutation path (its invariants, `on_open_change`, resetting related state). So the hook
owns the mutation path: a state hook (`use_*_state`) takes either

- `default_<x>: T` and `on_<x>_change: Option<Callback<T>>` (the hook stores the value), or
- `<x>: Option<ValueBinding<T>>`: the value lives in app state (a read `Signal` plus a setter), every change still
  goes through the hook, which calls the setter (`default_<x>` is then ignored),

and returns a `Copy` state struct (C3) of read-only `Signal`s and mutation methods (`open`, `toggle`, `set_value`,
...) that keep the invariants and call `on_<x>_change` on every change. Atoms build the binding from their `<x>` +
`set_<x>` props (`atoms-implementation.md`). Porting a hook on `useControlledState` maps `value`/`defaultValue`/
`onChange` onto this; it is a project-wide deviation (`hooks/mod.rs`). Examples: `UseToggleStateInput::value`,
`SelectionOptions::selection`; `UseTextFieldStateInput` is a small complete one.

## Animation Lifecycle

`hooks/animation/` follows elements' CSS animations and transitions with the Web Animations API
(`Element.getAnimations()` + the animations' `finished` promises; react-aria's `useAnimation`; no calls during SSR):
`use_enter_animation` reports `is_entering` and returns hide-until-ready `styles` to merge into the element's `style`;
`use_exit_animation` reports `exit_state` (`Open` → `Exiting` → `Closed`). An overlay stays mounted while exiting:

```rust
let is_mounted = Signal::derive(move || is_open.get() || exit.exit_state.get() == ExitState::Exiting);
```

## Localized Strings

Hooks take their texts (labels, descriptions, announcements) from react-aria's message bundles in its 34 locales
(`utils::intl_strings`; replaces `@internationalized/string`):

- `scripts/port-intl-strings.py` generates `utils/intl_strings/bundles/<family>.rs` from upstream's
  `intl/<locale>.json` (en-US always, the others behind the default feature `intl-strings`). Never edit the
  generated files; rerun the script after upstream changes. Its `FIXES` table repairs upstream translation errors; a
  new family is a line in `FAMILIES`.
- Each bundle is a typed struct with a method per message: `&str` for `{name}`, `usize` for a plural, `bool` for a
  `select` on `true`/`other`; two arguments of one type take a generated args struct
  (`InsertBetweenArgs { before_item_text, after_item_text }`), since positional ones would swap silently if upstream
  reordered the message.
- `let strings = use_localized_strings::<TableStrings>();` is a `Memo` following the locale; read it where the text
  is built (`strings.read().ascending_sort(&column)`). An input overriding a text (`aria_label`) wins.
- Messages are formatted at runtime by a small ICU MessageFormat formatter (`plural` with `=N` and CLDR categories,
  `#`, `select`, apostrophe quoting). A missing locale falls back to its language (`fr-CA` → `fr-FR`), then en-US. A
  test parses every message of every locale and checks its arguments against en-US.

## Form Validation

Field hooks (text, number, date fields, checkbox and radio groups, select, combo box, ...) compose three hooks:

- `use_form_validation_state`: combines `is_invalid` (OR-ed), server errors (`FormValidationContext`, the `Form`
  atom's `validation_errors` under the field's `names`), `validate` functions, the field's own `builtin_validation`
  and native validity. `ValidationBehavior::Aria` shows errors in realtime and submits anyway; `Native` (the atoms'
  default, as react-aria-components) shows them on commit and blocks submission through `setCustomValidity()`.
  Returns `FormValidationState` (C3) with `realtime_validation`, `display_validation` and
  `update`/`reset`/`commit_validation`.
- `use_form_validation`: the DOM side: sets custom validity in `Native` mode, re-reads native validity before each
  commit (`FormValidationState::register_native_validity_reader`, also for a field's own native checks), commits on
  `invalid` (submission) and `change` focusing the first invalid field (`focus` for fields whose focusable element
  isn't the validated one), resets on form reset.
- `use_form_reset`: restores the initial value when the field's `<form>` is reset.

**Reference**: `hooks/form/use_form_validation_state.rs`, `hooks/form/use_text_field.rs`.

## Reference Implementations

| Pattern                           | File                                                                                                         |
|-----------------------------------|--------------------------------------------------------------------------------------------------------------|
| API conventions (state, input)    | `leptonic/src/hooks/tabs/use_tab_list_state.rs`, `leptonic/src/hooks/table/use_table_column_resize_state.rs` |
| Hook composition, attrs           | `leptonic/src/hooks/button/use_button.rs`                                                                    |
| Props, handlers, propagation      | `leptonic/src/hooks/interactions/use_press.rs`                                                               |
| Id reference lists (`IdRefs`)     | `leptonic/src/utils/id_refs.rs`                                                                              |
| CustomAttr                        | `leptonic/src/hooks/focus/use_focus_ring.rs`                                                                 |
| Element from another hook         | `leptonic/src/hooks/overlay/use_overlay_position.rs`                                                         |
| Element capture                   | `leptonic/src/hooks/menu/use_menu_item.rs`                                                                   |
| Dynamic listeners, drag           | `leptonic/src/hooks/interactions/use_press.rs`, `leptonic/src/hooks/slider/use_slider.rs`                    |
| Animation lifecycle               | `leptonic/src/hooks/animation/use_enter_animation.rs`, `use_exit_animation.rs`                               |
| Form validation                   | `leptonic/src/hooks/form/use_form_validation_state.rs`, `use_form_validation.rs`, `use_form_reset.rs`        |
| Keyboard DnD                      | `leptonic/src/hooks/dnd/drag_manager.rs`                                                                     |
