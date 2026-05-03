use leptos::prelude::*;

use crate::{
    hooks::*,
    utils::{classes::Classes, styles::Styles},
};

#[component]
pub fn Pressable(
    #[prop(into)] disabled: Signal<bool>,
    on_press: Callback<PressEvent>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let UsePressReturn {
        props: press_props,
        is_pressed: _,
    } = use_press(UsePressInput {
        disabled,
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press,
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
        on_press_change: None,
        on_double_press: None,
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });

    let (press_attrs, press_styles) = press_props.into_parts();
    let styles = press_styles.merge(styles).add("display", "contents");

    view! {
        <div
            {..press_attrs}
            attr:data-pressable="true"
            class=classes
            style=styles
        >
            {children()}
        </div>
    }
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
    #[prop(into, optional)] disabled: Option<Signal<bool>>,
    #[prop(into, optional)] force_is_pressed: Option<Signal<bool>>,
    #[prop(optional)] prevent_focus_on_press: Option<bool>,
    #[prop(optional)] should_cancel_on_pointer_exit: Option<bool>,
    #[prop(optional)] allow_text_selection_on_press: Option<bool>,
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

    let registered = StoredValue::new(false);

    // Register with parent that we contain a PressResponder chain.
    if let Some(ref parent_ctx) = parent_ctx {
        parent_ctx.registered.set_value(true);
    }

    provide_context(PressResponderContext {
        on_press,
        on_press_start,
        on_press_end,
        on_press_up,
        on_press_change,
        disabled: disabled.or(parent_ctx.as_ref().and_then(|c| c.disabled)),
        force_is_pressed: force_is_pressed.or(parent_ctx.as_ref().and_then(|c| c.force_is_pressed)),
        prevent_focus_on_press: prevent_focus_on_press
            .or(parent_ctx.as_ref().and_then(|c| c.prevent_focus_on_press)),
        should_cancel_on_pointer_exit: should_cancel_on_pointer_exit.or(parent_ctx
            .as_ref()
            .and_then(|c| c.should_cancel_on_pointer_exit)),
        allow_text_selection_on_press: allow_text_selection_on_press.or(parent_ctx
            .as_ref()
            .and_then(|c| c.allow_text_selection_on_press)),
        registered,
    });

    children()
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
    provide_context(PressResponderContext::empty());
    children()
}
