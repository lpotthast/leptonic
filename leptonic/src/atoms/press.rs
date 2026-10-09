// Upstream: react-aria/src/interactions/PressResponder.tsx @ 99e6102368
// Upstream: react-aria/src/interactions/Pressable.tsx @ 99e6102368
// Upstream: react-aria/test/interactions/Pressable.test.js @ 99e6102368
// Upstream: react-aria/test/interactions/PressResponder.test.js @ 99e6102368
use leptos::prelude::*;

use crate::{
    atoms::focusable::{ChildKind, focusable_child_attrs, manage_child},
    hooks::{
        focus::{FocusableContext, UseFocusableInput, use_focusable},
        interactions::{
            LongPress, PressEvent, PressPropagation, PressResponderContext, PressResponderTrigger,
            UsePressInput, chain_optional_callbacks, merge_long_press, use_press,
        },
    },
    utils::{keyboard_shortcut::KeyboardShortcuts, scoped_context::scoped_view},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `PressResponder` takes long presses as one `long_press` group (see `use_press`); it also
//   carries an overlay trigger's props (`trigger`), keyboard shortcuts and a context menu handler
//   for its pressable child, which `use_button` applies (react-aria-components merges these
//   props into the child through the same context).
//
// ## DIFFERENT BEHAVIOR
// - `Pressable`'s child is an element by construction (`TypedChildren`), so there is no "must
//   forward its ref" error; a child without an interactive role is warned about in debug builds,
//   as upstream. A child without a `tabindex` is made focusable instead of warned about.
//
// ## ADDITIONS
// - `Pressable` takes `use_press`' additions: `on_double_press` and `long_press`.
//
// =============================================================================

/// Makes its child element pressable (react-aria's `Pressable`): the press and focus handlers go
/// onto the child itself, which becomes focusable and must have an interactive role (a `<button>`,
/// or a `<span role="button">`). A surrounding [`PressResponder`] applies to it.
///
/// ```ignore
/// <Pressable on_press=move |_| log!("pressed")>
///     <span role="button">"Press me"</span>
/// </Pressable>
/// ```
#[component]
#[allow(clippy::too_many_arguments)]
pub fn Pressable<V>(
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Keeps the focus where it is when the child is pressed.
    #[prop(into, optional)]
    prevent_focus_on_press: Signal<bool>,
    /// Cancels the press when the pointer leaves the child (instead of resuming it when the
    /// pointer comes back).
    #[prop(into, optional)]
    should_cancel_on_pointer_exit: Signal<bool>,
    /// Keeps text selectable while the child is pressed.
    #[prop(into, optional)]
    allow_text_selection_on_press: Signal<bool>,
    /// Shows the child pressed while `true` (react-aria's `isPressed`).
    #[prop(into, optional)]
    force_is_pressed: Signal<bool>,
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_start: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_end: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_up: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_change: Option<Callback<bool>>,
    /// Called when the child is double-clicked.
    #[prop(into, optional)]
    on_double_press: Option<Callback<PressEvent>>,
    /// Long press handling (see `use_press`).
    #[prop(optional)]
    long_press: Option<LongPress>,
    /// The pressable element.
    children: TypedChildren<V>,
) -> impl IntoView
where
    V: IntoView + 'static,
{
    use leptos::{ev, tachys::html::style::style};

    let press = use_press(UsePressInput {
        is_disabled,
        propagation: PressPropagation::Stop,
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
    });
    let (press, _) = press.props.into_inner();
    let element = crate::CapturedElement::new();
    // An overlay trigger's props from a surrounding `PressResponder` (a `DialogTrigger`'s), as
    // react-aria's `usePress` merges them: the ARIA attributes, and the element to position at.
    let trigger = use_context::<PressResponderContext>().and_then(|ctx| ctx.trigger);
    if let Some(trigger) = trigger {
        trigger.sync_id_once_mounted(element);
    }
    let focusable = use_focusable(UseFocusableInput {
        is_disabled,
        ..UseFocusableInput::default()
    })
    .props;

    manage_child(ChildKind::Pressable, element, focusable.tabindex, false);
    let (focusable, on_keydown) =
        focusable_child_attrs(focusable, element, Some(press.aria_describedby));

    children.into_inner()().add_any_attr((
        (
            focusable,
            trigger
                .map_or_else(crate::CapturedElement::new, |trigger| trigger.element)
                .attr(),
            leptos::attr::Attr(
                leptos::attr::AriaHaspopup,
                Signal::derive(move || trigger.and_then(|trigger| trigger.aria_haspopup.get())),
            ),
            leptos::attr::Attr(
                leptos::attr::AriaExpanded,
                Signal::derive(move || trigger.and_then(|trigger| trigger.aria_expanded.get())),
            ),
            leptos::attr::Attr(
                leptos::attr::AriaControls,
                Signal::derive(move || trigger.and_then(|trigger| trigger.aria_controls.get())),
            ),
            // Keyboard handlers before press handling, as `use_button`.
            on_keydown.chain(press.on_keydown).into_on(ev::keydown),
        ),
        (
            press.on_click.into_on(ev::click),
            press.on_pointerdown.into_on(ev::pointerdown),
            press.on_pointerup.into_on(ev::pointerup),
            press.on_mousedown.into_on(ev::mousedown),
            press.on_dragstart.into_on(ev::dragstart),
            press.on_dblclick.into_on(ev::dblclick),
            // One property, so the child's own styles stay (`use_press`' touch action).
            style(("touch-action", "pan-x pan-y pinch-zoom")),
        ),
    ))
}

/// Provides [`PressResponderContext`] to descendant pressable elements.
///
/// Parent components wrap their children in `<PressResponder>` to inject press behavior
/// (callbacks, disabled state, etc.) into any descendant that calls [`use_press`].
/// The descendant does not need to know about the parent — context merging happens
/// automatically inside [`use_press`].
///
/// # Nesting
///
/// `PressResponder` supports nesting. When a parent `PressResponderContext` already exists,
/// callbacks are chained (outer first, then inner). Config fields use inner-wins semantics
/// (inner `Some` takes precedence, falls back to parent).
///
/// # Example
///
/// ```ignore
/// <PressResponder
///     on_press=Callback::new(|_| { /* parent handler fires first */ })
///     force_is_pressed=is_open
/// >
///     <Button on_press=Callback::new(|_| { /* child handler fires second */ })>
///         "Click me"
///     </Button>
/// </PressResponder>
/// ```
#[component]
pub fn PressResponder(
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_start: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_end: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_up: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_change: Option<Callback<bool>>,
    /// Long press handling for the pressable child, merged with its own (see `use_press`).
    #[prop(optional)]
    long_press: Option<LongPress>,
    #[prop(into, optional)] is_disabled: Option<Signal<bool>>,
    #[prop(into, optional)] force_is_pressed: Option<Signal<bool>>,
    #[prop(into, optional)] prevent_focus_on_press: Option<Signal<bool>>,
    #[prop(into, optional)] should_cancel_on_pointer_exit: Option<Signal<bool>>,
    #[prop(into, optional)] allow_text_selection_on_press: Option<Signal<bool>>,
    /// The props of an overlay trigger for the pressable element (see `DialogTrigger`).
    #[prop(optional)]
    trigger: Option<PressResponderTrigger>,
    /// Keyboard shortcuts for the pressable element, handled after its own (see `MenuTrigger`).
    #[prop(optional)]
    shortcuts: Option<KeyboardShortcuts>,
    /// Called when the pressable element requests a context menu (see `MenuTrigger`).
    #[prop(into, optional)]
    on_context_menu: Option<Callback<crate::hooks::interactions::ContextMenuEvent>>,
    children: Children,
) -> impl IntoView {
    // Nesting: read parent context and merge (parent callbacks chain before ours).
    let parent_ctx = use_context::<PressResponderContext>();

    let on_press = chain_optional_callbacks(parent_ctx.as_ref().and_then(|c| c.on_press), on_press);
    let on_press_start = chain_optional_callbacks(
        parent_ctx.as_ref().and_then(|c| c.on_press_start),
        on_press_start,
    );
    let on_press_end = chain_optional_callbacks(
        parent_ctx.as_ref().and_then(|c| c.on_press_end),
        on_press_end,
    );
    let on_press_up =
        chain_optional_callbacks(parent_ctx.as_ref().and_then(|c| c.on_press_up), on_press_up);
    let on_press_change = chain_optional_callbacks(
        parent_ctx.as_ref().and_then(|c| c.on_press_change),
        on_press_change,
    );

    let long_press = merge_long_press(parent_ctx.as_ref().and_then(|c| c.long_press), long_press);

    let registered = StoredValue::new(false);

    // Register with parent that we contain a PressResponder chain.
    if let Some(ref parent_ctx) = parent_ctx {
        parent_ctx.registered.set_value(true);
    }

    let context = PressResponderContext {
        on_press,
        on_press_start,
        on_press_end,
        on_press_up,
        on_press_change,
        long_press,
        is_disabled: is_disabled.or(parent_ctx.as_ref().and_then(|c| c.is_disabled)),
        force_is_pressed: force_is_pressed.or(parent_ctx.as_ref().and_then(|c| c.force_is_pressed)),
        prevent_focus_on_press: prevent_focus_on_press
            .or(parent_ctx.as_ref().and_then(|c| c.prevent_focus_on_press)),
        should_cancel_on_pointer_exit: should_cancel_on_pointer_exit.or(parent_ctx
            .as_ref()
            .and_then(|c| c.should_cancel_on_pointer_exit)),
        allow_text_selection_on_press: allow_text_selection_on_press.or(parent_ctx
            .as_ref()
            .and_then(|c| c.allow_text_selection_on_press)),
        trigger: trigger.or(parent_ctx.as_ref().and_then(|c| c.trigger)),
        shortcuts: shortcuts
            .map(StoredValue::new)
            .or(parent_ctx.as_ref().and_then(|c| c.shortcuts)),
        on_context_menu: chain_optional_callbacks(
            parent_ctx.as_ref().and_then(|c| c.on_context_menu),
            on_context_menu,
        ),
        registered,
    };

    // Once rendered (client only), something below must have taken the press props
    // (react-aria-components warns likewise).
    Effect::new(move || {
        if !registered.get_value() {
            crate::utils::dev_warn!(
                "A PressResponder was rendered without a pressable child. Either call \
                 use_press (e.g. through a Button) below it, or remove the PressResponder."
            );
        }
    });
    scoped_view(move || provide_context(context), children)
}

/// Clears any ancestor [`PressResponderContext`] for descendant pressable elements.
///
/// Place this inside overlays, popovers, or other boundaries where parent press
/// behavior should not leak into the overlay's content.
///
/// # Example
///
/// ```ignore
/// <PressResponder on_press=parent_handler>
///     <Button>"Trigger"</Button>
///     <ClearPressResponder>
///         <OverlayContent>
///             // This button does NOT inherit parent_handler.
///             <Button on_press=action>"Action"</Button>
///         </OverlayContent>
///     </ClearPressResponder>
/// </PressResponder>
/// ```
#[component]
pub fn ClearPressResponder(children: Children) -> impl IntoView {
    scoped_view(|| provide_context(PressResponderContext::empty()), children)
}

/// An overlay's content (popover, modal, tooltip) is no trigger of what surrounds the overlay: its
/// buttons neither press through the trigger's [`PressResponder`] nor become a tooltip's trigger.
#[component]
pub(crate) fn ClearTriggerContexts(children: Children) -> impl IntoView {
    scoped_view(
        || {
            provide_context(PressResponderContext::empty());
            provide_context(FocusableContext::default());
            // Popovers inside take no defaults of the trigger around this overlay.
            provide_context(None::<super::popover::PopoverDefaults>);
            provide_context(None::<super::popover::SubmenuPopoverContext>);
        },
        children,
    )
}
