// Upstream: react-aria/src/interactions/usePress.ts @ 99e6102368
#![cfg_attr(feature = "ssr", allow(dead_code, unused_imports))]

use std::{sync::atomic::Ordering, time::Duration};

use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{
    DragEvent, EventTarget, HtmlElement, HtmlInputElement, HtmlTextAreaElement, KeyboardEvent,
    MouseEvent, PointerEvent,
};

use crate::{
    hooks::{IntoAttrs, PropsWithStyles},
    utils::prevent_focus::prevent_focus,
    utils::{
        CapturedElement, ContainsTarget, ElementExt, EventAccessors, EventHandler, EventModifiers,
        EventTargetExt, Modifiers, Propagation,
        aria::{AriaDescribedby, AriaExpanded, AriaHasPopup},
        css::{TouchAction, TouchActionGestures, TouchActionHorizontalPan, TouchActionVerticalPan},
        event_listeners::{Listener, listen_to},
        focus::focus_element,
        is_over,
        key::{KeyboardEventKey, KeyboardKey},
        keyboard_shortcut::KeyboardShortcuts,
        node_contains,
        open_link::open_link,
        platform::device,
        pointer_type::PointerType,
        propagation_control::{PropagationControl, Sealed},
        style::TouchActionProperty,
        styles::Styles,
        use_description::use_description,
        virtual_click::{is_virtual_click, is_virtual_pointer_event},
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/interactions/usePress.ts

// ## API DIFFERENCES
// - `propagation: PressPropagation` (`Stop`, `Continue`): links and anchor buttons let every press
//   event propagate, so client-side routers see their clicks. react-aria only has
//   `continuePropagation()` per event (offered too).
// - `force_is_pressed` is react-aria's `isPressed` (named apart from the returned `is_pressed`).
// - The configuration flags are signals; the long press description is a `MaybeProp`.
// - A `PressResponder`'s flags (`is_disabled`, `prevent_focus_on_press`, ...) are OR-ed with the
//   element's: either can switch them on. react-aria merges them (the element's own value wins), so
//   only an element explicitly passing `false` under a responder's `true` behaves differently;
//   a flag can't tell "unset" from `false` here.
//
// ## DIFFERENT BEHAVIOR
//
// - React-aria's `usePress` does not handle double-click. Double-click behavior
//   lives in `useSelectableItem` (where double-click triggers an action). We add
//   `on_double_press` here as a convenience so that any pressable element can opt
//   into double-press handling without requiring a full selection model.
//
// - React-aria has a separate `useLongPress` hook that wraps `usePress`. We merged
//   long press detection directly into `usePress` to avoid double-hook overhead
//   when both press and long press are needed on the same element (e.g. menu triggers).
//
// - React-aria sets `touch-action: manipulation` via a global `<style>` element
//   injected at runtime targeting `[data-pressable]` attributes. We use an inline
//   `style="touch-action: pan-x pan-y pinch-zoom"` instead. This avoids
//   programmatic DOM manipulation, is SSR-safe (inline styles serialize naturally
//   without hydration concerns), and eliminates the need for a `data_pressable`
//   field threaded through every press-based hook.
//
// ## INTENTIONAL OMISSIONS
//
// - React-aria's `onClick` compatibility alias is intentionally not provided.
//   Leptonic uses `on_press` as the primary interaction callback. The `onClick`
//   alias exists in react-aria for third-party library compatibility which is
//   not relevant in the Rust/Leptos ecosystem.

/// The default long press threshold.
pub const DEFAULT_LONG_PRESS_THRESHOLD: Duration = Duration::from_millis(500);

/// The type of long press event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongPressEventType {
    /// The long press interaction has started.
    LongPressStart,
    /// The long press threshold time was met.
    LongPress,
    /// The long press interaction has ended.
    LongPressEnd,
}

/// Event fired during long press interactions.
#[derive(Debug, Clone)]
pub struct LongPressEvent {
    /// The type of long press event.
    pub event_type: LongPressEventType,

    /// The pointer type that triggered the long press event.
    pub pointer_type: PointerType,

    /// The target element of the long press event.
    pub target: SendWrapper<EventTarget>,

    /// States which modifier keys were held during the long press event.
    pub modifiers: Modifiers,

    /// The X coordinate of the pointer at the time of the event.
    /// `None` for keyboard events.
    pub x: Option<f64>,

    /// The Y coordinate of the pointer at the time of the event.
    /// `None` for keyboard events.
    pub y: Option<f64>,
}

#[derive(Debug)]
pub enum PressEvents {
    PressStart(PressEvent),
    PressEnd(PressEvent),
    PressUp(PressEvent),
    Press(PressEvent),
}

#[derive(Debug, Clone)]
pub struct PressEvent {
    /// The pointer type that triggered the press event.
    pub pointer_type: PointerType,

    /// The target element of the press event.
    pub target: SendWrapper<EventTarget>,

    /// States which modifier keys were held during the press event.
    pub modifiers: Modifiers,

    /// The X coordinate of the pointer relative to the target element.
    /// `None` for keyboard events.
    pub x: Option<f64>,

    /// The Y coordinate of the pointer relative to the target element.
    /// `None` for keyboard events.
    pub y: Option<f64>,

    /// The keyboard key that triggered this press event.
    /// `None` for pointer/mouse/virtual events.
    pub key: Option<KeyboardKey>,

    propagation: PropagationControl,
}

impl Sealed for PressEvent {}
impl Propagation for PressEvent {
    fn propagation_control(&self) -> &PropagationControl {
        &self.propagation
    }
}

/// Whether press events propagate to the element's ancestors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PressPropagation {
    /// Stopped, unless a press callback calls `continue_propagation()` (as react-aria).
    #[default]
    Stop,
    /// Always propagate (e.g. links, whose clicks client-side routers handle in a document
    /// listener).
    Continue,
}

#[derive(Debug, Clone)]
pub struct UsePressInput {
    /// Whether the targeted element is currently disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the press events propagate. Default: stopped, unless a callback calls
    /// `continue_propagation()`.
    pub propagation: PressPropagation,

    /// When `true`, text selection is not disabled during press interactions.
    /// By default (`false`), text selection is disabled to prevent accidental selection
    /// while pressing.
    pub allow_text_selection_on_press: Signal<bool>,

    /// When `true`, the press is cancelled entirely when the pointer exits the target.
    /// By default (`false`), the user can press, drag outside, drag back in, and still
    /// complete the press.
    pub should_cancel_on_pointer_exit: Signal<bool>,

    /// When `true`, prevents the browser from moving focus to the pressed element.
    /// Useful for toolbar buttons near text editors where focus should remain in the editor.
    pub prevent_focus_on_press: Signal<bool>,

    /// When provided, the returned `is_pressed` signal will be `true` when either the
    /// internal pressed state or this signal is `true`, forcing the pressed visual state.
    /// This allows parent components (e.g., overlays) to force the pressed appearance.
    ///
    /// **Deviation from react-aria**: react-aria calls this `isPressed: boolean`. Renamed to
    /// `force_is_pressed` to avoid ambiguity with the returned `is_pressed` signal.
    pub force_is_pressed: Option<Signal<bool>>,

    pub on_press: Option<Callback<PressEvent>>,
    pub on_press_up: Option<Callback<PressEvent>>,
    pub on_press_start: Option<Callback<PressEvent>>,
    pub on_press_end: Option<Callback<PressEvent>>,

