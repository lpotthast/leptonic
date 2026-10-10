// Upstream: react-aria/src/interactions/usePress.ts @ 99e6102368
// Upstream: react-aria/src/interactions/useLongPress.ts @ 99e6102368
// Upstream: react-aria/src/interactions/context.ts @ 99e6102368
// Upstream: react-aria/test/interactions/usePress.test.js @ 99e6102368
// Upstream: react-aria/test/interactions/useLongPress.test.js @ 99e6102368

use std::time::Duration;

use leptos::{attr, attr::Attr, ev, prelude::*};
use send_wrapper::SendWrapper;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, KeyboardEvent, MouseEvent, PointerEvent};
#[cfg(not(feature = "ssr"))]
use web_sys::{HtmlButtonElement, HtmlElement, HtmlInputElement, HtmlTextAreaElement};

use crate::{
    CapturedElement, EventHandler, IntoAttrs, Modifiers, OnEvent, Propagation, PropsWithStyles,
    utils::{
        aria::{AriaExpanded, AriaHasPopup},
        key::KeyboardKey,
        keyboard_shortcut::KeyboardShortcuts,
        point::Point,
        pointer_type::PointerType,
        propagation_control::{PropagationControl, Sealed},
        styles::{
            Styles,
            css::{
                TouchAction, TouchActionGestures, TouchActionHorizontalPan, TouchActionVerticalPan,
            },
            property::TouchActionProperty,
        },
    },
};
#[cfg(not(feature = "ssr"))]
use crate::{
    ContainsTarget, EventModifiers,
    utils::{
        dom_ext::{ElementExt, EventAccessors, EventTargetExt, node_contains},
        event_listeners::{Listener, listen_to},
        focus::focus_element,
        key::KeyboardEventKey,
        open_link::{is_opening_link, open_link},
        platform::device,
        prevent_focus::{FocusPrevention, prevent_focus},
        shadow_dom,
        use_description::use_description,
        virtual_click::{is_virtual_click, is_virtual_pointer_event},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `propagation: PressPropagation` (`Stop`, `Continue`): links and anchor buttons let every press
//   event propagate, so client-side routers see their clicks. react-aria only has
//   `continuePropagation()` per event (offered too).
// - `force_is_pressed` is react-aria's `isPressed` (named apart from the returned `is_pressed`).
// - The configuration flags are signals; the long press description is a `MaybeProp`.
// - A `PressResponder`'s flags (`is_disabled`, `prevent_focus_on_press`, ...) are OR-ed with the
//   element's: either can switch them on. react-aria merges them (the element's own value wins), so
//   only an element explicitly passing `false` under a responder's `true` behaves differently;
//   a flag can't tell "unset" from `false` here (and the atoms above `use_press` take plain flags).
// - Long presses are part of `use_press`, as one optional group (`long_press: Option<LongPress>`:
//   callbacks, threshold, description, `is_disabled`): an element has one press handler, and
//   react-aria's `useLongPress` only wraps `usePress`. Without the group, no long press machinery
//   is set up. As upstream, `LongPressEvent` has no `continuePropagation` (no `Propagation`).
// - Press events carry their kind (`PressEventKind`, react-aria's `type`), the pressed element as
//   an `Element` and the position as a `Point` relative to it (react-aria: `x`, `y`).
//
// ## DIFFERENT BEHAVIOR
// - `touch-action: pan-x pan-y pinch-zoom` is an inline style of the pressable element, rendered
//   on the server too (react-aria injects a global `<style>` for `[data-react-aria-pressable]`):
//   no runtime DOM changes, no marker attribute.
// - Dragging out of and back into the element is tracked with `pointerenter`/`pointerleave`
//   listeners added to the element while pressed (react-aria: the element's `onPointerEnter`/
//   `onPointerLeave` props), so the props carry no handlers for them.
// - A press of a disabled element starts no press state (react-aria records one that triggers
//   nothing, which a disabled native button, firing pointer events but no click, would leave
//   stuck). Its pointer downs, key downs and clicks (prevented) all propagate. Upstream's do
//   while no press is recorded, and its clicks of a recorded pointer press and virtual clicks do
//   too. Upstream stops the rest: key repeats and the native click during the recorded keyboard
//   press (Enter on an `aria-disabled` submit button or link).
// - A native click following Space's keyup on an input or submit/reset button completes the
//   keyboard activation without firing a second virtual press. The default action still runs
//   (e.g. toggling a checkbox); upstream's synthetic keyboard tests do not send this native click.
//
// ## ADDITIONS
// - `on_double_press` (the native `dblclick`): any pressable can take double presses without a
//   selection model (react-aria handles double clicks in `useSelectableItem` only).
//
// ## OMITTED FEATURES
// - The `onClick` compatibility callback (for third-party libraries passing `onClick` instead of
//   `onPress`): leptonic's press callbacks are the only interface.
// - The mouse and touch event fallbacks for environments without `PointerEvent` (react-aria uses
//   them in tests only): every supported browser has pointer events.
// - `useLongPress`'s `pointerType` option (long presses of one pointer type only): no caller
//   needs it; long presses are those of a mouse or touch, as upstream's default.
//
// =============================================================================

/// The default long press threshold.
pub const DEFAULT_LONG_PRESS_THRESHOLD: Duration = Duration::from_millis(500);

/// Which press callback an event is for (react-aria's `PressEvent.type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressEventKind {
    /// The press started (`on_press_start`).
    PressStart,
    /// The press ended, pressed or not (`on_press_end`).
    PressEnd,
    /// The pointer or key was released over the element (`on_press_up`).
    PressUp,
    /// The element was pressed (`on_press`).
    Press,
    /// The element was double-clicked (`on_double_press`).
    DoublePress,
}

/// Which long press callback an event is for (react-aria's `LongPressEvent.type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LongPressEventKind {
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
    /// Which long press callback the event is for.
    pub kind: LongPressEventKind,

    /// The pointer type that triggered the long press event (`Mouse` or `Touch`).
    pub pointer_type: PointerType,

    /// The long-pressed element.
    pub target: SendWrapper<web_sys::Element>,

    /// States which modifier keys were held during the long press event.
    pub modifiers: Modifiers,

    /// Where the long press started, relative to the target's top left corner, in CSS pixels.
    pub point: Point,
}

/// A press interaction (react-aria's `PressEvent`).
#[derive(Debug, Clone)]
pub struct PressEvent {
    /// Which press callback the event is for.
    pub kind: PressEventKind,

    /// The pointer type that triggered the press event.
    pub pointer_type: PointerType,

    /// The pressed element.
    pub target: SendWrapper<web_sys::Element>,

    /// States which modifier keys were held during the press event.
    pub modifiers: Modifiers,

