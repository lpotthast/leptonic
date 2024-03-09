use leptos::*;
use leptos_router::AProps;

use crate::{
    hooks::{
        button::{use_button, UseButtonInput},
        focus::UseFocusInput,
        hover::{HoverEndEvent, HoverStartEvent},
        prelude::{UseButtonReturn, UseHoverInput},
        press::{PressEvent, UsePressInput},
    },
    utils::aria::{AriaExpanded, AriaHasPopup},
    OptMaybeSignal,
};

use super::AttributeExt;

#[component]
pub fn Button(
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    #[prop(into, optional)] disabled: OptMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] aria_haspopup: OptMaybeSignal<AriaHasPopup>,
    #[prop(into, optional)] aria_expanded: OptMaybeSignal<AriaExpanded>,
    /// Arbitrary additional attributes.
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let el: NodeRef<html::Button> = create_node_ref();

    let UseButtonReturn { props } = use_button(UseButtonInput {
        node_ref: el,
        disabled: disabled.or(false),
        aria_haspopup: aria_haspopup.or_default(),
        aria_expanded: aria_expanded.or_default(),
        use_press_input: UsePressInput {
            disabled: disabled.or(false),
            on_press: Callback::new(move |e| match on_press {
                Some(on_press) => Callback::call(&on_press, e),
                None => {}
            }),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
        },
        use_hover_input: UseHoverInput {
            disabled: disabled.or(false),
            on_hover_start,
            on_hover_end,
        },
        use_focus_input: UseFocusInput {
            disabled: disabled.or(false),
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },
    });

    view! {
        <button
            {..props.attrs}
            {..attributes}
            node_ref=el
            id=id
            class=class
            class:leptonic-btn=true
            style=style
            on:keydown=props.on_key_down
            on:click=props.on_click
            on:pointerdown=props.on_pointer_down
            on:pointerenter=props.on_pointer_enter
            on:pointerleave=props.on_pointer_leave
            on:focus=props.on_focus
            on:blur=props.on_blur
        >
            { children() }
        </button>
    }
}

#[component]
pub fn LinkButton<H>(
    href: H,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    #[prop(into, optional)] disabled: OptMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<Oco<'static, str>>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] aria_haspopup: OptMaybeSignal<AriaHasPopup>,
    #[prop(into, optional)] aria_expanded: OptMaybeSignal<AriaExpanded>,
    /// If `true`, the link is marked active when the location matches exactly;
    /// if false, link is marked active if the current route starts with it.
    #[prop(optional)]
    exact: bool,
    /// An object of any type that will be pushed to router state
    #[prop(optional)]
    state: Option<leptos_router::State>,
    /// If `true`, the link will not add to the browser's history (so, pressing `Back`
    /// will skip this page.)
    #[prop(optional)]
    replace: bool,
    /// Arbitrary additional attributes.
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView
where
    H: leptos_router::ToHref + 'static,
{
    let UseButtonReturn { mut props } = use_button(UseButtonInput {
        node_ref: NodeRef::<html::Custom>::new(),
        disabled: disabled.or(false),
        aria_haspopup: aria_haspopup.or_default(),
        aria_expanded: aria_expanded.or_default(),
        use_press_input: UsePressInput {
            disabled: disabled.or(false),
            on_press: Callback::new(move |_e| {}),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
        },
        use_hover_input: UseHoverInput {
            disabled: disabled.or(false),
            on_hover_start,
            on_hover_end,
        },
        use_focus_input: UseFocusInput {
            disabled: disabled.or(false),
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },
    });

    props.attrs.merge(attributes);
    if let Some(style) = style {
        props.attrs.insert("style", style.into_attribute_boxed());
    }

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

    leptos_router::A(AProps {
        href,
        target: None, // TODO: Propagate this?
        exact,
        active_class: None, // TODO: Propagate this?
        state,
        replace,
        class,
        id,
        attributes: props.attrs.map.into_iter().collect::<Vec<_>>(),
        children,
    })
    .into_view()
    .on(leptos::ev::keydown, props.on_key_down)
    .on(leptos::ev::click, props.on_click)
    .on(leptos::ev::pointerdown, props.on_pointer_down)
    .on(leptos::ev::focus, props.on_focus)
    .on(leptos::ev::blur, props.on_blur)
}

#[component]
pub fn ButtonWrapper(children: Children) -> impl IntoView {
    view! {
        <leptonic-btn-wrapper>
            { children() }
        </leptonic-btn-wrapper>
    }
}