    /// Called when the press state changes. Receives `true` when press starts,
    /// `false` when press ends.
    pub on_press_change: Option<Callback<bool>>,

    /// Called when the element receives a native `dblclick` event.
    pub on_double_press: Option<Callback<PressEvent>>,

    // Long press fields (all optional — when all are None, behavior is identical to press-only).
    /// Handler called when a long press interaction starts (mouse/touch only).
    pub on_long_press_start: Option<Callback<LongPressEvent>>,

    /// Handler called when the long press threshold time is met.
    pub on_long_press: Option<Callback<LongPressEvent>>,

    /// Handler called when a long press interaction ends.
    pub on_long_press_end: Option<Callback<LongPressEvent>>,

    /// The amount of time in milliseconds to wait before triggering a long press.
    /// Default is 500ms. Only used when at least one long press callback is set.
    pub long_press_threshold: Option<Signal<Duration>>,

    /// A description for assistive technology users indicating that a long press
    /// action is available, e.g. "Long press to open menu".
    /// Only applied when `on_long_press` is `Some`.
    pub long_press_accessibility_description: MaybeProp<String>,

    /// Turns long press off while `true` (e.g. while a collection item has no action), so presses
    /// aren't cancelled after the long press threshold.
    pub long_press_disabled: Signal<bool>,
}

impl Default for UsePressInput {
    /// Everything off, no callbacks (`on_press` does nothing).
    fn default() -> Self {
        Self {
            is_disabled: Signal::stored(false),
            propagation: PressPropagation::default(),
            allow_text_selection_on_press: Signal::stored(false),
            should_cancel_on_pointer_exit: Signal::stored(false),
            prevent_focus_on_press: Signal::stored(false),
            force_is_pressed: None,
            on_press: None,
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
            on_press_change: None,
            on_double_press: None,
            on_long_press_start: None,
            on_long_press: None,
            on_long_press_end: None,
            long_press_threshold: None,
            long_press_accessibility_description: MaybeProp::default(),
            long_press_disabled: Signal::stored(false),
        }
    }
}

/// Context for parent-to-child press prop forwarding.
///
/// Parent components (e.g., menu triggers, dialog triggers) that need to inject
/// press behavior into a descendant pressable element provide this context via
/// [`provide_context`] or the [`PressResponder`](crate::atoms::press::PressResponder) atom.
///
/// When provided, [`use_press`] automatically reads the context and merges the
/// parent's props with its own. Callbacks are chained (context first, then local).
/// Boolean/option fields use OR semantics so that either parent or local can
/// enable a behavior.
///
/// # Register pattern
///
/// The context includes a `registered` flag. When [`use_press`] reads a
/// `PressResponderContext`, it sets `registered` to `true`, signaling to the
/// parent that a pressable child exists within the subtree.
#[derive(Clone, Copy)]
pub struct PressResponderContext {
    /// Called when a complete press interaction is detected.
    pub on_press: Option<Callback<PressEvent>>,
    /// Called when a press interaction starts.
    pub on_press_start: Option<Callback<PressEvent>>,
    /// Called when a press interaction ends (regardless of success).
    pub on_press_end: Option<Callback<PressEvent>>,
    /// Called when the pointer is released over the target.
    pub on_press_up: Option<Callback<PressEvent>>,
    /// Called when the pressed state changes.
    pub on_press_change: Option<Callback<bool>>,
    /// Called when a long press starts, completes or ends (a `MenuTrigger` with long-press
    /// opening).
    pub on_long_press_start: Option<Callback<LongPressEvent>>,
    pub on_long_press: Option<Callback<LongPressEvent>>,
    pub on_long_press_end: Option<Callback<LongPressEvent>>,
    /// Describes the long-press action to assistive technology.
    pub long_press_accessibility_description: MaybeProp<String>,
    /// Whether the target is disabled.
    pub is_disabled: Option<Signal<bool>>,
    /// Controlled pressed state from parent (e.g., overlay trigger is open).
    pub force_is_pressed: Option<Signal<bool>>,
    /// When `true`, prevents focus on press.
    pub prevent_focus_on_press: Option<Signal<bool>>,
    /// When `true`, cancels press when pointer exits target.
    pub should_cancel_on_pointer_exit: Option<Signal<bool>>,
    /// When `true`, allows text selection during press.
    pub allow_text_selection_on_press: Option<Signal<bool>>,
    /// The props of an overlay trigger (`DialogTrigger`) for the pressable element.
    pub trigger: Option<PressResponderTrigger>,
    /// Keyboard shortcuts for the pressable element (a `MenuTrigger`'s: arrow keys open the menu),
    /// handled after the element's own. `use_button` applies them.
    pub shortcuts: Option<StoredValue<KeyboardShortcuts>>,
    /// Called when the pressable element requests a context menu (a `MenuTrigger` opening on it).
    /// `use_button` applies it.
    pub on_context_menu: Option<Callback<super::ContextMenuEvent>>,
    /// Signals that a pressable child has consumed this context.
    /// Set to `true` by [`use_press`] when it reads the context.
    pub registered: StoredValue<bool>,
}

/// The props an overlay trigger (`DialogTrigger`) gives the element that opens it: react-aria-components
/// passes its `triggerProps` through `PressResponder`. `use_button` applies them (its own values win)
/// and captures its element into `element`, at which the overlay is positioned.
#[derive(Debug, Clone, Copy)]
pub struct PressResponderTrigger {
    pub aria_haspopup: Signal<Option<AriaHasPopup>>,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_controls: Signal<Option<String>>,
    pub element: CapturedElement,
}

impl PressResponderTrigger {
    /// No trigger: no ARIA attributes, and an element nothing reads.
    pub fn empty() -> Self {
        Self {
            aria_haspopup: Signal::stored(None),
            aria_expanded: Signal::stored(None),
            aria_controls: Signal::stored(None),
            element: CapturedElement::new(),
        }
    }
}

impl PressResponderContext {
    /// Create an empty context with all fields set to `None`.
    /// Used by [`ClearPressResponder`](crate::atoms::press::ClearPressResponder)
    /// to shadow any ancestor context.
    pub fn empty() -> Self {
        Self {
            on_press: None,
            on_press_start: None,
            on_press_end: None,
            on_press_up: None,
            on_press_change: None,
            on_long_press_start: None,
            on_long_press: None,
            on_long_press_end: None,
            long_press_accessibility_description: MaybeProp::default(),
            is_disabled: None,
            force_is_pressed: None,
            prevent_focus_on_press: None,
            should_cancel_on_pointer_exit: None,
            allow_text_selection_on_press: None,
            trigger: None,
            shortcuts: None,
            on_context_menu: None,
            registered: StoredValue::new(false),
        }
    }
}

/// Chain two optional callbacks. Context fires first, then local.
/// Returns `None` only if both are `None`.
pub fn chain_optional_callbacks<T: Clone + Send + Sync + 'static>(
    ctx_cb: Option<Callback<T>>,
    local_cb: Option<Callback<T>>,
) -> Option<Callback<T>> {
    match (ctx_cb, local_cb) {
        (Some(ctx), Some(local)) => Some(Callback::new(move |e: T| {
            ctx.run(e.clone());
            local.run(e);
        })),
        (Some(ctx), None) => Some(ctx),
        (None, local) => local,
    }
}

#[derive(Debug)]
pub struct UsePressReturn {
    /// Props for programmatic merging. Call `.into_parts()` for view spreading and styles.
    pub props: PropsWithStyles<UsePressProps>,
    pub is_pressed: Signal<bool>,
}