    /// The position relative to the target's top left corner, in CSS pixels: the pointer's, or the
    /// target's center for keyboard and other presses without a pointer position.
    pub point: Point,

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

/// Long press handling of a pressable element (react-aria's `useLongPress`): a mouse or touch press
/// held for `threshold` fires `on_long_press` and cancels the element's press (no `on_press`, and
/// the click after the release is prevented, so a long-pressed link doesn't navigate).
#[derive(Debug, Clone, Copy)]
pub struct LongPress {
    /// Called when a long press interaction starts (a mouse or touch press starts).
    pub on_long_press_start: Option<Callback<LongPressEvent>>,
    /// Called when the press was held for `threshold`.
    pub on_long_press: Option<Callback<LongPressEvent>>,
    /// Called when a long press interaction ends, long-pressed or not.
    pub on_long_press_end: Option<Callback<LongPressEvent>>,
    /// How long a press must be held. Default: [`DEFAULT_LONG_PRESS_THRESHOLD`] (500 ms).
    pub threshold: Signal<Duration>,
    /// Describes the long press action to assistive technology, e.g. "Long press to open menu".
    /// Only applied with an `on_long_press` handler.
    pub accessibility_description: MaybeProp<String>,
    /// Turns long presses off while `true` (e.g. while a collection item has no long press
    /// action), so presses aren't cancelled after the threshold.
    pub is_disabled: Signal<bool>,
}

impl Default for LongPress {
    /// No callbacks, the default threshold, no description, enabled.
    fn default() -> Self {
        Self {
            on_long_press_start: None,
            on_long_press: None,
            on_long_press_end: None,
            threshold: Signal::stored(DEFAULT_LONG_PRESS_THRESHOLD),
            accessibility_description: MaybeProp::default(),
            is_disabled: Signal::stored(false),
        }
    }
}

/// Merges two long press groups on one element (e.g. a responder's and the element's own), as
/// two `useLongPress` hooks on one element in react-aria: each group's callbacks run only while
/// that group is enabled (`first`'s before `second`'s), and the merged group is disabled while
/// both are. The threshold is `second`'s unless it is disabled; the description is that of an
/// enabled group with an `on_long_press` handler (`second`'s first).
pub(crate) fn merge_long_press(
    first: Option<LongPress>,
    second: Option<LongPress>,
) -> Option<LongPress> {
    fn while_enabled<T: Clone + Send + Sync + 'static>(
        callback: Option<Callback<T>>,
        is_disabled: Signal<bool>,
    ) -> Option<Callback<T>> {
        callback.map(|callback| {
            Callback::new(move |e: T| {
                if !is_disabled.get_untracked() {
                    callback.run(e);
                }
            })
        })
    }

    match (first, second) {
        (Some(first), Some(second)) => Some(LongPress {
            on_long_press_start: chain_optional_callbacks(
                while_enabled(first.on_long_press_start, first.is_disabled),
                while_enabled(second.on_long_press_start, second.is_disabled),
            ),
            on_long_press: chain_optional_callbacks(
                while_enabled(first.on_long_press, first.is_disabled),
                while_enabled(second.on_long_press, second.is_disabled),
            ),
            on_long_press_end: chain_optional_callbacks(
                while_enabled(first.on_long_press_end, first.is_disabled),
                while_enabled(second.on_long_press_end, second.is_disabled),
            ),
            threshold: Signal::derive(move || {
                if second.is_disabled.get() {
                    first.threshold.get()
                } else {
                    second.threshold.get()
                }
            }),
            accessibility_description: MaybeProp::derive(move || {
                [second, first]
                    .into_iter()
                    .filter(|group| group.on_long_press.is_some() && !group.is_disabled.get())
                    .find_map(|group| group.accessibility_description.get())
            }),
            is_disabled: Signal::derive(move || {
                first.is_disabled.get() && second.is_disabled.get()
            }),
        }),
        (first, second) => first.or(second),
    }
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

    /// While `true`, the returned `is_pressed` is `true` too: the pressed appearance forced from
    /// outside (e.g. while the overlay the element triggers is open). react-aria's `isPressed`.
    pub force_is_pressed: Signal<bool>,

    pub on_press: Option<Callback<PressEvent>>,
    pub on_press_up: Option<Callback<PressEvent>>,
    pub on_press_start: Option<Callback<PressEvent>>,
    pub on_press_end: Option<Callback<PressEvent>>,

    /// Called when the press state changes. Receives `true` when press starts,
    /// `false` when press ends.
    pub on_press_change: Option<Callback<bool>>,

    /// Called when the element receives a native `dblclick` event.
    pub on_double_press: Option<Callback<PressEvent>>,

    /// Long press handling. `None`: no long presses (and none of their machinery).
    pub long_press: Option<LongPress>,
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
            force_is_pressed: Signal::stored(false),
            on_press: None,
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
            on_press_change: None,
            on_double_press: None,
            long_press: None,
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
    /// Long press handling for the pressable element (a `MenuTrigger` opening on a long press),
    /// merged with the element's own.
    pub long_press: Option<LongPress>,
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
    /// The trigger element's id, known while rendering: generated by the trigger, which the
    /// element takes unless it has its own; an element with its own id writes it here
    /// (react-aria merges both ids, `mergeIds`). References to the trigger (a disclosure panel's
    /// `aria-labelledby`, an untitled dialog's) read it. `use_button` takes or writes it while
    /// rendering; other pressables (`Pressable`, `use_link`) synchronize it once mounted.
    pub id: RwSignal<String>,
}

impl PressResponderTrigger {
    /// For a pressable that doesn't render its id (`Pressable`, `use_link`): once its element is
    /// mounted, it gets the trigger's id unless it has its own, which then becomes the trigger's.
    pub(crate) fn sync_id_once_mounted(self, element: CapturedElement) {
        let id = self.id;
        Effect::new(move || {
            if let Some(el) = element.get() {
                let own = el.id();
                if own.is_empty() {
                    el.set_id(&id.get_untracked());
                } else if id.with_untracked(|id| *id != own) {
                    id.set(own);
                }
            }
        });
    }

