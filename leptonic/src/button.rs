use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

use leptos::{ev::MouseEvent, *};
use leptos_icons::BsIcon;

use crate::{icon::Icon, OptionalMaybeSignal};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Flat,
    Outlined,
    #[default]
    Filled,
}

impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonVariant::Flat => "flat",
            ButtonVariant::Outlined => "outlined",
            ButtonVariant::Filled => "filled",
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
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonColor::Primary => "primary",
            ButtonColor::Secondary => "secondary",
            ButtonColor::Success => "success",
            ButtonColor::Info => "info",
            ButtonColor::Warn => "warn",
            ButtonColor::Danger => "danger",
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
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Small => "small",
            ButtonSize::Normal => "normal",
            ButtonSize::Big => "big",
        }
    }
}

impl Display for ButtonSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[component]
pub fn Button<F>(
    cx: Scope,
    on_click: F,
    #[prop(into, optional)] variant: OptionalMaybeSignal<ButtonVariant>,
    #[prop(into, optional)] color: OptionalMaybeSignal<ButtonColor>,
    #[prop(into, optional)] size: OptionalMaybeSignal<ButtonSize>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] active: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] variations: OptionalMaybeSignal<View>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionalMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let (dropdown_open, set_dropdown_open): (ReadSignal<bool>, WriteSignal<bool>) =
        create_signal(cx, false);

    let has_variations = variations.0.as_ref().is_some();

    let variations = move || {
        if has_variations {
            Some(
                view! {cx,
                    <div class="dropdown-trigger" on:click=move |_| {
                        if !disabled.get_untracked() {
                            set_dropdown_open.update(|it| *it = !*it);
                        }
                    }>
                        { move || {
                            let icon = match dropdown_open.get() {
                                true => BsIcon::BsCaretUp,
                                false => BsIcon::BsCaretDown,
                            };
                            view! {cx,
                                <Icon icon=icon/>
                            }
                        }}
                    </div>

                    <div class="dropdown" class:active=move || dropdown_open.get() && !disabled.get()>
                        { variations.get() }
                    </div>
                }
                .into_view(cx),
            )
        } else {
            None
        }
    };

    view! { cx,
        <button
            id=id
            class=move || class.0.as_ref().map(|it| Cow::Owned(format!("{} leptonic-btn", it.get()))).unwrap_or(Cow::Borrowed("leptonic-btn"))
            class:has-variations=has_variations
            class:active=move || active.get()
            variant=move || variant.get().as_str()
            color=move || color.get().as_str()
            size=move || size.get().as_str()
            style=style
            aria-disabled=move || disabled.get()
            on:click=move |e| {
                if !disabled.get_untracked() {
                    on_click(e);
                }
            }
        >
            <div class="name">
                { children(cx) }
            </div>

            { variations }
        </button>
    }
}

#[component]
pub fn ButtonGroup(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <leptonic-btn-group>
            { children(cx) }
        </leptonic-btn-group>
    }
}

#[component]
pub fn ButtonWrapper(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <leptonic-btn-wrapper>
            { children(cx) }
        </leptonic-btn-wrapper>
    }
}