/// Props from `use_press` that can be extracted and merged programmatically.
///
/// Use [`UsePressProps::into_attrs()`] to convert to an attributes-tuple spreadable using Leptos's
/// spreading syntax (`<div {..props.into_attrs()}>`) (taking ownership).
/// # Example
///
/// ```ignore
/// // Basic usage
/// let press = use_press(input);
/// view! { <button {..press.props.into_attrs()}>"Click"</button> }
///
/// // Merging multiple press hooks
/// let press1 = use_press(input1);
/// let press2 = use_press(input2);
/// let merged = press1.props.merge(press2.props);
/// view! { <button {..merged.into_attrs()}>"Both handlers fire"</button> }
/// ```
#[derive(Debug)]
pub struct UsePressProps {
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    /// Set when `on_long_press` and `long_press_accessibility_description` are provided.
    /// `None` otherwise.
    pub aria_describedby: Signal<Option<AriaDescribedby>>,
}

impl IntoAttrs for UsePressProps {
    type Attrs = UsePressAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_mousedown.into_on(ev::mousedown),
            self.on_pointerup.into_on(ev::pointerup),
            self.on_dragstart.into_on(ev::dragstart),
            self.on_dblclick.into_on(ev::dblclick),
            Attr(attr::AriaDescribedby, self.aria_describedby),
        )
    }
}

/// These attributes must be spread onto the target element using the spread syntax `<div {..attrs}/>`.
pub type UsePressAttrs = (
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
    On<ev::pointerup, SharedEventCallback<PointerEvent>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    On<ev::dblclick, SharedEventCallback<MouseEvent>>,
    Attr<attr::AriaDescribedby, Signal<Option<AriaDescribedby>>>,
);

struct PressState {
    pointer_id: i32,
    pointer_type: PointerType,

    /// The element this press hook was bound to.
    current_target: EventTarget,

    is_over_target: bool,

    /// Tracks whether `trigger_press_start` was actually fired, to prevent
    /// firing `press_end` without a corresponding `press_start`.
    did_fire_press_start: bool,

    /// Handle for the 80ms fallback timeout that triggers `target.click()`
    /// when iOS long press doesn't naturally fire a click event.
    click_timeout_handle: Option<TimeoutHandle>,

    /// The press's global listeners (on the document), removed when dropped.
    global_listeners: Vec<Listener>,

    // Long press tracking
    /// Timeout handle for the long press threshold timer.
    long_press_timeout_handle: Option<TimeoutHandle>,
    /// Whether the long press threshold was met during this interaction.
    long_press_triggered: bool,
}

impl PressState {
    fn cleanup_event_handlers(&mut self) {
        self.global_listeners.clear();
    }

    fn clear_click_timeout(&mut self) {
        if let Some(handle) = self.click_timeout_handle.take() {
            handle.clear();
        }
    }

    fn clear_long_press_timeout(&mut self) {
        if let Some(handle) = self.long_press_timeout_handle.take() {
            handle.clear();
        }
    }

    fn restore_text_selection_if_needed(&self, allow_text_selection_on_press: bool) {
        if !allow_text_selection_on_press && let Some(element) = self.current_target.to_element() {
            element.restore_text_selection();
        }
    }

    fn is_pointer_over_target(&self, e: &PointerEvent) -> bool {
        is_over(e, self.current_target.as_element().expect("element"))
    }
}