    /// The element's id for a pressable rendering its id (`use_button`): its own (which becomes
    /// the trigger's), else the trigger's.
    pub(crate) fn element_id(self, own: Option<String>) -> String {
        match own {
            Some(own) => {
                if self.id.with_untracked(|id| *id != own) {
                    self.id.set(own.clone());
                }
                own
            }
            None => self.id.get_untracked(),
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
            long_press: None,
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
pub(crate) fn chain_optional_callbacks<T: Clone + Send + Sync + 'static>(
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

/// The handlers and attributes of `use_press` for the pressable element.
///
/// [`UsePressReturn::props`] wraps them with the element's styles: `into_parts()` gives the
/// attributes to spread (`<div {..attrs}>`) and the styles to apply; `into_inner()` gives these
/// props, e.g. to chain a handler of the element's own before spreading them with
/// [`into_attrs()`](IntoAttrs::into_attrs).
///
/// ```ignore
/// let press = use_press(input);
/// let (attrs, styles) = press.props.into_parts();
/// view! { <button {..attrs} style=styles>"Click"</button> }
///
/// // A key handler of the element's own before the press handling.
/// let (props, styles) = use_press(input).props.into_inner();
/// let on_keydown = own_keydown.chain(props.on_keydown).into_on(ev::keydown);
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
    /// The long press description's id, with a long press group having an `on_long_press`
    /// handler and a description (once mounted). `None` otherwise.
    pub aria_describedby: Signal<Option<String>>,
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
    OnEvent<ev::keydown>,
    OnEvent<ev::click>,
    OnEvent<ev::pointerdown>,
    OnEvent<ev::mousedown>,
    OnEvent<ev::pointerup>,
    OnEvent<ev::dragstart>,
    OnEvent<ev::dblclick>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
);

/// The press in progress (react-aria's `PressState` while `isPressed`).
#[cfg(not(feature = "ssr"))]
struct PressState {
    pointer_id: i32,
    pointer_type: PointerType,

    /// The pressed element (the element this press hook was bound to).
    target: web_sys::Element,

    is_over_target: bool,

    /// Whether `trigger_press_start` fired (so `trigger_press_end` fires once per start).
    did_fire_press_start: bool,

    /// Whether the press start was a long press start (so its end fires `on_long_press_end`).
    is_long_press: bool,

    /// Handle for the 80ms fallback timeout that triggers `target.click()`
    /// when iOS long press doesn't naturally fire a click event.
    click_timeout_handle: Option<TimeoutHandle>,

    /// The long press threshold timer.
    long_press_timeout_handle: Option<TimeoutHandle>,

    /// The press's listeners (on the document, and `pointerenter`/`pointerleave` on the element),
    /// removed when dropped.
    listeners: Vec<Listener>,
}

/// The DOM event behind a press callback.
#[cfg(not(feature = "ssr"))]
enum EventRef<'a> {
    Pointer(&'a PointerEvent),
    Keyboard(&'a KeyboardEvent),
    Mouse(&'a MouseEvent),
    /// A press that ends without a DOM event, e.g. because the element became disabled while
    /// pressed. It has no modifiers, pointer position or key.
    Synthetic,
}

#[cfg(not(feature = "ssr"))]
impl EventRef<'_> {
    fn modifiers(&self) -> Modifiers {
        match self {
            EventRef::Pointer(e) => e.modifiers(),
            EventRef::Keyboard(e) => e.modifiers(),
            EventRef::Mouse(e) => e.modifiers(),
            EventRef::Synthetic => Modifiers::default(),
        }
    }

    fn stop_propagation(&self) {
        match self {
            EventRef::Pointer(e) => e.stop_propagation(),
            EventRef::Keyboard(e) => e.stop_propagation(),
            EventRef::Mouse(e) => e.stop_propagation(),
            EventRef::Synthetic => {}
        }
    }

    /// The keyboard key that triggered this event, if any.
    fn key(&self) -> Option<KeyboardKey> {
        match self {
            EventRef::Keyboard(e) => Some(e.typed_key()),
            EventRef::Pointer(_) | EventRef::Mouse(_) | EventRef::Synthetic => None,
        }
    }

    /// The pointer position in the viewport, if the event has one.
    fn client_point(&self) -> Option<Point> {
        match self {
            EventRef::Pointer(e) => Some(Point::new(e.client_x(), e.client_y())),
            EventRef::Mouse(e) => Some(Point::new(e.client_x(), e.client_y())),
            EventRef::Keyboard(_) | EventRef::Synthetic => None,
        }
    }

    /// The press position relative to `target` (react-aria's `PressEvent` `x`/`y`): the pointer's,
    /// or the target's center without one (keyboard and synthetic events).
    fn point_in(&self, target: &web_sys::Element) -> Point {
        let rect = target.get_bounding_client_rect();
        match self.client_point() {
            Some(client) => Point::new(client.x - rect.left(), client.y - rect.top()),
            None => Point::new(rect.width() / 2.0, rect.height() / 2.0),
        }
    }
}

#[cfg(not(feature = "ssr"))]
thread_local! {
    /// The last key up that opened a link: several press hooks on one link (react-aria's
    /// `LINK_CLICKED` marker) open it once.
    static LINK_OPENING_KEY_UP: std::cell::RefCell<Option<KeyboardEvent>> =
        const { std::cell::RefCell::new(None) };
}

/// The pressable element's style: `touch-action: pan-x pan-y pinch-zoom` (react-aria's
/// `[data-react-aria-pressable] { touch-action: pan-x pan-y pinch-zoom }`; no double-tap zoom delay).
fn press_styles() -> Styles {
    Styles::new().add(
        TouchActionProperty.declare(TouchAction::Gestures(
            TouchActionGestures::horizontal(TouchActionHorizontalPan::PanX)
                .with_vertical(TouchActionVerticalPan::PanY)
                .with_pinch_zoom(),
        )),
    )
}

/// Merges a responder's and the element's `force_is_pressed`: either forces the pressed
/// appearance.
fn merge_force_is_pressed(responder: Option<Signal<bool>>, own: Signal<bool>) -> Signal<bool> {
    match responder {
        Some(responder) => Signal::derive(move || responder.get() || own.get()),
        None => own,
    }
}

/// Handles press interactions across mouse, touch, keyboard and screen readers (react-aria's
/// `usePress`), and long presses (react-aria's `useLongPress`, with
/// [`long_press`](UsePressInput::long_press)).
#[allow(clippy::too_many_lines)]
pub fn use_press(input: UsePressInput) -> UsePressReturn {
    #[cfg(feature = "ssr")]
    {
        // The responder's forced state too, so `data-pressed` agrees with the client.
        let is_pressed = merge_force_is_pressed(
            use_context::<PressResponderContext>().and_then(|c| c.force_is_pressed),
            input.force_is_pressed,
        );
        UsePressReturn {
            props: PropsWithStyles::new(
                UsePressProps {
                    on_keydown: EventHandler::empty(),
                    on_click: EventHandler::empty(),
                    on_pointerdown: EventHandler::empty(),
                    on_pointerup: EventHandler::empty(),
                    on_mousedown: EventHandler::empty(),
                    on_dragstart: EventHandler::empty(),
                    on_dblclick: EventHandler::empty(),
                    // Set after mount on the client, as on the client before hydration.
                    aria_describedby: Signal::stored(None),
                },
                press_styles(),
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
            long_press,
        } = input;

        // --- PressResponderContext merging ---
        // If a parent component provided a PressResponderContext (via PressResponder or
        // provide_context), merge its props with our local input. This allows parent components
        // to inject press behavior into child pressable elements.
        let ctx = use_context::<PressResponderContext>();
        if let Some(ref ctx) = ctx {
            ctx.registered.set_value(true);
        }

        // Merge boolean config fields: OR semantics, so either the context or the input can turn
        // them on.
        let or = |own: Signal<bool>, ctx: Option<Signal<bool>>| match ctx {
            Some(ctx) => Signal::derive(move || own.get() || ctx.get()),
            None => own,
        };
        let disabled = or(disabled, ctx.as_ref().and_then(|c| c.is_disabled));
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
        let long_press = merge_long_press(ctx.as_ref().and_then(|c| c.long_press), long_press);
        // --- End PressResponderContext merging ---

        let (is_pressed, set_is_pressed) = signal(false);

        // The press in progress (react-aria's `state` while `isPressed`). Never borrowed while a
        // user callback runs: callbacks may start or end presses themselves.
        let state: StoredValue<Option<PressState>, LocalStorage> = StoredValue::new_local(None);

        // Tracks the pointer type from the most recently completed press,
        // so the dblclick handler can construct a PressEvent after state has been cleared.
        let last_pointer_type: StoredValue<Option<PointerType>, LocalStorage> =
            StoredValue::new_local(None);

        // Re-entrancy guard: prevents infinite loops when a press callback
        // synchronously triggers a click event on the same element.
        let is_triggering_event: StoredValue<bool, LocalStorage> = StoredValue::new_local(false);

        // Whether the last pointer down was a virtual one (e.g. VoiceOver on iOS, react-aria's
        // `state.pointerType === 'virtual'`): its click presses, its pointer up doesn't.
        let saw_virtual_pointer_event: StoredValue<bool, LocalStorage> =
            StoredValue::new_local(false);

        // A native activation click can follow Space's keyup, after its press state ended.
        // Its propagation follows the completed keyboard press instead of starting a virtual one.
        let keyboard_click_stop: StoredValue<Option<bool>, LocalStorage> =
            StoredValue::new_local(None);

        // Tracks meta key events for macOS workaround
        let meta_key_events: StoredValue<
            Option<std::collections::HashMap<KeyboardKey, KeyboardEvent>>,
            LocalStorage,
        > = StoredValue::new_local(None);

        // The long press listeners (react-aria's `useLongPress` global listeners): blocking the
        // touch context menu and the click after a long press, kept until 100 ms after the
        // pointer up, as the menu or the click may come after it.
        let long_press_listeners: StoredValue<Vec<Listener>, LocalStorage> =
            StoredValue::new_local(Vec::new());

        // `prevent_focus_on_press`' focus preventions of the press, disposed when it ends
        // (react-aria's `state.disposables`).
        let focus_preventions: StoredValue<Vec<FocusPrevention>, LocalStorage> =
            StoredValue::new_local(Vec::new());

        let fire = move |callback: Option<Callback<PressEvent>>,
                         kind: PressEventKind,
                         pointer_type: PointerType,
                         target: &web_sys::Element,
                         e: &EventRef<'_>|
              -> bool {
            // Without a callback, the event stops (react-aria's `shouldStopPropagation` defaults
            // to `true`).
            let Some(callback) = callback else {
                return true;
            };
            let propagation = PropagationControl::new();
            is_triggering_event.set_value(true);
            callback.run(PressEvent {
                kind,
                pointer_type,
                target: SendWrapper::new(target.clone()),
                modifiers: e.modifiers(),
                point: e.point_in(target),
                key: e.key(),
                propagation: propagation.clone(),
            });
            is_triggering_event.set_value(false);
            propagation.is_propagation_stopped()
        };
        let fire_long = move |callback: Option<Callback<LongPressEvent>>,
                              kind: LongPressEventKind,
                              pointer_type: PointerType,
                              target: &web_sys::Element,
                              modifiers: Modifiers,
                              point: Point| {
            if let Some(callback) = callback {
                is_triggering_event.set_value(true);
                callback.run(LongPressEvent {
                    kind,
                    pointer_type,
                    target: SendWrapper::new(target.clone()),
                    modifiers,
                    point,
                });
                is_triggering_event.set_value(false);
            }
        };

        // Listens on `target` for the next `event` only (react-aria: `{once: true}`), preventing
        // its default, until the long press listeners are cleared.
        let prevent_next = move |target: &web_sys::Element, event: &'static str| {
            let prevented = std::cell::Cell::new(false);
            let listener = crate::utils::event_listeners::listen(
                target,
                event,
                false,
                move |e: web_sys::Event| {
                    if !prevented.replace(true) {
                        e.prevent_default();
                    }
                },
            );
            long_press_listeners.update_value(|listeners| listeners.push(listener));
        };

        // The long press threshold passed (react-aria's `useLongPress` timeout).
        let long_press_reached = move |pointer_type: PointerType,
                                       target: web_sys::Element,
                                       modifiers: Modifiers,
                                       point: Point| {
            // Cancel this and every other press of the element (they listen on the document).
            let init = web_sys::PointerEventInit::new();
            init.set_bubbles(true);
            if let Ok(cancel) = PointerEvent::new_with_event_init_dict("pointercancel", &init) {
                let _ = target.dispatch_event(&cancel);
            }
            // Prevent the click after the release (a long-pressed link mustn't navigate).
            prevent_next(&target, "click");
            // Touch devices focus on the pointer up, which a long press has none of.
            let is_focused = target
                .owner_document()
                .and_then(|document| shadow_dom::get_active_element(&document))
                .is_some_and(|active| active == target);
            if !is_focused {
                focus_element(&target, true);
            }
            if let Some(long_press) = long_press {
                fire_long(
                    long_press.on_long_press,
                    LongPressEventKind::LongPress,
                    pointer_type,
                    &target,
                    modifiers,
                    point,
                );
            }
        };

        let long_press_accepts = move |pointer_type: PointerType| {
            long_press.is_some_and(|long_press| !long_press.is_disabled.get_untracked())
                && matches!(pointer_type, PointerType::Mouse | PointerType::Touch)
        };

        // The triggers return whether the event should stop propagating (react-aria's
        // `shouldStopPropagation`): unless a callback continued it. The handlers stop it.

        // Starts the press of the press state (once per start: again only after its end).
        let trigger_press_start = move |e: &EventRef<'_>| -> bool {
            if disabled.get_untracked() {
                return false;
            }
            let Some((pointer_type, target)) = state
                .try_update_value(|s| {
                    s.as_mut().filter(|s| !s.did_fire_press_start).map(|s| {
                        s.did_fire_press_start = true;
                        (s.pointer_type, s.target.clone())
                    })
                })
                .flatten()
            else {
                return false;
            };

            // Long press start first: react-aria's `useLongPress` handlers precede `usePress`'.
            if let Some(long_press) = long_press
                && long_press_accepts(pointer_type)
            {
                let modifiers = e.modifiers();
                let point = e.point_in(&target);
                fire_long(
                    long_press.on_long_press_start,
                    LongPressEventKind::LongPressStart,
                    pointer_type,
                    &target,
                    modifiers,
                    point,
                );
                let timeout_target = target.clone();
                let handle = set_timeout_with_handle(
                    move || long_press_reached(pointer_type, timeout_target, modifiers, point),
                    long_press.threshold.get_untracked(),
                )
                .ok();
                state.update_value(|s| {
                    if let Some(s) = s.as_mut() {
                        s.is_long_press = true;
                        s.long_press_timeout_handle = handle;
                    }
                });
                // Touch devices may open a context menu on a long press.
                if pointer_type == PointerType::Touch {
                    prevent_next(&target, "contextmenu");
                }
                // The blockers stay until 100 ms after the pointer up: the context menu or click
                // may come after it.
                if let Some(window) = target.owner_document().and_then(|d| d.default_view()) {
                    let done = std::cell::Cell::new(false);
                    let listener =
                        listen_to(&window, ev::pointerup, false, move |_: PointerEvent| {
                            if !done.replace(true) {
                                set_timeout(
                                    move || {
                                        long_press_listeners.try_update_value(Vec::clear);
                                    },
                                    Duration::from_millis(100),
                                );
                            }
                        });
                    long_press_listeners.update_value(|listeners| listeners.push(listener));
                }
            }

            let should_stop = fire(
                on_press_start,
                PressEventKind::PressStart,
                pointer_type,
                &target,
                e,
            );
            if let Some(on_press_change) = on_press_change {
                is_triggering_event.set_value(true);
                on_press_change.run(true);
                is_triggering_event.set_value(false);
            }
            set_is_pressed.set(true);
            should_stop
        };

        // Ends the press of the press state (if it started), pressing it if `was_pressed`.
        let trigger_press_end = move |e: &EventRef<'_>, was_pressed: bool| -> bool {
            let Some((pointer_type, target, was_long_press, long_press_timeout)) = state
                .try_update_value(|s| {
                    s.as_mut().filter(|s| s.did_fire_press_start).map(|s| {
                        s.did_fire_press_start = false;
                        (
                            s.pointer_type,
                            s.target.clone(),
                            std::mem::take(&mut s.is_long_press),
                            s.long_press_timeout_handle.take(),
                        )
                    })
                })
                .flatten()
            else {
                return false;
            };
            if let Some(timeout) = long_press_timeout {
                timeout.clear();
            }

            // Long press end first: react-aria's `useLongPress` handlers precede `usePress`'.
            if was_long_press && let Some(long_press) = long_press {
                fire_long(
                    long_press.on_long_press_end,
                    LongPressEventKind::LongPressEnd,
                    pointer_type,
                    &target,
                    e.modifiers(),
                    e.point_in(&target),
                );
            }

            let mut should_stop = fire(
                on_press_end,
                PressEventKind::PressEnd,
                pointer_type,
                &target,
                e,
            );
            if let Some(on_press_change) = on_press_change {
                is_triggering_event.set_value(true);
                on_press_change.run(false);
                is_triggering_event.set_value(false);
            }
            set_is_pressed.set(false);

            if was_pressed && !disabled.get_untracked() {
                should_stop &= fire(on_press, PressEventKind::Press, pointer_type, &target, e);
            }
            should_stop
        };

        let trigger_press_up = move |e: &EventRef<'_>| -> bool {
            if disabled.get_untracked() {
                return false;
            }
            let Some((pointer_type, target)) =
                state.with_value(|s| s.as_ref().map(|s| (s.pointer_type, s.target.clone())))
            else {
                return false;
            };
            fire(
                on_press_up,
                PressEventKind::PressUp,
                pointer_type,
                &target,
                e,
            )
        };

        let stop_unless_forced = move |should_stop: bool, e: &EventRef<'_>| {
            if should_stop && !force_propagation {
                e.stop_propagation();
            }
        };

