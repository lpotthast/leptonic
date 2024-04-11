use leptos::*;
use leptos_router::AProps;
use web_sys::FocusEvent;

use crate::{
    hooks::*,
    utils::aria::{AriaExpanded, AriaHasPopup},
    OptMaybeSignal, Transparent,
};

use super::AttributeExt;

#[component]
pub fn Button(
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] on_focus: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] disabled: OptMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] aria_haspopup: OptMaybeSignal<AriaHasPopup>,
    #[prop(into, optional)] aria_expanded: OptMaybeSignal<AriaExpanded>,
    /// Arbitrary additional attributes. Can be declared using the `attr:` syntax.
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let el: NodeRef<html::Button> = create_node_ref();

    // TODO: This is extremely ugly, bu necessary when using `strip_option` on our builder.
    // His could be changed when https://github.com/idanarye/rust-typed-builder/issues/117 is resolved.
    let focus_input = UseFocusInput::builder();
    let focus_input = focus_input.disabled(disabled.or(false));
    let focus_input = if let Some(on_focus) = on_focus {
        let focus_input = focus_input.on_focus(on_focus);
        if let Some(on_blur) = on_blur {
            let focus_input = focus_input.on_blur(on_blur);
            if let Some(on_focus_change) = on_focus_change {
                let focus_input = focus_input.on_focus_change(on_focus_change);
                focus_input.build()
            } else {
                focus_input.build()
            }
        } else {
            focus_input.build()
        }
    } else {
        focus_input.build()
    };

    // TODO: This is extremely ugly, but necessary when using `strip_option` on our builder.
    // His could be changed when https://github.com/idanarye/rust-typed-builder/issues/117 is resolved.
    let hover_input = UseHoverInput::builder();
    let hover_input = hover_input.disabled(disabled.or(false));
    let hover_input = if let Some(on_hover_start) = on_hover_start {
        let hover_input = hover_input.on_hover_start(on_hover_start);
        if let Some(on_hover_end) = on_hover_end {
            let hover_input = hover_input.on_hover_end(on_hover_end);
            hover_input.build()
        } else {
            hover_input.build()
        }
    } else {
        hover_input.build()
    };

    // TODO: This is extremely ugly, but necessary when using `strip_option` on our builder.
    // His could be changed when https://github.com/idanarye/rust-typed-builder/issues/117 is resolved.
    let press_input = UsePressInput::builder();
    let press_input = press_input.disabled(disabled.or(false));
    let press_input = if let Some(on_press) = on_press {
        press_input.on_press(on_press).build()
    } else {
        press_input.build()
    };

    let btn = use_button(
        UseButtonInput::builder()
            .node_ref(el)
            .disabled(disabled.or(false))
            .aria_haspopup(aria_haspopup.or_default())
            .aria_expanded(aria_expanded.or_default())
            .use_press_input(press_input)
            .use_focus_input(focus_input)
            .use_hover_input(hover_input)
            .build(),
    );

    let attributes = btn.props.attrs.merge(attributes);

    view! {
        <Transparent>
            <button
                {..attributes}
                {..btn.props.handlers}
                node_ref=el
                id=id
                class=class
                class:leptonic-btn=true
                style=style
            >
                { children() }
            </button>
        </Transparent>
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
    let UseButtonReturn {
        props,
        press_responder: _,
    } = use_button(UseButtonInput {
        node_ref: NodeRef::<html::Custom>::new(),
        disabled: disabled.or(false),
        aria_haspopup: aria_haspopup.or_default(),
        aria_expanded: aria_expanded.or_default(),
        use_press_input: UsePressInput {
            disabled: disabled.or(false),
            force_prevent_default: false,
            on_press: None,
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
        },
        use_hover_input: Some(UseHoverInput {
            disabled: disabled.or(false),
            on_hover_start,
            on_hover_end,
        }),
        use_focus_input: Some(UseFocusInput {
            disabled: disabled.or(false),
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        }),
    });

    let mut attrs = props.attrs.merge(attributes);
    if let Some(style) = style {
        attrs = attrs.insert("style", style.into_attribute_boxed());
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
        attributes: attrs.map.into_iter().collect::<Vec<_>>(),
        children,
    })
    .into_view()
    // TODO(handlers)
    //.bindings(props.handlers)
}

#[component]
pub fn ButtonWrapper(children: Children) -> impl IntoView {
    view! {
        <leptonic-btn-wrapper>
            { children() }
        </leptonic-btn-wrapper>
    }
}
