use crate::hooks::{use_press, PressEvent, UsePressInput, UsePressReturn};
use leptos::html;
use leptos::prelude::*;
use leptos_router::components::{ToHref, A};

// TODO: Use router state again (leptos_router::location::State) (accepting a prop)
#[component]
#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::type_complexity)]
pub fn Link<H>(
    /// Used to calculate the link's `href` attribute. Will be resolved relative
    /// to the current route.
    href: H,

    /// If `true`, the link is marked active when the location matches exactly;
    /// if false, link is marked active if the current route starts with it.
    #[prop(optional)]
    exact: bool,

    children: Children,

    #[prop(into, optional)] on_press: Option<
        Callback<(PressEvent, NodeRef<html::Custom<&'static str>>)>,
    >,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    let el: NodeRef<html::Custom<&str>> = NodeRef::new();

    // We make links "use_press", so that optional PressResponder's higher up the component tree can react on link interactions
    // and so that a custom `on_press` handler can immediately work with the underlying link element.
    let UsePressReturn {
        props,
        is_pressed: _,
    } = use_press(UsePressInput {
        // Links cannot be disabled (for now).
        disabled: false.into(),
        force_prevent_default: false,
        // Without setting this, Leptos' client-side navigation would not take place.
        allow_propagation: true,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        on_press: Callback::new(move |e: PressEvent| {
            if let Some(on_press) = on_press {
                on_press.run((e, el));
            }
        }),
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
        on_press_change: None,
        on_double_press: None,
    });

    // TODO: propagate missing A props
    // TODO: do not wrap A, make this an atom
    view! {
        <leptonic-link {..props.into_attrs()} node_ref=el>
            <A href=href exact=exact>
                {children()}
            </A>
        </leptonic-link>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkExtTarget {
    Blank,
    Parent,
    Sel,
    Top,
}

impl std::fmt::Display for LinkExtTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blank => f.write_str("_blank"),
            Self::Parent => f.write_str("_parent"),
            Self::Sel => f.write_str("_self"),
            Self::Top => f.write_str("_top"),
        }
    }
}

#[component]
#[allow(clippy::needless_pass_by_value)] // `H` could be `&H`.
pub fn LinkExt<H>(
    /// Used to calculate the link's `href` attribute.
    href: H,
    target: LinkExtTarget,
    #[prop(into, optional)] disabled: Signal<bool>,
    // TODO: Impl this prop
    // /// If `true`, the link will not add to the browser's history (so, pressing `Back`
    // /// will skip this page.)
    // #[prop(optional)]
    // replace: bool,
    children: Children,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    // NOTE(lukas): rel="noopener" is added for security reasons. See: https://developer.chrome.com/docs/lighthouse/best-practices/external-anchors-use-rel-noopener/
    view! {
        <leptonic-link>
            <a
                href=move || href.to_href()()
                target=format!("{target}")
                prop:disabled=move || disabled.get()
                rel=match target {
                    LinkExtTarget::Blank => Some("noopener"),
                    _ => None,
                }
            >
                {children()}
            </a>
        </leptonic-link>
    }
}