        // Starts a press state (react-aria's `state.isPressed = true` with its target).
        let initialize_press_state =
            move |pointer_id: i32,
                  pointer_type: PointerType,
                  target: web_sys::Element,
                  is_over_target: bool,
                  listeners: Vec<Listener>| {
                state.set_value(Some(PressState {
                    pointer_id,
                    pointer_type,
                    target,
                    is_over_target,
                    did_fire_press_start: false,
                    is_long_press: false,
                    click_timeout_handle: None,
                    long_press_timeout_handle: None,
                    listeners,
                }));
            };

        // Ends the press state without pressing (react-aria's `cancel`), also cleaning up after
        // a press the click completed.
        let cancel = move |e: &EventRef<'_>| {
            trigger_press_end(e, false);
            let Some(s) = state.try_update_value(Option::take).flatten() else {
                return;
            };
            if let Some(handle) = s.click_timeout_handle {
                handle.clear();
            }
            if let Some(handle) = s.long_press_timeout_handle {
                handle.clear();
            }
            if !allow_text_selection_on_press.get_untracked() {
                s.target.restore_text_selection();
            }
            // Dropping the state removes its listeners.
            drop(s.listeners);
            for prevention in focus_preventions
                .try_update_value(std::mem::take)
                .unwrap_or_default()
            {
                prevention.dispose();
            }
        };

        // Cancel an active press when the element becomes disabled. The events that would
        // normally end the press are ignored while disabled, so without this, e.g. a spin button
        // whose first step disables it would keep spinning.
        Effect::new(move |_| {
            if disabled.get() && state.with_value(Option::is_some) {
                cancel(&EventRef::Synthetic);
            }
        });

        let handle_key_up = move |e: KeyboardEvent| {
            let key = e.typed_key();
            // The pressed element (the listener is on the document).
            let Some(press_target) = state.with_value(|s| s.as_ref().map(|s| s.target.clone()))
            else {
                return;
            };
            if disabled.get_untracked() || !is_valid_keyboard_event(&e, &press_target) {
                // macOS fires no key up for keys released while Meta is held: when Meta itself is
                // released, act as if the keys pressed meanwhile were released too, with key ups
                // on the pressed element. Dispatched after this key up's dispatch (dispatching a
                // key up from this key up listener would re-enter it).
                if key == KeyboardKey::Meta
                    && let Some(events) = meta_key_events.try_update_value(Option::take).flatten()
                    && !events.is_empty()
                {
                    let press_target = SendWrapper::new(press_target);
                    let events = SendWrapper::new(events.into_values().collect::<Vec<_>>());
                    queue_microtask(move || {
                        for event in events.iter() {
                            if let Some(key_up) = key_up_like(event) {
                                let _ = press_target.dispatch_event(&key_up);
                            }
                        }
                    });
                }
                return;
            }

            let target = shadow_dom::get_event_target(&e).unwrap_or_else(|| e.expect_target());
            if target
                .as_element()
                .is_some_and(|el| should_prevent_default_keyboard(el, &key))
            {
                e.prevent_default();
            }

            // Whether the key up happened on the pressed element (focus may have moved since the
            // key down): only then press up and press. A document listener: nothing to stop
            // (react-aria).
            let was_pressed = node_contains(Some(press_target.as_ref()), target.as_node().as_ref())
                .unwrap_or(false);
            let stop_up = !was_pressed || e.repeat() || trigger_press_up(&EventRef::Keyboard(&e));
            let stop_end = trigger_press_end(&EventRef::Keyboard(&e), was_pressed);
            // Dropping the state removes the key up listener.
            state.try_update_value(Option::take);

            if was_pressed && e.is_trusted() && key == KeyboardKey::Space && !e.default_prevented()
            {
                keyboard_click_stop.set_value(Some(stop_up && stop_end));
                // The default activation belongs to this event's task. A later listener may
                // prevent it, so do not leave a flag that could swallow a screen reader click.
                set_timeout(
                    move || {
                        keyboard_click_stop.try_update_value(Option::take);
                    },
                    Duration::ZERO,
                );
            }

            // A link pressed with a key other than Enter has a role override (only Enter follows a
            // link natively): open it ourselves, once for all press hooks on the link.
            if key != KeyboardKey::Enter && was_pressed && press_target.is_anchor_link() {
                let first = LINK_OPENING_KEY_UP.with_borrow_mut(|opened| {
                    if opened.as_ref() == Some(&e) {
                        false
                    } else {
                        *opened = Some(e.clone());
                        true
                    }
                });
                if first {
                    open_link(&press_target, e.modifiers());
                }
            }
            meta_key_events.update_value(|events| {
                if let Some(events) = events {
                    events.remove(&key);
                }
            });
        };

        let handle_key_down = move |e: KeyboardEvent| {
            if !e.current_target_contains_target() {
                return;
            }
            let Some(current_target) = e.expect_current_target().to_element() else {
                return;
            };
            let key = e.typed_key();

            if is_valid_keyboard_event(&e, &current_target) {
                if e.expect_target()
                    .as_element()
                    .is_some_and(|el| should_prevent_default_keyboard(el, &key))
                {
                    e.prevent_default();
                }

                // Only the first key down starts a press, not repeats (the press may have started
                // on another element before focus moved here). Repeats and keys of an active
                // press stop (react-aria); a disabled element lets them propagate.
                let mut should_stop = !disabled.get_untracked();
                if state.with_value(Option::is_none) && !disabled.get_untracked() && !e.repeat() {
                    // Capturing (as react-aria): a keyup handler that stops propagation (e.g.
                    // `use_keyboard` on this element or a child) must not leave the press stuck.
                    let listeners = current_target
                        .owner_document()
                        .map(|doc| listen_to(&doc, ev::keyup, true, handle_key_up))
                        .into_iter()
                        .collect();
                    initialize_press_state(
                        0,
                        PointerType::Keyboard,
                        current_target,
                        false,
                        listeners,
                    );
                    should_stop = trigger_press_start(&EventRef::Keyboard(&e));
                }
                stop_unless_forced(should_stop, &EventRef::Keyboard(&e));

                // Keys pressed while Meta is held on macOS, which fires no key up for them (also
                // during an active press).
                if device::is_mac() && e.meta_key() && key != KeyboardKey::Meta {
                    meta_key_events.update_value(|map| {
                        if let Some(map) = map {
                            map.insert(key, e.clone());
                        }
                    });
                }
            } else if key == KeyboardKey::Meta {
                // Initialize the map when Meta key itself is pressed.
                meta_key_events.set_value(Some(std::collections::HashMap::new()));
            }
        };

        let handle_click = move |e: MouseEvent| {
            // Re-entrancy guard: a press callback that clicks the element synchronously doesn't
            // press it again.
            if is_triggering_event.get_value() || !e.current_target_contains_target() {
                return;
            }

            // Only the primary button presses (react-aria), and not the click `open_link`
            // dispatches to follow a link this press opened.
            if e.button() != 0 || is_opening_link() {
                return;
            }

            // A disabled element's click does nothing and propagates. react-aria triggers its
            // (inert) press callbacks for the click, which therefore stop nothing: that of the
            // pointer press it recorded on pointer down, or a virtual press.
            if disabled.get_untracked() {
                e.prevent_default();
                saw_virtual_pointer_event.set_value(false);
                return;
            }

            if e.is_trusted()
                && is_virtual_click(&e)
                && let Some(should_stop) =
                    keyboard_click_stop.try_update_value(Option::take).flatten()
            {
                stop_unless_forced(should_stop, &EventRef::Mouse(&e));
                return;
            }

            let active_pointer_type = state.with_value(|s| s.as_ref().map(|s| s.pointer_type));
            // As react-aria: stopped unless the triggered press callbacks continued it.
            let mut should_stop = true;
            match active_pointer_type {
                // A pointer press completes with its click (after the pointer up, for DOM
                // mutation safety).
                Some(pointer_type)
                    if pointer_type != PointerType::Keyboard
                        && pointer_type != PointerType::Virtual =>
                {
                    last_pointer_type.set_value(Some(pointer_type));
                    let stop_up = trigger_press_up(&EventRef::Mouse(&e));
                    let stop_end = trigger_press_end(&EventRef::Mouse(&e), true);
                    should_stop = stop_up && stop_end;
                    cancel(&EventRef::Mouse(&e));
                }
                // A click from a screen reader or `element.click()` (also after a virtual pointer
                // down, e.g. VoiceOver on iOS): a whole press, as a keyboard click. Not while
                // another press is active.
                None if saw_virtual_pointer_event.get_value() || is_virtual_click(&e) => {
                    saw_virtual_pointer_event.set_value(false);
                    if let Some(target) = e.expect_current_target().to_element() {
                        last_pointer_type.set_value(Some(PointerType::Virtual));
                        initialize_press_state(0, PointerType::Virtual, target, false, Vec::new());
                        let stop_start = trigger_press_start(&EventRef::Mouse(&e));
                        let stop_up = trigger_press_up(&EventRef::Mouse(&e));
                        let stop_end = trigger_press_end(&EventRef::Mouse(&e), true);
                        should_stop = stop_start && stop_up && stop_end;
                        state.try_update_value(Option::take);
                    }
                }
                _ => {}
            }
            stop_unless_forced(should_stop, &EventRef::Mouse(&e));
        };

        // Dragging out of and back into the element (react-aria's `onPointerEnter`/
        // `onPointerLeave`, listened to on the element while pressed): leaving ends the press
        // without pressing (or cancels it for good with `should_cancel_on_pointer_exit`),
        // re-entering starts it again. Stops nothing (react-aria).
        let handle_pointer_enter = move |e: PointerEvent| {
            let entered = state
                .try_update_value(|s| match s.as_mut() {
                    Some(s) if e.pointer_id() == s.pointer_id && !s.is_over_target => {
                        s.is_over_target = true;
                        true
                    }
                    _ => false,
                })
                .unwrap_or(false);
            if entered {
                trigger_press_start(&EventRef::Pointer(&e));
            }
        };
        let handle_pointer_leave = move |e: PointerEvent| {
            let left = state
                .try_update_value(|s| match s.as_mut() {
                    Some(s) if e.pointer_id() == s.pointer_id && s.is_over_target => {
                        s.is_over_target = false;
                        true
                    }
                    _ => false,
                })
                .unwrap_or(false);
            if left {
                trigger_press_end(&EventRef::Pointer(&e), false);
                if should_cancel_on_pointer_exit.get_untracked() {
                    cancel(&EventRef::Pointer(&e));
                }
            }
        };

        // Pointer up (on the document): over the element, the click completes the press.
        let handle_pointer_up = move |e: PointerEvent| {
            if e.button() != 0 {
                return;
            }
            // Only the pointer that started the press.
            let Some(target) = state.with_value(|s| {
                s.as_ref()
                    .filter(|s| e.pointer_id() == s.pointer_id)
                    .map(|s| s.target.clone())
            }) else {
                return;
            };

            // DOM containment (like react-aria), not the bounding rect, which is zero-sized for
            // `display: contents` elements.
            let over_target = node_contains(
                Some(target.as_ref()),
                e.target()
                    .as_ref()
                    .and_then(|t| t.dyn_ref::<web_sys::Node>()),
            )
            .unwrap_or(false);
            if !over_target {
                cancel(&EventRef::Pointer(&e));
                return;
            }

            // The click completes the press (`handle_click`), which avoids browser issues when the
            // DOM changes between pointer up and click. iOS and Android fire no click after a long
            // press: then click ourselves after 80 ms, unless a click happened that didn't reach
            // us (a child stopped it), which cancels the press. A capture listener sees every
            // click.
            let clicked = std::rc::Rc::new(std::cell::Cell::new(false));
            let click_listener = target.owner_document().map(|doc| {
                let clicked = std::rc::Rc::clone(&clicked);
                listen_to(&doc, ev::click, true, move |_: MouseEvent| {
                    clicked.set(true);
                })
            });
            let pointer_up = e.clone();
            let click_timeout_handle = set_timeout_with_handle(
                move || {
                    if state.try_with_value(Option::is_none).unwrap_or(true) {
                        return;
                    }
                    if clicked.get() {
                        cancel(&EventRef::Pointer(&pointer_up));
                        return;
                    }
                    // Focus without scrolling, then click (react-aria).
                    focus_element(&target, true);
                    if let Some(html_el) = target.dyn_ref::<HtmlElement>() {
                        html_el.click();
                    }
                },
                Duration::from_millis(80),
            )
            .ok();
            state.update_value(|s| {
                if let Some(s) = s {
                    // Ignore the pointer leave touch devices fire before the click.
                    s.is_over_target = false;
                    s.listeners.extend(click_listener);
                    s.click_timeout_handle = click_timeout_handle;
                }
            });
        };

        // Start a press.
        let handle_pointer_down = move |e: PointerEvent| {
            if e.button() != 0 || !e.current_target_contains_target() {
                return;
            }

            // iOS Safari fires VoiceOver's pointer events with wrong coordinates and targets: the
            // click presses (`handle_click`).
            if is_virtual_pointer_event(&e) {
                saw_virtual_pointer_event.set_value(true);
                return;
            }
            saw_virtual_pointer_event.set_value(false);

            // As react-aria: a press that is already active keeps its listeners (it ends with its
            // own pointer up or click) and stops the event; a disabled element lets it propagate.
            let mut should_stop = !disabled.get_untracked();
            if !disabled.get_untracked()
                && state.with_value(Option::is_none)
                && let Some(current_target) = e.expect_current_target().to_element()
            {
                // On the pressed element (react-aria's `state.target`), which the press restores.
                if !allow_text_selection_on_press.get_untracked() {
                    current_target.disable_text_selection();
                }

                // Release pointer capture to enable pointerleave/pointerenter on touch.
                // By default, the browser captures pointer events to the original target,
                // which prevents these events from firing correctly.
                if let Some(element) = e.expect_target().dyn_ref::<web_sys::Element>()
                    && element.has_pointer_capture(e.pointer_id())
                {
                    let _ = element.release_pointer_capture(e.pointer_id());
                }

                let mut listeners = vec![
                    listen_to(
                        &current_target,
                        ev::pointerenter,
                        false,
                        handle_pointer_enter,
                    ),
                    listen_to(
                        &current_target,
                        ev::pointerleave,
                        false,
                        handle_pointer_leave,
                    ),
                ];
                if let Some(doc) = current_target.owner_document() {
                    listeners.extend([
                        listen_to(&doc, ev::pointerup, false, handle_pointer_up),
                        listen_to(&doc, ev::pointercancel, false, move |e: PointerEvent| {
                            cancel(&EventRef::Pointer(&e));
                        }),
                    ]);
                }
                initialize_press_state(
                    e.pointer_id(),
                    PointerType::of(&e),
                    current_target,
                    true,
                    listeners,
                );
                should_stop = trigger_press_start(&EventRef::Pointer(&e));
            }
            stop_unless_forced(should_stop, &EventRef::Pointer(&e));
        };

        // Safari doesn't fire pointercancel when a drag starts, Chrome and Firefox do (react-aria).
        let handle_dragstart = move |e: DragEvent| {
            if e.current_target_contains_target() {
                cancel(&EventRef::Mouse(&e));
            }
        };

        // Handle native dblclick for on_double_press.
        // By the time dblclick fires, the press state has already been cleared by the second click
        // handler. We use `last_pointer_type` (saved before clearing state) to construct the event.
        let handle_dblclick = move |e: MouseEvent| {
            if on_double_press.is_none() || disabled.get_untracked() {
                return;
            }
            let Some(pointer_type) = last_pointer_type.get_value() else {
                return;
            };
            let Some(target) = e.expect_current_target().to_element() else {
                return;
            };
            let should_stop = fire(
                on_double_press,
                PressEventKind::DoublePress,
                pointer_type,
                &target,
                &EventRef::Mouse(&e),
            );
            stop_unless_forced(should_stop, &EventRef::Mouse(&e));
        };

        // Prevent focus on mousedown when prevent_focus_on_press is enabled.
        let handle_mousedown = move |e: MouseEvent| {
            if !e.current_target_contains_target() || e.button() != 0 {
                return;
            }
            // Keep the focus where it is (react-aria's `preventFocus`, which, unlike
            // `preventDefault`, leaves text selection and dragging alone), until the press ends.
            if prevent_focus_on_press.get_untracked()
                && let Some(prevention) =
                    prevent_focus(e.target().and_then(|target| target.dyn_into().ok()))
            {
                focus_preventions.update_value(|preventions| preventions.push(prevention));
            }
            if !force_propagation {
                e.stop_propagation();
            }
        };

        // A pointer up over the element without a press of its own (react-aria's element
        // `onPointerUp`), e.g. a drag ending here: press up only.
        let handle_element_pointer_up = move |e: PointerEvent| {
            if !e.current_target_contains_target()
                || saw_virtual_pointer_event.get_value()
                || e.button() != 0
                || disabled.get_untracked()
                || state.with_value(Option::is_some)
            {
                return;
            }
            let Some(target) = e.expect_current_target().to_element() else {
                return;
            };
            // Not stopped: react-aria ignores whether this press up continued.
            fire(
                on_press_up,
                PressEventKind::PressUp,
                PointerType::of(&e),
                &target,
                &EventRef::Pointer(&e),
            );
        };

        // The long press description (react-aria's `useDescription` in `useLongPress`): a hidden
        // element referenced by id once mounted (the server renders none, so hydration agrees),
        // only with an `on_long_press` handler, and not while disabled.
        let aria_describedby = match long_press.filter(|lp| lp.on_long_press.is_some()) {
            Some(long_press) => use_description(Signal::derive(move || {
                long_press
                    .accessibility_description
                    .get()
                    .filter(|_| !disabled.get() && !long_press.is_disabled.get())
            })),
            None => Signal::stored(None),
        };

        // Cleanup on unmount: restore text selection, remove listeners, and clear timeouts.
        on_cleanup(move || {
            long_press_listeners.try_update_value(Vec::clear);
            if let Some(Some(s)) = state.try_update_value(Option::take) {
                if let Some(handle) = s.click_timeout_handle {
                    handle.clear();
                }
                if let Some(handle) = s.long_press_timeout_handle {
                    handle.clear();
                }
                if !allow_text_selection_on_press.get_untracked() {
                    s.target.restore_text_selection();
                }
            }
            for prevention in focus_preventions
                .try_update_value(std::mem::take)
                .unwrap_or_default()
            {
                prevention.dispose();
            }
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
                    on_dblclick: if on_double_press.is_some() {
                        EventHandler::new(handle_dblclick)
                    } else {
                        EventHandler::empty()
                    },
                    aria_describedby,
                },
                press_styles(),
            ),
            is_pressed: Signal::derive(move || is_pressed.get() || force_is_pressed.get()),
        }
    }
}

