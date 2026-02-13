use leptos::{prelude::*, tachys::html::class::class};
use leptos_router::components::{AProps, ToHref, A};

use crate::{
    hooks::{LinkTarget, *},
    utils::{
        aria::{AriaExpanded, AriaHasPopup},
        classes::Classes,
    },
};

#[component]
pub fn Button(
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] aria_haspopup: Signal<AriaHasPopup>,
    #[prop(into, optional)] aria_expanded: Signal<Option<AriaExpanded>>,
    #[prop(into, optional)] classes: Classes,
    children: Children,
) -> impl IntoView {
    let UseButtonReturn {
        props,
        is_hovered: _,
        is_pressed: _,
        is_focus_visible: _,
    } = use_button(UseButtonInput {
        disabled,
        aria_haspopup,
        aria_expanded,
        use_press_input: UsePressInput {
            disabled,
            force_prevent_default: false,
            force_propagation: false,
            allow_text_selection_on_press: false,
            should_cancel_on_pointer_exit: false,
            prevent_focus_on_press: false,
            force_is_pressed: None,
            on_press: Callback::new(move |e| {
                if let Some(on_press) = on_press {
                    on_press.run(e);
                }
            }),
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
        },
        use_hover_input: UseHoverInput {
            disabled,
            on_hover_start,
            on_hover_end,
            on_hover_change: None,
        },
        use_focus_ring_input: UseFocusRingInput {
            disabled,
            within: false,
            auto_focus: false,
            is_text_input: false,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },
    });

    view! {
        <button {..props.into_attrs()} class=classes>
            {children()}
        </button>
    }
}

#[component]
pub fn LinkButton<H>(
    href: H,

    /// Where to display the linked URL, as the name for a browsing context (a tab, window, or `<iframe>`).
    #[prop(into, optional)]
    target: Option<LinkTarget>,

    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,

    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,

    #[prop(into, optional)] disabled: Option<Signal<bool>>,

    #[prop(into, optional)] aria_haspopup: Option<Signal<AriaHasPopup>>,

    #[prop(into, optional)] aria_expanded: Option<Signal<Option<AriaExpanded>>>,

    #[prop(into, optional)] classes: Classes,

    /// If `true`, the link is marked active when the location matches exactly;
    /// if false, link is marked active if the current route starts with it.
    #[prop(optional)]
    exact: bool,

    children: Children,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    let disabled = disabled.unwrap_or(Signal::from(false));

    let UseButtonReturn {
        props,
        is_hovered: _,
        is_pressed: _,
        is_focus_visible: _,
    } = use_button(UseButtonInput {
        disabled,
        aria_haspopup: aria_haspopup.unwrap_or_default(),
        aria_expanded: aria_expanded.unwrap_or_default(),
        use_press_input: UsePressInput {
            disabled,
            force_prevent_default: false,
            // Without setting this, Leptos' client-side navigation would not take place.
            force_propagation: true,
            allow_text_selection_on_press: false,
            should_cancel_on_pointer_exit: false,
            prevent_focus_on_press: false,
            force_is_pressed: None,
            on_press: Callback::new(move |_e| {}),
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
        },
        use_hover_input: UseHoverInput {
            disabled,
            on_hover_start,
            on_hover_end,
            on_hover_change: None,
        },
        use_focus_ring_input: UseFocusRingInput {
            disabled,
            within: false,
            auto_focus: false,
            is_text_input: false,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },
    });

    let target: Option<Oco<'static, str>> = Some(target.unwrap_or_default())
        .filter(|it| it != &LinkTarget::_Self)
        .map(|it| it.to_oco());

    // TODO: Propagate scroll and strict_trailing_slash?
    // TODO (new): Does a class in props.attrs override this? Do we need the old "prepend" logic?

    A(AProps {
        href,
        target,
        exact,
        strict_trailing_slash: false,
        scroll: true,
        children,
    })
    .add_any_attr(class(classes))
    .add_any_attr(props.into_attrs())
}

#[component]
pub fn ButtonWrapper(children: Children) -> impl IntoView {
    view! { <div class="leptonic-btn-wrapper">{children()}</div> }
}
