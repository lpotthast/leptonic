use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{KeyboardEvent, MouseEvent};

use crate::utils::aria::{AriaCurrent, AriaDisabled};

/// Input parameters for the `use_breadcrumb_item` hook.
#[derive(Debug, Clone)]
pub struct UseBreadcrumbItemInput {
    /// The href for the breadcrumb link.
    pub href: Option<String>,

    /// Whether this is the current/last item.
    pub is_current: bool,

    /// Whether the item is disabled.
    pub is_disabled: Signal<bool>,

    /// Callback when the item is pressed.
    pub on_press: Option<Callback<()>>,
}

impl Default for UseBreadcrumbItemInput {
    fn default() -> Self {
        Self {
            href: None,
            is_current: false,
            is_disabled: Signal::derive(|| false),
            on_press: None,
        }
    }
}

/// The return value of the `use_breadcrumb_item` hook.
pub struct UseBreadcrumbItemReturn {
    /// Props for the breadcrumb item container (li).
    pub item_props: UseBreadcrumbItemAttrs,

    /// Props for the link/span element.
    pub link_props: UseBreadcrumbLinkAttrs,

    /// Whether this is the current item.
    pub is_current: bool,
}

/// Attributes for the breadcrumb item container.
/// Note: CSS class should be applied directly in the component.
pub type UseBreadcrumbItemAttrs = ();

/// Attributes for the breadcrumb link element.
pub type UseBreadcrumbLinkAttrs = (
    Attr<attr::Href, Option<String>>,
    Attr<attr::AriaCurrent, Option<AriaCurrent>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::Tabindex, &'static str>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Provides the behavior and accessibility for a breadcrumb item.
///
/// # Example
///
/// ```ignore
/// let item = use_breadcrumb_item(UseBreadcrumbItemInput {
///     href: Some("/home".to_string()),
///     is_current: false,
///     on_press: Some(Callback::new(|_| navigate("/home"))),
///     ..Default::default()
/// });
///
/// view! {
///     <li {..item.item_props}>
///         <a {..item.link_props}>"Home"</a>
///     </li>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_breadcrumb_item(input: UseBreadcrumbItemInput) -> UseBreadcrumbItemReturn {
    let UseBreadcrumbItemInput {
        href,
        is_current,
        is_disabled: disabled,
        on_press,
    } = input;

    let aria_current = is_current.then_some(AriaCurrent::Page);

    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    // Current item doesn't need to be a link
    let tabindex = if is_current { "-1" } else { "0" };

    let handle_click = move |e: MouseEvent| {
        if disabled.get_untracked() || is_current {
            e.prevent_default();
            return;
        }
        if let Some(on_press) = on_press {
            on_press.run(());
        }
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() || is_current {
            return;
        }

        let key = e.key();
        if key == "Enter" || key == " " {
            e.prevent_default();
            if let Some(on_press) = on_press {
                on_press.run(());
            }
        }
    };

    UseBreadcrumbItemReturn {
        item_props: (),
        link_props: (
            Attr(attr::Href, if is_current { None } else { href }),
            Attr(attr::AriaCurrent, aria_current),
            Attr(attr::AriaDisabled, aria_disabled),
            Attr(attr::Tabindex, tabindex),
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
        ),
        is_current,
    }
}