/// A `keyup` with the key, code, location and modifiers of `event` (a key down), as macOS would
/// have fired it (react-aria: `new KeyboardEvent('keyup', event)`).
#[cfg(not(feature = "ssr"))]
fn key_up_like(event: &KeyboardEvent) -> Option<KeyboardEvent> {
    let init = web_sys::KeyboardEventInit::new();
    init.set_key(&event.key());
    init.set_code(&event.code());
    init.set_location(event.location());
    init.set_ctrl_key(event.ctrl_key());
    init.set_shift_key(event.shift_key());
    init.set_alt_key(event.alt_key());
    init.set_meta_key(event.meta_key());
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_composed(true);
    KeyboardEvent::new_with_keyboard_event_init_dict("keyup", &init).ok()
}

/// Tests whether a keyboard event's default action should be prevented when `key` was pressed.
#[cfg(not(feature = "ssr"))]
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

    // The `type` property: the attribute normalized (lowercase, missing or invalid: `submit`).
    if let Some(button) = element.dyn_ref::<HtmlButtonElement>() {
        return !matches!(button.type_().as_str(), "submit" | "reset");
    }

    !element.is_anchor_link()
}

#[cfg(not(feature = "ssr"))]
const NON_TEXT_INPUT_TYPES: [&str; 9] = [
    "checkbox", "radio", "range", "color", "file", "image", "button", "submit", "reset",
];

