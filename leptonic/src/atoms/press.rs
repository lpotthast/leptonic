// Upstream: react-aria/src/interactions/PressResponder.tsx @ 99e6102368
// Upstream: react-aria/src/interactions/Pressable.tsx @ 99e6102368
use leptos::prelude::*;

use crate::{
    atoms::focusable::{ChildKind, focusable_child_attrs, manage_child},
    hooks::*,
    utils::{keyboard_shortcut::KeyboardShortcuts, scoped_context::scoped_view},
};

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
pub fn Pressable<V>(
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_start: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_end: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_up: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_change: Option<Callback<bool>>,
    /// The pressable element.
    children: TypedChildren<V>,
) -> impl IntoView
where
    V: IntoView + 'static,
{
    use leptos::{ev, tachys::html::style::style};

    let press = use_press(UsePressInput {
        is_disabled,
        on_press,
        on_press_up,
        on_press_start,
        on_press_end,
        on_press_change,
        ..UsePressInput::default()
    });
    let (press, _) = press.props.into_inner();
    let element = crate::utils::CapturedElement::new();
    // An overlay trigger's props from a surrounding `PressResponder` (a `DialogTrigger`'s), as
    // react-aria's `usePress` merges them: the ARIA attributes, and the element to position at.
    let trigger = use_context::<PressResponderContext>()
        .and_then(|ctx| ctx.trigger)
        .unwrap_or_else(PressResponderTrigger::empty);
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
            trigger.element.attr(),
            leptos::attr::Attr(leptos::attr::AriaHaspopup, trigger.aria_haspopup),
            leptos::attr::Attr(leptos::attr::AriaExpanded, trigger.aria_expanded),
            leptos::attr::Attr(leptos::attr::AriaControls, trigger.aria_controls),
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
    #[prop(into, optional)] on_long_press_start: Option<Callback<LongPressEvent>>,
    #[prop(into, optional)] on_long_press: Option<Callback<LongPressEvent>>,
    #[prop(into, optional)] on_long_press_end: Option<Callback<LongPressEvent>>,
    /// Describes the long-press action to assistive technology.
    #[prop(into, optional)]
    long_press_accessibility_description: MaybeProp<String>,
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
    on_context_menu: Option<Callback<crate::hooks::ContextMenuEvent>>,
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

    let on_long_press_start = chain_optional_callbacks(
        parent_ctx.as_ref().and_then(|c| c.on_long_press_start),
        on_long_press_start,
    );
    let on_long_press = chain_optional_callbacks(
        parent_ctx.as_ref().and_then(|c| c.on_long_press),
        on_long_press,
    );
    let on_long_press_end = chain_optional_callbacks(
        parent_ctx.as_ref().and_then(|c| c.on_long_press_end),
        on_long_press_end,
    );

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
        on_long_press_start,
        on_long_press,
        on_long_press_end,
        long_press_accessibility_description: {
            let parent = parent_ctx
                .as_ref()
                .map(|c| c.long_press_accessibility_description)
                .unwrap_or_default();
            MaybeProp::derive(move || {
                long_press_accessibility_description
                    .get()
                    .or_else(|| parent.get())
            })
        },
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
