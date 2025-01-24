use leptos::html;
use leptos::prelude::*;
use leptos_router::components::{AProps, ToHref, A};

use crate::{
    hooks::{
        use_button, HoverEndEvent, HoverStartEvent, PressEvent, UseButtonInput, UseButtonReturn,
        UseFocusInput, UseHoverInput, UsePressInput,
    },
    utils::aria::{AriaExpanded, AriaHasPopup},
};

#[component]
pub fn Button(
    #[prop(into, optional)] on_press: Option<Callback<(PressEvent,)>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<(HoverStartEvent,)>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<(HoverEndEvent,)>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] aria_haspopup: Signal<AriaHasPopup>,
    #[prop(into, optional)] aria_expanded: Signal<AriaExpanded>,
    children: Children,
) -> impl IntoView {
    let el: NodeRef<html::Button> = NodeRef::new();

    let UseButtonReturn {
        attrs,
        is_hovered: _,
        is_pressed: _,
    } = use_button(UseButtonInput {
        node_ref: el,
        disabled,
        aria_haspopup,
        aria_expanded,
        use_press_input: UsePressInput {
            disabled,
            force_prevent_default: false,
            on_press: Callback::new(move |e| match on_press {
                Some(on_press) => on_press.run(e),
                None => {}
            }),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
        },
        use_hover_input: UseHoverInput {
            disabled,
            on_hover_start,
            on_hover_end,
        },
        use_focus_input: UseFocusInput {
            disabled,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },
    });

    view! {
        <button
            node_ref=el
            class="leptonic-btn"
            {..attrs}
        >
            { children() }
        </button>
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum LinkTarget {
    /// Opens the linked document in a new window or tab.
    _Blank,
    /// Opens the linked document in the same frame as it was clicked (this is the default).
    #[default]
    _Self,
    /// Opens the linked document in the parent frame.
    _Parent,
    /// Opens the linked document in the full body of the window.
    _Top,
    /// Opens the linked document in the frame with the given name.
    Frame { with_name: Oco<'static, str> },
}

impl LinkTarget {
    fn to_oco(&self) -> Oco<'static, str> {
        match self {
            LinkTarget::_Blank => Oco::Borrowed("_blank"),
            LinkTarget::_Self => Oco::Borrowed("_self"),
            LinkTarget::_Parent => Oco::Borrowed("_parent"),
            LinkTarget::_Top => Oco::Borrowed("_top"),
            LinkTarget::Frame { with_name } => with_name.clone(),
        }
    }
}

#[component]
pub fn LinkButton<H>(
    href: H,

    /// Where to display the linked URL, as the name for a browsing context (a tab, window, or `<iframe>`).
    #[prop(into, optional)]
    target: Option<LinkTarget>,

    #[prop(into, optional)] on_hover_start: Option<Callback<(HoverStartEvent,)>>,

    #[prop(into, optional)] on_hover_end: Option<Callback<(HoverEndEvent,)>>,

    #[prop(into, optional)] disabled: Option<Signal<bool>>,

    #[prop(into, optional)] aria_haspopup: Option<Signal<AriaHasPopup>>,

    #[prop(into, optional)] aria_expanded: Option<Signal<AriaExpanded>>,

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
        attrs,
        is_hovered: _,
        is_pressed: _,
    } = use_button(UseButtonInput {
        node_ref: NodeRef::<html::Custom<&str>>::new(),
        disabled,
        aria_haspopup: aria_haspopup.unwrap_or_default(),
        aria_expanded: aria_expanded.unwrap_or_default(),
        use_press_input: UsePressInput {
            disabled,
            force_prevent_default: false,
            on_press: Callback::new(move |_e| {}),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
        },
        use_hover_input: UseHoverInput {
            disabled,
            on_hover_start,
            on_hover_end,
        },
        use_focus_input: UseFocusInput {
            disabled,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },
    });

    /*
    let default_class = "leptonic-btn";
    let class: Option<Box<dyn IntoAttribute>> = class
        .map(|c| {
            c.into_attribute_boxed()
                .prepend(Oco::Borrowed(default_class))
        })
        .or_else(|| Some(Attribute::String(Oco::Borrowed(default_class))))
        .and_then(|new| {
            let as_dyn: Box<dyn IntoAttribute> = Box::new(new);
            Some(as_dyn)
        });
     */

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
    .attr("class", "leptonic-btn")
    .add_any_attr(attrs)
}

#[component]
pub fn ButtonWrapper(children: Children) -> impl IntoView {
    view! {
        <leptonic-btn-wrapper>
            { children() }
        </leptonic-btn-wrapper>
    }
}