enum EventRef<'a> {
    Pointer(&'a PointerEvent),
    Keyboard(&'a KeyboardEvent),
    Mouse(&'a MouseEvent),
    /// A press that ends without a DOM event, e.g. because the element became disabled while
    /// pressed. Carries the press target; it has no modifiers, coordinates or key.
    Synthetic(&'a EventTarget),
}

impl EventRef<'_> {
    fn modifiers(&self) -> Modifiers {
        match self {
            EventRef::Pointer(e) => e.modifiers(),
            EventRef::Keyboard(e) => e.modifiers(),
            EventRef::Mouse(e) => e.modifiers(),
            EventRef::Synthetic(_) => Modifiers {
                shift_key: false,
                ctrl_key: false,
                meta_key: false,
                alt_key: false,
            },
        }
    }

    fn stop_propagation(&self) {
        match self {
            EventRef::Pointer(e) => e.stop_propagation(),
            EventRef::Keyboard(e) => e.stop_propagation(),
            EventRef::Mouse(e) => e.stop_propagation(),
            EventRef::Synthetic(_) => {}
        }
    }

    fn current_target(&self) -> EventTarget {
        match self {
            EventRef::Pointer(e) => e.expect_current_target(),
            EventRef::Keyboard(e) => e.expect_current_target(),
            EventRef::Mouse(e) => e.expect_current_target(),
            EventRef::Synthetic(target) => (*target).clone(),
        }
    }

    /// Returns the keyboard key that triggered this event, if any.
    fn key(&self) -> Option<KeyboardKey> {
        match self {
            EventRef::Keyboard(e) => Some(e.typed_key()),
            EventRef::Pointer(_) | EventRef::Mouse(_) | EventRef::Synthetic(_) => None,
        }
    }

    /// Returns coordinates relative to the target element's bounding rect.
    /// For pointer/mouse events, returns the event position relative to the element.
    /// For keyboard events, returns the element's center point (`width/2`, `height/2`),
    /// matching react-aria behavior. This is useful for consumers that position UI relative
    /// to the press point (e.g., ripple effects).
    /// Returns `None` if the target has no client bounding rect.
    fn coordinates(&self) -> (Option<f64>, Option<f64>) {
        match self {
            EventRef::Pointer(e) => {
                let (client_x, client_y) = (e.client_x(), e.client_y());
                self.current_target()
                    .to_element()
                    .map(|el| el.get_bounding_client_rect())
                    .map_or((None, None), |rect| {
                        (Some(client_x - rect.left()), Some(client_y - rect.top()))
                    })
            }
            EventRef::Mouse(e) => {
                let (client_x, client_y) = (e.client_x(), e.client_y());
                self.current_target()
                    .to_element()
                    .map(|el| el.get_bounding_client_rect())
                    .map_or((None, None), |rect| {
                        (Some(client_x - rect.left()), Some(client_y - rect.top()))
                    })
            }
            EventRef::Keyboard(_) => self
                .current_target()
                .to_element()
                .map(|el| el.get_bounding_client_rect())
                .map_or((None, None), |rect| {
                    (Some(rect.width() / 2.0), Some(rect.height() / 2.0))
                }),
            EventRef::Synthetic(_) => (None, None),
        }
    }
}

/// Runs a press callback, if any. Returns whether the event should stop propagating: press events
/// stop propagation unless the callback calls `continue_propagation()` (react-aria's
/// `shouldStopPropagation`, which defaults to `true`, also without a callback).
fn fire_press_callback(
    callback: Option<Callback<PressEvent>>,
    state: &PressState,
    event: &EventRef<'_>,
    is_triggering_event: StoredValue<bool, LocalStorage>,
) -> bool {
    let Some(callback) = callback else {
        return true;
    };
    let (propagation, propagation_state) = PropagationControl::new();
    let (x, y) = event.coordinates();
    let key = event.key();
    is_triggering_event.set_value(true);
    callback.run(PressEvent {
        pointer_type: state.pointer_type.clone(),
        target: SendWrapper::new(state.current_target.clone()),
        modifiers: event.modifiers(),
        x,
        y,
        key,
        propagation,
    });
    is_triggering_event.set_value(false);
    !propagation_state.load(Ordering::Acquire)
}

/// Merges a responder's and the element's `force_is_pressed`: either forces the pressed
/// appearance.
fn merge_force_is_pressed(
    responder: Option<Signal<bool>>,
    own: Option<Signal<bool>>,
) -> Option<Signal<bool>> {
    match (responder, own) {
        (Some(responder), Some(own)) => Some(Signal::derive(move || responder.get() || own.get())),
        (responder, own) => responder.or(own),
    }
}

/// # Panics
///
/// Panics if the press state is initialized while already active (debug assertion),
/// or if the current target of the pointer event is not available.
#[allow(clippy::too_many_lines)]
pub fn use_press(input: UsePressInput) -> UsePressReturn {
    #[cfg(feature = "ssr")]
    {
        // The responder's forced state too, so `data-pressed` agrees with the client.
        let is_pressed = merge_force_is_pressed(
            use_context::<PressResponderContext>().and_then(|c| c.force_is_pressed),
            input.force_is_pressed,
        )
        .unwrap_or_else(|| Signal::stored(false));
        UsePressReturn {
            props: PropsWithStyles::new(
                UsePressProps {
                    on_keydown: EventHandler::new(|_: KeyboardEvent| {}),
                    on_click: EventHandler::new(|_: MouseEvent| {}),
                    on_pointerdown: EventHandler::new(|_: PointerEvent| {}),
                    on_pointerup: EventHandler::new(|_: PointerEvent| {}),
                    on_mousedown: EventHandler::new(|_: MouseEvent| {}),
                    on_dragstart: EventHandler::new(|_: DragEvent| {}),
                    on_dblclick: EventHandler::new(|_: MouseEvent| {}),
                    // Set after mount on the client, as on the client before hydration.
                    aria_describedby: Signal::stored(None),
                },
                Styles::new(),
            ),
            is_pressed,
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        let UsePressInput {
            is_disabled: disabled,
            propagation,
            allow_text_selection_on_press,
            should_cancel_on_pointer_exit,
            prevent_focus_on_press,
            force_is_pressed,
            on_press,
            on_press_up,
            on_press_start,
            on_press_end,
            on_press_change,
            on_double_press,
            on_long_press_start,
            on_long_press,
            on_long_press_end,
            long_press_threshold,
            long_press_accessibility_description,
            long_press_disabled,
        } = input;

        // --- PressResponderContext merging ---
        // If a parent component provided a PressResponderContext (via PressResponder or
        // provide_context), merge its props with our local input. This allows parent components
        // to inject press behavior into child pressable elements.
        let ctx = use_context::<PressResponderContext>();
        if let Some(ref ctx) = ctx {
            ctx.registered.set_value(true);
        }

        // Merge disabled: OR semantics (if either parent or local says disabled, element is disabled).
        let disabled = match ctx.as_ref().and_then(|c| c.is_disabled) {
            Some(ctx_disabled) => Signal::derive(move || disabled.get() || ctx_disabled.get()),
            None => disabled,
        };

        // Merge boolean config fields: OR semantics, so either the context or the input can turn
        // them on.
        let or = |own: Signal<bool>, ctx: Option<Signal<bool>>| match ctx {
            Some(ctx) => Signal::derive(move || own.get() || ctx.get()),
            None => own,
        };
        let prevent_focus_on_press = or(
            prevent_focus_on_press,
            ctx.as_ref().and_then(|c| c.prevent_focus_on_press),
        );
        let should_cancel_on_pointer_exit = or(
            should_cancel_on_pointer_exit,
            ctx.as_ref().and_then(|c| c.should_cancel_on_pointer_exit),
        );
        let allow_text_selection_on_press = or(
            allow_text_selection_on_press,
            ctx.as_ref().and_then(|c| c.allow_text_selection_on_press),
        );
        let force_propagation = propagation == PressPropagation::Continue;

        let force_is_pressed = merge_force_is_pressed(
            ctx.as_ref().and_then(|c| c.force_is_pressed),
            force_is_pressed,
        );

        // Chain callbacks: context fires first, then local.
        let on_press = chain_optional_callbacks(ctx.as_ref().and_then(|c| c.on_press), on_press);
        let on_press_start =
            chain_optional_callbacks(ctx.as_ref().and_then(|c| c.on_press_start), on_press_start);
        let on_press_end =
            chain_optional_callbacks(ctx.as_ref().and_then(|c| c.on_press_end), on_press_end);
        let on_press_up =
            chain_optional_callbacks(ctx.as_ref().and_then(|c| c.on_press_up), on_press_up);
        let on_press_change = chain_optional_callbacks(
            ctx.as_ref().and_then(|c| c.on_press_change),
            on_press_change,
        );
        let on_long_press_start = chain_optional_callbacks(
            ctx.as_ref().and_then(|c| c.on_long_press_start),
            on_long_press_start,
        );
        let on_long_press =
            chain_optional_callbacks(ctx.as_ref().and_then(|c| c.on_long_press), on_long_press);
        let on_long_press_end = chain_optional_callbacks(
            ctx.as_ref().and_then(|c| c.on_long_press_end),
            on_long_press_end,
        );
        let ctx_description = ctx
            .as_ref()
            .map(|c| c.long_press_accessibility_description)
            .unwrap_or_default();
        let long_press_accessibility_description = MaybeProp::derive(move || {
            long_press_accessibility_description
                .get()
                .or_else(|| ctx_description.get())
        });
        // --- End PressResponderContext merging ---

        let (is_pressed, set_is_pressed) = signal(false);

        let has_long_press_handlers =
            on_long_press.is_some() || on_long_press_start.is_some() || on_long_press_end.is_some();
        let supports_long_press =
            move || has_long_press_handlers && !long_press_disabled.get_untracked();
        let long_press_threshold =
            long_press_threshold.unwrap_or_else(|| Signal::stored(DEFAULT_LONG_PRESS_THRESHOLD));

        let state: StoredValue<Option<PressState>, LocalStorage> = StoredValue::new_local(None);

        // Tracks the pointer type from the most recently completed press,
        // so the dblclick handler can construct a PressEvent after state has been cleared.
        let last_pointer_type: StoredValue<Option<PointerType>, LocalStorage> =
            StoredValue::new_local(None);

        // Re-entrancy guard: prevents infinite loops when a press callback
        // synchronously triggers a click event on the same element.
        let is_triggering_event: StoredValue<bool, LocalStorage> = StoredValue::new_local(false);

        // Tracks whether a virtual pointer event (e.g. VoiceOver on iOS) was seen in pointerdown,
        // so that the click handler can detect it and fire a full virtual press cycle.
        let saw_virtual_pointer_event: StoredValue<bool, LocalStorage> =
            StoredValue::new_local(false);

        // Tracks meta key events for macOS workaround
        let meta_key_events: StoredValue<
            Option<std::collections::HashMap<KeyboardKey, KeyboardEvent>>,
            LocalStorage,
        > = StoredValue::new_local(None);

        let initialize_press_state = move |e: EventRef<'_>, global_listeners: Vec<Listener>| {
            // If a press is already active, ignore this initialization request.
            // This can happen when a second pointerdown fires before the first press's
            // click event completes the cycle (the window between pointerup and click).
            // We follow React Aria's approach of ignoring the second press rather than
            // cancelling + re-initializing, because cancel+re-init causes click event
            // cross-pollination: click#1 (from interaction 1) would complete press#2
            // (from interaction 2), since the click handler cannot distinguish which
            // press a click belongs to.
            if state.with_value(Option::is_some) {
                return;
            }

            state.set_value(Some(PressState {
                pointer_id: match e {
                    EventRef::Pointer(e) => e.pointer_id(),
                    EventRef::Keyboard(_) | EventRef::Mouse(_) | EventRef::Synthetic(_) => 0,
                },
                pointer_type: match e {
                    EventRef::Pointer(e) => PointerType::from(e.pointer_type()),
                    EventRef::Keyboard(_e) => PointerType::Keyboard,
                    EventRef::Mouse(_) | EventRef::Synthetic(_) => PointerType::Virtual,
                },
                current_target: e.current_target(),
                is_over_target: match e {
                    // The pointer event fired on this element, so the pointer is definitionally
                    // over it. react-aria also sets `isOverTarget = true` unconditionally here.
                    // Using `is_over()` breaks for `display: contents` (zero-sized bounding rect).
                    EventRef::Pointer(_) => true,
                    EventRef::Keyboard(_) | EventRef::Mouse(_) | EventRef::Synthetic(_) => false,
                },
                did_fire_press_start: false,
                click_timeout_handle: None,
                global_listeners,
                long_press_timeout_handle: None,
                long_press_triggered: false,
            }));
        };

        // The triggers return whether the event should stop propagating (react-aria's
        // `shouldStopPropagation`): unless a callback continued it. The handlers stop it.

        // Has no effect if press is already started. Calling this multiple times only executes the effect once.
        let trigger_press_start = move |s: &mut PressState, e: EventRef<'_>| -> bool {
            if s.did_fire_press_start {
                return false;
            }
            s.did_fire_press_start = true;

            let should_stop = fire_press_callback(on_press_start, s, &e, is_triggering_event);

            if let Some(on_press_change) = on_press_change {
                is_triggering_event.set_value(true);
                on_press_change.run(true);
                is_triggering_event.set_value(false);
            }

            set_is_pressed.set(true);
            should_stop
        };

        // Has no effect if press was not started. Calling this multiple times only executes the effect once.
        // When `was_pressed` is true, also fires the `on_press` callback.
        let trigger_press_end =
            move |s: &mut PressState, e: EventRef<'_>, was_pressed: bool| -> bool {
                if !s.did_fire_press_start {
                    return false;
                }
                s.did_fire_press_start = false;

                // Clear long press timeout on press end.
                s.clear_long_press_timeout();

                // Long press end first: react-aria's `useLongPress` handlers precede `usePress`'.
                // Fire long press end for mouse/touch when long press is configured.
                if supports_long_press()
                    && (s.pointer_type == PointerType::Mouse
                        || s.pointer_type == PointerType::Touch)
                    && let Some(on_long_press_end) = on_long_press_end
                {
                    let (x, y) = e.coordinates();
                    is_triggering_event.set_value(true);
                    on_long_press_end.run(LongPressEvent {
                        event_type: LongPressEventType::LongPressEnd,
                        pointer_type: s.pointer_type.clone(),
                        target: SendWrapper::new(s.current_target.clone()),
                        modifiers: e.modifiers(),
                        x,
                        y,
                    });
                    is_triggering_event.set_value(false);
                }

                let mut should_stop_propagation =
                    fire_press_callback(on_press_end, s, &e, is_triggering_event);

                if let Some(on_press_change) = on_press_change {
                    is_triggering_event.set_value(true);
                    on_press_change.run(false);
                    is_triggering_event.set_value(false);
                }

                set_is_pressed.set(false);

                // Do NOT fire on_press if the long press threshold was met.
                // The short press was consumed by the long press interaction.
                if was_pressed && !s.long_press_triggered {
                    should_stop_propagation &=
                        fire_press_callback(on_press, s, &e, is_triggering_event);
                }
                should_stop_propagation
            };

        let trigger_press_up = move |s: &PressState, e: EventRef<'_>| -> bool {
            fire_press_callback(on_press_up, s, &e, is_triggering_event)
        };
        let stop_unless_forced = move |should_stop: bool, e: EventRef<'_>| {
            if should_stop && !force_propagation {
                e.stop_propagation();
            }
        };

        let cancel_active_press = move |e: EventRef<'_>| {
            state.update_value(|s| {
                if let Some(s) = s {
                    s.clear_click_timeout();
                    s.clear_long_press_timeout();
                    trigger_press_end(s, e, false);
                    s.restore_text_selection_if_needed(
                        allow_text_selection_on_press.get_untracked(),
                    );
                    s.cleanup_event_handlers();
                }
            });
            state.set_value(None);
        };

        // Cancel an active press when the element becomes disabled. The events that would
        // normally end the press are ignored while disabled, so without this, e.g. a spin button
        // whose first step disables it would keep spinning.
        Effect::new(move |_| {
            if disabled.get() {
                let target = state.with_value(|s| s.as_ref().map(|s| s.current_target.clone()));
                if let Some(target) = target {
                    cancel_active_press(EventRef::Synthetic(&target));
                }
            }
        });

        let handle_key_up = move |e: KeyboardEvent| {
            // First check if we should handle this event (immutable check).
            // Use the stored press target, not e.current_target(), because the keyup listener
            // is registered on the document — e.current_target() would be the Document, not the
            // pressed element, causing is_valid_keyboard_event to bypass validation.
            let should_handle = state.with_value(|s| {
                s.as_ref().is_some_and(|s| {
                    !disabled.get_untracked()
                        && is_valid_keyboard_event(&e, s.current_target.clone())
                })
            });
            if !should_handle {
                return;
            }

            let key = e.typed_key();
            if e.expect_target()
                .to_element()
                .is_some_and(|el| should_prevent_default_keyboard(&el, &key))
            {
                e.prevent_default();
            }

            // If a link was triggered with a key other than Enter, open the URL ourselves.
            // This means the link has a role override, and the default browser behavior
            // only applies when using the Enter key.
            if key != KeyboardKey::Enter
                && let Some(current_target) =
                    state.with_value(|s| s.as_ref().map(|s| s.current_target.clone()))
                && let Some(true) = node_contains(
                    current_target.as_node().as_ref(),
                    e.expect_target()
                        .to_element()
                        .and_then(|el| el.as_node())
                        .as_ref(),
                )
                && let Some(el) = current_target.as_element()
                && el.is_anchor_link()
            {
                open_link(el, e.modifiers());
            }

            // macOS Meta key workaround: if Meta key is up, dispatch synthetic
            // keyup events for any keys that were pressed while Meta was held.
            if key == KeyboardKey::Meta {
                meta_key_events.update_value(|map| {
                    if let Some(events) = map.take() {
                        for (_key, stored_e) in events {
                            if let Some(ct) = stored_e.current_target() {
                                let _ = ct.dispatch_event(&stored_e);
                            }
                        }
                    }
                });
            }

            // Check if the keyup target is still inside the original press target.
            // If focus moved away during the keypress, we should not fire on_press.
            let was_pressed = state.with_value(|s| {
                s.as_ref().is_some_and(|s| {
                    node_contains(
                        s.current_target.as_node().as_ref(),
                        e.expect_target().as_node().as_ref(),
                    )
                    .unwrap_or(false)
                })
            });

            // Now perform mutable operations: fire press_up and press_end.
            state.update_value(move |s| {
                if let Some(s) = s.as_mut() {
                    // A document listener: nothing to stop (react-aria).
                    trigger_press_up(s, EventRef::Keyboard(&e));
                    trigger_press_end(s, EventRef::Keyboard(&e), was_pressed);
                    s.cleanup_event_handlers();
                }
            });
            state.set_value(None);
        };

        let handle_key_down = move |e: KeyboardEvent| {
            if !node_contains(
                e.expect_current_target().as_node().as_ref(),
                e.expect_target().as_node().as_ref(),
            )
            .unwrap_or(true)
            {
                tracing::debug!(
                    "Aborting handle_key_down, as current_target did not contain target."
                );
                return;
            }

            let key = e.typed_key();

            if is_valid_keyboard_event(&e, e.expect_current_target()) {
                if e.expect_target()
                    .as_element()
                    .is_some_and(|el| should_prevent_default_keyboard(el, &key))
                {
                    e.prevent_default();
                }

                // Read meta_key before the closure moves `e`.
                let is_meta_held = device::is_mac() && e.meta_key() && key != KeyboardKey::Meta;
                let e_for_meta = if is_meta_held { Some(e.clone()) } else { None };

                // Only initialize press on the first keydown, not on repeats. Repeats and keys of
                // an active press stop (react-aria); a disabled element lets them propagate.
                let mut should_stop = !disabled.get_untracked();
                if state.with_value(Option::is_none) && !disabled.get_untracked() && !e.repeat() {
                    initialize_press_state(
                        EventRef::Keyboard(&e),
                        e.expect_current_target()
                            .get_owner_document()
                            .map(|doc| listen_to(&doc, ev::keyup, false, handle_key_up))
                            .into_iter()
                            .collect(),
                    );

                    state.update_value(|s| {
                        if let Some(s) = s {
                            should_stop = trigger_press_start(s, EventRef::Keyboard(&e));
                        }
                    });
                }
                stop_unless_forced(should_stop, EventRef::Keyboard(&e));

                // macOS Meta key workaround: store events pressed while Meta is held
                // because macOS doesn't fire keyup for non-Meta keys while Meta is down.
                // This must be OUTSIDE the state.is_none() check so it captures keys
                // pressed while Meta is held even during an active press.
                if let Some(e) = e_for_meta {
                    meta_key_events.update_value(|map| {
                        let map = map.get_or_insert_with(std::collections::HashMap::new);
                        map.insert(key.clone(), e.clone());
                    });
                }
            } else if key == KeyboardKey::Meta {
                // Initialize the map when Meta key itself is pressed.
                meta_key_events.set_value(Some(std::collections::HashMap::new()));
            }
        };

        let handle_click = move |e: MouseEvent| {
            // Re-entrancy guard: if we are currently inside a press callback that
            // synchronously triggered a click, skip this handler to prevent infinite loops.
            if is_triggering_event.get_value() {
                return;
            }

            if !node_contains(
                e.expect_current_target().as_node().as_ref(),
                e.expect_target().as_node().as_ref(),
            )
            .unwrap_or(true)
            {
                tracing::debug!("Aborting handle_click, as current_target did not contain target.");
                return;
            }

            // Only the primary button presses (react-aria).
            if e.button() != 0 {
                return;
            }

            if disabled.get_untracked() {
                e.prevent_default();
                // Nothing triggered: stopped, as react-aria's clicks that start no press.
                stop_unless_forced(true, EventRef::Mouse(&e));
                return;
            }

            // Check if this is the completion of a pointer-initiated press.
            // After pointerup, we defer press completion to onClick for DOM mutation safety.
            let was_pointer_press = state.with_value(|s| {
                s.as_ref().is_some_and(|s| {
                    s.pointer_type != PointerType::Keyboard
                        && s.pointer_type != PointerType::Virtual
                        && is_pressed.get_untracked()
                })
            });

            // As react-aria: stopped unless the triggered press callbacks continued it.
            let mut should_stop = true;
            if was_pointer_press {
                state.update_value(|s| {
                    if let Some(s) = s {
                        last_pointer_type.set_value(Some(s.pointer_type.clone()));
                        s.clear_click_timeout();
                        let stop_up = trigger_press_up(s, EventRef::Mouse(&e));
                        let stop_end = trigger_press_end(s, EventRef::Mouse(&e), true);
                        should_stop = stop_up && stop_end;
                        s.restore_text_selection_if_needed(
                            allow_text_selection_on_press.get_untracked(),
                        );
                        s.cleanup_event_handlers();
                    }
                });
                state.set_value(None);
                stop_unless_forced(should_stop, EventRef::Mouse(&e));
                return;
            }

            // Handle virtual click (screen reader / assistive technology).
            // Also handle deferred virtual pointer events from VoiceOver on iOS.
            if !is_pressed.get_untracked()
                && (saw_virtual_pointer_event.get_value() || is_virtual_click(&e))
            {
                saw_virtual_pointer_event.set_value(false);
                // Fire full virtual press cycle
                initialize_press_state(
                    EventRef::Mouse(&e),
                    // No global listener needed for virtual clicks: they complete immediately.
                    Vec::new(),
                );

                state.update_value(|s| {
                    if let Some(s) = s {
                        let stop_start = trigger_press_start(s, EventRef::Mouse(&e));
                        let stop_up = trigger_press_up(s, EventRef::Mouse(&e));
                        let stop_end = trigger_press_end(s, EventRef::Mouse(&e), true);
                        should_stop = stop_start && stop_up && stop_end;
                    }
                });
                state.set_value(None);
            }
            stop_unless_forced(should_stop, EventRef::Mouse(&e));
        };

        // Pointer move handler for drag-in / drag-out behavior.
        let handle_pointer_move = move |e: PointerEvent| {
            state.update_value(|s| {
                if let Some(s) = s.as_mut() {
                    if e.pointer_id() != s.pointer_id {
                        return;
                    }
                    let is_over_target = s.is_pointer_over_target(&e);

                    if should_cancel_on_pointer_exit.get_untracked()
                        && s.is_over_target
                        && !is_over_target
                    {
                        // Cancel the entire press when configured to do so.
                        trigger_press_end(s, EventRef::Pointer(&e), false);
                        s.cleanup_event_handlers();
                        s.restore_text_selection_if_needed(
                            allow_text_selection_on_press.get_untracked(),
                        );
                    } else {
                        // Leaving and re-entering stop nothing (react-aria).
                        match (s.is_over_target, is_over_target) {
                            (true, false) => {
                                trigger_press_end(s, EventRef::Pointer(&e), false);
                            }
                            (false, true) => {
                                trigger_press_start(s, EventRef::Pointer(&e));
                            }
                            _ => {}
                        }
                    }
                    s.is_over_target = is_over_target;
                }
            });

            // If should_cancel_on_pointer_exit caused a full cancel, clear state.
            if should_cancel_on_pointer_exit.get_untracked() {
                let should_clear = state.with_value(|s| {
                    s.as_ref()
                        .is_some_and(|s| !s.did_fire_press_start && !s.is_over_target)
                });
                if should_clear {
                    state.set_value(None);
                }
            }
        };

        // Pointer up: defer press completion to onClick for DOM mutation safety.
        let handle_pointer_up = move |e: PointerEvent| {
            // Only handle primary button releases.
            if e.button() != 0 {
                return;
            }

            // Only handle the pointer that started the press.
            let should_handle =
                state.with_value(|s| s.as_ref().is_some_and(|s| e.pointer_id() == s.pointer_id));
            if !should_handle {
                return;
            }

            let should_clear = state.with_value(|s| {
                let Some(s) = s.as_ref() else {
                    return false;
                };

                // Use DOM containment (like react-aria) instead of bounding-rect overlap.
                // `is_over()` compares pointer coordinates against `getBoundingClientRect()`,
                // which returns a zero-sized rect for `display: contents` elements, causing
                // every press to be incorrectly cancelled.
                !node_contains(
                    s.current_target.as_node().as_ref(),
                    e.target()
                        .as_ref()
                        .and_then(|t| t.dyn_ref::<web_sys::Node>()),
                )
                .unwrap_or(false)
            });

            if should_clear {
                cancel_active_press(EventRef::Pointer(&e));
                return;
            }

            // Pointer is over the target. Keep state.is_pressed=true and defer
            // actual completion to the onClick handler (Phase 3).
            // Set up an 80ms timeout fallback: on iOS, long press interactions
            // may not naturally fire a click event, so we programmatically trigger one.
            state.update_value(|s| {
                if let Some(s) = s {
                    // Prevent duplicate pointerleave handling
                    s.is_over_target = false;

                    // Set up 80ms fallback to programmatically click the target.
                    let current_target = s.current_target.clone();
                    s.click_timeout_handle = set_timeout_with_handle(
                        move || {
                            // Focus the element without scrolling before clicking,
                            // matching react-aria's focusWithoutScrolling behavior.
                            if let Some(el) = current_target.as_element() {
                                focus_element(el, true);
                            }
                            if let Some(html_el) = current_target.as_html_element() {
                                html_el.click();
                            }
                        },
                        Duration::from_millis(80),
                    )
                    .ok();
                }
            });
        };

        // Cancel the ongoing press.
        let handle_pointer_cancel = move |e: PointerEvent| {
            cancel_active_press(EventRef::Pointer(&e));
        };

        // Start a press.
        let handle_pointer_down = move |e: PointerEvent| {
            if e.button() != 0 {
                return;
            }

            if !e.current_target_contains_target() {
                tracing::trace!(
                    "Aborting handle_pointer_down, as current_target did not contain target."
                );
                return;
            }

            // Handle virtual pointer events (e.g., VoiceOver on iOS).
            // These are deferred to the onClick handler.
            if is_virtual_pointer_event(&e) {
                // Store that we saw a virtual event; onClick will handle the full press cycle.
                saw_virtual_pointer_event.set_value(true);
                return;
            }

            let target = e.expect_target();

            // As react-aria: a press that is already active keeps its listeners (it ends with its
            // own pointer up or click) and stops the event; a disabled element lets it propagate.
            let mut should_stop = !disabled.get_untracked();
            if !disabled.get_untracked() && state.with_value(Option::is_none) {
                // On the pressed element (react-aria's `state.target`), which the press restores.
                if !allow_text_selection_on_press.get_untracked()
                    && let Some(element) = e.expect_current_target().as_element()
                {
                    element.disable_text_selection();
                }

                // Release pointer capture to enable pointerleave/pointerenter on touch.
                // By default, the browser captures pointer events to the original target,
                // which prevents these events from firing correctly.
                if let Some(element) = target.dyn_ref::<web_sys::Element>()
                    && element.has_pointer_capture(e.pointer_id())
                {
                    let _ = element.release_pointer_capture(e.pointer_id());
                }

                let global_listeners = e
                    .expect_current_target()
                    .get_owner_document()
                    .map(|doc| {
                        vec![
                            listen_to(&doc, ev::pointermove, false, handle_pointer_move),
                            listen_to(&doc, ev::pointerup, false, handle_pointer_up),
                            listen_to(&doc, ev::pointercancel, false, handle_pointer_cancel),
                        ]
                    })
                    .unwrap_or_default();
                initialize_press_state(EventRef::Pointer(&e), global_listeners);

                state.update_value(|s| {
                    if let Some(s) = s {
                        // Long presses for mouse/touch when configured. Their start comes first:
                        // react-aria's `useLongPress` handlers precede `usePress`'.
                        let long_press = supports_long_press()
                            && (s.pointer_type == PointerType::Mouse
                                || s.pointer_type == PointerType::Touch);
                        if long_press && let Some(on_long_press_start) = on_long_press_start {
                            let (x, y) = EventRef::Pointer(&e).coordinates();
                            is_triggering_event.set_value(true);
                            on_long_press_start.run(LongPressEvent {
                                event_type: LongPressEventType::LongPressStart,
                                pointer_type: s.pointer_type.clone(),
                                target: SendWrapper::new(s.current_target.clone()),
                                modifiers: EventRef::Pointer(&e).modifiers(),
                                x,
                                y,
                            });
                            is_triggering_event.set_value(false);
                        }

                        should_stop = trigger_press_start(s, EventRef::Pointer(&e));

                        // Start the long press timer.
                        if long_press {
                            // Capture values for the timeout closure
                            let pointer_type = s.pointer_type.clone();
                            let modifiers = EventRef::Pointer(&e).modifiers();
                            let current_target = s.current_target.clone();
                            let (x, y) = EventRef::Pointer(&e).coordinates();

                            s.long_press_timeout_handle = set_timeout_with_handle(
                                move || {
                                    // Dispatch pointercancel on the target to cancel the press
                                    // interaction. This is synchronous — the pointercancel handler
                                    // (and thus trigger_press_end / on_long_press_end) will fire
                                    // before the code after dispatch_event.
                                    if let Some(el) = current_target.as_element() {
                                        // Bubbling, as react-aria's: the press listens for it
                                        // on the document.
                                        let init = web_sys::PointerEventInit::new();
                                        init.set_bubbles(true);
                                        let cancel_event = PointerEvent::new_with_event_init_dict(
                                            "pointercancel",
                                            &init,
                                        )
                                        .expect("should create pointercancel event");
                                        let _ = el.dispatch_event(&cancel_event);

                                        // Focus the element without scrolling.
                                        focus_element(el, true);
                                    }

                                    // Mark long press as triggered so on_press is suppressed.
                                    state.update_value(|s| {
                                        if let Some(s) = s.as_mut() {
                                            s.long_press_triggered = true;
                                            s.long_press_timeout_handle = None;
                                        }
                                    });

                                    // Fire the long press callback.
                                    if let Some(on_long_press) = on_long_press {
                                        on_long_press.run(LongPressEvent {
                                            event_type: LongPressEventType::LongPress,
                                            pointer_type: pointer_type.clone(),
                                            target: SendWrapper::new(current_target),
                                            modifiers,
                                            x,
                                            y,
                                        });
                                    }
                                },
                                long_press_threshold.get_untracked(),
                            )
                            .ok();

                            // For touch, prevent the context menu on the event target.
                            if s.pointer_type == PointerType::Touch {
                                s.current_target.prevent_default_once("contextmenu");
                            }
                        }
                    }
                });
            }
            stop_unless_forced(should_stop, EventRef::Pointer(&e));
        };

        // Safari doesn't fire pointercancel on drag. Handle dragstart to cancel the press.
        let handle_dragstart = move |_e: DragEvent| {
            cancel_active_press(EventRef::Pointer(
                &PointerEvent::new("pointercancel").expect("should create pointercancel event"),
            ));
        };

        // Handle native dblclick for on_double_press.
        // By the time dblclick fires, the press state has already been cleared by the second click
        // handler. We use `last_pointer_type` (saved before clearing state) to construct the event.
        let handle_dblclick = move |e: MouseEvent| {
            let Some(on_double_press) = on_double_press else {
                return;
            };
            if disabled.get_untracked() {
                return;
            }

            let Some(pointer_type) = last_pointer_type.get_value() else {
                tracing::warn!("no pointer type saved for dblclick");
                return;
            };

            let e = EventRef::Mouse(&e);
            let (propagation, propagation_state) = PropagationControl::new();
            let (x, y) = e.coordinates();
            let key = e.key();
            is_triggering_event.set_value(true);
            on_double_press.run(PressEvent {
                pointer_type,
                target: SendWrapper::new(e.current_target()),
                modifiers: e.modifiers(),
                x,
                y,
                key,
                propagation,
            });
            is_triggering_event.set_value(false);
            if !force_propagation && !propagation_state.load(Ordering::Acquire) {
                e.stop_propagation();
            }
        };

        // Prevent focus on mousedown when prevent_focus_on_press is enabled.
        let handle_mousedown = move |e: MouseEvent| {
            if !e.current_target_contains_target() || e.button() != 0 {
                return;
            }
            // Keep the focus where it is (react-aria's `preventFocus`, which, unlike
            // `preventDefault`, leaves text selection and dragging alone).
            if prevent_focus_on_press.get_untracked() {
                prevent_focus(e.target().and_then(|target| target.dyn_into().ok()));
            }
            if !force_propagation {
                e.stop_propagation();
            }
        };

        // Element-level pointerup handler: fires on_press_up for pointer-up events
        // over the element that didn't have a corresponding press-down (no active press state).
        let handle_element_pointer_up = move |e: PointerEvent| {
            let ev = e;
            let e = EventRef::Pointer(&ev);

            if !ev.current_target_contains_target() || saw_virtual_pointer_event.get_value() {
                return;
            }
            if ev.button() != 0 {
                return;
            }
            if disabled.get_untracked() {
                return;
            }
            // Only fire when there is no active press (the global pointerup handles active presses).
            if state.with_value(Option::is_some) {
                return;
            }
            if let Some(on_press_up) = on_press_up {
                let (x, y) = e.coordinates();
                // Not stopped: react-aria ignores whether this press up continued.
                let (propagation, _) = PropagationControl::new();
                is_triggering_event.set_value(true);
                on_press_up.run(PressEvent {
                    pointer_type: PointerType::from(ev.pointer_type()),
                    target: SendWrapper::new(e.current_target()),
                    modifiers: e.modifiers(),
                    x,
                    y,
                    key: None,
                    propagation,
                });
                is_triggering_event.set_value(false);
            }
        };

        // Only set aria-describedby when on_long_press is provided and a description is given.
        // Creates a hidden element and references it by ID, once mounted (react-aria's
        // `useDescription` uses a layout effect): the server renders none, so hydration agrees.
        // Disabled, the element can't be long-pressed: no description (as upstream's `useLongPress`).
        let has_long_press = on_long_press.is_some();
        let description_id = use_description(Signal::derive(move || {
            long_press_accessibility_description
                .get()
                .filter(|_| has_long_press && !disabled.get())
        }));
        let aria_describedby =
            Signal::derive(move || description_id.get().map(AriaDescribedby::element_with_id));

        // Cleanup on unmount: restore text selection, remove global listeners, and clear timeouts.
        on_cleanup(move || {
            state.update_value(|s| {
                if let Some(s) = s.as_mut() {
                    s.restore_text_selection_if_needed(
                        allow_text_selection_on_press.get_untracked(),
                    );
                    s.cleanup_event_handlers();
                    s.clear_click_timeout();
                    s.clear_long_press_timeout();
                }
            });
            state.set_value(None);
        });

        UsePressReturn {
            props: PropsWithStyles::new(
                UsePressProps {
                    on_keydown: EventHandler::new(handle_key_down),
                    on_click: EventHandler::new(handle_click),
                    on_pointerdown: EventHandler::new(handle_pointer_down),
                    on_pointerup: EventHandler::new(handle_element_pointer_up),
                    on_mousedown: EventHandler::new(handle_mousedown),
                    on_dragstart: EventHandler::new(handle_dragstart),
                    on_dblclick: EventHandler::new(handle_dblclick),
                    aria_describedby,
                },
                Styles::new().add(
                    TouchActionProperty.declare(TouchAction::Gestures(
                        TouchActionGestures::horizontal(TouchActionHorizontalPan::PanX)
                            .with_vertical(TouchActionVerticalPan::PanY)
                            .with_pinch_zoom(),
                    )),
                ),
            ),
            is_pressed: match force_is_pressed {
                Some(prop) => Signal::derive(move || is_pressed.get() || prop.get()),
                None => is_pressed.into(),
            },
        }
    }
}

