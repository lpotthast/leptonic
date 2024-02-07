use std::fmt::{Display, Formatter};

use leptos::*;
use leptos_router::{State, ToHref, A};

use crate::{
    hooks::{
        button::{use_button, InitialButtonProps},
        focus::{use_focus, UseFocusOptions},
        press::{use_press, PressEvent, UsePressOptions},
    },
    prelude::Consumer,
    utils::aria::{AriaExpanded, AriaHasPopup},
    OptionalMaybeSignal,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Flat,
    Outlined,
    #[default]
    Filled,
}

impl ButtonVariant {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Flat => "flat",
            Self::Outlined => "outlined",
            Self::Filled => "filled",
        }
    }
}

impl Display for ButtonVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonColor {
    #[default]
    Primary,
    Secondary,
    Success,
    Info,
    Warn,
    Danger,
}

impl ButtonColor {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Success => "success",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Danger => "danger",
        }
    }
}

impl Display for ButtonColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    #[default]
    Normal,
    Big,
}

impl ButtonSize {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Normal => "normal",
            Self::Big => "big",
        }
    }
}

impl Display for ButtonSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[component]
pub fn Button(
    #[prop(into)] on_press: Consumer<PressEvent>,
    #[prop(into, optional)] variant: OptionalMaybeSignal<ButtonVariant>,
    #[prop(into, optional)] color: OptionalMaybeSignal<ButtonColor>,
    #[prop(into, optional)] size: OptionalMaybeSignal<ButtonSize>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] active: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionalMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] aria_haspopup: OptionalMaybeSignal<AriaHasPopup>,
    #[prop(into, optional)] aria_expanded: OptionalMaybeSignal<AriaExpanded>,
    children: Children,
) -> impl IntoView {
    let el: NodeRef<html::Button> = create_node_ref();

    let btn = use_button(InitialButtonProps {
        node_ref: el,
        disabled: disabled.or(false),
        aria_haspopup: aria_haspopup.or_default(),
        aria_expanded: aria_expanded.or_default(),
    });

    let focus = use_focus(UseFocusOptions {
        disabled: disabled.or(false),
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    let press = use_press(UsePressOptions {
        on_press: Callback::new(move |e| {
            if !disabled.get_untracked() {
                //e.stop_propagation();
                on_press.consume(e);
            }
        }),
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
    });

    view! {
        <button
            {..btn.props}
            {..press.props.attrs}
            node_ref=el
            id=id
            class=move || class.0.as_ref().map(|it| format!("{} leptonic-btn", it.get())).unwrap_or_else(|| "leptonic-btn".to_string())
            class:active=move || active.get()
            style=style
            data-variant=move || variant.get().as_str()
            data-color=move || color.get().as_str()
            data-size=move || size.get().as_str()
            on:keydown=press.props.on_key_down
            on:click=press.props.on_click
            on:touchstart=press.props.on_touch_start
            on:touchmove=press.props.on_touch_move
            on:touchend=press.props.on_touch_end
            on:focus=focus.on_focus
            on:blur=focus.on_blur
        >
            { children() }
        </button>
    }
}

#[component]
pub fn ButtonGroup(children: Children) -> impl IntoView {
    view! {
        <leptonic-btn-group>
            { children() }
        </leptonic-btn-group>
    }
}

#[component]
pub fn ButtonWrapper(children: Children) -> impl IntoView {
    view! {
        <leptonic-btn-wrapper>
            { children() }
        </leptonic-btn-wrapper>
    }
}

#[component]
#[allow(clippy::needless_pass_by_value)] // title: Option<AttributeValue>
pub fn LinkButton<H>(
    href: H,
    #[prop(into, optional)] variant: OptionalMaybeSignal<ButtonVariant>,
    #[prop(into, optional)] color: OptionalMaybeSignal<ButtonColor>,
    #[prop(into, optional)] size: OptionalMaybeSignal<ButtonSize>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] active: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionalMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[allow(unused)] // TODO: Remove this when leptos's A component supports the title attribute.
    #[prop(into, optional)]
    title: Option<AttributeValue>, // TODO: This should be limited to string attributes...
    /// If `true`, the link is marked active when the location matches exactly;
    /// if false, link is marked active if the current route starts with it.
    #[prop(optional)]
    exact: bool,
    /// An object of any type that will be pushed to router state
    #[prop(optional)]
    state: Option<State>,
    /// If `true`, the link will not add to the browser's history (so, pressing `Back`
    /// will skip this page.)
    #[prop(optional)]
    replace: bool,
    children: Children,
) -> impl IntoView
where
    H: ToHref + 'static,
{
    view! {
        <leptonic-link
            id=id
            class=move || {
                let user = class.get();
                let active = active.get().then_some("active").unwrap_or_default();
                format!("leptonic-btn {user} {active}")
            }
            data-variant=move || variant.get().as_str()
            data-color=move || color.get().as_str()
            data-size=move || size.get().as_str()
            aria-disabled=move || match disabled.get() {
                true => "true",
                false => "false",
            }
            style=style
        >
            <A href=href exact=exact state=state.unwrap_or_default() replace=replace>
                <div class="name">
                    { children() }
                </div>
            </A>
        </leptonic-link>
    }
}