/// Whether `key` presses the input: Space a checkbox or radio, Enter and Space other non-text
/// inputs; never a text input (whose keys type). The `type` property: the attribute normalized
/// (lowercase, missing or invalid: `text`).
#[cfg(not(feature = "ssr"))]
fn is_valid_input_key(element: &HtmlInputElement, key: &KeyboardKey) -> bool {
    match element.type_().as_str() {
        "checkbox" | "radio" => *key == KeyboardKey::Space,
        other => NON_TEXT_INPUT_TYPES.contains(&other),
    }
}

/// Accessibility for keyboards. Space and Enter only.
#[cfg(not(feature = "ssr"))]
fn is_valid_keyboard_event(e: &KeyboardEvent, current_target: &web_sys::Element) -> bool {
    let key = e.typed_key();
    let resembles_press =
        matches!(key, KeyboardKey::Enter | KeyboardKey::Space) || e.code() == "Space";
    if !resembles_press {
        return false;
    }

    let is_input = current_target.is_instance_of::<HtmlInputElement>();
    let is_text_area = current_target.is_instance_of::<HtmlTextAreaElement>();
    let is_content_editable = current_target
        .dyn_ref::<HtmlElement>()
        .is_some_and(HtmlElement::is_content_editable);
    // Role-aware link detection: an `<a href role="button">` is a button, not a link (react-aria:
    // `role === 'link' || (!role && isHTMLAnchorLink(element))`).
    let role = current_target.get_attribute("role");
    let is_link =
        role.as_deref() == Some("link") || (role.is_none() && current_target.is_anchor_link());

    // Links should only trigger with Enter key
    !(is_text_area
        || is_content_editable
        || is_input
            && !is_valid_input_key(current_target.unchecked_ref::<HtmlInputElement>(), &key)
        || is_link && key != KeyboardKey::Enter)
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    fn group(threshold_ms: u64, description: &str, is_disabled: RwSignal<bool>) -> LongPress {
        LongPress {
            on_long_press: Some(Callback::new(|_: LongPressEvent| {})),
            threshold: Signal::stored(Duration::from_millis(threshold_ms)),
            accessibility_description: MaybeProp::from(description.to_owned()),
            is_disabled: is_disabled.into(),
            ..LongPress::default()
        }
    }

    /// A disabled group's threshold and description don't apply; the merged group is disabled
    /// only while both are.
    #[test]
    fn merged_long_press_uses_the_enabled_group() {
        with_owner(|| {
            let first_disabled = RwSignal::new(false);
            let second_disabled = RwSignal::new(true);
            let merged = merge_long_press(
                Some(group(300, "first", first_disabled)),
                Some(group(700, "second", second_disabled)),
            )
            .unwrap();
            assert_that!(merged.threshold.get()).is_equal_to(Duration::from_millis(300));
            assert_that!(merged.accessibility_description.get()).is_equal_to(Some("first".into()));
            assert_that!(merged.is_disabled.get()).is_false();

            second_disabled.set(false);
            assert_that!(merged.threshold.get()).is_equal_to(Duration::from_millis(700));
            assert_that!(merged.accessibility_description.get()).is_equal_to(Some("second".into()));

            first_disabled.set(true);
            second_disabled.set(true);
            assert_that!(merged.accessibility_description.get()).is_none();
            assert_that!(merged.is_disabled.get()).is_true();
        });
    }
}