/// Tests whether a keyboard event's default action should be presented when the given `key` was pressed.
fn should_prevent_default_keyboard(element: &web_sys::Element, key: &KeyboardKey) -> bool {
    // Don't prevent the context menu shortcut on macOS.
    if *key == KeyboardKey::Enter && device::is_mac() {
        return false;
    }

    if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
        // Enter on a checkbox or radio should submit the surrounding form (implicit submission),
        // not toggle the input.
        if *key == KeyboardKey::Enter && matches!(input.type_().as_str(), "checkbox" | "radio") {
            return false;
        }
        return !is_valid_input_key(input, key);
    }

    if element.is_instance_of::<web_sys::HtmlButtonElement>() {
        return match element.get_attribute("type") {
            Some(ty) => ty != "submit" && ty != "reset",
            None => false,
        };
    }

    !element.is_anchor_link()
}

const NON_TEXT_INPUT_TYPES: [&str; 9] = [
    "checkbox", "radio", "range", "color", "file", "image", "button", "submit", "reset",
];

fn is_valid_input_key(element: &HtmlInputElement, key: &KeyboardKey) -> bool {
    // Checkboxes and radio-buttons should only toggle with space, not enter.
    match element.get_attribute("type") {
        Some(ty) => match ty.as_str() {
            "checkbox" | "radio" => *key == KeyboardKey::Space,
            other => NON_TEXT_INPUT_TYPES.contains(&other),
        },
        None => true,
    }
}

