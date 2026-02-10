use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{KeyboardEvent, MouseEvent};

use crate::utils::aria::{AriaCurrent, AriaDisabled};
use crate::utils::EventHandler;

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
    pub item_props: UseBreadcrumbItemProps,

    /// Props for the link/span element.
    pub link_props: UseBreadcrumbLinkProps,

    /// Whether this is the current item.
    pub is_current: bool,
}

/// Props from `use_breadcrumb_item` for the item container element.
///
/// Currently empty — CSS class should be applied directly in the component.
#[derive(Debug, Clone)]
pub struct UseBreadcrumbItemProps;

impl UseBreadcrumbItemProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseBreadcrumbItemAttrs {
        ()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseBreadcrumbItemAttrs {
        ()
    }
}

/// Attributes for the breadcrumb item container.
pub type UseBreadcrumbItemAttrs = ();

/// Props from `use_breadcrumb_item` for the link element that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseBreadcrumbLinkProps {
    pub href: Option<String>,
    pub aria_current: Option<AriaCurrent>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub tabindex: &'static str,
    pub on_click: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl UseBreadcrumbLinkProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseBreadcrumbLinkAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseBreadcrumbLinkAttrs {
        (
            Attr(attr::Href, self.href),
            Attr(attr::AriaCurrent, self.aria_current),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::Tabindex, self.tabindex),
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

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
///     <li {..item.item_props.into_attrs()}>
///         <a {..item.link_props.into_attrs()}>"Home"</a>
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
        item_props: UseBreadcrumbItemProps,
        link_props: UseBreadcrumbLinkProps {
            href: if is_current { None } else { href },
            aria_current,
            aria_disabled,
            tabindex,
            on_click: EventHandler::new(handle_click),
            on_keydown: EventHandler::new(handle_keydown),
        },
        is_current,
    }
}