/// Accessibility for keyboards. Space and Enter only.
#[allow(clippy::needless_pass_by_value)]
fn is_valid_keyboard_event(e: &KeyboardEvent, current_target: EventTarget) -> bool {
    let key = e.typed_key();
    let code = e.code();
    let resembles_press = matches!(key, KeyboardKey::Enter | KeyboardKey::Space) || code == "Space";

    if !resembles_press {
        return false;
    }

    match current_target.as_element() {
        Some(element) => {
            let is_input = element.is_instance_of::<HtmlInputElement>();
            let is_text_area = element.is_instance_of::<HtmlTextAreaElement>();
            let is_content_editable = element
                .dyn_ref::<HtmlElement>()
                .is_some_and(HtmlElement::is_content_editable);
            // Role-aware link detection: respect role overrides on anchors.
            // An `<a href role="button">` should be treated as a button, not a link.
            // React-aria checks: role === 'link' || (!role && isHTMLAnchorLink(element)).
            // We do NOT use element.is_link() here because it ignores role overrides.
            let role = element.get_attribute("role");
            let is_link =
                role.as_deref() == Some("link") || (role.is_none() && element.is_anchor_link());

            // Links should only trigger with Enter key
            !(is_text_area
                || is_content_editable
                || is_input
                    && !is_valid_input_key(element.unchecked_ref::<HtmlInputElement>(), &key)
                || is_link && key != KeyboardKey::Enter)
        }
        None => true,
    }
}
